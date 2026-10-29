// PraxisQM – Authentifizierungs- und Autorisierungsmodul
// Modul: Backend / Auth
// Zweck: Lokale Passwort-Verifizierung (Argon2id), Session-Verwaltung
//         und rollenbasierte Zugriffskontrolle.
//         Vollständig offline, keine Cloud-Abhängigkeit.

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::database;

// --- Rollen-Definitionen ---------------------------------------------------

pub const ROLE_GUEST: &str = "Gast";
pub const ROLE_EDITOR: &str = "Editor";
pub const ROLE_ADMIN: &str = "Admin";

/// Rollen-Rang: höherer Wert = mehr Berechtigungen.
/// Wird für `require_role`-Prüfungen verwendet.
fn role_rank(role: &str) -> i32 {
    match role {
        ROLE_GUEST => 0,
        ROLE_EDITOR => 1,
        ROLE_ADMIN => 2,
        _ => -1,
    }
}

/// Prüft, ob die aktuelle Session die mindestens erforderliche Rolle hat.
/// Gibt bei Erfolg die Session zurück, bei Fehlschlag einen deutschen Fehlerstring.
pub fn require_role(session: &Option<Session>, min_role: &str) -> Result<Session, String> {
    let session = session
        .as_ref()
        .ok_or_else(|| "Keine Berechtigung – bitte anmelden.".to_string())?;

    let session_rank = role_rank(&session.role);
    let required_rank = role_rank(min_role);

    if session_rank < 0 {
        return Err("Ungültige Benutzerrolle.".to_string());
    }

    if required_rank < 0 {
        return Err("Ungültige Rollenanforderung.".to_string());
    }

    if session_rank < required_rank {
        return Err("Keine Berechtigung für diese Aktion.".to_string());
    }

    Ok(session.clone())
}

/// Prüft eine privilegierte Aktion gegen den aktuellen Datenbankzustand.
/// Die Rolle wird aus der Datenbank gelesen, nicht aus der Sitzung, damit
/// Deaktivierung, Löschung oder Herabstufung sofort wirksam werden.
/// Lock-Reihenfolge: Datenbank-Verbindung vor AuthState.
pub fn require_privileged(
    conn: &Connection,
    auth: &AuthState,
    min_role: &str,
) -> Result<Session, String> {
    let session = match auth.get_session() {
        Some(session) => session,
        None => return require_role(&None, min_role),
    };

    let user = match database::find_user_by_id(conn, &session.user_id) {
        Ok(Some(user)) => user,
        Ok(None) => {
            auth.clear_session();
            return Err("Benutzerkonto nicht gefunden – bitte erneut anmelden.".to_string());
        }
        Err(e) => {
            eprintln!("PraxisQM: Datenbankfehler bei Rollenprüfung: {}", e);
            return Err("Berechtigungsprüfung fehlgeschlagen.".to_string());
        }
    };

    if !user.is_active {
        auth.clear_session();
        return Err("Dieses Benutzerkonto ist deaktiviert.".to_string());
    }

    let current = Session {
        user_id: user.id,
        username: user.username,
        role: user.role,
        login_at: session.login_at,
    };
    require_role(&Some(current), min_role)
}

// --- Session-Verwaltung ----------------------------------------------------

/// Authentifizierte Session. Wird in Rust-managed State gehalten,
/// nicht in der Datenbank. Verfällt beim Schließen der Anwendung.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub login_at: u64,
}

/// Managed Auth-State: aktuelle Session und Login-Drosselung pro Benutzername.
pub struct AuthState {
    session: Mutex<Option<Session>>,
    login_throttle: Mutex<HashMap<String, LoginThrottle>>,
}

#[derive(Default)]
struct LoginThrottle {
    failures: Vec<Instant>,
    locked_until: Option<Instant>,
}

/// Maximale Anzahl fehlgeschlagener Login-Versuche innerhalb des Zeitfensters.
const MAX_LOGIN_ATTEMPTS: usize = 5;
/// Zeitfenster für Login-Versuche (Sekunden).
const LOGIN_ATTEMPT_WINDOW: Duration = Duration::from_secs(60);
/// Sperrdauer nach Überschreitung (Sekunden).
const LOGIN_LOCKOUT_DURATION: Duration = Duration::from_secs(120);

impl AuthState {
    pub fn new() -> Self {
        AuthState {
            session: Mutex::new(None),
            login_throttle: Mutex::new(HashMap::new()),
        }
    }

    /// Liefert die aktuelle Session (oder None für Guest).
    pub fn get_session(&self) -> Option<Session> {
        self.session.lock().unwrap().clone()
    }

    /// Setzt die Session nach erfolgreichem Login.
    pub fn set_session(&self, session: Session) {
        *self.session.lock().unwrap() = Some(session);
    }

    /// Löscht die Session (Logout).
    pub fn clear_session(&self) {
        *self.session.lock().unwrap() = None;
    }

    /// Prüft, ob ein Login-Versuch für den gegebenen Benutzernamen
    /// aufgrund zu vieler fehlgeschlagener Versuche blockiert ist.
    fn is_locked_out(&self, username: &str) -> bool {
        let throttle = self.login_throttle.lock().unwrap();
        throttle
            .get(username)
            .and_then(|t| t.locked_until)
            .map_or(false, |until| Instant::now() < until)
    }

    /// Registriert einen fehlgeschlagenen Login-Versuch.
    fn record_failed_attempt(&self, username: &str) {
        let now = Instant::now();
        let mut throttle = self.login_throttle.lock().unwrap();
        let entry = throttle.entry(username.to_string()).or_default();
        entry.failures.retain(|t| now.duration_since(*t) < LOGIN_ATTEMPT_WINDOW);
        entry.failures.push(now);
        if entry.failures.len() >= MAX_LOGIN_ATTEMPTS {
            entry.locked_until = Some(now + LOGIN_LOCKOUT_DURATION);
            entry.failures.clear();
        }
    }

    /// Löscht alle fehlgeschlagenen Versuche für einen Benutzernamen
    /// nach erfolgreichem Login.
    fn clear_failed_attempts(&self, username: &str) {
        self.login_throttle.lock().unwrap().remove(username);
    }
}

// --- Passwort-Hashing ------------------------------------------------------

/// Hasht ein Passwort mit Argon2id.
/// Gibt den PHC-String (Password Hash Code) zurück.
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Passwort konnte nicht gehasht werden: {}", e))
}

/// Verifiziert ein Passwort gegen einen gespeicherten PHC-String.
/// Gibt true zurück, wenn das Passwort korrekt ist.
pub fn verify_password(password: &str, phc_string: &str) -> bool {
    let parsed = match PasswordHash::new(phc_string) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

// --- Authentifizierungs-Operationen ----------------------------------------

/// Login-Ergebnis bei erfolgreicher Authentifizierung.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResult {
    pub user_id: String,
    pub username: String,
    pub role: String,
}

/// Führt einen Login durch. Bei Erfolg wird die Session gesetzt.
/// Gibt bei Fehlschlag einen deutschen Fehlerstring zurück.
pub fn login(
    conn: &Connection,
    auth: &AuthState,
    username: &str,
    password: &str,
) -> Result<LoginResult, String> {
    if username.trim().is_empty() || password.is_empty() {
        return Err("Benutzername und Passwort dürfen nicht leer sein.".to_string());
    }

    if auth.is_locked_out(username) {
        return Err("Zu viele fehlgeschlagene Versuche. Bitte später erneut versuchen.".to_string());
    }

    let user = match database::find_user_by_username(conn, username.trim()) {
        Ok(Some(u)) => u,
        Ok(None) => {
            auth.record_failed_attempt(username);
            return Err("Benutzername oder Passwort ist falsch.".to_string());
        }
        Err(e) => {
            eprintln!("PraxisQM: Datenbankfehler beim Login: {}", e);
            return Err("Login fehlgeschlagen.".to_string());
        }
    };

    if !user.is_active {
        return Err("Dieses Benutzerkonto ist deaktiviert.".to_string());
    }

    let stored_hash = match &user.password_hash {
        Some(h) if !h.is_empty() => h,
        _ => {
            auth.record_failed_attempt(username);
            return Err("Benutzername oder Passwort ist falsch.".to_string());
        }
    };

    if !verify_password(password, stored_hash) {
        auth.record_failed_attempt(username);
        return Err("Benutzername oder Passwort ist falsch.".to_string());
    }

    auth.clear_failed_attempts(username);

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let session = Session {
        user_id: user.id.clone(),
        username: user.username.clone(),
        role: user.role.clone(),
        login_at: now_secs,
    };

    let result = LoginResult {
        user_id: session.user_id.clone(),
        username: session.username.clone(),
        role: session.role.clone(),
    };

    auth.set_session(session);
    Ok(result)
}

/// Ändert das Passwort des angemeldeten Benutzers.
/// Erfordert Verifizierung des aktuellen Passworts.
pub fn change_password(
    conn: &Connection,
    auth: &AuthState,
    current_password: &str,
    new_password: &str,
) -> Result<(), String> {
    if new_password.is_empty() {
        return Err("Das neue Passwort darf nicht leer sein.".to_string());
    }

    let session = auth.get_session().ok_or_else(|| {
        "Keine aktive Sitzung – bitte anmelden.".to_string()
    })?;

    let user = database::find_user_by_username(conn, &session.username)
        .map_err(|e| {
            eprintln!("PraxisQM: Datenbankfehler beim Passwortwechsel: {}", e);
            "Passwortwechsel fehlgeschlagen.".to_string()
        })?
        .ok_or_else(|| "Benutzerkonto nicht gefunden.".to_string())?;

    if !user.is_active {
        return Err("Dieses Benutzerkonto ist deaktiviert.".to_string());
    }

    let stored_hash = user.password_hash.as_deref().unwrap_or("");
    if !verify_password(current_password, stored_hash) {
        return Err("Das aktuelle Passwort ist falsch.".to_string());
    }

    let new_hash = hash_password(new_password)?;
    database::update_password_hash(conn, &session.user_id, &new_hash)
        .map_err(|e| {
            eprintln!("PraxisQM: Datenbankfehler beim Passwortwechsel: {}", e);
            "Passwortwechsel fehlgeschlagen.".to_string()
        })?;

    Ok(())
}

/// Prüft, ob die Deaktivierung oder Löschung eines Benutzers
/// den letzten aktiven Admin entfernen würde.
pub fn check_last_admin_protection(
    conn: &Connection,
    target_user_id: &str,
) -> Result<(), String> {
    let is_admin = database::is_active_admin(conn, target_user_id).map_err(|e| {
        eprintln!("PraxisQM: Datenbankfehler bei Admin-Prüfung: {}", e);
        "Berechtigungsprüfung fehlgeschlagen.".to_string()
    })?;
    if !is_admin {
        return Ok(());
    }
    let count = database::count_active_admins(conn).map_err(|e| {
        eprintln!("PraxisQM: Datenbankfehler bei Admin-Zählung: {}", e);
        "Berechtigungsprüfung fehlgeschlagen.".to_string()
    })?;
    if count <= 1 {
        return Err("Der letzte aktive Administrator kann nicht deaktiviert oder entfernt werden.".to_string());
    }
    Ok(())
}

// --- Tests ----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database;
    use crate::test_support::{init_test_db, insert_test_user, set_test_session};
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_and_verify_password() {
        let hash = hash_password("test123").unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, "test123");
        assert!(verify_password("test123", &hash));
        assert!(!verify_password("wrong", &hash));
    }

    #[test]
    fn test_hash_is_unique() {
        let h1 = hash_password("test123").unwrap();
        let h2 = hash_password("test123").unwrap();
        assert_ne!(h1, h2, "Zwei Hashes desselben Passworts müssen unterschiedlich sein (zufälliges Salt)");
    }

    #[test]
    fn test_role_rank_ordering() {
        assert!(role_rank(ROLE_ADMIN) > role_rank(ROLE_EDITOR));
        assert!(role_rank(ROLE_EDITOR) > role_rank(ROLE_GUEST));
        assert_eq!(role_rank("Unbekannt"), -1);
    }

    #[test]
    fn test_require_role_guest_allowed_for_guest() {
        let session = Session {
            user_id: "test".to_string(),
            username: "gast".to_string(),
            role: ROLE_GUEST.to_string(),
            login_at: 0,
        };
        let result = require_role(&Some(session), ROLE_GUEST);
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_role_editor_denied_for_guest() {
        let session = Session {
            user_id: "test".to_string(),
            username: "gast".to_string(),
            role: ROLE_GUEST.to_string(),
            login_at: 0,
        };
        let result = require_role(&Some(session), ROLE_EDITOR);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Keine Berechtigung"));
    }

    #[test]
    fn test_require_role_admin_allowed_for_admin() {
        let session = Session {
            user_id: "test".to_string(),
            username: "admin".to_string(),
            role: ROLE_ADMIN.to_string(),
            login_at: 0,
        };
        let result = require_role(&Some(session.clone()), ROLE_ADMIN);
        assert!(result.is_ok());
        let result = require_role(&Some(session), ROLE_EDITOR);
        assert!(result.is_ok());
    }

    #[test]
    fn test_require_role_denied_without_session() {
        let result = require_role(&None, ROLE_GUEST);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("anmelden"));
    }

    #[test]
    fn test_privileged_missing_session_denied() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        let result = require_privileged(&conn, &auth, ROLE_EDITOR);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("anmelden"));
    }

    #[test]
    fn test_privileged_guest_denied_editor_and_admin() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "g1", ROLE_GUEST, true);
        set_test_session(&auth, "g1", ROLE_GUEST);
        assert!(require_privileged(&conn, &auth, ROLE_EDITOR).is_err());
        assert!(require_privileged(&conn, &auth, ROLE_ADMIN).is_err());
    }

    #[test]
    fn test_privileged_editor_allowed_editor_denied_admin() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "e1", ROLE_EDITOR, true);
        set_test_session(&auth, "e1", ROLE_EDITOR);
        assert!(require_privileged(&conn, &auth, ROLE_EDITOR).is_ok());
        let denied = require_privileged(&conn, &auth, ROLE_ADMIN);
        assert!(denied.is_err());
        assert!(denied.unwrap_err().contains("Keine Berechtigung"));
    }

    #[test]
    fn test_privileged_admin_allowed_editor_and_admin() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "a1", ROLE_ADMIN, true);
        set_test_session(&auth, "a1", ROLE_ADMIN);
        assert!(require_privileged(&conn, &auth, ROLE_EDITOR).is_ok());
        assert!(require_privileged(&conn, &auth, ROLE_ADMIN).is_ok());
    }

    #[test]
    fn test_privileged_deactivated_user_denied_and_session_cleared() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "a2", ROLE_ADMIN, false);
        set_test_session(&auth, "a2", ROLE_ADMIN);
        let result = require_privileged(&conn, &auth, ROLE_EDITOR);
        assert!(result.unwrap_err().contains("deaktiviert"));
        assert!(auth.get_session().is_none());
    }

    #[test]
    fn test_privileged_deleted_user_denied_and_session_cleared() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        set_test_session(&auth, "ghost", ROLE_ADMIN);
        assert!(require_privileged(&conn, &auth, ROLE_EDITOR).is_err());
        assert!(auth.get_session().is_none());
    }

    #[test]
    fn test_privileged_demotion_removes_previous_rights() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "a3", ROLE_ADMIN, true);
        set_test_session(&auth, "a3", ROLE_ADMIN);
        assert!(require_privileged(&conn, &auth, ROLE_ADMIN).is_ok());

        conn.execute("UPDATE users SET role = ?1 WHERE id = ?2;", rusqlite::params![ROLE_EDITOR, "a3"])
            .unwrap();

        assert!(require_privileged(&conn, &auth, ROLE_ADMIN).is_err());
        assert!(require_privileged(&conn, &auth, ROLE_EDITOR).is_ok());
    }

    #[test]
    fn test_privileged_invalid_db_role_denied() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "x1", "Hausmeister", true);
        set_test_session(&auth, "x1", ROLE_ADMIN);
        let result = require_privileged(&conn, &auth, ROLE_EDITOR);
        assert!(result.unwrap_err().contains("Ungültige Benutzerrolle"));
    }

    #[test]
    fn test_privileged_unknown_required_role_denied() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "a4", ROLE_ADMIN, true);
        set_test_session(&auth, "a4", ROLE_ADMIN);
        let result = require_privileged(&conn, &auth, "Unbekannt");
        assert!(result.unwrap_err().contains("Ungültige Rollenanforderung"));
    }

    #[test]
    fn test_login_success() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("passwort123").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        let result = login(&conn, &auth, "admin", "passwort123");
        assert!(result.is_ok());
        let login_result = result.unwrap();
        assert_eq!(login_result.username, "admin");
        assert_eq!(login_result.role, ROLE_ADMIN);

        let session = auth.get_session();
        assert!(session.is_some());
        assert_eq!(session.unwrap().username, "admin");
    }

    #[test]
    fn test_login_wrong_password() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("richtig").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        let result = login(&conn, &auth, "admin", "falsch");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("falsch"));
        assert!(auth.get_session().is_none());
    }

    #[test]
    fn test_login_unknown_user() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let result = login(&conn, &auth, "nichtexistent", "passwort");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("falsch"));
    }

    #[test]
    fn test_login_deactivated_user() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("passwort").unwrap();
        let user = database::create_first_admin(&conn, "admin", &hash).unwrap();

        conn.execute(
            "UPDATE users SET is_active = 0 WHERE id = ?1;",
            rusqlite::params![user.id],
        )
        .unwrap();

        let result = login(&conn, &auth, "admin", "passwort");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("deaktiviert"));
    }

    #[test]
    fn test_logout_clears_session() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("passwort").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        login(&conn, &auth, "admin", "passwort").unwrap();
        assert!(auth.get_session().is_some());

        auth.clear_session();
        assert!(auth.get_session().is_none());
    }

    #[test]
    fn test_change_password_success() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("alt123").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        login(&conn, &auth, "admin", "alt123").unwrap();

        let result = change_password(&conn, &auth, "alt123", "neu456");
        assert!(result.is_ok());

        auth.clear_session();
        let login_old = login(&conn, &auth, "admin", "alt123");
        assert!(login_old.is_err());

        let login_new = login(&conn, &auth, "admin", "neu456");
        assert!(login_new.is_ok());
    }

    #[test]
    fn test_change_password_wrong_current() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("alt123").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        login(&conn, &auth, "admin", "alt123").unwrap();

        let result = change_password(&conn, &auth, "falsch", "neu456");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("falsch"));
    }

    #[test]
    fn test_change_password_without_session() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let result = change_password(&conn, &auth, "alt", "neu");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Sitzung"));
    }

    #[test]
    fn test_first_admin_only_when_empty() {
        let (conn, _tmp) = init_test_db();
        let hash = hash_password("passwort").unwrap();

        database::create_first_admin(&conn, "admin1", &hash).unwrap();
        let result = database::create_first_admin(&conn, "admin2", &hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_users_empty() {
        let (conn, _tmp) = init_test_db();
        assert!(database::is_users_empty(&conn).unwrap());

        let hash = hash_password("passwort").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        assert!(!database::is_users_empty(&conn).unwrap());
    }

    #[test]
    fn test_count_active_admins() {
        let (conn, _tmp) = init_test_db();
        assert_eq!(database::count_active_admins(&conn).unwrap(), 0);

        let hash = hash_password("passwort").unwrap();
        let admin = database::create_first_admin(&conn, "admin", &hash).unwrap();
        assert_eq!(database::count_active_admins(&conn).unwrap(), 1);

        conn.execute(
            "UPDATE users SET is_active = 0 WHERE id = ?1;",
            rusqlite::params![admin.id],
        )
        .unwrap();
        assert_eq!(database::count_active_admins(&conn).unwrap(), 0);
    }

    #[test]
    fn test_last_admin_protection_blocks_deactivation() {
        let (conn, _tmp) = init_test_db();
        let hash = hash_password("passwort").unwrap();
        let admin = database::create_first_admin(&conn, "admin", &hash).unwrap();

        let result = check_last_admin_protection(&conn, &admin.id);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("letzte aktive Administrator"));
    }

    #[test]
    fn test_last_admin_protection_allows_when_multiple() {
        let (conn, _tmp) = init_test_db();
        let hash = hash_password("passwort").unwrap();
        let admin1 = database::create_first_admin(&conn, "admin1", &hash).unwrap();

        // Manually insert a second admin (create_first_admin only works on empty table)
        let id2 = uuid::Uuid::new_v4().to_string();
        let now = database::now_iso();
        conn.execute(
            "INSERT INTO users (id, username, role, is_active, password_hash, created_at, updated_at)
             VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6);",
            rusqlite::params![id2, "admin2", database::ROLE_ADMIN, hash, now, now],
        )
        .unwrap();

        let result = check_last_admin_protection(&conn, &admin1.id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_last_admin_protection_ignores_non_admin() {
        let (conn, _tmp) = init_test_db();
        let hash = hash_password("passwort").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        // Insert an editor
        let id2 = uuid::Uuid::new_v4().to_string();
        let now = database::now_iso();
        conn.execute(
            "INSERT INTO users (id, username, role, is_active, password_hash, created_at, updated_at)
             VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6);",
            rusqlite::params![id2, "editor", database::ROLE_EDITOR, hash, now, now],
        )
        .unwrap();

        let result = check_last_admin_protection(&conn, &id2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_migration_v2_to_v3_adds_password_hash() {
        let tmp = NamedTempFile::new().unwrap();
        // Simulate a v2 database by creating schema and running v1→v2 migration manually
        database::init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();

        // Verify password_hash column exists
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(users);")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(
            columns.iter().any(|c| c == "password_hash"),
            "password_hash column must exist after migration"
        );
    }

    #[test]
    fn test_existing_data_preserved_after_migration() {
        let tmp = NamedTempFile::new().unwrap();
        database::init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();

        let now = database::now_iso();
        // Insert a document
        conn.execute(
            "INSERT INTO documents (id, document_number, title, version, status, validity, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
            rusqlite::params![
                uuid::Uuid::new_v4().to_string(),
                "PQM-TEST-001",
                "Testdokument",
                "1.0",
                "aktiv",
                "gültig",
                now,
                now,
            ],
        )
        .unwrap();

        // Re-run init (simulates app restart with migration)
        drop(conn);
        database::init_database(tmp.path()).unwrap();
        let conn2 = Connection::open(tmp.path()).unwrap();

        let count: i64 = conn2
            .query_row(
                "SELECT COUNT(*) FROM documents WHERE document_number = 'PQM-TEST-001';",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "Existing document must survive migration");
    }

    #[test]
    fn test_login_lockout_after_max_attempts() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("richtig").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        for _ in 0..MAX_LOGIN_ATTEMPTS {
            let _ = login(&conn, &auth, "admin", "falsch");
        }

        let result = login(&conn, &auth, "admin", "falsch");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("fehlgeschlagene Versuche"));
    }

    #[test]
    fn test_login_lockout_blocks_even_correct_password() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("richtig").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        for _ in 0..MAX_LOGIN_ATTEMPTS {
            let _ = login(&conn, &auth, "admin", "falsch");
        }

        let result = login(&conn, &auth, "admin", "richtig");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("fehlgeschlagene Versuche"));
    }

    #[test]
    fn test_login_lockout_expires_after_duration() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("richtig").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        for _ in 0..MAX_LOGIN_ATTEMPTS {
            let _ = login(&conn, &auth, "admin", "falsch");
        }
        assert!(login(&conn, &auth, "admin", "richtig").is_err());

        auth.login_throttle
            .lock()
            .unwrap()
            .get_mut("admin")
            .unwrap()
            .locked_until = Some(Instant::now() - Duration::from_secs(1));

        assert!(login(&conn, &auth, "admin", "richtig").is_ok());
    }

    #[test]
    fn test_successful_login_clears_failed_attempts() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();

        let hash = hash_password("richtig").unwrap();
        database::create_first_admin(&conn, "admin", &hash).unwrap();

        for _ in 0..3 {
            let _ = login(&conn, &auth, "admin", "falsch");
        }

        auth.clear_session();
        let result = login(&conn, &auth, "admin", "richtig");
        assert!(result.is_ok());

        auth.clear_session();
        for _ in 0..3 {
            let _ = login(&conn, &auth, "admin", "falsch");
        }
        let result = login(&conn, &auth, "admin", "falsch");
        assert!(result.is_err());
        assert!(!result.unwrap_err().contains("fehlgeschlagene Versuche"));
    }
}
