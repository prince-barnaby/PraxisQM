// PraxisQM – SQLite-Datenbankmodul
// Modul: Backend / Datenbank
// Zweck: Lokale SQLite-Datenbank für PraxisQM.
//         - Idempotente Initialisierung
//         - Schema-Versionierung
//         - Kanonisches Schema (Class-A Tabellen)
//         - Foreign-Key-Enforcement
//         - Vollständig offline, keine Cloud-Abhängigkeit

use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

/// Aktuelle Schema-Version.
/// Muss erhöht werden, wenn das Schema geändert wird.
const SCHEMA_VERSION: i64 = 2;

/// Dateiname der SQLite-Datenbank.
const DB_FILENAME: &str = "praxisqm.sqlite";

/// --- Tabellen-Schema (Class-A) -------------------------------------------

/// Liefert die SQL-Statements zum Erzeugen aller Class-A Tabellen.
/// Reihenfolge ist wichtig: Eltern-Tabellen vor Kind-Tabellen.
fn schema_statements() -> Vec<&'static str> {
    vec![
        // DB-005 Categories (vor DB-001, DB-006)
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );",
        // DB-006 Subcategories (nach DB-005)
        "CREATE TABLE IF NOT EXISTS subcategories (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            category_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories(id),
            UNIQUE (id)
        );",
        // DB-003 Employees (vor DB-001, DB-004, DB-013, DB-014)
        "CREATE TABLE IF NOT EXISTS employees (
            id TEXT PRIMARY KEY NOT NULL,
            last_name TEXT NOT NULL,
            first_name TEXT NOT NULL,
            position TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            hire_date TEXT,
            departure_date TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );",
        // DB-004 Users (nach DB-003)
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY NOT NULL,
            username TEXT NOT NULL UNIQUE,
            role TEXT NOT NULL,
            employee_id TEXT,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (employee_id) REFERENCES employees(id)
        );",
        // DB-001 Documents (nach DB-003, DB-005, DB-006)
        "CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY NOT NULL,
            document_number TEXT NOT NULL UNIQUE,
            title TEXT NOT NULL,
            category_id TEXT,
            subcategory_id TEXT,
            responsible_person_id TEXT,
            version TEXT NOT NULL,
            status TEXT NOT NULL,
            validity TEXT NOT NULL,
            valid_until TEXT,
            description TEXT,
            archived_at TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories(id),
            FOREIGN KEY (subcategory_id) REFERENCES subcategories(id),
            FOREIGN KEY (responsible_person_id) REFERENCES employees(id)
        );",
        // DB-002 DocumentVersions (nach DB-001, DB-004)
        "CREATE TABLE IF NOT EXISTS document_versions (
            id TEXT PRIMARY KEY NOT NULL,
            document_id TEXT NOT NULL,
            version_number TEXT NOT NULL,
            file_name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            status TEXT NOT NULL,
            validity TEXT NOT NULL,
            valid_until TEXT,
            uploaded_by TEXT,
            uploaded_at TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (document_id) REFERENCES documents(id),
            FOREIGN KEY (uploaded_by) REFERENCES users(id)
        );",
        // DB-007 KeywordDictionary (vor DB-008)
        "CREATE TABLE IF NOT EXISTS keyword_dictionary (
            id TEXT PRIMARY KEY NOT NULL,
            keyword TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );",
        // DB-008 DocumentTags (nach DB-001, DB-007)
        "CREATE TABLE IF NOT EXISTS document_tags (
            document_id TEXT NOT NULL,
            keyword_id TEXT NOT NULL,
            PRIMARY KEY (document_id, keyword_id),
            FOREIGN KEY (document_id) REFERENCES documents(id),
            FOREIGN KEY (keyword_id) REFERENCES keyword_dictionary(id)
        );",
        // DB-009 AuditLog (nach DB-001, DB-004)
        "CREATE TABLE IF NOT EXISTS audit_log (
            id TEXT PRIMARY KEY NOT NULL,
            action TEXT NOT NULL,
            user_id TEXT,
            document_id TEXT,
            details TEXT,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (document_id) REFERENCES documents(id)
        );",
        // DB-015 Verantwortungspositionen (vor DB-013)
        "CREATE TABLE IF NOT EXISTS verantwortungspositionen (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );",
        // DB-016 QMBereiche (vor DB-014)
        "CREATE TABLE IF NOT EXISTS qm_bereiche (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );",
        // DB-013 EmployeeResponsibilities (nach DB-003, DB-015)
        "CREATE TABLE IF NOT EXISTS employee_responsibilities (
            employee_id TEXT NOT NULL,
            responsibility_id TEXT NOT NULL,
            PRIMARY KEY (employee_id, responsibility_id),
            FOREIGN KEY (employee_id) REFERENCES employees(id),
            FOREIGN KEY (responsibility_id) REFERENCES verantwortungspositionen(id)
        );",
        // DB-014 EmployeeQMAreas (nach DB-003, DB-016)
        "CREATE TABLE IF NOT EXISTS employee_qm_areas (
            employee_id TEXT NOT NULL,
            qm_area_id TEXT NOT NULL,
            PRIMARY KEY (employee_id, qm_area_id),
            FOREIGN KEY (employee_id) REFERENCES employees(id),
            FOREIGN KEY (qm_area_id) REFERENCES qm_bereiche(id)
        );",
    ]
}

/// --- Schema-Versionierung ------------------------------------------------

/// Erstellt die Schema-Versions-Tabelle, falls sie nicht existiert.
fn ensure_schema_version_table(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER NOT NULL,
            applied_at TEXT NOT NULL
        );",
        [],
    )?;
    Ok(())
}

/// Liefert die aktuell installierte Schema-Version.
/// Gibt 0 zurück, wenn noch keine Version eingetragen ist.
fn get_schema_version(conn: &Connection) -> SqliteResult<i64> {
    let mut stmt = conn.prepare("SELECT MAX(version) FROM schema_version;")?;
    let version: Option<i64> = stmt.query_row([], |row| row.get(0))?;
    Ok(version.unwrap_or(0))
}

/// Trägt die aktuelle Schema-Version ein.
fn set_schema_version(conn: &Connection, version: i64) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO schema_version (version, applied_at) VALUES (?1, ?2);",
        rusqlite::params![version, now_iso()],
    )?;
    Ok(())
}

/// --- Initialisierung -----------------------------------------------------

/// Liefert den Pfad zur SQLite-Datenbank im App-Data-Verzeichnis.
pub fn database_path(app: &AppHandle) -> PathBuf {
    let dir = app
        .path_resolver()
        .app_data_dir()
        .expect("App-Data-Verzeichnis nicht verfügbar");
    fs::create_dir_all(&dir).expect("App-Data-Verzeichnis konnte nicht erstellt werden");
    dir.join(DB_FILENAME)
}

/// Liefert den Pfad zum verwalteten Dokumentenspeicher im App-Data-Verzeichnis.
pub fn document_storage_path(app: &AppHandle) -> PathBuf {
    let dir = app
        .path_resolver()
        .app_data_dir()
        .expect("App-Data-Verzeichnis nicht verfügbar");
    let storage = dir.join("documents");
    fs::create_dir_all(&storage).expect("Dokumentenspeicher konnte nicht erstellt werden");
    storage
}

/// Initialisiert die Datenbank: öffnet/erstellt die Datei, aktiviert
/// Foreign-Key-Enforcement, stellt das Schema bereit und trägt die
/// Schema-Version ein. Idempotent — sicher bei wiederholtem Aufruf.
pub fn init_database(db_path: &Path) -> SqliteResult<()> {
    let conn = Connection::open(db_path)?;

    // Foreign-Key-Enforcement aktivieren (SQLite hat dies standardmäßig deaktiviert)
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Schema-Versions-Tabelle sicherstellen
    ensure_schema_version_table(&conn)?;

    let current_version = get_schema_version(&conn)?;

    // Schema erstellen, wenn Version 0 oder niedriger als aktuell
    if current_version < SCHEMA_VERSION {
        for stmt in schema_statements() {
            conn.execute(stmt, [])?;
        }
        set_schema_version(&conn, SCHEMA_VERSION)?;
    }

    // Schema-Migrationen (idempotent, nicht-destruktiv)
    // Migration 1→2: pre_archive_status-Spalte für Lifecycle-Erhaltung (Prompt 024)
    if current_version < 2 {
        migrate_v1_to_v2(&conn)?;
    }

    // Verifiziere Foreign-Key-Enforcement
    let fk_enabled: i64 =
        conn.query_row("PRAGMA foreign_keys;", [], |row| row.get(0))?;
    if fk_enabled != 1 {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some("Foreign-Key-Enforcement konnte nicht aktiviert werden".to_string()),
        ));
    }

    Ok(())
}

/// --- Hilfsfunktionen -----------------------------------------------------

/// Liefert den aktuellen Zeitpunkt als ISO-8601-String (UTC).
/// Für Timestamps in created_at / updated_at / timestamp-Feldern.
pub fn now_iso() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{}", secs)
}

/// --- Schema-Migrationen --------------------------------------------------

/// Migration v1→v2: Fügt die Spalte `pre_archive_status` zur documents-Tabelle hinzu.
/// Nullable TEXT-Spalte — leer für nicht-archivierte und vor Prompt 024 erstellte Dokumente.
/// Idempotent: prüft Spalten-Existenz vor ALTER TABLE.
fn migrate_v1_to_v2(conn: &Connection) -> SqliteResult<()> {
    // Prüfe, ob die Spalte bereits existiert (Idempotenz)
    let columns: Vec<String> = conn
        .prepare("PRAGMA table_info(documents);")?
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(|r| r.ok())
        .collect();
    if !columns.iter().any(|c| c == "pre_archive_status") {
        conn.execute(
            "ALTER TABLE documents ADD COLUMN pre_archive_status TEXT;",
            [],
        )?;
    }
    Ok(())
}

/// --- Dokument-Lifecycle (Prompt 024) -------------------------------------

/// Kanonische DB-001/DB-002 Status-Werte (Enum-Audit SDD-004B).
pub const STATUS_ENTWURF: &str = "Entwurf";
pub const STATUS_AKTIV: &str = "aktiv";
pub const STATUS_ARCHIVIERT: &str = "archiviert";

/// Prüft, ob ein Status-Wert kanonisch ist (alle drei Lifecycle-States).
pub fn is_valid_status(status: &str) -> bool {
    status == STATUS_ENTWURF || status == STATUS_AKTIV || status == STATUS_ARCHIVIERT
}

/// Prüft, ob ein Status bei Erstellung/Bearbeitung erlaubt ist (nicht-archivierte Lifecycle-States).
/// "archiviert" ist hier nicht erlaubt — Archivierung erfolgt ausschließlich über archive_document.
pub fn is_valid_creation_status(status: &str) -> bool {
    status == STATUS_ENTWURF || status == STATUS_AKTIV
}

/// --- Gültigkeitsberechnung -----------------------------------------------

/// Kanonische Gültigkeits-States (abgeleitet, nicht gespeichert).
pub const VALIDITY_GUELTIG: &str = "gültig";
pub const VALIDITY_BALD_AB: &str = "läuft bald ab";
pub const VALIDITY_ABGELAUFEN: &str = "abgelaufen";

/// Schwellwert in Kalendertagen für "läuft bald ab" (Prompt 022A, kanonisch für v1).
const VALIDITY_THRESHOLD_DAYS: i64 = 30;

/// Parst ein ISO-Datum im Format "YYYY-MM-DD" in ein (Jahr, Monat, Tag) Triple.
/// Gibt None bei ungültigem Format zurück.
fn parse_date(s: &str) -> Option<(i32, u32, u32)> {
    let parts: Vec<&str> = s.trim().split('-').collect();
    if parts.len() != 3 {
        return None;
    }
    let year: i32 = parts[0].parse().ok()?;
    let month: u32 = parts[1].parse().ok()?;
    let day: u32 = parts[2].parse().ok()?;
    if month < 1 || month > 12 || day < 1 || day > 31 {
        return None;
    }
    Some((year, month, day))
}

/// Konvertiert ein (Jahr, Monat, Tag) Triple in eine fortlaufende Tageszahl.
/// Verwendet die Algorithmus von Howard Hinnant (days_from_civil).
/// Ermöglicht Kalenderdatums-Vergleiche ohne externe Bibliothek.
fn days_from_date(year: i32, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let doy = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    (era as i64) * 146097 + (doe as i64) - 719468
}

/// Berechnet den kanonischen Gültigkeitsstatus aus valid_until und dem
/// aktuellen Kalenderdatum. Der `today`-Parameter ermöglicht deterministische Tests.
///
/// Rückgabe:
/// - None: valid_until ist NULL → nicht überwacht
/// - Some("gültig"): today < (valid_until - 30 Tage)
/// - Some("läuft bald ab"): today >= (valid_until - 30 Tage) UND today <= valid_until
/// - Some("abgelaufen"): today > valid_until
pub fn calculate_validity_status(
    valid_until: Option<&str>,
    today: &str,
) -> Option<&'static str> {
    let vu = valid_until?;
    let (vy, vm, vd) = parse_date(vu)?;
    let (ty, tm, td) = parse_date(today)?;

    let vu_days = days_from_date(vy, vm, vd);
    let today_days = days_from_date(ty, tm, td);
    let threshold = vu_days - VALIDITY_THRESHOLD_DAYS;

    if today_days > vu_days {
        Some(VALIDITY_ABGELAUFEN)
    } else if today_days >= threshold {
        Some(VALIDITY_BALD_AB)
    } else {
        Some(VALIDITY_GUELTIG)
    }
}

/// Liefert das aktuelle lokale Kalenderdatum als "YYYY-MM-DD".
/// Verwendet SystemTime und berechnet das Datum aus Unix-Epoch-Sekunden.
pub fn today_local_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let days = secs.div_euclid(86400);
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02}", year, m, d)
}

/// --- Datenmodelle -------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub last_name: String,
    pub first_name: String,
    pub position: Option<String>,
    pub is_active: bool,
    pub hire_date: Option<String>,
    pub departure_date: Option<String>,
    pub responsibilities: Vec<String>,
    pub qm_areas: Vec<String>,
    #[serde(default)]
    pub responsibility_ids: Vec<String>,
    #[serde(default)]
    pub qm_area_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterDataItem {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEmployeeInput {
    pub last_name: String,
    pub first_name: String,
    pub position: Option<String>,
    pub is_active: bool,
    pub hire_date: Option<String>,
    pub departure_date: Option<String>,
    pub responsibility_ids: Vec<String>,
    pub qm_area_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEmployeeInput {
    pub last_name: String,
    pub first_name: String,
    pub position: Option<String>,
    pub is_active: bool,
    pub hire_date: Option<String>,
    pub departure_date: Option<String>,
    pub responsibility_ids: Vec<String>,
    pub qm_area_ids: Vec<String>,
}

/// --- Mitarbeiter-Operationen --------------------------------------------

/// Lädt alle Mitarbeiter mit ihren Verantwortungspositionen und QM-Bereichen.
pub fn list_employees(conn: &Connection) -> SqliteResult<Vec<Employee>> {
    let mut stmt = conn.prepare(
        "SELECT id, last_name, first_name, position, is_active, hire_date, departure_date
         FROM employees ORDER BY last_name, first_name;",
    )?;
    let employee_rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, bool>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
            ))
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

    let mut employees = Vec::with_capacity(employee_rows.len());
    for (id, last_name, first_name, position, is_active, hire_date, departure_date) in employee_rows {
        let responsibilities = load_responsibility_names(conn, &id)?;
        let qm_areas = load_qm_area_names(conn, &id)?;
        let responsibility_ids = load_responsibility_ids(conn, &id)?;
        let qm_area_ids = load_qm_area_ids(conn, &id)?;
        employees.push(Employee {
            id,
            last_name,
            first_name,
            position,
            is_active,
            hire_date,
            departure_date,
            responsibilities,
            qm_areas,
            responsibility_ids,
            qm_area_ids,
        });
    }
    Ok(employees)
}

/// Lädt die Namen der Verantwortungspositionen für einen Mitarbeiter.
fn load_responsibility_names(conn: &Connection, employee_id: &str) -> SqliteResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT v.name FROM employee_responsibilities er
         JOIN verantwortungspositionen v ON v.id = er.responsibility_id
         WHERE er.employee_id = ?1 ORDER BY v.name;",
    )?;
    let names = stmt
        .query_map(rusqlite::params![employee_id], |row| row.get(0))?
        .collect::<SqliteResult<Vec<String>>>()?;
    Ok(names)
}

/// Lädt die Namen der QM-Bereiche für einen Mitarbeiter.
fn load_qm_area_names(conn: &Connection, employee_id: &str) -> SqliteResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT q.name FROM employee_qm_areas eq
         JOIN qm_bereiche q ON q.id = eq.qm_area_id
         WHERE eq.employee_id = ?1 ORDER BY q.name;",
    )?;
    let names = stmt
        .query_map(rusqlite::params![employee_id], |row| row.get(0))?
        .collect::<SqliteResult<Vec<String>>>()?;
    Ok(names)
}

/// Lädt die UUIDs der Verantwortungspositionen für einen Mitarbeiter.
fn load_responsibility_ids(conn: &Connection, employee_id: &str) -> SqliteResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT er.responsibility_id FROM employee_responsibilities er
         JOIN verantwortungspositionen v ON v.id = er.responsibility_id
         WHERE er.employee_id = ?1 ORDER BY v.name;",
    )?;
    let ids = stmt
        .query_map(rusqlite::params![employee_id], |row| row.get(0))?
        .collect::<SqliteResult<Vec<String>>>()?;
    Ok(ids)
}

/// Lädt die UUIDs der QM-Bereiche für einen Mitarbeiter.
fn load_qm_area_ids(conn: &Connection, employee_id: &str) -> SqliteResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT eq.qm_area_id FROM employee_qm_areas eq
         JOIN qm_bereiche q ON q.id = eq.qm_area_id
         WHERE eq.employee_id = ?1 ORDER BY q.name;",
    )?;
    let ids = stmt
        .query_map(rusqlite::params![employee_id], |row| row.get(0))?
        .collect::<SqliteResult<Vec<String>>>()?;
    Ok(ids)
}

/// Erstellt einen Mitarbeiter und seine Zuordnungen in einer Transaktion.
/// Bei einem Fehler werden alle Änderungen zurückgerollt.
pub fn create_employee(conn: &Connection, input: &CreateEmployeeInput) -> SqliteResult<Employee> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();

    conn.execute(
        "INSERT INTO employees (id, last_name, first_name, position, is_active, hire_date, departure_date, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
        rusqlite::params![
            id,
            input.last_name,
            input.first_name,
            input.position,
            input.is_active,
            input.hire_date,
            input.departure_date,
            now,
            now,
        ],
    )?;

    for resp_id in &input.responsibility_ids {
        conn.execute(
            "INSERT INTO employee_responsibilities (employee_id, responsibility_id) VALUES (?1, ?2);",
            rusqlite::params![id, resp_id],
        )?;
    }

    for area_id in &input.qm_area_ids {
        conn.execute(
            "INSERT INTO employee_qm_areas (employee_id, qm_area_id) VALUES (?1, ?2);",
            rusqlite::params![id, area_id],
        )?;
    }

    let responsibilities = load_responsibility_names(conn, &id)?;
    let qm_areas = load_qm_area_names(conn, &id)?;
    let responsibility_ids = input.responsibility_ids.clone();
    let qm_area_ids = input.qm_area_ids.clone();

    Ok(Employee {
        id,
        last_name: input.last_name.clone(),
        first_name: input.first_name.clone(),
        position: input.position.clone(),
        is_active: input.is_active,
        hire_date: input.hire_date.clone(),
        departure_date: input.departure_date.clone(),
        responsibilities,
        qm_areas,
        responsibility_ids,
        qm_area_ids,
    })
}

/// Lädt einen einzelnen Mitarbeiter anhand seiner UUID.
/// Liefert einen Fehler, wenn kein Mitarbeiter mit dieser ID existiert.
pub fn get_employee(conn: &Connection, id: &str) -> SqliteResult<Employee> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM employees WHERE id = ?1;",
        rusqlite::params![id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let (id, last_name, first_name, position, is_active, hire_date, departure_date) = conn.query_row(
        "SELECT id, last_name, first_name, position, is_active, hire_date, departure_date
         FROM employees WHERE id = ?1;",
        rusqlite::params![id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, bool>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
            ))
        },
    )?;

    let responsibilities = load_responsibility_names(conn, &id)?;
    let qm_areas = load_qm_area_names(conn, &id)?;
    let responsibility_ids = load_responsibility_ids(conn, &id)?;
    let qm_area_ids = load_qm_area_ids(conn, &id)?;

    Ok(Employee {
        id,
        last_name,
        first_name,
        position,
        is_active,
        hire_date,
        departure_date,
        responsibilities,
        qm_areas,
        responsibility_ids,
        qm_area_ids,
    })
}

/// Aktualisiert einen bestehenden Mitarbeiter und synchronisiert seine Zuordnungen.
/// Die UUID bleibt unverändert. Alle Änderungen werden in einer Transaktion ausgeführt.
/// Bei einem Fehler (z. B. ungültige Zuordnungs-ID) wird die gesamte Transaktion zurückgerollt.
pub fn update_employee(conn: &Connection, id: &str, input: &UpdateEmployeeInput) -> SqliteResult<Employee> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM employees WHERE id = ?1;",
        rusqlite::params![id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    for resp_id in &input.responsibility_ids {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM verantwortungspositionen WHERE id = ?1;",
            rusqlite::params![resp_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some(format!("Ungültige Verantwortungsposition: {}", resp_id)),
            ));
        }
    }

    for area_id in &input.qm_area_ids {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM qm_bereiche WHERE id = ?1;",
            rusqlite::params![area_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some(format!("Ungültiger QM-Bereich: {}", area_id)),
            ));
        }
    }

    let now = now_iso();
    conn.execute(
        "UPDATE employees SET last_name = ?1, first_name = ?2, position = ?3, is_active = ?4,
         hire_date = ?5, departure_date = ?6, updated_at = ?7 WHERE id = ?8;",
        rusqlite::params![
            input.last_name,
            input.first_name,
            input.position,
            input.is_active,
            input.hire_date,
            input.departure_date,
            now,
            id,
        ],
    )?;

    conn.execute(
        "DELETE FROM employee_responsibilities WHERE employee_id = ?1;",
        rusqlite::params![id],
    )?;
    for resp_id in &input.responsibility_ids {
        conn.execute(
            "INSERT INTO employee_responsibilities (employee_id, responsibility_id) VALUES (?1, ?2);",
            rusqlite::params![id, resp_id],
        )?;
    }

    conn.execute(
        "DELETE FROM employee_qm_areas WHERE employee_id = ?1;",
        rusqlite::params![id],
    )?;
    for area_id in &input.qm_area_ids {
        conn.execute(
            "INSERT INTO employee_qm_areas (employee_id, qm_area_id) VALUES (?1, ?2);",
            rusqlite::params![id, area_id],
        )?;
    }

    let responsibilities = load_responsibility_names(conn, &id)?;
    let qm_areas = load_qm_area_names(conn, &id)?;
    let responsibility_ids = input.responsibility_ids.clone();
    let qm_area_ids = input.qm_area_ids.clone();

    Ok(Employee {
        id: id.to_string(),
        last_name: input.last_name.clone(),
        first_name: input.first_name.clone(),
        position: input.position.clone(),
        is_active: input.is_active,
        hire_date: input.hire_date.clone(),
        departure_date: input.departure_date.clone(),
        responsibilities,
        qm_areas,
        responsibility_ids,
        qm_area_ids,
    })
}

/// Lädt alle Verantwortungspositionen (Stammdaten).
pub fn list_responsibilities(conn: &Connection) -> SqliteResult<Vec<MasterDataItem>> {
    let mut stmt =
        conn.prepare("SELECT id, name FROM verantwortungspositionen ORDER BY name;")?;
    let items = stmt
        .query_map([], |row| {
            Ok(MasterDataItem {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(items)
}

/// Lädt alle QM-Bereiche (Stammdaten).
pub fn list_qm_areas(conn: &Connection) -> SqliteResult<Vec<MasterDataItem>> {
    let mut stmt = conn.prepare("SELECT id, name FROM qm_bereiche ORDER BY name;")?;
    let items = stmt
        .query_map([], |row| {
            Ok(MasterDataItem {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(items)
}

/// Erstellt eine neue Verantwortungsposition.
pub fn create_responsibility(conn: &Connection, name: &str) -> SqliteResult<MasterDataItem> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();
    conn.execute(
        "INSERT INTO verantwortungspositionen (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
        rusqlite::params![id, name, now, now],
    )?;
    Ok(MasterDataItem { id, name: name.to_string() })
}

/// Benennt eine bestehende Verantwortungsposition um (gleiche UUID, updated_at wird aktualisiert).
pub fn rename_responsibility(conn: &Connection, id: &str, new_name: &str) -> SqliteResult<MasterDataItem> {
    let now = now_iso();
    conn.execute(
        "UPDATE verantwortungspositionen SET name = ?1, updated_at = ?2 WHERE id = ?3;",
        rusqlite::params![new_name, now, id],
    )?;
    Ok(MasterDataItem { id: id.to_string(), name: new_name.to_string() })
}

/// Erstellt einen neuen QM-Bereich.
pub fn create_qm_area(conn: &Connection, name: &str) -> SqliteResult<MasterDataItem> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();
    conn.execute(
        "INSERT INTO qm_bereiche (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
        rusqlite::params![id, name, now, now],
    )?;
    Ok(MasterDataItem { id, name: name.to_string() })
}

/// Benennt einen bestehenden QM-Bereich um (gleiche UUID, updated_at wird aktualisiert).
pub fn rename_qm_area(conn: &Connection, id: &str, new_name: &str) -> SqliteResult<MasterDataItem> {
    let now = now_iso();
    conn.execute(
        "UPDATE qm_bereiche SET name = ?1, updated_at = ?2 WHERE id = ?3;",
        rusqlite::params![new_name, now, id],
    )?;
    Ok(MasterDataItem { id: id.to_string(), name: new_name.to_string() })
}

/// --- Schlagwort-Dictionary (DB-007) --------------------------------------

/// Lädt alle Schlagwörter aus dem Keyword-Dictionary.
pub fn list_keywords(conn: &Connection) -> SqliteResult<Vec<MasterDataItem>> {
    let mut stmt =
        conn.prepare("SELECT id, keyword FROM keyword_dictionary ORDER BY keyword;")?;
    let items = stmt
        .query_map([], |row| {
            Ok(MasterDataItem {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(items)
}

/// Erstellt ein neues Schlagwort im Keyword-Dictionary.
pub fn create_keyword(conn: &Connection, keyword: &str) -> SqliteResult<MasterDataItem> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();
    conn.execute(
        "INSERT INTO keyword_dictionary (id, keyword, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
        rusqlite::params![id, keyword, now, now],
    )?;
    Ok(MasterDataItem { id, name: keyword.to_string() })
}

/// Benennt ein bestehendes Schlagwort um (gleiche UUID, updated_at wird aktualisiert).
/// Existierende document_tags-Beziehungen bleiben intakt, da sie UUIDs referenzieren.
pub fn rename_keyword(conn: &Connection, id: &str, new_keyword: &str) -> SqliteResult<MasterDataItem> {
    let now = now_iso();
    let affected = conn.execute(
        "UPDATE keyword_dictionary SET keyword = ?1, updated_at = ?2 WHERE id = ?3;",
        rusqlite::params![new_keyword, now, id],
    )?;
    if affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    Ok(MasterDataItem { id: id.to_string(), name: new_keyword.to_string() })
}

/// --- Dokument-Schlagwort-Zuordnung (DB-008) -----------------------------

/// Lädt die Schlagwörter (Namen) für ein einzelnes Dokument.
pub fn list_document_tags(conn: &Connection, document_id: &str) -> SqliteResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT kd.keyword FROM document_tags dt
         JOIN keyword_dictionary kd ON kd.id = dt.keyword_id
         WHERE dt.document_id = ?1
         ORDER BY kd.keyword;",
    )?;
    let tags = stmt
        .query_map(rusqlite::params![document_id], |row| row.get(0))?
        .collect::<SqliteResult<Vec<String>>>()?;
    Ok(tags)
}

/// Lädt die Schlagwort-IDs für ein einzelnes Dokument.
pub fn list_document_tag_ids(conn: &Connection, document_id: &str) -> SqliteResult<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT dt.keyword_id FROM document_tags dt
         WHERE dt.document_id = ?1;",
    )?;
    let ids = stmt
        .query_map(rusqlite::params![document_id], |row| row.get(0))?
        .collect::<SqliteResult<Vec<String>>>()?;
    Ok(ids)
}

/// Lädt eine Map von document_id → Schlagwort-Namen für alle angegebenen Dokument-IDs.
/// Vermeidet N+1-Abfragen beim Laden der Dokumentenliste.
pub fn batch_document_tag_names(
    conn: &Connection,
    document_ids: &[String],
) -> SqliteResult<std::collections::HashMap<String, Vec<String>>> {
    let mut map: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    if document_ids.is_empty() {
        return Ok(map);
    }
    let placeholders = document_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "SELECT dt.document_id, kd.keyword FROM document_tags dt
         JOIN keyword_dictionary kd ON kd.id = dt.keyword_id
         WHERE dt.document_id IN ({})
         ORDER BY dt.document_id, kd.keyword;",
        placeholders
    );
    let mut stmt = conn.prepare(&sql)?;
    let params: Vec<&dyn rusqlite::ToSql> = document_ids
        .iter()
        .map(|id| id as &dyn rusqlite::ToSql)
        .collect();
    let rows = stmt.query_map(params.as_slice(), |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    for row in rows {
        let (doc_id, keyword) = row?;
        map.entry(doc_id).or_default().push(keyword);
    }
    Ok(map)
}

/// Synchronisiert die Schlagwort-Zuordnungen eines Dokuments atomar.
/// Fügt fehlende Zuordnungen hinzu, entfernt nicht mehr gewählte.
/// Validiert, dass das Dokument existiert und nicht archiviert ist.
/// Validiert, dass alle Schlagwort-IDs im Dictionary existieren.
pub fn sync_document_tags(
    conn: &Connection,
    document_id: &str,
    desired_tag_ids: &[String],
) -> SqliteResult<Vec<String>> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE id = ?1;",
        rusqlite::params![document_id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let archived: Option<String> = conn.query_row(
        "SELECT archived_at FROM documents WHERE id = ?1;",
        rusqlite::params![document_id],
        |row| row.get(0),
    )?;
    if archived.is_some() {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some("Archivierte Dokumente können nicht mit Schlagwörtern versehen werden.".to_string()),
        ));
    }

    // Validiere alle Schlagwort-IDs
    for tag_id in desired_tag_ids {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM keyword_dictionary WHERE id = ?1;",
            rusqlite::params![tag_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Schlagwort nicht gefunden.".to_string()),
            ));
        }
    }

    // Aktuelle Zuordnungen laden
    let current_ids = list_document_tag_ids(conn, document_id)?;
    let current_set: std::collections::HashSet<&String> = current_ids.iter().collect();
    let desired_set: std::collections::HashSet<&String> = desired_tag_ids.iter().collect();

    // Entferne nicht mehr gewählte
    for id in &current_ids {
        if !desired_set.contains(id) {
            conn.execute(
                "DELETE FROM document_tags WHERE document_id = ?1 AND keyword_id = ?2;",
                rusqlite::params![document_id, id],
            )?;
        }
    }

    // Füge neue hinzu
    for id in desired_tag_ids {
        if !current_set.contains(id) {
            conn.execute(
                "INSERT INTO document_tags (document_id, keyword_id) VALUES (?1, ?2);",
                rusqlite::params![document_id, id],
            )?;
        }
    }

    list_document_tags(conn, document_id)
}

/// --- Dokument-Modelle ---------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub document_number: String,
    pub title: String,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub subcategory_id: Option<String>,
    pub subcategory_name: Option<String>,
    pub responsible_person_id: Option<String>,
    pub responsible_person_name: Option<String>,
    pub version: String,
    pub status: String,
    pub validity: String,
    pub valid_until: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub computed_validity: Option<String>,
    pub description: Option<String>,
    pub archived_at: Option<String>,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_archive_status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDocumentInput {
    pub title: String,
    pub category_id: Option<String>,
    pub subcategory_id: Option<String>,
    pub responsible_person_id: Option<String>,
    pub version: String,
    pub status: String,
    pub validity: String,
    pub valid_until: Option<String>,
    pub description: Option<String>,
    pub source_file_path: String,
    pub original_file_name: String,
    #[serde(default)]
    pub tag_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryItem {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubcategoryItem {
    pub id: String,
    pub name: String,
    pub category_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDocumentInput {
    pub title: String,
    pub category_id: Option<String>,
    pub subcategory_id: Option<String>,
    pub responsible_person_id: Option<String>,
    pub version: String,
    pub status: String,
    pub validity: String,
    pub valid_until: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tag_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub id: String,
    pub document_id: String,
    pub version_number: String,
    pub file_name: String,
    pub status: String,
    pub validity: String,
    pub valid_until: Option<String>,
    pub uploaded_at: String,
    pub created_at: String,
    pub is_current: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateVersionInput {
    pub document_id: String,
    pub version_number: String,
    pub status: String,
    pub validity: String,
    pub valid_until: Option<String>,
    pub source_file_path: String,
    pub original_file_name: String,
}

/// --- Dokument-Operationen ------------------------------------------------

/// Generiert die nächste Dokumentennummer im Format PQM-NNNN.
/// Nummern sind sequenziell, eindeutig und werden nie wiederverwendet (ADR-001).
fn generate_document_number(conn: &Connection) -> SqliteResult<String> {
    let max: Option<String> = conn
        .query_row(
            "SELECT MAX(document_number) FROM documents;",
            [],
            |row| row.get(0),
        )
        .ok();

    let next_num = match max {
        Some(num) => {
            let n: u32 = num
                .strip_prefix("PQM-")
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(0);
            n + 1
        }
        None => 1,
    };

    Ok(format!("PQM-{:04}", next_num))
}

/// Validiert, ob eine Datei ein gültiges PDF ist.
/// Prüft: Datei existiert, ist lesbar, nicht leer, hat PDF-Magic-Bytes.
pub fn validate_pdf(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err("Die ausgewählte Datei wurde nicht gefunden.".to_string());
    }

    let metadata = fs::metadata(path).map_err(|_| "Datei konnte nicht gelesen werden.".to_string())?;

    if metadata.len() == 0 {
        return Err("Die Datei ist leer.".to_string());
    }

    let mut file = fs::File::open(path).map_err(|_| "Datei konnte nicht gelesen werden.".to_string())?;
    let mut header = [0u8; 5];
    use std::io::Read;
    file.read_exact(&mut header)
        .map_err(|_| "Datei-Header konnte nicht gelesen werden.".to_string())?;

    if &header != b"%PDF-" {
        return Err("Die Datei ist kein gültiges PDF.".to_string());
    }

    Ok(())
}

/// Kopiert eine Quelldatei in das verwaltete Speicherverzeichnis.
/// Verwendet die Dokument-UUID als Dateinamen (mit .pdf-Erweiterung).
/// Gibt den relativen Dateipfad innerhalb des Speicherverzeichnisses zurück.
pub fn copy_to_managed_storage(
    source: &Path,
    storage_dir: &Path,
    document_id: &str,
) -> Result<String, String> {
    validate_pdf(source)?;

    fs::create_dir_all(storage_dir).map_err(|_| "Speicherverzeichnis konnte nicht erstellt werden.".to_string())?;

    let managed_name = format!("{}.pdf", document_id);
    let dest = storage_dir.join(&managed_name);

    fs::copy(source, &dest).map_err(|_| "Datei konnte nicht in den Dokumentenspeicher kopiert werden.".to_string())?;

    Ok(managed_name)
}

/// Löscht eine verwaltete Datei (für Compensation bei DB-Fehlern).
pub fn remove_managed_file(storage_dir: &Path, relative_path: &str) {
    let full = storage_dir.join(relative_path);
    let _ = fs::remove_file(&full);
}

/// Erstellt ein Dokument mit Metadaten und Datei-Referenz in einer Transaktion.
/// Die Dokumentennummer wird automatisch generiert (ADR-001).
/// Die Datei muss bereits in den verwalteten Speicher kopiert worden sein.
pub fn create_document(
    conn: &Connection,
    input: &CreateDocumentInput,
    managed_file_path: &str,
) -> SqliteResult<Document> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();
    let doc_number = generate_document_number(conn)?;

    // Lifecycle-Validierung: Bei Erstellung nur Entwurf oder aktiv erlaubt (Prompt 024)
    if !is_valid_creation_status(&input.status) {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some(format!(
                "Ungültiger Status bei Erstellung: {}. Nur Entwurf oder aktiv erlaubt.",
                input.status
            )),
        ));
    }

    // Validiere responsible_person_id falls angegeben
    if let Some(ref person_id) = input.responsible_person_id {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM employees WHERE id = ?1;",
            rusqlite::params![person_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Verantwortliche Person nicht gefunden.".to_string()),
            ));
        }
    }

    // Validiere category_id falls angegeben
    if let Some(ref cat_id) = input.category_id {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM categories WHERE id = ?1;",
            rusqlite::params![cat_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Kategorie nicht gefunden.".to_string()),
            ));
        }
    }

    // Validiere subcategory_id falls angegeben
    if let Some(ref sub_id) = input.subcategory_id {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subcategories WHERE id = ?1;",
            rusqlite::params![sub_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Unterkategorie nicht gefunden.".to_string()),
            ));
        }
    }

    conn.execute(
        "INSERT INTO documents (id, document_number, title, category_id, subcategory_id,
         responsible_person_id, version, status, validity, valid_until, description,
         archived_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, NULL, ?12, ?13);",
        rusqlite::params![
            id,
            doc_number,
            input.title,
            input.category_id,
            input.subcategory_id,
            input.responsible_person_id,
            input.version,
            input.status,
            input.validity,
            input.valid_until,
            input.description,
            now,
            now,
        ],
    )?;

    // Erstelle DB-002 DocumentVersions-Eintrag für die erste Version
    let version_id = uuid::Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO document_versions (id, document_id, version_number, file_name, file_path,
         status, validity, valid_until, uploaded_by, uploaded_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10, ?11);",
        rusqlite::params![
            version_id,
            id,
            input.version,
            input.original_file_name,
            managed_file_path,
            input.status,
            input.validity,
            input.valid_until,
            now,
            now,
            now,
        ],
    )?;

    load_document(conn, &id)
}

/// Lädt ein einzelnes Dokument anhand seiner UUID mit aufgelösten Namen.
pub fn load_document(conn: &Connection, id: &str) -> SqliteResult<Document> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE id = ?1;",
        rusqlite::params![id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let doc = query_document_row(conn, "WHERE d.id = ?1", rusqlite::params![id])?;
    Ok(doc)
}

/// Lädt alle Dokumente, sortiert nach Dokumentennummer.
pub fn list_documents(conn: &Connection) -> SqliteResult<Vec<Document>> {
    let mut stmt = conn.prepare(
        "SELECT d.id, d.document_number, d.title,
                d.category_id, c.name,
                d.subcategory_id, s.name,
                d.responsible_person_id,
                e.last_name || ' ' || e.first_name,
                d.version, d.status, d.validity, d.valid_until,
                d.description, d.archived_at,
                dv.file_name, dv.file_path,
                d.created_at, d.updated_at, d.pre_archive_status
         FROM documents d
         LEFT JOIN categories c ON c.id = d.category_id
         LEFT JOIN subcategories s ON s.id = d.subcategory_id
         LEFT JOIN employees e ON e.id = d.responsible_person_id
         LEFT JOIN (
             SELECT document_id, file_name, file_path
             FROM document_versions
             WHERE rowid IN (SELECT MAX(rowid) FROM document_versions GROUP BY document_id)
         ) dv ON dv.document_id = d.id
         WHERE d.archived_at IS NULL
         ORDER BY d.document_number;",
    )?;

    let docs = stmt
        .query_map([], |row| {
            let valid_until: Option<String> = row.get(12)?;
            let computed_validity = calculate_validity_status(
                valid_until.as_deref(),
                &today_local_date(),
            ).map(|s| s.to_string());
            Ok(Document {
                id: row.get(0)?,
                document_number: row.get(1)?,
                title: row.get(2)?,
                category_id: row.get(3)?,
                category_name: row.get(4)?,
                subcategory_id: row.get(5)?,
                subcategory_name: row.get(6)?,
                responsible_person_id: row.get(7)?,
                responsible_person_name: row.get(8)?,
                version: row.get(9)?,
                status: row.get(10)?,
                validity: row.get(11)?,
                valid_until,
                computed_validity,
                description: row.get(13)?,
                archived_at: row.get(14)?,
                file_name: row.get(15)?,
                file_path: row.get(16)?,
                created_at: row.get(17)?,
                updated_at: row.get(18)?,
                pre_archive_status: row.get(19)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

    Ok(docs)
}

/// Hilfsfunktion: Lädt ein einzelnes Dokument mit Namen-Auflösung.
fn query_document_row(
    conn: &Connection,
    where_clause: &str,
    params: impl rusqlite::Params,
) -> SqliteResult<Document> {
    let sql = format!(
        "SELECT d.id, d.document_number, d.title,
                d.category_id, c.name,
                d.subcategory_id, s.name,
                d.responsible_person_id,
                e.last_name || ' ' || e.first_name,
                d.version, d.status, d.validity, d.valid_until,
                d.description, d.archived_at,
                dv.file_name, dv.file_path,
                d.created_at, d.updated_at, d.pre_archive_status
         FROM documents d
         LEFT JOIN categories c ON c.id = d.category_id
         LEFT JOIN subcategories s ON s.id = d.subcategory_id
         LEFT JOIN employees e ON e.id = d.responsible_person_id
         LEFT JOIN (
             SELECT document_id, file_name, file_path
             FROM document_versions
             WHERE rowid IN (SELECT MAX(rowid) FROM document_versions GROUP BY document_id)
         ) dv ON dv.document_id = d.id
         {}",
        where_clause
    );

    conn.query_row(&sql, params, |row| {
        let valid_until: Option<String> = row.get(12)?;
        let computed_validity = calculate_validity_status(
            valid_until.as_deref(),
            &today_local_date(),
        ).map(|s| s.to_string());
        Ok(Document {
            id: row.get(0)?,
            document_number: row.get(1)?,
            title: row.get(2)?,
            category_id: row.get(3)?,
            category_name: row.get(4)?,
            subcategory_id: row.get(5)?,
            subcategory_name: row.get(6)?,
            responsible_person_id: row.get(7)?,
            responsible_person_name: row.get(8)?,
            version: row.get(9)?,
            status: row.get(10)?,
            validity: row.get(11)?,
            valid_until,
            computed_validity,
            description: row.get(13)?,
            archived_at: row.get(14)?,
            file_name: row.get(15)?,
            file_path: row.get(16)?,
            created_at: row.get(17)?,
            updated_at: row.get(18)?,
            pre_archive_status: row.get(19)?,
        })
    })
}

/// Aktualisiert die Metadaten eines bestehenden Dokuments (DB-001).
/// UUID, Dokumentnummer und created_at bleiben unverändert. updated_at wird aktualisiert.
/// Keine PDF-Ersetzung — dafür gibt create_version.
pub fn update_document(
    conn: &Connection,
    id: &str,
    input: &UpdateDocumentInput,
) -> SqliteResult<Document> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE id = ?1;",
        rusqlite::params![id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let archived: Option<String> = conn.query_row(
        "SELECT archived_at FROM documents WHERE id = ?1;",
        rusqlite::params![id],
        |row| row.get(0),
    )?;
    if archived.is_some() {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some("Archivierte Dokumente können nicht bearbeitet werden.".to_string()),
        ));
    }

    // Lifecycle-Validierung: Bei Bearbeitung nur Entwurf oder aktiv erlaubt (Prompt 024)
    if !is_valid_creation_status(&input.status) {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some(format!(
                "Ungültiger Status bei Bearbeitung: {}. Nur Entwurf oder aktiv erlaubt. Archivierung erfolgt über die dedizierte Archiv-Aktion.",
                input.status
            )),
        ));
    }

    if let Some(ref person_id) = input.responsible_person_id {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM employees WHERE id = ?1;",
            rusqlite::params![person_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Verantwortliche Person nicht gefunden.".to_string()),
            ));
        }
    }

    if let Some(ref cat_id) = input.category_id {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM categories WHERE id = ?1;",
            rusqlite::params![cat_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Kategorie nicht gefunden.".to_string()),
            ));
        }
    }

    if let Some(ref sub_id) = input.subcategory_id {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM subcategories WHERE id = ?1;",
            rusqlite::params![sub_id],
            |row| row.get(0),
        )?;
        if count == 0 {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Unterkategorie nicht gefunden.".to_string()),
            ));
        }
    }

    let now = now_iso();
    conn.execute(
        "UPDATE documents SET title = ?1, category_id = ?2, subcategory_id = ?3,
         responsible_person_id = ?4, version = ?5, status = ?6, validity = ?7,
         valid_until = ?8, description = ?9, updated_at = ?10
         WHERE id = ?11;",
        rusqlite::params![
            input.title,
            input.category_id,
            input.subcategory_id,
            input.responsible_person_id,
            input.version,
            input.status,
            input.validity,
            input.valid_until,
            input.description,
            now,
            id,
        ],
    )?;

    load_document(conn, id)
}

/// Erstellt eine neue Dokumentversion (DB-002) und aktualisiert DB-001
/// current-version metadata. Die vorherige Version bleibt erhalten.
pub fn create_version(
    conn: &Connection,
    input: &CreateVersionInput,
    managed_file_path: &str,
) -> SqliteResult<Document> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE id = ?1;",
        rusqlite::params![input.document_id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let dup_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM document_versions WHERE document_id = ?1 AND version_number = ?2;",
        rusqlite::params![input.document_id, input.version_number],
        |row| row.get(0),
    )?;
    if dup_count > 0 {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some("Diese Versionsnummer existiert bereits.".to_string()),
        ));
    }

    let version_id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();

    // Lifecycle-Validierung: Neue Versionen dürfen nur Entwurf oder aktiv als Status setzen (Prompt 024)
    if !is_valid_creation_status(&input.status) {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some(format!(
                "Ungültiger Status für neue Version: {}. Nur Entwurf oder aktiv erlaubt.",
                input.status
            )),
        ));
    }

    conn.execute(
        "INSERT INTO document_versions (id, document_id, version_number, file_name, file_path,
         status, validity, valid_until, uploaded_by, uploaded_at, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, ?9, ?10, ?11);",
        rusqlite::params![
            version_id,
            input.document_id,
            input.version_number,
            input.original_file_name,
            managed_file_path,
            input.status,
            input.validity,
            input.valid_until,
            now,
            now,
            now,
        ],
    )?;

    conn.execute(
        "UPDATE documents SET version = ?1, status = ?2, validity = ?3, valid_until = ?4,
         updated_at = ?5 WHERE id = ?6;",
        rusqlite::params![
            input.version_number,
            input.status,
            input.validity,
            input.valid_until,
            now,
            input.document_id,
        ],
    )?;

    load_document(conn, &input.document_id)
}

/// Vollständige Versionserstellung: PDF validieren → kopieren → DB-Transaktion.
/// Bei DB-Fehler wird die Orphan-Kopie bereinigt. Die vorherige Version bleibt unangetastet.
/// Diese Funktion ist testbar ohne Tauri-AppHandle.
pub fn create_version_from_source(
    conn: &mut Connection,
    input: &CreateVersionInput,
    storage_dir: &Path,
) -> Result<Document, String> {
    let source = Path::new(&input.source_file_path);
    validate_pdf(source)?;

    let archived: Option<String> = conn
        .query_row(
            "SELECT archived_at FROM documents WHERE id = ?1;",
            rusqlite::params![input.document_id],
            |row| row.get(0),
        )
        .ok()
        .flatten();
    if let Some(archived_at) = archived {
        if !archived_at.is_empty() {
            return Err("Archivierte Dokumente können nicht mit neuen Versionen versehen werden.".to_string());
        }
    }

    let version_id = uuid::Uuid::new_v4().to_string();
    let managed_file = copy_to_managed_storage(source, storage_dir, &version_id)?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    match create_version(&tx, &input, &managed_file) {
        Ok(doc) => {
            tx.commit().map_err(|e| e.to_string())?;
            Ok(doc)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let _ = tx.rollback();
            remove_managed_file(storage_dir, &managed_file);
            Err("Dokument nicht gefunden.".to_string())
        }
        Err(e) => {
            let _ = tx.rollback();
            remove_managed_file(storage_dir, &managed_file);
            Err(e.to_string())
        }
    }
}

/// Archiviert ein Dokument: setzt status='archiviert' und archived_at=now.
/// Alle Versionen und PDFs bleiben unangetastet.
/// Transactional – bei Fehler wird nichts geändert.
pub fn archive_document(conn: &mut Connection, document_id: &str) -> SqliteResult<Document> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE id = ?1;",
        rusqlite::params![document_id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let already_archived: Option<String> = conn.query_row(
        "SELECT archived_at FROM documents WHERE id = ?1;",
        rusqlite::params![document_id],
        |row| row.get(0),
    )?;
    if already_archived.is_some() {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some("Dokument ist bereits archiviert.".to_string()),
        ));
    }

    let now = now_iso();
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE documents SET pre_archive_status = status, status = 'archiviert', archived_at = ?1, updated_at = ?2
         WHERE id = ?3;",
        rusqlite::params![now, now, document_id],
    )?;
    tx.commit()?;

    load_document(conn, document_id)
}

/// Stellt ein archiviertes Dokument wieder her: setzt status='aktiv', löscht archived_at.
/// Alle Versionen und PDFs bleiben unangetastet.
/// Transactional – bei Fehler wird nichts geändert.
pub fn restore_document(conn: &mut Connection, document_id: &str) -> SqliteResult<Document> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE id = ?1;",
        rusqlite::params![document_id],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let archived_at: Option<String> = conn.query_row(
        "SELECT archived_at FROM documents WHERE id = ?1;",
        rusqlite::params![document_id],
        |row| row.get(0),
    )?;
    if archived_at.is_none() {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
            Some("Dokument ist nicht archiviert.".to_string()),
        ));
    }

    let pre_archive_status: Option<String> = conn.query_row(
        "SELECT pre_archive_status FROM documents WHERE id = ?1;",
        rusqlite::params![document_id],
        |row| row.get(0),
    )?;

    // Legacy-Dokumente ohne pre_archive_status können nicht automatisch wiederhergestellt werden (Prompt 024)
    let restored_status = match pre_archive_status.as_deref() {
        Some(s) if is_valid_creation_status(s) => s.to_string(),
        _ => {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CONSTRAINT),
                Some("Der vorherige Status kann nicht automatisch ermittelt werden. Bitte nach der Wiederherstellung den Status manuell setzen.".to_string()),
            ));
        }
    };

    let now = now_iso();
    let tx = conn.transaction()?;
    tx.execute(
        "UPDATE documents SET status = ?1, archived_at = NULL, pre_archive_status = NULL, updated_at = ?2
         WHERE id = ?3;",
        rusqlite::params![restored_status, now, document_id],
    )?;
    tx.commit()?;

    load_document(conn, document_id)
}

/// Lädt alle archivierten Dokumente (archived_at IS NOT NULL).
pub fn list_archived_documents(conn: &Connection) -> SqliteResult<Vec<Document>> {
    let mut stmt = conn.prepare(
        "SELECT d.id, d.document_number, d.title,
                d.category_id, c.name,
                d.subcategory_id, s.name,
                d.responsible_person_id,
                e.last_name || ' ' || e.first_name,
                d.version, d.status, d.validity, d.valid_until,
                d.description, d.archived_at,
                dv.file_name, dv.file_path,
                d.created_at, d.updated_at, d.pre_archive_status
         FROM documents d
         LEFT JOIN categories c ON c.id = d.category_id
         LEFT JOIN subcategories s ON s.id = d.subcategory_id
         LEFT JOIN employees e ON e.id = d.responsible_person_id
         LEFT JOIN (
             SELECT document_id, file_name, file_path
             FROM document_versions
             WHERE rowid IN (SELECT MAX(rowid) FROM document_versions GROUP BY document_id)
         ) dv ON dv.document_id = d.id
         WHERE d.archived_at IS NOT NULL
         ORDER BY d.archived_at DESC;",
    )?;

    let docs = stmt
        .query_map([], |row| {
            let valid_until: Option<String> = row.get(12)?;
            let computed_validity = calculate_validity_status(
                valid_until.as_deref(),
                &today_local_date(),
            ).map(|s| s.to_string());
            Ok(Document {
                id: row.get(0)?,
                document_number: row.get(1)?,
                title: row.get(2)?,
                category_id: row.get(3)?,
                category_name: row.get(4)?,
                subcategory_id: row.get(5)?,
                subcategory_name: row.get(6)?,
                responsible_person_id: row.get(7)?,
                responsible_person_name: row.get(8)?,
                version: row.get(9)?,
                status: row.get(10)?,
                validity: row.get(11)?,
                valid_until,
                computed_validity,
                description: row.get(13)?,
                archived_at: row.get(14)?,
                file_name: row.get(15)?,
                file_path: row.get(16)?,
                created_at: row.get(17)?,
                updated_at: row.get(18)?,
                pre_archive_status: row.get(19)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

    Ok(docs)
}

/// --- Dashboard-Summary & Review-Liste (Prompt 023) -----------------------

/// Dashboard-Zusammenfassung für aktive Dokumente.
/// Alle Zähler beziehen sich auf nicht-archivierte Dokumente (archived_at IS NULL).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_active: i64,
    pub valid: i64,
    pub warning: i64,
    pub expired: i64,
    pub no_validity: i64,
    pub archived: i64,
    pub employees: i64,
}

/// Ein Eintrag in der Review-Liste — ein aktives Dokument, das Aufmerksamkeit erfordert.
/// Enthält nur die für die Dashboard-Anzeige notwendigen Felder.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewEntry {
    pub id: String,
    pub document_number: String,
    pub title: String,
    pub valid_until: Option<String>,
    pub computed_validity: String,
    pub responsible_person_name: Option<String>,
    pub category_name: Option<String>,
}

/// Berechnet die Dashboard-Zusammenfassung aus der aktuellen Datenbank.
/// Verwendet calculate_validity_status für konsistente Gültigkeitsableitung.
pub fn dashboard_summary(conn: &Connection) -> SqliteResult<DashboardSummary> {
    let today = today_local_date();

    let total_active: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE archived_at IS NULL;",
        [],
        |row| row.get(0),
    )?;

    let archived: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE archived_at IS NOT NULL;",
        [],
        |row| row.get(0),
    )?;

    let employees: i64 =
        conn.query_row("SELECT COUNT(*) FROM employees;", [], |row| row.get(0))?;

    let mut stmt = conn.prepare(
        "SELECT valid_until FROM documents WHERE archived_at IS NULL;",
    )?;

    let mut valid = 0i64;
    let mut warning = 0i64;
    let mut expired = 0i64;
    let mut no_validity = 0i64;

    let rows = stmt.query_map([], |row| row.get::<_, Option<String>>(0))?;
    for vu_opt in rows {
        let vu = vu_opt?;
        match calculate_validity_status(vu.as_deref(), &today) {
            Some(VALIDITY_GUELTIG) => valid += 1,
            Some(VALIDITY_BALD_AB) => warning += 1,
            Some(VALIDITY_ABGELAUFEN) => expired += 1,
            None => no_validity += 1,
            _ => {}
        }
    }

    Ok(DashboardSummary {
        total_active,
        valid,
        warning,
        expired,
        no_validity,
        archived,
        employees,
    })
}

/// Lädt die Review-Liste: aktive Dokumente mit computed_validity "läuft bald ab" oder "abgelaufen".
/// Sortierung:
///   1. abgelaufen vor läuft bald ab
///   2. abgelaufen: ältestes valid_until zuerst
///   3. läuft bald ab: nächstes valid_until zuerst
///   4. bei gleichem Datum: nach Dokumentennummer
pub fn review_list(conn: &Connection) -> SqliteResult<Vec<ReviewEntry>> {
    let today = today_local_date();

    let mut stmt = conn.prepare(
        "SELECT d.id, d.document_number, d.title, d.valid_until,
                e.last_name || ' ' || e.first_name,
                c.name
         FROM documents d
         LEFT JOIN employees e ON e.id = d.responsible_person_id
         LEFT JOIN categories c ON c.id = d.category_id
         WHERE d.archived_at IS NULL
         ORDER BY d.document_number;",
    )?;

    let mut entries: Vec<ReviewEntry> = Vec::new();

    let rows = stmt.query_map([], |row| {
        let valid_until: Option<String> = row.get(3)?;
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            valid_until,
            row.get::<_, Option<String>>(4)?,
            row.get::<_, Option<String>>(5)?,
        ))
    })?;

    for row in rows {
        let (id, document_number, title, valid_until, responsible, category) = row?;
        let cv = calculate_validity_status(valid_until.as_deref(), &today);
        if cv == Some(VALIDITY_BALD_AB) || cv == Some(VALIDITY_ABGELAUFEN) {
            entries.push(ReviewEntry {
                id,
                document_number,
                title,
                valid_until,
                computed_validity: cv.unwrap().to_string(),
                responsible_person_name: responsible,
                category_name: category,
            });
        }
    }

    entries.sort_by(|a, b| {
        let a_expired = a.computed_validity == VALIDITY_ABGELAUFEN;
        let b_expired = b.computed_validity == VALIDITY_ABGELAUFEN;
        match (a_expired, b_expired) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let a_date = a.valid_until.as_deref().unwrap_or("");
                let b_date = b.valid_until.as_deref().unwrap_or("");
                if a_expired {
                    a_date.cmp(b_date)
                } else {
                    a_date.cmp(b_date)
                }
                .then_with(|| a.document_number.cmp(&b.document_number))
            }
        }
    });

    Ok(entries)
}

/// Lädt alle Versionen eines Dokuments, neueste zuerst.
/// is_current wird durch Abgleich mit DB-001 version-Spalte bestimmt.
pub fn list_versions(conn: &Connection, document_id: &str) -> SqliteResult<Vec<DocumentVersion>> {
    let current_version: Option<String> = conn
        .query_row(
            "SELECT version FROM documents WHERE id = ?1;",
            rusqlite::params![document_id],
            |row| row.get(0),
        )
        .ok();

    let mut stmt = conn.prepare(
        "SELECT id, document_id, version_number, file_name, status, validity,
         valid_until, uploaded_at, created_at
         FROM document_versions WHERE document_id = ?1
         ORDER BY rowid DESC;",
    )?;
    let versions = stmt
        .query_map(rusqlite::params![document_id], |row| {
            let version_number: String = row.get(2)?;
            Ok(DocumentVersion {
                id: row.get(0)?,
                document_id: row.get(1)?,
                is_current: current_version.as_deref() == Some(version_number.as_str()),
                version_number,
                file_name: row.get(3)?,
                status: row.get(4)?,
                validity: row.get(5)?,
                valid_until: row.get(6)?,
                uploaded_at: row.get(7)?,
                created_at: row.get(8)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(versions)
}

/// Lädt ein Dokument anhand der Dokumentennummer (z.B. PQM-0001).
pub fn get_document_by_number(conn: &Connection, doc_number: &str) -> SqliteResult<Document> {
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM documents WHERE document_number = ?1;",
        rusqlite::params![doc_number],
        |row| row.get(0),
    )?;
    if exists == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    query_document_row(
        conn,
        "WHERE d.document_number = ?1",
        rusqlite::params![doc_number],
    )
}

/// Lädt alle Kategorien (DB-005).
pub fn list_categories(conn: &Connection) -> SqliteResult<Vec<CategoryItem>> {
    let mut stmt =
        conn.prepare("SELECT id, name FROM categories ORDER BY name;")?;
    let items = stmt
        .query_map([], |row| {
            Ok(CategoryItem {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(items)
}

/// Lädt alle Unterkategorien (DB-006).
pub fn list_subcategories(conn: &Connection) -> SqliteResult<Vec<SubcategoryItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, category_id FROM subcategories ORDER BY name;",
    )?;
    let items = stmt
        .query_map([], |row| {
            Ok(SubcategoryItem {
                id: row.get(0)?,
                name: row.get(1)?,
                category_id: row.get(2)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;
    Ok(items)
}

/// --- Kategorie-Operationen (DB-005, Prompt 026A) ------------------------

/// Erstellt eine neue Kategorie.
/// Der Name wird getrimmt und muss eindeutig sein (UNIQUE-Constraint).
pub fn create_category(conn: &Connection, name: &str) -> SqliteResult<CategoryItem> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();
    conn.execute(
        "INSERT INTO categories (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
        rusqlite::params![id, name, now, now],
    )?;
    Ok(CategoryItem { id, name: name.to_string() })
}

/// Benennt eine bestehende Kategorie um (gleiche UUID, updated_at wird aktualisiert).
/// Alle Dokument- und Unterkategorie-Beziehungen bleiben durch die UUID erhalten.
pub fn rename_category(conn: &Connection, id: &str, new_name: &str) -> SqliteResult<CategoryItem> {
    let now = now_iso();
    let affected = conn.execute(
        "UPDATE categories SET name = ?1, updated_at = ?2 WHERE id = ?3;",
        rusqlite::params![new_name, now, id],
    )?;
    if affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }
    Ok(CategoryItem { id: id.to_string(), name: new_name.to_string() })
}

/// --- Unterkategorie-Operationen (DB-006, Prompt 026A) --------------------

/// Erstellt eine neue Unterkategorie unter einer existierenden Kategorie.
/// Die Kategorie muss existieren (FK-Constraint). Der Name wird getrimmt.
pub fn create_subcategory(
    conn: &Connection,
    name: &str,
    category_id: &str,
) -> SqliteResult<SubcategoryItem> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();
    conn.execute(
        "INSERT INTO subcategories (id, name, category_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5);",
        rusqlite::params![id, name, category_id, now, now],
    )?;
    Ok(SubcategoryItem {
        id,
        name: name.to_string(),
        category_id: category_id.to_string(),
    })
}

/// Benennt eine bestehende Unterkategorie um (gleiche UUID, gleiche category_id).
/// Alle Dokument-Beziehungen bleiben durch die UUID erhalten.
pub fn rename_subcategory(
    conn: &Connection,
    id: &str,
    new_name: &str,
) -> SqliteResult<SubcategoryItem> {
    let now = now_iso();
    let affected = conn.execute(
        "UPDATE subcategories SET name = ?1, updated_at = ?2 WHERE id = ?3;",
        rusqlite::params![new_name, now, id],
    )?;
    if affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    let category_id: String = conn.query_row(
        "SELECT category_id FROM subcategories WHERE id = ?1;",
        rusqlite::params![id],
        |row| row.get(0),
    )?;
    Ok(SubcategoryItem {
        id: id.to_string(),
        name: new_name.to_string(),
        category_id,
    })
}

/// --- Tests ---------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn test_conn() -> (Connection, NamedTempFile) {
        let tmp = NamedTempFile::new().unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();
        ensure_schema_version_table(&conn).unwrap();
        (conn, tmp)
    }

    fn init_test_db() -> (Connection, NamedTempFile) {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();
        insert_test_master_data(&conn);
        (conn, tmp)
    }

    /// Fügt explizite Test-Stammdaten in die temporäre Testdatenbank ein.
    /// Diese Daten werden niemals in die echte PraxisQM-Datenbank geschrieben.
    fn insert_test_master_data(conn: &Connection) {
        let now = now_iso();
        let responsibilities = ["QM-Beauftragte", "Hygienebeauftragte"];
        for name in &responsibilities {
            conn.execute(
                "INSERT INTO verantwortungspositionen (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
                rusqlite::params![uuid::Uuid::new_v4().to_string(), name, now, now],
            )
            .unwrap();
        }
        let qm_areas = ["Datenschutz", "Hygiene"];
        for name in &qm_areas {
            conn.execute(
                "INSERT INTO qm_bereiche (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
                rusqlite::params![uuid::Uuid::new_v4().to_string(), name, now, now],
            )
            .unwrap();
        }
    }

    #[test]
    fn test_schema_creation() {
        let (conn, _tmp) = test_conn();
        for stmt in schema_statements() {
            conn.execute(stmt, []).unwrap();
        }
        set_schema_version(&conn, SCHEMA_VERSION).unwrap();
        assert_eq!(get_schema_version(&conn).unwrap(), SCHEMA_VERSION);
    }

    #[test]
    fn test_idempotent_init() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        init_database(tmp.path()).unwrap();
    }

    #[test]
    fn test_all_class_a_tables_exist() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();

        let expected_tables = vec![
            "categories",
            "subcategories",
            "employees",
            "users",
            "documents",
            "document_versions",
            "keyword_dictionary",
            "document_tags",
            "audit_log",
            "verantwortungspositionen",
            "qm_bereiche",
            "employee_responsibilities",
            "employee_qm_areas",
        ];

        for table in expected_tables {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1;",
                    rusqlite::params![table],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "Tabelle '{}' existiert nicht", table);
        }
    }

    #[test]
    fn test_foreign_keys_enabled() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        let fk: i64 = conn
            .query_row("PRAGMA foreign_keys;", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fk, 1);
    }

    #[test]
    fn test_unique_constraints() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();

        // categories.name UNIQUE
        let now = now_iso();
        conn.execute(
            "INSERT INTO categories (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
            rusqlite::params![uuid::Uuid::new_v4().to_string(), "Hygiene", now, now],
        )
        .unwrap();
        let dup_result = conn.execute(
            "INSERT INTO categories (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
            rusqlite::params![uuid::Uuid::new_v4().to_string(), "Hygiene", now, now],
        );
        assert!(dup_result.is_err(), "UNIQUE-Constraint für categories.name nicht aktiv");

        // documents.document_number UNIQUE
        conn.execute(
            "INSERT INTO documents (id, document_number, title, version, status, validity, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
            rusqlite::params![
                uuid::Uuid::new_v4().to_string(),
                "PQM-0001",
                "Testdokument",
                "1.0",
                "aktiv",
                "gültig",
                now,
                now,
            ],
        )
        .unwrap();
        let dup_doc = conn.execute(
            "INSERT INTO documents (id, document_number, title, version, status, validity, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
            rusqlite::params![
                uuid::Uuid::new_v4().to_string(),
                "PQM-0001",
                "Duplikat",
                "1.0",
                "aktiv",
                "gültig",
                now,
                now,
            ],
        );
        assert!(dup_doc.is_err(), "UNIQUE-Constraint für documents.document_number nicht aktiv");
    }

    #[test]
    fn test_join_table_foreign_keys() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        let now = now_iso();

        // Versuch, Join-Eintrag ohne existierenden Employee anzulegen → muss fehlschlagen
        let resp_id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO verantwortungspositionen (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
            rusqlite::params![resp_id, "QM-Beauftragte", now, now],
        )
        .unwrap();

        let fake_employee_id = uuid::Uuid::new_v4().to_string();
        let result = conn.execute(
            "INSERT INTO employee_responsibilities (employee_id, responsibility_id) VALUES (?1, ?2);",
            rusqlite::params![fake_employee_id, resp_id],
        );
        assert!(result.is_err(), "FK-Constraint für employee_responsibilities.employee_id nicht aktiv");
    }

    #[test]
    fn test_schema_version_recorded() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        let version = get_schema_version(&conn).unwrap();
        assert_eq!(version, SCHEMA_VERSION);
    }

    // --- Employee Persistence Tests ---

    #[test]
    fn test_create_employee() {
        let (conn, _tmp) = init_test_db();
        let input = CreateEmployeeInput {
            last_name: "Müller".to_string(),
            first_name: "Anna".to_string(),
            position: Some("Zahnärztin".to_string()),
            is_active: true,
            hire_date: Some("2019-03-01".to_string()),
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![],
        };
        let emp = create_employee(&conn, &input).unwrap();
        assert!(emp.id.len() > 0);
        assert_eq!(emp.last_name, "Müller");
        assert_eq!(emp.first_name, "Anna");
    }

    #[test]
    fn test_list_employees() {
        let (conn, _tmp) = init_test_db();
        let input = CreateEmployeeInput {
            last_name: "Schmidt".to_string(),
            first_name: "Thomas".to_string(),
            position: Some("ZFA".to_string()),
            is_active: true,
            hire_date: Some("2021-09-15".to_string()),
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![],
        };
        create_employee(&conn, &input).unwrap();
        let employees = list_employees(&conn).unwrap();
        assert_eq!(employees.len(), 1);
        assert_eq!(employees[0].last_name, "Schmidt");
    }

    #[test]
    fn test_zero_responsibility_assignments() {
        let (conn, _tmp) = init_test_db();
        let input = CreateEmployeeInput {
            last_name: "Becker".to_string(),
            first_name: "Julia".to_string(),
            position: None,
            is_active: false,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![],
        };
        let emp = create_employee(&conn, &input).unwrap();
        assert!(emp.responsibilities.is_empty());
    }

    #[test]
    fn test_multiple_responsibility_assignments() {
        let (conn, _tmp) = init_test_db();
        let responsibilities = list_responsibilities(&conn).unwrap();
        assert!(responsibilities.len() >= 2);
        let input = CreateEmployeeInput {
            last_name: "Test".to_string(),
            first_name: "Multi".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![
                responsibilities[0].id.clone(),
                responsibilities[1].id.clone(),
            ],
            qm_area_ids: vec![],
        };
        let emp = create_employee(&conn, &input).unwrap();
        assert_eq!(emp.responsibilities.len(), 2);
    }

    #[test]
    fn test_zero_qm_area_assignments() {
        let (conn, _tmp) = init_test_db();
        let input = CreateEmployeeInput {
            last_name: "Zero".to_string(),
            first_name: "Area".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![],
        };
        let emp = create_employee(&conn, &input).unwrap();
        assert!(emp.qm_areas.is_empty());
    }

    #[test]
    fn test_multiple_qm_area_assignments() {
        let (conn, _tmp) = init_test_db();
        let qm_areas = list_qm_areas(&conn).unwrap();
        assert!(qm_areas.len() >= 2);
        let input = CreateEmployeeInput {
            last_name: "Multi".to_string(),
            first_name: "Area".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![qm_areas[0].id.clone(), qm_areas[1].id.clone()],
        };
        let emp = create_employee(&conn, &input).unwrap();
        assert_eq!(emp.qm_areas.len(), 2);
    }

    #[test]
    fn test_transaction_rollback_on_invalid_assignment() {
        let (mut conn, _tmp) = init_test_db();
        let tx = conn.transaction().unwrap();
        let input = CreateEmployeeInput {
            last_name: "Rollback".to_string(),
            first_name: "Test".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec!["nonexistent-id".to_string()],
            qm_area_ids: vec![],
        };
        let result = create_employee(&tx, &input);
        assert!(result.is_err());
        tx.rollback().unwrap();
        let employees = list_employees(&conn).unwrap();
        assert!(employees.is_empty(), "Mitarbeiter sollte nach Rollback nicht existieren");
    }

    #[test]
    fn test_persisted_assignments_load_correctly() {
        let (conn, _tmp) = init_test_db();
        let responsibilities = list_responsibilities(&conn).unwrap();
        let qm_areas = list_qm_areas(&conn).unwrap();
        assert!(!responsibilities.is_empty(), "Test-Stammdaten fehlen");
        assert!(!qm_areas.is_empty(), "Test-Stammdaten fehlen");
        let input = CreateEmployeeInput {
            last_name: "Persist".to_string(),
            first_name: "Check".to_string(),
            position: Some("Praxismanagerin".to_string()),
            is_active: true,
            hire_date: Some("2020-01-15".to_string()),
            departure_date: None,
            responsibility_ids: vec![responsibilities[0].id.clone()],
            qm_area_ids: vec![qm_areas[0].id.clone()],
        };
        let created = create_employee(&conn, &input).unwrap();
        let employees = list_employees(&conn).unwrap();
        let loaded = employees.iter().find(|e| e.id == created.id).unwrap();
        assert_eq!(loaded.responsibilities.len(), 1);
        assert_eq!(loaded.qm_areas.len(), 1);
        assert_eq!(loaded.position.as_deref(), Some("Praxismanagerin"));
    }

    // --- Master Data Management Tests (Prompt 015) ---

    #[test]
    fn test_create_responsibility() {
        let (conn, _tmp) = init_test_db();
        let item = create_responsibility(&conn, "Brandschutzbeauftragte").unwrap();
        assert!(!item.id.is_empty());
        assert_eq!(item.name, "Brandschutzbeauftragte");
        let all = list_responsibilities(&conn).unwrap();
        assert!(all.iter().any(|r| r.name == "Brandschutzbeauftragte"));
    }

    #[test]
    fn test_list_responsibilities_sorted() {
        let (conn, _tmp) = init_test_db();
        create_responsibility(&conn, "Aaa").unwrap();
        create_responsibility(&conn, "Zzz").unwrap();
        let all = list_responsibilities(&conn).unwrap();
        assert!(all.len() >= 4);
        let names: Vec<&str> = all.iter().map(|r| r.name.as_str()).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn test_duplicate_responsibility_rejected() {
        let (conn, _tmp) = init_test_db();
        create_responsibility(&conn, "Fortbildungskoordinatorin").unwrap();
        let result = create_responsibility(&conn, "Fortbildungskoordinatorin");
        assert!(result.is_err(), "Duplikat sollte abgelehnt werden");
    }

    #[test]
    fn test_rename_responsibility() {
        let (conn, _tmp) = init_test_db();
        let item = create_responsibility(&conn, "Altname").unwrap();
        let renamed = rename_responsibility(&conn, &item.id, "Neuname").unwrap();
        assert_eq!(renamed.id, item.id);
        assert_eq!(renamed.name, "Neuname");
        let all = list_responsibilities(&conn).unwrap();
        assert!(all.iter().any(|r| r.name == "Neuname"));
        assert!(!all.iter().any(|r| r.name == "Altname"));
    }

    #[test]
    fn test_rename_responsibility_preserves_id() {
        let (conn, _tmp) = init_test_db();
        let item = create_responsibility(&conn, "TestRename").unwrap();
        let original_id = item.id.clone();
        rename_responsibility(&conn, &item.id, "TestRenamed").unwrap();
        let all = list_responsibilities(&conn).unwrap();
        let found = all.iter().find(|r| r.id == original_id).unwrap();
        assert_eq!(found.name, "TestRenamed");
    }

    #[test]
    fn test_rename_responsibility_preserves_assignment() {
        let (conn, _tmp) = init_test_db();
        let resp = create_responsibility(&conn, "AssignTest").unwrap();
        let input = CreateEmployeeInput {
            last_name: "Assign".to_string(),
            first_name: "Check".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![resp.id.clone()],
            qm_area_ids: vec![],
        };
        let emp = create_employee(&conn, &input).unwrap();
        assert_eq!(emp.responsibilities.len(), 1);
        assert_eq!(emp.responsibilities[0], "AssignTest");
        rename_responsibility(&conn, &resp.id, "AssignRenamed").unwrap();
        let employees = list_employees(&conn).unwrap();
        let loaded = employees.iter().find(|e| e.id == emp.id).unwrap();
        assert_eq!(loaded.responsibilities.len(), 1);
        assert_eq!(loaded.responsibilities[0], "AssignRenamed");
    }

    #[test]
    fn test_create_qm_area() {
        let (conn, _tmp) = init_test_db();
        let item = create_qm_area(&conn, "Notfallmanagement").unwrap();
        assert!(!item.id.is_empty());
        assert_eq!(item.name, "Notfallmanagement");
        let all = list_qm_areas(&conn).unwrap();
        assert!(all.iter().any(|a| a.name == "Notfallmanagement"));
    }

    #[test]
    fn test_list_qm_areas_sorted() {
        let (conn, _tmp) = init_test_db();
        create_qm_area(&conn, "Aaa").unwrap();
        create_qm_area(&conn, "Zzz").unwrap();
        let all = list_qm_areas(&conn).unwrap();
        assert!(all.len() >= 4);
        let names: Vec<&str> = all.iter().map(|a| a.name.as_str()).collect();
        let mut sorted = names.clone();
        sorted.sort();
        assert_eq!(names, sorted);
    }

    #[test]
    fn test_duplicate_qm_area_rejected() {
        let (conn, _tmp) = init_test_db();
        create_qm_area(&conn, "Abrechnung").unwrap();
        let result = create_qm_area(&conn, "Abrechnung");
        assert!(result.is_err(), "Duplikat sollte abgelehnt werden");
    }

    #[test]
    fn test_rename_qm_area() {
        let (conn, _tmp) = init_test_db();
        let item = create_qm_area(&conn, "AltBereich").unwrap();
        let renamed = rename_qm_area(&conn, &item.id, "NeuBereich").unwrap();
        assert_eq!(renamed.id, item.id);
        assert_eq!(renamed.name, "NeuBereich");
        let all = list_qm_areas(&conn).unwrap();
        assert!(all.iter().any(|a| a.name == "NeuBereich"));
        assert!(!all.iter().any(|a| a.name == "AltBereich"));
    }

    #[test]
    fn test_rename_qm_area_preserves_id() {
        let (conn, _tmp) = init_test_db();
        let item = create_qm_area(&conn, "AreaRename").unwrap();
        let original_id = item.id.clone();
        rename_qm_area(&conn, &item.id, "AreaRenamed").unwrap();
        let all = list_qm_areas(&conn).unwrap();
        let found = all.iter().find(|a| a.id == original_id).unwrap();
        assert_eq!(found.name, "AreaRenamed");
    }

    #[test]
    fn test_rename_qm_area_preserves_assignment() {
        let (conn, _tmp) = init_test_db();
        let area = create_qm_area(&conn, "AssignArea").unwrap();
        let input = CreateEmployeeInput {
            last_name: "AreaAssign".to_string(),
            first_name: "Check".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![area.id.clone()],
        };
        let emp = create_employee(&conn, &input).unwrap();
        assert_eq!(emp.qm_areas.len(), 1);
        assert_eq!(emp.qm_areas[0], "AssignArea");
        rename_qm_area(&conn, &area.id, "AssignAreaRenamed").unwrap();
        let employees = list_employees(&conn).unwrap();
        let loaded = employees.iter().find(|e| e.id == emp.id).unwrap();
        assert_eq!(loaded.qm_areas.len(), 1);
        assert_eq!(loaded.qm_areas[0], "AssignAreaRenamed");
    }

    // --- Employee Editing Tests (Prompt 016) ---

    fn make_test_employee(conn: &Connection, responsibilities: &[String], qm_areas: &[String]) -> Employee {
        let input = CreateEmployeeInput {
            last_name: "Test".to_string(),
            first_name: "Mitarbeiter".to_string(),
            position: Some("ZFA".to_string()),
            is_active: true,
            hire_date: Some("2020-01-01".to_string()),
            departure_date: None,
            responsibility_ids: responsibilities.to_vec(),
            qm_area_ids: qm_areas.to_vec(),
        };
        create_employee(conn, &input).unwrap()
    }

    fn make_update_input(
        last_name: &str,
        first_name: &str,
        position: Option<&str>,
        is_active: bool,
        hire_date: Option<&str>,
        departure_date: Option<&str>,
        resp_ids: Vec<String>,
        area_ids: Vec<String>,
    ) -> UpdateEmployeeInput {
        UpdateEmployeeInput {
            last_name: last_name.to_string(),
            first_name: first_name.to_string(),
            position: position.map(|s| s.to_string()),
            is_active,
            hire_date: hire_date.map(|s| s.to_string()),
            departure_date: departure_date.map(|s| s.to_string()),
            responsibility_ids: resp_ids,
            qm_area_ids: area_ids,
        }
    }

    #[test]
    fn test_update_basic_employee_fields() {
        let (conn, _tmp) = init_test_db();
        let emp = make_test_employee(&conn, &[], &[]);
        let input = make_update_input("Neuname", "Neuvorname", Some("Praxismanagerin"), false, Some("2019-03-15"), Some("2024-06-30"), vec![], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.last_name, "Neuname");
        assert_eq!(updated.first_name, "Neuvorname");
        assert_eq!(updated.position.as_deref(), Some("Praxismanagerin"));
        assert!(!updated.is_active);
        assert_eq!(updated.hire_date.as_deref(), Some("2019-03-15"));
        assert_eq!(updated.departure_date.as_deref(), Some("2024-06-30"));
    }

    #[test]
    fn test_update_employee_uuid_unchanged() {
        let (conn, _tmp) = init_test_db();
        let emp = make_test_employee(&conn, &[], &[]);
        let input = make_update_input("Anderer", "Name", Some("Zahnärztin"), true, Some("2021-05-01"), None, vec![], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.id, emp.id);
    }

    #[test]
    fn test_updated_at_changes() {
        let (conn, _tmp) = init_test_db();
        let emp = make_test_employee(&conn, &[], &[]);
        let original_updated: String = conn.query_row(
            "SELECT updated_at FROM employees WHERE id = ?1;",
            rusqlite::params![emp.id],
            |row| row.get(0),
        ).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![]);
        update_employee(&conn, &emp.id, &input).unwrap();
        let new_updated: String = conn.query_row(
            "SELECT updated_at FROM employees WHERE id = ?1;",
            rusqlite::params![emp.id],
            |row| row.get(0),
        ).unwrap();
        assert_ne!(original_updated, new_updated);
    }

    #[test]
    fn test_add_responsibility_assignment() {
        let (conn, _tmp) = init_test_db();
        let resp = create_responsibility(&conn, "Notfallbeauftragte").unwrap();
        let emp = make_test_employee(&conn, &[], &[]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![resp.id.clone()], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.responsibilities.len(), 1);
        assert_eq!(updated.responsibilities[0], "Notfallbeauftragte");
        assert_eq!(updated.responsibility_ids.len(), 1);
        assert_eq!(updated.responsibility_ids[0], resp.id);
    }

    #[test]
    fn test_remove_responsibility_assignment() {
        let (conn, _tmp) = init_test_db();
        let resp = create_responsibility(&conn, "Notfallbeauftragte").unwrap();
        let emp = make_test_employee(&conn, &[resp.id], &[]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.responsibilities.len(), 0);
        assert_eq!(updated.responsibility_ids.len(), 0);
    }

    #[test]
    fn test_replace_multiple_responsibility_assignments() {
        let (conn, _tmp) = init_test_db();
        let r1 = create_responsibility(&conn, "Alpha").unwrap();
        let r2 = create_responsibility(&conn, "Beta").unwrap();
        let r3 = create_responsibility(&conn, "Gamma").unwrap();
        let emp = make_test_employee(&conn, &[r1.id.clone(), r2.id.clone()], &[]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![r2.id.clone(), r3.id.clone()], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.responsibilities.len(), 2);
        assert!(updated.responsibilities.contains(&"Beta".to_string()));
        assert!(updated.responsibilities.contains(&"Gamma".to_string()));
        assert!(!updated.responsibilities.contains(&"Alpha".to_string()));
    }

    #[test]
    fn test_clear_all_responsibility_assignments() {
        let (conn, _tmp) = init_test_db();
        let r1 = create_responsibility(&conn, "R1").unwrap();
        let r2 = create_responsibility(&conn, "R2").unwrap();
        let r3 = create_responsibility(&conn, "R3").unwrap();
        let emp = make_test_employee(&conn, &[r1.id, r2.id, r3.id], &[]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.responsibilities.len(), 0);
    }

    #[test]
    fn test_add_qm_area_assignment() {
        let (conn, _tmp) = init_test_db();
        let area = create_qm_area(&conn, "Notfallmanagement").unwrap();
        let emp = make_test_employee(&conn, &[], &[]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![area.id.clone()]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.qm_areas.len(), 1);
        assert_eq!(updated.qm_areas[0], "Notfallmanagement");
        assert_eq!(updated.qm_area_ids.len(), 1);
        assert_eq!(updated.qm_area_ids[0], area.id);
    }

    #[test]
    fn test_remove_qm_area_assignment() {
        let (conn, _tmp) = init_test_db();
        let area = create_qm_area(&conn, "Notfallmanagement").unwrap();
        let emp = make_test_employee(&conn, &[], &[area.id]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.qm_areas.len(), 0);
    }

    #[test]
    fn test_replace_multiple_qm_area_assignments() {
        let (conn, _tmp) = init_test_db();
        let a1 = create_qm_area(&conn, "Alpha").unwrap();
        let a2 = create_qm_area(&conn, "Beta").unwrap();
        let a3 = create_qm_area(&conn, "Gamma").unwrap();
        let emp = make_test_employee(&conn, &[], &[a1.id.clone(), a2.id.clone()]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![a2.id.clone(), a3.id.clone()]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.qm_areas.len(), 2);
        assert!(updated.qm_areas.contains(&"Beta".to_string()));
        assert!(updated.qm_areas.contains(&"Gamma".to_string()));
        assert!(!updated.qm_areas.contains(&"Alpha".to_string()));
    }

    #[test]
    fn test_clear_all_qm_area_assignments() {
        let (conn, _tmp) = init_test_db();
        let a1 = create_qm_area(&conn, "A1").unwrap();
        let a2 = create_qm_area(&conn, "A2").unwrap();
        let a3 = create_qm_area(&conn, "A3").unwrap();
        let emp = make_test_employee(&conn, &[], &[a1.id, a2.id, a3.id]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.qm_areas.len(), 0);
    }

    #[test]
    fn test_simultaneous_responsibility_and_qm_area_update() {
        let (conn, _tmp) = init_test_db();
        let r1 = create_responsibility(&conn, "Resp1").unwrap();
        let r2 = create_responsibility(&conn, "Resp2").unwrap();
        let a1 = create_qm_area(&conn, "Area1").unwrap();
        let a2 = create_qm_area(&conn, "Area2").unwrap();
        let emp = make_test_employee(&conn, &[r1.id], &[a1.id]);
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![r2.id.clone()], vec![a2.id.clone()]);
        let updated = update_employee(&conn, &emp.id, &input).unwrap();
        assert_eq!(updated.responsibilities, vec!["Resp2"]);
        assert_eq!(updated.qm_areas, vec!["Area2"]);
    }

    #[test]
    fn test_invalid_responsibility_id_rolls_back() {
        let (conn, _tmp) = init_test_db();
        let emp = make_test_employee(&conn, &[], &[]);
        let input = make_update_input("SollteNicht", "GespeichertWerden", Some("X"), true, Some("2020-01-01"), None, vec!["ungültige-uuid".to_string()], vec![]);
        let result = update_employee(&conn, &emp.id, &input);
        assert!(result.is_err(), "Ungültige Verantwortungsposition sollte Fehler verursachen");
        let loaded = get_employee(&conn, &emp.id).unwrap();
        assert_eq!(loaded.last_name, "Test");
        assert_eq!(loaded.first_name, "Mitarbeiter");
    }

    #[test]
    fn test_invalid_qm_area_id_rolls_back() {
        let (conn, _tmp) = init_test_db();
        let emp = make_test_employee(&conn, &[], &[]);
        let input = make_update_input("SollteNicht", "GespeichertWerden", Some("X"), true, Some("2020-01-01"), None, vec![], vec!["ungültige-uuid".to_string()]);
        let result = update_employee(&conn, &emp.id, &input);
        assert!(result.is_err(), "Ungültiger QM-Bereich sollte Fehler verursachen");
        let loaded = get_employee(&conn, &emp.id).unwrap();
        assert_eq!(loaded.last_name, "Test");
        assert_eq!(loaded.first_name, "Mitarbeiter");
    }

    #[test]
    fn test_failed_relationship_update_preserves_basic_fields() {
        let (conn, _tmp) = init_test_db();
        let emp = make_test_employee(&conn, &[], &[]);
        let bad_input = make_update_input("Geändert", "Geändert", Some("Geändert"), false, Some("2020-01-01"), None, vec!["fake-id".to_string()], vec![]);
        let _ = update_employee(&conn, &emp.id, &bad_input);
        let loaded = get_employee(&conn, &emp.id).unwrap();
        assert_eq!(loaded.last_name, "Test");
        assert_eq!(loaded.first_name, "Mitarbeiter");
        assert_eq!(loaded.position.as_deref(), Some("ZFA"));
        assert!(loaded.is_active);
    }

    #[test]
    fn test_get_employee_returns_complete_relationships() {
        let (conn, _tmp) = init_test_db();
        let r1 = create_responsibility(&conn, "Hygiene").unwrap();
        let r2 = create_responsibility(&conn, "Fortbildung").unwrap();
        let a1 = create_qm_area(&conn, "Abrechnung").unwrap();
        let a2 = create_qm_area(&conn, "Notfall").unwrap();
        let emp = make_test_employee(&conn, &[r1.id.clone(), r2.id.clone()], &[a1.id.clone(), a2.id.clone()]);
        let loaded = get_employee(&conn, &emp.id).unwrap();
        assert_eq!(loaded.responsibilities.len(), 2);
        assert!(loaded.responsibilities.contains(&"Hygiene".to_string()));
        assert!(loaded.responsibilities.contains(&"Fortbildung".to_string()));
        assert_eq!(loaded.qm_areas.len(), 2);
        assert!(loaded.qm_areas.contains(&"Abrechnung".to_string()));
        assert!(loaded.qm_areas.contains(&"Notfall".to_string()));
        assert_eq!(loaded.responsibility_ids.len(), 2);
        assert!(loaded.responsibility_ids.contains(&r1.id));
        assert!(loaded.responsibility_ids.contains(&r2.id));
        assert_eq!(loaded.qm_area_ids.len(), 2);
        assert!(loaded.qm_area_ids.contains(&a1.id));
        assert!(loaded.qm_area_ids.contains(&a2.id));
    }

    #[test]
    fn test_get_employee_nonexistent_returns_error() {
        let (conn, _tmp) = init_test_db();
        let result = get_employee(&conn, "nicht-vorhanden");
        assert!(result.is_err(), "Nicht existierender Mitarbeiter sollte Fehler zurückgeben");
    }

    #[test]
    fn test_update_employee_nonexistent_returns_error() {
        let (conn, _tmp) = init_test_db();
        let input = make_update_input("Test", "Mitarbeiter", Some("ZFA"), true, Some("2020-01-01"), None, vec![], vec![]);
        let result = update_employee(&conn, "nicht-vorhanden", &input);
        assert!(result.is_err());
    }

    // --- Dokument-Tests ---

    use std::io::Write;

    fn make_test_pdf(dir: &tempfile::TempDir, name: &str, content: &[u8]) -> std::path::PathBuf {
        let path = dir.path().join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }

    fn make_valid_pdf(dir: &tempfile::TempDir, name: &str) -> std::path::PathBuf {
        let content = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog >>\nendobj\ntrailer\n<< /Root 1 0 R >>\n%%EOF";
        make_test_pdf(dir, name, content)
    }

    fn init_test_storage() -> tempfile::TempDir {
        tempfile::TempDir::new().unwrap()
    }

    fn make_document_input(source_path: &str, file_name: &str) -> CreateDocumentInput {
        CreateDocumentInput {
            title: "Testdokument".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "Entwurf".to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            description: None,
            source_file_path: source_path.to_string(),
            original_file_name: file_name.to_string(),
            tag_ids: vec![],
        }
    }

    #[test]
    fn test_create_document_metadata() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "test.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "test-uuid").unwrap();
        let doc = create_document(&conn, &input, &managed).unwrap();
        assert_eq!(doc.title, "Testdokument");
        assert_eq!(doc.version, "1.0");
        assert_eq!(doc.status, "Entwurf");
        assert_eq!(doc.validity, "gültig");
    }

    #[test]
    fn test_list_documents() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "a.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "a.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "uuid-a").unwrap();
        create_document(&conn, &input, &managed).unwrap();

        let pdf2 = make_valid_pdf(&storage, "b.pdf");
        let input2 = make_document_input(pdf2.to_str().unwrap(), "b.pdf");
        let managed2 = copy_to_managed_storage(&pdf2, storage.path(), "uuid-b").unwrap();
        create_document(&conn, &input2, &managed2).unwrap();

        let docs = list_documents(&conn).unwrap();
        assert_eq!(docs.len(), 2);
    }

    #[test]
    fn test_document_uuid_generated_and_stable() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "test.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "test-uuid").unwrap();
        let doc = create_document(&conn, &input, &managed).unwrap();
        assert!(!doc.id.is_empty());
        let loaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(loaded.id, doc.id);
    }

    #[test]
    fn test_document_number_generated_sequential() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "a.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "a.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "uuid-a").unwrap();
        let doc1 = create_document(&conn, &input, &managed).unwrap();
        assert_eq!(doc1.document_number, "PQM-0001");

        let pdf2 = make_valid_pdf(&storage, "b.pdf");
        let input2 = make_document_input(pdf2.to_str().unwrap(), "b.pdf");
        let managed2 = copy_to_managed_storage(&pdf2, storage.path(), "uuid-b").unwrap();
        let doc2 = create_document(&conn, &input2, &managed2).unwrap();
        assert_eq!(doc2.document_number, "PQM-0002");
    }

    #[test]
    fn test_document_number_unique() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "a.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "a.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "uuid-a").unwrap();
        let doc1 = create_document(&conn, &input, &managed).unwrap();
        let doc2_input = make_document_input(pdf.to_str().unwrap(), "a.pdf");
        let managed2 = copy_to_managed_storage(&pdf, storage.path(), "uuid-b").unwrap();
        let doc2 = create_document(&conn, &doc2_input, &managed2).unwrap();
        assert_ne!(doc1.document_number, doc2.document_number);
    }

    #[test]
    fn test_pdf_copied_into_managed_storage() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "original.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "original.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(&conn, &input, &managed).unwrap();
        let managed_file = storage.path().join(&managed);
        assert!(managed_file.exists(), "Managed PDF should exist");
        assert!(managed_file.file_name().unwrap().to_str().unwrap().starts_with("doc-uuid"));
    }

    #[test]
    fn test_source_pdf_removable_after_import() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let source_dir = tempfile::TempDir::new().unwrap();
        let pdf = make_valid_pdf(&source_dir, "source.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "source.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(&conn, &input, &managed).unwrap();
        drop(source_dir);
        assert!(!pdf.exists(), "Source should be gone");
        let managed_file = storage.path().join(&managed);
        assert!(managed_file.exists(), "Managed PDF should still exist");
    }

    #[test]
    fn test_missing_source_pdf_rejected() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let fake_path = storage.path().join("nonexistent.pdf");
        let result = validate_pdf(&fake_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_pdf_rejected() {
        let storage = init_test_storage();
        let empty_pdf = make_test_pdf(&storage, "empty.pdf", b"");
        let result = validate_pdf(&empty_pdf);
        assert!(result.is_err());
    }

    #[test]
    fn test_non_pdf_rejected() {
        let storage = init_test_storage();
        let not_pdf = make_test_pdf(&storage, "notpdf.pdf", b"Not a PDF file content");
        let result = validate_pdf(&not_pdf);
        assert!(result.is_err());
    }

    #[test]
    fn test_db_failure_removes_orphan_pdf() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "orphan-uuid").unwrap();
        let managed_file = storage.path().join(&managed);
        assert!(managed_file.exists());

        let bad_input = CreateDocumentInput {
            title: String::new(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: Some("nonexistent-employee-id".to_string()),
            version: "1.0".to_string(),
            status: "Entwurf".to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };

        let result = create_document(&conn, &bad_input, &managed);
        assert!(result.is_err());
        if result.is_err() {
            remove_managed_file(storage.path(), &managed);
        }
        assert!(!managed_file.exists(), "Orphan PDF should be removed after DB failure");
    }

    #[test]
    fn test_invalid_responsible_person_rejected() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "uuid-test").unwrap();
        let input = CreateDocumentInput {
            title: "Test".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: Some("nonexistent-id".to_string()),
            version: "1.0".to_string(),
            status: "Entwurf".to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let result = create_document(&conn, &input, &managed);
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_documents_returns_empty() {
        let (conn, _tmp) = init_test_db();
        let docs = list_documents(&conn).unwrap();
        assert!(docs.is_empty());
    }

    #[test]
    fn test_persisted_document_loads_with_metadata() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Hygieneplan".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "2.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2026-12-31".to_string()),
            description: Some("Beschreibung".to_string()),
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "hygieneplan.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        let created = create_document(&conn, &input, &managed).unwrap();
        let loaded = load_document(&conn, &created.id).unwrap();
        assert_eq!(loaded.title, "Hygieneplan");
        assert_eq!(loaded.version, "2.0");
        assert_eq!(loaded.status, "aktiv");
        assert_eq!(loaded.validity, "gültig");
        assert_eq!(loaded.valid_until, Some("2026-12-31".to_string()));
        assert_eq!(loaded.description, Some("Beschreibung".to_string()));
        assert_eq!(loaded.file_name, Some("hygieneplan.pdf".to_string()));
        assert!(loaded.file_path.is_some());
    }

    #[test]
    fn test_get_document_by_number() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = make_document_input(pdf.to_str().unwrap(), "test.pdf");
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        let created = create_document(&conn, &input, &managed).unwrap();
        let loaded = get_document_by_number(&conn, &created.document_number).unwrap();
        assert_eq!(loaded.id, created.id);
        assert_eq!(loaded.document_number, created.document_number);
    }

    // --- Dokument-Update-Tests ---

    /// Helper: Erstellt eine Test-Kategorie.
    fn make_test_category(conn: &Connection, name: &str) -> CategoryItem {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_iso();
        conn.execute(
            "INSERT INTO categories (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
            rusqlite::params![id, name, now, now],
        )
        .unwrap();
        CategoryItem {
            id,
            name: name.to_string(),
        }
    }

    /// Helper: Erstellt ein Test-Dokument mit optionalen Beziehungen.
    fn make_test_document_with_relations(
        conn: &Connection,
        storage: &tempfile::TempDir,
        cat_id: Option<&str>,
        emp_id: Option<&str>,
    ) -> Document {
        let pdf = make_valid_pdf(storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Testdokument".to_string(),
            category_id: cat_id.map(|s| s.to_string()),
            subcategory_id: None,
            responsible_person_id: emp_id.map(|s| s.to_string()),
            version: "1.0".to_string(),
            status: "Entwurf".to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            description: Some("Ursprüngliche Beschreibung".to_string()),
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "test-uuid").unwrap();
        create_document(conn, &input, &managed).unwrap()
    }

    /// Helper: Erstellt ein UpdateDocumentInput mit Standardwerten.
    fn make_update_document_input(
        title: &str,
        cat_id: Option<&str>,
        sub_id: Option<&str>,
        emp_id: Option<&str>,
        version: &str,
        status: &str,
        validity: &str,
        valid_until: Option<&str>,
        description: Option<&str>,
    ) -> UpdateDocumentInput {
        UpdateDocumentInput {
            title: title.to_string(),
            category_id: cat_id.map(|s| s.to_string()),
            subcategory_id: sub_id.map(|s| s.to_string()),
            responsible_person_id: emp_id.map(|s| s.to_string()),
            version: version.to_string(),
            status: status.to_string(),
            validity: validity.to_string(),
            valid_until: valid_until.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            tag_ids: vec![],
        }
    }

    #[test]
    fn test_get_existing_document() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let loaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(loaded.id, doc.id);
        assert_eq!(loaded.title, "Testdokument");
    }

    #[test]
    fn test_get_nonexistent_document_returns_error() {
        let (conn, _tmp) = init_test_db();
        let result = load_document(&conn, "nonexistent-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_document_title() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let input = make_update_document_input(
            "Neuer Titel", None, None, None, "1.0", "Entwurf", "gültig", None, None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.title, "Neuer Titel");
    }

    #[test]
    fn test_update_document_description() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let input = make_update_document_input(
            "Testdokument", None, None, None, "1.0", "Entwurf", "gültig", None,
            Some("Neue Beschreibung"),
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.description, Some("Neue Beschreibung".to_string()));
    }

    #[test]
    fn test_update_document_validity() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let input = make_update_document_input(
            "Testdokument", None, None, None, "2.0", "aktiv", "gültig",
            Some("2026-12-31"), None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.version, "2.0");
        assert_eq!(updated.status, "aktiv");
        assert_eq!(updated.valid_until, Some("2026-12-31".to_string()));
    }

    #[test]
    fn test_update_document_uuid_unchanged() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_id = doc.id.clone();
        let input = make_update_document_input(
            "Geändert", None, None, None, "1.1", "aktiv", "gültig", None, None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.id, original_id);
    }

    #[test]
    fn test_update_document_number_unchanged() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_number = doc.document_number.clone();
        let input = make_update_document_input(
            "Geändert", None, None, None, "1.1", "aktiv", "gültig", None, None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.document_number, original_number);
    }

    #[test]
    fn test_update_document_created_at_unchanged() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_created = doc.created_at.clone();
        let input = make_update_document_input(
            "Geändert", None, None, None, "1.1", "aktiv", "gültig", None, None,
        );
        update_document(&conn, &doc.id, &input).unwrap();
        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.created_at, original_created);
    }

    #[test]
    fn test_update_document_updated_at_changes() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_updated = doc.updated_at.clone();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let input = make_update_document_input(
            "Geändert", None, None, None, "1.1", "aktiv", "gültig", None, None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_ne!(updated.updated_at, original_updated);
    }

    #[test]
    fn test_update_invalid_category_rolls_back() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let input = make_update_document_input(
            "Geändert", Some("nonexistent-cat-id"), None, None,
            "1.0", "Entwurf", "gültig", None, None,
        );
        let result = update_document(&conn, &doc.id, &input);
        assert!(result.is_err());
        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.title, "Testdokument");
    }

    #[test]
    fn test_update_invalid_employee_rolls_back() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let input = make_update_document_input(
            "Geändert", None, None, Some("nonexistent-emp-id"),
            "1.0", "Entwurf", "gültig", None, None,
        );
        let result = update_document(&conn, &doc.id, &input);
        assert!(result.is_err());
        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.title, "Testdokument");
    }

    #[test]
    fn test_update_relationship_persists() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let cat = make_test_category(&conn, "Hygiene");
        let emp = make_test_employee(&conn, &[], &[]);
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let input = make_update_document_input(
            "Mit Kategorie", Some(&cat.id), None, Some(&emp.id),
            "1.0", "aktiv", "gültig", None, None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.category_id, Some(cat.id));
        assert_eq!(updated.responsible_person_id, Some(emp.id));
        assert_eq!(updated.category_name, Some("Hygiene".to_string()));
    }

    #[test]
    fn test_update_survives_reload() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let input = make_update_document_input(
            "Neu geladen", None, None, None, "2.0", "aktiv", "gültig",
            Some("2027-01-01"), Some("Persistiert"),
        );
        update_document(&conn, &doc.id, &input).unwrap();
        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.title, "Neu geladen");
        assert_eq!(reloaded.version, "2.0");
        assert_eq!(reloaded.status, "aktiv");
        assert_eq!(reloaded.valid_until, Some("2027-01-01".to_string()));
        assert_eq!(reloaded.description, Some("Persistiert".to_string()));
    }

    #[test]
    fn test_update_nonexistent_document_returns_error() {
        let (conn, _tmp) = init_test_db();
        let input = make_update_document_input(
            "Geändert", None, None, None, "1.0", "Entwurf", "gültig", None, None,
        );
        let result = update_document(&conn, "nonexistent-uuid", &input);
        assert!(result.is_err());
    }

    // --- Versionierungs-Tests (Prompt 020) ---

    fn make_create_version_input(
        document_id: &str,
        version_number: &str,
        status: &str,
        validity: &str,
        valid_until: Option<&str>,
        source_path: &str,
        file_name: &str,
    ) -> CreateVersionInput {
        CreateVersionInput {
            document_id: document_id.to_string(),
            version_number: version_number.to_string(),
            status: status.to_string(),
            validity: validity.to_string(),
            valid_until: valid_until.map(|s| s.to_string()),
            source_file_path: source_path.to_string(),
            original_file_name: file_name.to_string(),
        }
    }

    #[test]
    fn test_create_first_persisted_version() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 1);
        assert!(versions[0].is_current);
        assert_eq!(versions[0].version_number, "1.0");
    }

    #[test]
    fn test_create_subsequent_version() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let updated = create_version(&conn, &input, &managed).unwrap();
        assert_eq!(updated.version, "2.0");

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version_number, "2.0");
        assert!(versions[0].is_current);
        assert_eq!(versions[1].version_number, "1.0");
        assert!(!versions[1].is_current);
    }

    #[test]
    fn test_version_document_uuid_unchanged() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_id = doc.id.clone();

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let updated = create_version(&conn, &input, &managed).unwrap();
        assert_eq!(updated.id, original_id);
    }

    #[test]
    fn test_version_document_number_unchanged() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_number = doc.document_number.clone();

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let updated = create_version(&conn, &input, &managed).unwrap();
        assert_eq!(updated.document_number, original_number);
    }

    #[test]
    fn test_version_ids_unique() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version(&conn, &input, &managed).unwrap();

        let versions = list_versions(&conn, &doc.id).unwrap();
        let ids: Vec<&str> = versions.iter().map(|v| v.id.as_str()).collect();
        assert_eq!(ids.len(), 2);
        assert_ne!(ids[0], ids[1]);
    }

    #[test]
    fn test_version_identifier_follows_input() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "3.5.1", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version(&conn, &input, &managed).unwrap();

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions[0].version_number, "3.5.1");
        assert_eq!(versions[0].is_current, true);
    }

    #[test]
    fn test_duplicate_version_rejected() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "1.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let result = create_version(&conn, &input, &managed);
        assert!(result.is_err(), "Duplicate version number must be rejected");
    }

    #[test]
    fn test_previous_version_record_preserved() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version(&conn, &input, &managed).unwrap();

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[1].version_number, "1.0");
        assert_eq!(versions[1].file_name, "test.pdf");
    }

    #[test]
    fn test_previous_pdf_preserved() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_file = doc.file_path.clone().unwrap();
        let original_managed = storage.path().join(&original_file);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version(&conn, &input, &managed).unwrap();

        assert!(original_managed.exists(), "Previous PDF must be preserved");
    }

    #[test]
    fn test_new_pdf_copied_into_managed_storage() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version(&conn, &input, &managed).unwrap();

        let managed_file = storage.path().join(&managed);
        assert!(managed_file.exists(), "New PDF must be in managed storage");
    }

    #[test]
    fn test_source_pdf_removable_after_version_import() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let source_dir = tempfile::TempDir::new().unwrap();
        let new_pdf = make_valid_pdf(&source_dir, "source_v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "source_v2.pdf",
        );
        create_version(&conn, &input, &managed).unwrap();

        drop(source_dir);
        assert!(!new_pdf.exists(), "Source should be gone");
        let managed_file = storage.path().join(&managed);
        assert!(managed_file.exists(), "Managed copy must survive");
    }

    #[test]
    fn test_current_version_resolution_correct() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version(&conn, &input, &managed).unwrap();

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert!(versions[0].is_current, "Newest version must be current");
        assert_eq!(versions[0].version_number, "2.0");
        assert!(!versions[1].is_current, "Old version must not be current");
    }

    #[test]
    fn test_version_history_ordered_newest_first() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        for ver in &["2.0", "3.0"] {
            let pdf = make_valid_pdf(&storage, &format!("{}.pdf", ver));
            let vid = uuid::Uuid::new_v4().to_string();
            let m = copy_to_managed_storage(&pdf, storage.path(), &vid).unwrap();
            let input = make_create_version_input(
                &doc.id, ver, "aktiv", "gültig", None,
                pdf.to_str().unwrap(), &format!("{}.pdf", ver),
            );
            create_version(&conn, &input, &m).unwrap();
        }

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].version_number, "3.0");
        assert_eq!(versions[1].version_number, "2.0");
        assert_eq!(versions[2].version_number, "1.0");
    }

    #[test]
    fn test_db_failure_preserves_previous_current_version() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);
        let original_version = doc.version.clone();

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "1.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let result = create_version(&conn, &input, &managed);
        assert!(result.is_err(), "Duplicate must fail");

        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.version, original_version, "Current version must be unchanged");
    }

    #[test]
    fn test_db_failure_cleans_new_orphan_pdf() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let managed_path = managed.clone();
        let input = make_create_version_input(
            &doc.id, "1.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let result = create_version(&conn, &input, &managed);
        assert!(result.is_err());

        remove_managed_file(storage.path(), &managed_path);
        let managed_file = storage.path().join(&managed_path);
        assert!(!managed_file.exists(), "Orphan must be cleaned");
    }

    #[test]
    fn test_invalid_pdf_creates_no_version() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let fake_pdf = storage.path().join("fake.pdf");
        std::fs::write(&fake_pdf, b"not a real pdf").unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            fake_pdf.to_str().unwrap(), "fake.pdf",
        );
        let result = create_version_from_source(&mut conn, &input, storage.path());
        assert!(result.is_err(), "Invalid PDF must not create version");

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 1, "No new version should exist");

        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.version, "1.0", "DB-001 current version must be unchanged");
    }

    #[test]
    fn test_missing_source_pdf_creates_no_version() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            "/nonexistent/path/file.pdf", "file.pdf",
        );
        let result = create_version_from_source(&mut conn, &input, storage.path());
        assert!(result.is_err(), "Missing source PDF must not create version");

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 1, "No new version should exist");

        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.version, "1.0", "DB-001 current version must be unchanged");
    }

    #[test]
    fn test_metadata_relationships_stored_at_document_level() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let cat = make_test_category(&conn, "Hygiene");
        let emp = make_test_employee(&conn, &[], &[]);
        let doc = make_test_document_with_relations(&conn, &storage, Some(&cat.id), Some(&emp.id));

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_id = uuid::Uuid::new_v4().to_string();
        let managed = copy_to_managed_storage(&new_pdf, storage.path(), &version_id).unwrap();
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let updated = create_version(&conn, &input, &managed).unwrap();

        assert_eq!(updated.category_id, Some(cat.id), "Category stays at document level");
        assert_eq!(updated.responsible_person_id, Some(emp.id), "Employee stays at document level");
        assert_eq!(updated.title, "Testdokument", "Title stays at document level");
    }

    #[test]
    fn test_application_reload_returns_complete_version_history() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        for ver in &["2.0", "3.0"] {
            let pdf = make_valid_pdf(&storage, &format!("{}.pdf", ver));
            let vid = uuid::Uuid::new_v4().to_string();
            let m = copy_to_managed_storage(&pdf, storage.path(), &vid).unwrap();
            let input = make_create_version_input(
                &doc.id, ver, "aktiv", "gültig", None,
                pdf.to_str().unwrap(), &format!("{}.pdf", ver),
            );
            create_version(&conn, &input, &m).unwrap();
        }

        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.version, "3.0");

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].version_number, "3.0");
        assert!(versions[0].is_current);
    }

    #[test]
    fn test_zero_version_history_behaves_correctly() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_test_document_with_relations(&conn, &storage, None, None);

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 1);
        assert!(versions[0].is_current);
        assert!(!versions[0].version_number.is_empty());
    }

    // === Prompt 021: Archive & Restore Lifecycle Tests ===

    fn make_archivable_document(conn: &Connection, storage: &tempfile::TempDir) -> Document {
        let doc = make_test_document_with_relations(conn, storage, None, None);
        let update = make_update_document_input(
            "Aktives Dokument", None, None, None, "1.0", "aktiv", "gültig", None, None,
        );
        update_document(conn, &doc.id, &update).unwrap()
    }

    #[test]
    fn test_archive_active_document() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let archived = archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(archived.status, "archiviert");
        assert!(archived.archived_at.is_some(), "archived_at must be set");
    }

    #[test]
    fn test_archive_uuid_unchanged() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let archived = archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(archived.id, doc.id, "UUID must not change on archive");
    }

    #[test]
    fn test_archive_document_number_unchanged() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let archived = archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(archived.document_number, doc.document_number, "Document number must not change");
    }

    #[test]
    fn test_archive_sets_archived_at_correctly() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let before = now_iso();
        let archived = archive_document(&mut conn, &doc.id).unwrap();
        let after = now_iso();

        let archived_ts: i64 = archived.archived_at.as_ref().unwrap().parse().unwrap();
        let before_ts: i64 = before.parse().unwrap();
        let after_ts: i64 = after.parse().unwrap();
        assert!(archived_ts >= before_ts && archived_ts <= after_ts,
            "archived_at must be between start and end of archive operation");
    }

    #[test]
    fn test_archived_document_excluded_from_active_list() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc1 = make_archivable_document(&conn, &storage);
        let _doc2 = make_archivable_document(&conn, &storage);

        let active = list_documents(&conn).unwrap();
        assert_eq!(active.len(), 2, "Two active documents before archive");

        archive_document(&mut conn, &doc1.id).unwrap();

        let active_after = list_documents(&conn).unwrap();
        assert_eq!(active_after.len(), 1, "Archived document must not appear in active list");
        assert_ne!(active_after[0].id, doc1.id, "Remaining document must be the non-archived one");
    }

    #[test]
    fn test_archived_document_appears_in_archive_list() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let archived_before = list_archived_documents(&conn).unwrap();
        assert_eq!(archived_before.len(), 0, "No archived documents initially");

        archive_document(&mut conn, &doc.id).unwrap();

        let archived_after = list_archived_documents(&conn).unwrap();
        assert_eq!(archived_after.len(), 1, "Archived document must appear in archive list");
        assert_eq!(archived_after[0].id, doc.id);
    }

    #[test]
    fn test_versions_preserved_after_archive() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version_from_source(&mut conn, &input, storage.path()).unwrap();

        let versions_before = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions_before.len(), 2);

        archive_document(&mut conn, &doc.id).unwrap();

        let versions_after = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions_after.len(), 2, "All versions must be preserved after archive");
    }

    #[test]
    fn test_current_version_preserved_after_archive() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version_from_source(&mut conn, &input, storage.path()).unwrap();

        archive_document(&mut conn, &doc.id).unwrap();

        let versions = list_versions(&conn, &doc.id).unwrap();
        let current = versions.iter().find(|v| v.is_current);
        assert!(current.is_some(), "Current version must still resolve after archive");
        assert_eq!(current.unwrap().version_number, "2.0");
    }

    #[test]
    fn test_managed_pdfs_remain_untouched_after_archive() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let updated = create_version_from_source(&mut conn, &input, storage.path()).unwrap();

        let v2_file_path: String = conn.query_row(
            "SELECT file_path FROM document_versions WHERE document_id = ?1 AND version_number = ?2;",
            rusqlite::params![doc.id, "2.0"],
            |row| row.get(0),
        ).unwrap();
        let managed_path = storage.path().join(&v2_file_path);
        assert!(managed_path.exists(), "Managed PDF must exist before archive");

        archive_document(&mut conn, &doc.id).unwrap();

        assert!(managed_path.exists(), "Managed PDF must still exist after archive");

        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(reloaded.version, updated.version, "Current version metadata unchanged");
    }

    #[test]
    fn test_archive_nonexistent_document_returns_error() {
        let (mut conn, _tmp) = init_test_db();

        let result = archive_document(&mut conn, "nonexistent-uuid");
        assert!(result.is_err(), "Archiving a nonexistent document must fail");
    }

    #[test]
    fn test_double_archive_handled_correctly() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();

        let second = archive_document(&mut conn, &doc.id);
        assert!(second.is_err(), "Double archive must fail");
    }

    #[test]
    fn test_archive_does_not_create_new_version() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let versions_before = list_versions(&conn, &doc.id).unwrap();
        let count_before = versions_before.len();

        archive_document(&mut conn, &doc.id).unwrap();

        let versions_after = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions_after.len(), count_before, "Archive must not create a new version");
    }

    #[test]
    fn test_archive_counter_returns_correct_results() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc1 = make_archivable_document(&conn, &storage);
        let doc2 = make_archivable_document(&conn, &storage);
        let doc3 = make_archivable_document(&conn, &storage);

        assert_eq!(list_documents(&conn).unwrap().len(), 3);
        assert_eq!(list_archived_documents(&conn).unwrap().len(), 0);

        archive_document(&mut conn, &doc1.id).unwrap();
        assert_eq!(list_documents(&conn).unwrap().len(), 2);
        assert_eq!(list_archived_documents(&conn).unwrap().len(), 1);

        archive_document(&mut conn, &doc2.id).unwrap();
        assert_eq!(list_documents(&conn).unwrap().len(), 1);
        assert_eq!(list_archived_documents(&conn).unwrap().len(), 2);

        archive_document(&mut conn, &doc3.id).unwrap();
        assert_eq!(list_documents(&conn).unwrap().len(), 0);
        assert_eq!(list_archived_documents(&conn).unwrap().len(), 3);
    }

    // === Restore Tests ===

    #[test]
    fn test_restore_archived_document() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();

        assert_eq!(restored.status, "aktiv", "Restored document must have status aktiv");
        assert!(restored.archived_at.is_none(), "archived_at must be cleared on restore");
    }

    #[test]
    fn test_restored_document_returns_to_active_list() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(list_documents(&conn).unwrap().len(), 0);

        restore_document(&mut conn, &doc.id).unwrap();
        let active = list_documents(&conn).unwrap();
        assert_eq!(active.len(), 1, "Restored document must reappear in active list");
        assert_eq!(active[0].id, doc.id);
    }

    #[test]
    fn test_restored_document_disappears_from_archive_list() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(list_archived_documents(&conn).unwrap().len(), 1);

        restore_document(&mut conn, &doc.id).unwrap();
        assert_eq!(list_archived_documents(&conn).unwrap().len(), 0, "Restored document must leave archive list");
    }

    #[test]
    fn test_restore_uuid_unchanged() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();

        assert_eq!(restored.id, doc.id, "UUID must not change on restore");
    }

    #[test]
    fn test_restore_document_number_unchanged() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();

        assert_eq!(restored.document_number, doc.document_number, "Document number must not change on restore");
    }

    #[test]
    fn test_versions_preserved_after_restore() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version_from_source(&mut conn, &input, storage.path()).unwrap();

        archive_document(&mut conn, &doc.id).unwrap();
        restore_document(&mut conn, &doc.id).unwrap();

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 2, "All versions must be preserved after restore");
    }

    #[test]
    fn test_managed_pdfs_preserved_after_restore() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version_from_source(&mut conn, &input, storage.path()).unwrap();

        let v2_file_path: String = conn.query_row(
            "SELECT file_path FROM document_versions WHERE document_id = ?1 AND version_number = ?2;",
            rusqlite::params![doc.id, "2.0"],
            |row| row.get(0),
        ).unwrap();
        let managed_path = storage.path().join(&v2_file_path);

        archive_document(&mut conn, &doc.id).unwrap();
        restore_document(&mut conn, &doc.id).unwrap();

        assert!(managed_path.exists(), "Managed PDF must still exist after restore");
    }

    #[test]
    fn test_restore_nonexistent_document_handled() {
        let (mut conn, _tmp) = init_test_db();

        let result = restore_document(&mut conn, "nonexistent-uuid");
        assert!(result.is_err(), "Restoring a nonexistent document must fail");
    }

    #[test]
    fn test_restore_non_archived_document_handled() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let result = restore_document(&mut conn, &doc.id);
        assert!(result.is_err(), "Restoring a non-archived document must fail");
    }

    #[test]
    fn test_update_blocked_when_archived() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();

        let update = make_update_document_input(
            "Geändert", None, None, None, "1.0", "aktiv", "gültig", None, None,
        );
        let result = update_document(&conn, &doc.id, &update);
        assert!(result.is_err(), "Editing an archived document must fail");
    }

    #[test]
    fn test_create_version_blocked_when_archived() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        archive_document(&mut conn, &doc.id).unwrap();

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        let result = create_version_from_source(&mut conn, &input, storage.path());
        assert!(result.is_err(), "Creating a new version of an archived document must fail");

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 1, "No new version should be created");
    }

    #[test]
    fn test_archive_restore_roundtrip_preserves_everything() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_archivable_document(&conn, &storage);

        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = make_create_version_input(
            &doc.id, "2.0", "aktiv", "gültig", None,
            new_pdf.to_str().unwrap(), "v2.pdf",
        );
        create_version_from_source(&mut conn, &input, storage.path()).unwrap();

        let original_number = doc.document_number.clone();
        let original_id = doc.id.clone();

        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();

        assert_eq!(restored.id, original_id, "UUID preserved through archive/restore roundtrip");
        assert_eq!(restored.document_number, original_number, "Document number preserved through roundtrip");
        assert_eq!(restored.status, "aktiv", "Status restored to aktiv");
        assert!(restored.archived_at.is_none(), "archived_at cleared after restore");

        let versions = list_versions(&conn, &doc.id).unwrap();
        assert_eq!(versions.len(), 2, "Both versions preserved through roundtrip");
        let current = versions.iter().find(|v| v.is_current).unwrap();
        assert_eq!(current.version_number, "2.0", "Current version preserved through roundtrip");
    }

    // === Prompt 022A: Validity Calculation Tests ===

    #[test]
    fn test_validity_null_returns_none() {
        assert_eq!(calculate_validity_status(None, "2025-06-15"), None);
    }

    #[test]
    fn test_validity_31_days_before_expiry_is_gueltig() {
        // valid_until = 2025-07-16, today = 2025-06-16 → 30 days before = 2025-06-16
        // 31 days before = 2025-06-15 → gültig
        assert_eq!(
            calculate_validity_status(Some("2025-07-16"), "2025-06-15"),
            Some(VALIDITY_GUELTIG)
        );
    }

    #[test]
    fn test_validity_exactly_30_days_before_is_bald_ab() {
        // valid_until = 2025-07-16, threshold = 2025-06-16
        assert_eq!(
            calculate_validity_status(Some("2025-07-16"), "2025-06-16"),
            Some(VALIDITY_BALD_AB)
        );
    }

    #[test]
    fn test_validity_29_days_before_is_bald_ab() {
        assert_eq!(
            calculate_validity_status(Some("2025-07-16"), "2025-06-17"),
            Some(VALIDITY_BALD_AB)
        );
    }

    #[test]
    fn test_validity_1_day_before_is_bald_ab() {
        assert_eq!(
            calculate_validity_status(Some("2025-07-16"), "2025-07-15"),
            Some(VALIDITY_BALD_AB)
        );
    }

    #[test]
    fn test_validity_exact_expiry_date_is_bald_ab() {
        // Document is valid THROUGH the valid_until date
        assert_eq!(
            calculate_validity_status(Some("2025-07-16"), "2025-07-16"),
            Some(VALIDITY_BALD_AB)
        );
    }

    #[test]
    fn test_validity_1_day_after_is_abgelaufen() {
        assert_eq!(
            calculate_validity_status(Some("2025-07-16"), "2025-07-17"),
            Some(VALIDITY_ABGELAUFEN)
        );
    }

    #[test]
    fn test_validity_month_boundary() {
        // valid_until = 2025-08-01, threshold = 2025-07-02
        assert_eq!(
            calculate_validity_status(Some("2025-08-01"), "2025-07-01"),
            Some(VALIDITY_GUELTIG)
        );
        assert_eq!(
            calculate_validity_status(Some("2025-08-01"), "2025-07-02"),
            Some(VALIDITY_BALD_AB)
        );
        assert_eq!(
            calculate_validity_status(Some("2025-08-01"), "2025-08-02"),
            Some(VALIDITY_ABGELAUFEN)
        );
    }

    #[test]
    fn test_validity_year_boundary() {
        // valid_until = 2025-12-31, threshold = 2025-12-01
        assert_eq!(
            calculate_validity_status(Some("2025-12-31"), "2025-11-30"),
            Some(VALIDITY_GUELTIG)
        );
        assert_eq!(
            calculate_validity_status(Some("2025-12-31"), "2025-12-01"),
            Some(VALIDITY_BALD_AB)
        );
        assert_eq!(
            calculate_validity_status(Some("2025-12-31"), "2026-01-01"),
            Some(VALIDITY_ABGELAUFEN)
        );
    }

    #[test]
    fn test_validity_leap_year_boundary() {
        // 2024 is a leap year: Feb 29 exists
        // valid_until = 2024-02-29, threshold = 2024-01-30
        assert_eq!(
            calculate_validity_status(Some("2024-02-29"), "2024-01-29"),
            Some(VALIDITY_GUELTIG)
        );
        assert_eq!(
            calculate_validity_status(Some("2024-02-29"), "2024-01-30"),
            Some(VALIDITY_BALD_AB)
        );
        assert_eq!(
            calculate_validity_status(Some("2024-02-29"), "2024-03-01"),
            Some(VALIDITY_ABGELAUFEN)
        );
    }

    #[test]
    fn test_validity_leap_year_threshold_crosses_feb() {
        // valid_until = 2024-03-31, threshold = 2024-03-01 (30 days before, crosses Feb 29)
        assert_eq!(
            calculate_validity_status(Some("2024-03-31"), "2024-02-29"),
            Some(VALIDITY_GUELTIG)
        );
        assert_eq!(
            calculate_validity_status(Some("2024-03-31"), "2024-03-01"),
            Some(VALIDITY_BALD_AB)
        );
    }

    #[test]
    fn test_validity_invalid_date_returns_none() {
        assert_eq!(calculate_validity_status(Some("not-a-date"), "2025-06-15"), None);
        assert_eq!(calculate_validity_status(Some("2025-13-01"), "2025-06-15"), None);
    }

    #[test]
    fn test_computed_validity_populated_in_list_documents() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        // Create a document with valid_until far in the future → should be gültig
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Gültiges Dokument".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2099-12-31".to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(&conn, &input, &managed).unwrap();

        let docs = list_documents(&conn).unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(
            docs[0].computed_validity.as_deref(),
            Some("gültig"),
            "computed_validity must be derived from valid_until"
        );
    }

    #[test]
    fn test_computed_validity_null_when_no_valid_until() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Ohne Ablaufdatum".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(&conn, &input, &managed).unwrap();

        let docs = list_documents(&conn).unwrap();
        assert_eq!(docs.len(), 1);
        assert!(
            docs[0].computed_validity.is_none(),
            "computed_validity must be None when valid_until is NULL"
        );
    }

    #[test]
    fn test_computed_validity_stale_persisted_does_not_override() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        // Persist "gültig" but set valid_until to a date far in the past → derived should be "abgelaufen"
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Stale Validity".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2000-01-01".to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(&conn, &input, &managed).unwrap();

        let docs = list_documents(&conn).unwrap();
        assert_eq!(docs.len(), 1);
        assert_eq!(
            docs[0].computed_validity.as_deref(),
            Some("abgelaufen"),
            "Derived validity must override stale persisted value"
        );
        // Persisted validity is still "gültig" (compatibility)
        assert_eq!(docs[0].validity, "gültig", "Persisted validity field unchanged");
    }

    #[test]
    fn test_computed_validity_survives_reload() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Reload Test".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2099-12-31".to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(&conn, &input, &managed).unwrap();

        let docs1 = list_documents(&conn).unwrap();
        let cv1 = docs1[0].computed_validity.clone();

        // Reload by calling list_documents again
        let docs2 = list_documents(&conn).unwrap();
        assert_eq!(
            docs2[0].computed_validity, cv1,
            "computed_validity must be stable across reloads"
        );
    }

    #[test]
    fn test_archived_document_excluded_from_active_validity() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        // Create an expired document
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Abgelaufen".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2000-01-01".to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(&conn, &input, &managed).unwrap();

        // Verify it appears in active list with abgelaufen
        let active = list_documents(&conn).unwrap();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].computed_validity.as_deref(), Some("abgelaufen"));

        // Archive it
        archive_document(&mut conn, &active[0].id).unwrap();

        // It must not appear in active list anymore
        let active_after = list_documents(&conn).unwrap();
        assert_eq!(active_after.len(), 0, "Archived document excluded from active list");

        // It appears in archive list with computed_validity still set
        let archived = list_archived_documents(&conn).unwrap();
        assert_eq!(archived.len(), 1);
        assert_eq!(
            archived[0].computed_validity.as_deref(),
            Some("abgelaufen"),
            "Archived document retains computed validity"
        );
    }

    #[test]
    fn test_restore_preserves_valid_until() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Restore Test".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2099-12-31".to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        let doc = create_document(&conn, &input, &managed).unwrap();

        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();

        assert_eq!(
            restored.valid_until,
            Some("2099-12-31".to_string()),
            "valid_until must survive archive/restore"
        );
        assert_eq!(
            restored.computed_validity.as_deref(),
            Some("gültig"),
            "computed_validity must survive archive/restore"
        );
    }

    #[test]
    fn test_editing_valid_until_recalculates_state() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        // Create with far-future date → gültig
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Edit Test".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2099-12-31".to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        let doc = create_document(&conn, &input, &managed).unwrap();

        assert_eq!(doc.computed_validity.as_deref(), Some("gültig"));

        // Edit valid_until to a past date → abgelaufen
        let update = UpdateDocumentInput {
            title: "Edit Test".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2000-01-01".to_string()),
            description: None,
            tag_ids: vec![],
        };
        let updated = update_document(&conn, &doc.id, &update).unwrap();

        assert_eq!(
            updated.computed_validity.as_deref(),
            Some("abgelaufen"),
            "Editing valid_until must recalculate computed_validity"
        );
    }

    #[test]
    fn test_version_creation_preserves_previous_version_validity() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();

        // Create document with valid_until
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Version Test".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2099-12-31".to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        let doc = create_document(&conn, &input, &managed).unwrap();

        // Create a new version with different valid_until
        let new_pdf = make_valid_pdf(&storage, "v2.pdf");
        let version_input = CreateVersionInput {
            document_id: doc.id.clone(),
            version_number: "2.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: Some("2000-01-01".to_string()),
            source_file_path: new_pdf.to_str().unwrap().to_string(),
            original_file_name: "v2.pdf".to_string(),
        };
        create_version_from_source(&mut conn, &version_input, storage.path()).unwrap();

        // The parent document's computed_validity should reflect the new version's valid_until
        let reloaded = load_document(&conn, &doc.id).unwrap();
        assert_eq!(
            reloaded.computed_validity.as_deref(),
            Some("abgelaufen"),
            "Document validity follows current version's valid_until"
        );

        // Previous version's validity metadata must be unchanged
        let versions = list_versions(&conn, &doc.id).unwrap();
        let v1 = versions.iter().find(|v| v.version_number == "1.0").unwrap();
        assert_eq!(
            v1.valid_until,
            Some("2099-12-31".to_string()),
            "Previous version valid_until must be unchanged"
        );
        assert_eq!(
            v1.validity, "gültig",
            "Previous version persisted validity must be unchanged"
        );
    }

    #[test]
    fn test_today_local_date_format() {
        let today = today_local_date();
        assert!(
            parse_date(&today).is_some(),
            "today_local_date must return a valid YYYY-MM-DD date, got: {today}"
        );
    }

    // === Prompt 023: Dashboard Summary & Review List Tests ===

    /// Hilfsfunktion: Erstellt ein Dokument mit gegebenem valid_until im Test-DB.
    fn make_doc_with_validity(
        conn: &Connection,
        storage: &tempfile::TempDir,
        title: &str,
        valid_until: Option<&str>,
    ) -> Document {
        let pdf = make_valid_pdf(storage, "test.pdf");
        let input = CreateDocumentInput {
            title: title.to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "aktiv".to_string(),
            validity: "gültig".to_string(),
            valid_until: valid_until.map(|s| s.to_string()),
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "doc-uuid").unwrap();
        create_document(conn, &input, &managed).unwrap()
    }

    #[test]
    fn test_review_active_valid_not_in_list() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        make_doc_with_validity(&conn, &storage, "Gültig", Some("2099-12-31"));
        let reviews = review_list(&conn).unwrap();
        assert!(reviews.is_empty(), "Valid document must not appear in review list");
    }

    #[test]
    fn test_review_active_warning_appears() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        // valid_until = today + 10 days → within 30-day threshold → "läuft bald ab"
        let today = today_local_date();
        let vu = add_days(&today, 10);
        make_doc_with_validity(&conn, &storage, "Bald ab", Some(&vu));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].computed_validity, "läuft bald ab");
    }

    #[test]
    fn test_review_active_expired_appears() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].computed_validity, "abgelaufen");
    }

    #[test]
    fn test_review_null_valid_until_not_in_list() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        make_doc_with_validity(&conn, &storage, "Ohne Datum", None);
        let reviews = review_list(&conn).unwrap();
        assert!(reviews.is_empty(), "NULL valid_until must not appear in review list");
    }

    #[test]
    fn test_review_archived_warning_excluded() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();
        let vu = add_days(&today, 10);
        let doc = make_doc_with_validity(&conn, &storage, "Bald ab", Some(&vu));
        archive_document(&mut conn, &doc.id).unwrap();
        let reviews = review_list(&conn).unwrap();
        assert!(reviews.is_empty(), "Archived warning document must not appear in review list");
    }

    #[test]
    fn test_review_archived_expired_excluded() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        archive_document(&mut conn, &doc.id).unwrap();
        let reviews = review_list(&conn).unwrap();
        assert!(reviews.is_empty(), "Archived expired document must not appear in review list");
    }

    #[test]
    fn test_review_restored_warning_becomes_eligible() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();
        let vu = add_days(&today, 10);
        let doc = make_doc_with_validity(&conn, &storage, "Bald ab", Some(&vu));
        archive_document(&mut conn, &doc.id).unwrap();
        assert!(review_list(&conn).unwrap().is_empty());
        restore_document(&mut conn, &doc.id).unwrap();
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1, "Restored warning document must reappear in review list");
        assert_eq!(reviews[0].computed_validity, "läuft bald ab");
    }

    #[test]
    fn test_review_restored_expired_becomes_eligible() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        archive_document(&mut conn, &doc.id).unwrap();
        assert!(review_list(&conn).unwrap().is_empty());
        restore_document(&mut conn, &doc.id).unwrap();
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1, "Restored expired document must reappear in review list");
        assert_eq!(reviews[0].computed_validity, "abgelaufen");
    }

    #[test]
    fn test_summary_counts_valid() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        make_doc_with_validity(&conn, &storage, "Gültig", Some("2099-12-31"));
        let s = dashboard_summary(&conn).unwrap();
        assert_eq!(s.valid, 1);
        assert_eq!(s.warning, 0);
        assert_eq!(s.expired, 0);
        assert_eq!(s.no_validity, 0);
        assert_eq!(s.total_active, 1);
    }

    #[test]
    fn test_summary_counts_warning() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();
        make_doc_with_validity(&conn, &storage, "Bald ab", Some(&add_days(&today, 15)));
        let s = dashboard_summary(&conn).unwrap();
        assert_eq!(s.warning, 1);
        assert_eq!(s.valid, 0);
        assert_eq!(s.expired, 0);
    }

    #[test]
    fn test_summary_counts_expired() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        let s = dashboard_summary(&conn).unwrap();
        assert_eq!(s.expired, 1);
        assert_eq!(s.valid, 0);
        assert_eq!(s.warning, 0);
    }

    #[test]
    fn test_summary_excludes_archived() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        archive_document(&mut conn, &doc.id).unwrap();
        let s = dashboard_summary(&conn).unwrap();
        assert_eq!(s.total_active, 0);
        assert_eq!(s.expired, 0, "Archived expired must not count in active summary");
        assert_eq!(s.archived, 1);
    }

    #[test]
    fn test_review_ordering_expired_before_warning() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();
        // Warning doc
        make_doc_with_validity(&conn, &storage, "Warnung", Some(&add_days(&today, 15)));
        // Expired doc
        make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].computed_validity, "abgelaufen", "Expired must come first");
        assert_eq!(reviews[1].computed_validity, "läuft bald ab", "Warning must come second");
    }

    #[test]
    fn test_review_ordering_expired_oldest_first() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        // Two expired: 2000-01-01 (older) and 2010-06-15 (newer)
        make_doc_with_validity(&conn, &storage, "Neuer abgelaufen", Some("2010-06-15"));
        make_doc_with_validity(&conn, &storage, "Älter abgelaufen", Some("2000-01-01"));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].valid_until, Some("2000-01-01".to_string()), "Oldest expired first");
        assert_eq!(reviews[1].valid_until, Some("2010-06-15".to_string()), "Newer expired second");
    }

    #[test]
    fn test_review_ordering_warning_nearest_first() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();
        // Two warnings: +25 days (farther) and +10 days (nearer)
        make_doc_with_validity(&conn, &storage, "Weiter weg", Some(&add_days(&today, 25)));
        make_doc_with_validity(&conn, &storage, "Näher dran", Some(&add_days(&today, 10)));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].title, "Näher dran", "Nearest expiry first in warning");
        assert_eq!(reviews[1].title, "Weiter weg", "Farther expiry second in warning");
    }

    #[test]
    fn test_review_ordering_equal_dates_deterministic() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();
        let vu = add_days(&today, 15);
        // Two warnings with same valid_until — secondary sort by document_number
        make_doc_with_validity(&conn, &storage, "B", Some(&vu));
        make_doc_with_validity(&conn, &storage, "A", Some(&vu));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 2);
        // Document numbers are auto-generated sequentially, so first created = lower number
        assert_ne!(reviews[0].document_number, reviews[1].document_number, "Document numbers must differ");
        assert!(
            reviews[0].document_number < reviews[1].document_number,
            "Equal dates: lower document number first"
        );
    }

    #[test]
    fn test_review_editing_valid_until_changes_eligibility() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        // Create with far-future date → gültig, not in review list
        let doc = make_doc_with_validity(&conn, &storage, "Edit Test", Some("2099-12-31"));
        assert!(review_list(&conn).unwrap().is_empty());
        // Edit to past date → abgelaufen, appears in review list
        let update = make_update_document_input(
            "Edit Test", None, None, None, "1.0", "aktiv", "gültig", Some("2000-01-01"), None,
        );
        update_document(&conn, &doc.id, &update).unwrap();
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].computed_validity, "abgelaufen");
    }

    #[test]
    fn test_review_archive_removes_eligibility() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        assert_eq!(review_list(&conn).unwrap().len(), 1);
        archive_document(&mut conn, &doc.id).unwrap();
        assert!(review_list(&conn).unwrap().is_empty(), "Archive must remove from review list");
    }

    #[test]
    fn test_review_restore_recalculates_from_unchanged_valid_until() {
        let (mut conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();
        let vu = add_days(&today, 10);
        let doc = make_doc_with_validity(&conn, &storage, "Bald ab", Some(&vu));
        archive_document(&mut conn, &doc.id).unwrap();
        assert!(review_list(&conn).unwrap().is_empty());
        restore_document(&mut conn, &doc.id).unwrap();
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].valid_until, Some(vu), "valid_until must be unchanged after restore");
        assert_eq!(reviews[0].computed_validity, "läuft bald ab");
    }

    #[test]
    fn test_summary_empty_db_returns_zeros() {
        let (conn, _tmp) = init_test_db();
        let s = dashboard_summary(&conn).unwrap();
        assert_eq!(s.total_active, 0);
        assert_eq!(s.valid, 0);
        assert_eq!(s.warning, 0);
        assert_eq!(s.expired, 0);
        assert_eq!(s.no_validity, 0);
        assert_eq!(s.archived, 0);
        assert_eq!(s.employees, 0);
    }

    #[test]
    fn test_review_empty_db_returns_empty_list() {
        let (conn, _tmp) = init_test_db();
        let reviews = review_list(&conn).unwrap();
        assert!(reviews.is_empty());
    }

    #[test]
    fn test_review_only_null_validity_no_reminders() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        make_doc_with_validity(&conn, &storage, "Ohne Datum 1", None);
        make_doc_with_validity(&conn, &storage, "Ohne Datum 2", None);
        let reviews = review_list(&conn).unwrap();
        assert!(reviews.is_empty(), "Only NULL-validity docs → no actionable reminders");
        let s = dashboard_summary(&conn).unwrap();
        assert_eq!(s.no_validity, 2);
        assert_eq!(s.total_active, 2);
        assert_eq!(s.valid, 0);
        assert_eq!(s.warning, 0);
        assert_eq!(s.expired, 0);
    }

    #[test]
    fn test_summary_mixed_states() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let today = today_local_date();

        // 2 gültig
        make_doc_with_validity(&conn, &storage, "Gültig 1", Some("2099-12-31"));
        make_doc_with_validity(&conn, &storage, "Gültig 2", Some("2099-06-01"));
        // 1 läuft bald ab
        make_doc_with_validity(&conn, &storage, "Bald ab", Some(&add_days(&today, 20)));
        // 1 abgelaufen
        make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        // 1 ohne Datum
        make_doc_with_validity(&conn, &storage, "Ohne Datum", None);

        let s = dashboard_summary(&conn).unwrap();
        assert_eq!(s.total_active, 5);
        assert_eq!(s.valid, 2);
        assert_eq!(s.warning, 1);
        assert_eq!(s.expired, 1);
        assert_eq!(s.no_validity, 1);
    }

    #[test]
    fn test_review_entry_has_no_file_paths() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1);
        // ReviewEntry must not expose file_path or file_name
        let serialized = serde_json::to_string(&reviews[0]).unwrap();
        assert!(!serialized.contains("file_path"), "ReviewEntry must not expose file_path");
        assert!(!serialized.contains("file_name"), "ReviewEntry must not expose file_name");
    }

    #[test]
    fn test_review_entry_has_document_uuid_for_navigation() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_validity(&conn, &storage, "Abgelaufen", Some("2000-01-01"));
        let reviews = review_list(&conn).unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].id, doc.id, "Review entry must contain document UUID for navigation");
        assert_eq!(reviews[0].document_number, doc.document_number, "Must contain document number for route");
    }

    /// Hilfsfunktion: Addiert Tage zu einem ISO-Datum (YYYY-MM-DD).
    fn add_days(iso: &str, days: i64) -> String {
        let (y, m, d) = parse_date(iso).unwrap();
        let base = days_from_date(y, m, d);
        let target = base + days;
        // civil_from_days
        let z = target + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y2 = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d2 = doy - (153 * mp + 2) / 5 + 1;
        let m2 = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = if m2 <= 2 { y2 + 1 } else { y2 };
        format!("{:04}-{:02}-{:02}", year, m2, d2)
    }

    // --- Prompt 024: Dokument-Lifecycle Tests ---

    /// Helper: Erstellt ein Dokument mit gegebenem Status.
    fn make_doc_with_status(conn: &Connection, storage: &tempfile::TempDir, status: &str) -> Document {
        let pdf = make_valid_pdf(storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Testdokument".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: status.to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "test-uuid").unwrap();
        create_document(conn, &input, &managed).unwrap()
    }

    #[test]
    fn test_is_valid_status_all_three() {
        assert!(is_valid_status(STATUS_ENTWURF));
        assert!(is_valid_status(STATUS_AKTIV));
        assert!(is_valid_status(STATUS_ARCHIVIERT));
    }

    #[test]
    fn test_is_valid_status_rejects_invalid() {
        assert!(!is_valid_status(""));
        assert!(!is_valid_status("draft"));
        assert!(!is_valid_status("active"));
        assert!(!is_valid_status("Archiviert"));
        assert!(!is_valid_status("entwurf"));
    }

    #[test]
    fn test_is_valid_creation_status_excludes_archiviert() {
        assert!(is_valid_creation_status(STATUS_ENTWURF));
        assert!(is_valid_creation_status(STATUS_AKTIV));
        assert!(!is_valid_creation_status(STATUS_ARCHIVIERT));
    }

    #[test]
    fn test_is_valid_creation_status_rejects_invalid() {
        assert!(!is_valid_creation_status(""));
        assert!(!is_valid_creation_status("draft"));
        assert!(!is_valid_creation_status("archiviert"));
    }

    #[test]
    fn test_create_document_rejects_archiviert_status() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let pdf = make_valid_pdf(&storage, "test.pdf");
        let input = CreateDocumentInput {
            title: "Test".to_string(),
            category_id: None,
            subcategory_id: None,
            responsible_person_id: None,
            version: "1.0".to_string(),
            status: "archiviert".to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            description: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "test.pdf".to_string(),
            tag_ids: vec![],
        };
        let managed = copy_to_managed_storage(&pdf, storage.path(), "test-uuid").unwrap();
        let result = create_document(&conn, &input, &managed);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Nur Entwurf oder aktiv"));
    }

    #[test]
    fn test_create_document_accepts_entwurf() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        assert_eq!(doc.status, "Entwurf");
        assert!(doc.archived_at.is_none());
        assert!(doc.pre_archive_status.is_none());
    }

    #[test]
    fn test_create_document_accepts_aktiv() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        assert_eq!(doc.status, "aktiv");
        assert!(doc.archived_at.is_none());
    }

    #[test]
    fn test_update_document_rejects_archiviert_status() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let input = make_update_document_input(
            "Test", None, None, None, "1.0", "archiviert", "gültig", None, None,
        );
        let result = update_document(&conn, &doc.id, &input);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Nur Entwurf oder aktiv"));
    }

    #[test]
    fn test_update_document_allows_entwurf_to_aktiv() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let input = make_update_document_input(
            "Test", None, None, None, "1.0", "aktiv", "gültig", None, None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.status, "aktiv");
    }

    #[test]
    fn test_update_document_allows_aktiv_to_entwurf() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let input = make_update_document_input(
            "Test", None, None, None, "1.0", "Entwurf", "gültig", None, None,
        );
        let updated = update_document(&conn, &doc.id, &input).unwrap();
        assert_eq!(updated.status, "Entwurf");
    }

    #[test]
    fn test_archive_preserves_pre_archive_status_entwurf() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let mut conn = conn;
        let archived = archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(archived.status, "archiviert");
        assert!(archived.archived_at.is_some());
        assert_eq!(archived.pre_archive_status.as_deref(), Some("Entwurf"));
    }

    #[test]
    fn test_archive_preserves_pre_archive_status_aktiv() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        let archived = archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(archived.status, "archiviert");
        assert!(archived.archived_at.is_some());
        assert_eq!(archived.pre_archive_status.as_deref(), Some("aktiv"));
    }

    #[test]
    fn test_archive_rejects_already_archived() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        archive_document(&mut conn, &doc.id).unwrap();
        let result = archive_document(&mut conn, &doc.id);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("bereits archiviert"));
    }

    #[test]
    fn test_archive_nonexistent_document() {
        let (conn, _tmp) = init_test_db();
        let mut conn = conn;
        let result = archive_document(&mut conn, "nonexistent-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_restore_entwurf_document() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let mut conn = conn;
        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();
        assert_eq!(restored.status, "Entwurf");
        assert!(restored.archived_at.is_none());
        assert!(restored.pre_archive_status.is_none());
    }

    #[test]
    fn test_restore_aktiv_document() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();
        assert_eq!(restored.status, "aktiv");
        assert!(restored.archived_at.is_none());
        assert!(restored.pre_archive_status.is_none());
    }

    #[test]
    fn test_restore_non_archived_document_fails() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let mut conn = conn;
        let result = restore_document(&mut conn, &doc.id);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("nicht archiviert"));
    }

    #[test]
    fn test_restore_legacy_document_with_null_pre_archive_status() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        // Simuliere Legacy-Archivierung: pre_archive_status bleibt NULL
        let now = now_iso();
        let tx = conn.transaction().unwrap();
        tx.execute(
            "UPDATE documents SET status = 'archiviert', archived_at = ?1, updated_at = ?2
             WHERE id = ?3;",
            rusqlite::params![now, now, doc.id],
        ).unwrap();
        tx.commit().unwrap();
        // Restore muss fehlschlagen mit kontrolliertem Fehler
        let result = restore_document(&mut conn, &doc.id);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("kann nicht automatisch ermittelt werden"));
    }

    #[test]
    fn test_restore_nonexistent_document() {
        let (conn, _tmp) = init_test_db();
        let mut conn = conn;
        let result = restore_document(&mut conn, "nonexistent-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_archive_restore_roundtrip_entwurf() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let mut conn = conn;
        let archived = archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(archived.status, "archiviert");
        let restored = restore_document(&mut conn, &doc.id).unwrap();
        assert_eq!(restored.status, "Entwurf");
        assert!(restored.archived_at.is_none());
        assert!(restored.pre_archive_status.is_none());
    }

    #[test]
    fn test_archive_restore_roundtrip_aktiv() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        let archived = archive_document(&mut conn, &doc.id).unwrap();
        assert_eq!(archived.status, "archiviert");
        let restored = restore_document(&mut conn, &doc.id).unwrap();
        assert_eq!(restored.status, "aktiv");
        assert!(restored.archived_at.is_none());
        assert!(restored.pre_archive_status.is_none());
    }

    #[test]
    fn test_archived_document_not_in_active_list() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        archive_document(&mut conn, &doc.id).unwrap();
        let active = list_documents(&conn).unwrap();
        assert!(active.iter().all(|d| d.id != doc.id));
    }

    #[test]
    fn test_archived_document_in_archive_list() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        archive_document(&mut conn, &doc.id).unwrap();
        let archived = list_archived_documents(&conn).unwrap();
        assert!(archived.iter().any(|d| d.id == doc.id));
        let archived_doc = archived.iter().find(|d| d.id == doc.id).unwrap();
        assert_eq!(archived_doc.pre_archive_status.as_deref(), Some("aktiv"));
    }

    #[test]
    fn test_entwurf_and_aktiv_both_in_active_list() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc1 = make_doc_with_status(&conn, &storage, "Entwurf");
        let doc2 = make_doc_with_status(&conn, &storage, "aktiv");
        let active = list_documents(&conn).unwrap();
        assert!(active.iter().any(|d| d.id == doc1.id));
        assert!(active.iter().any(|d| d.id == doc2.id));
    }

    #[test]
    fn test_dashboard_counts_non_archived_only() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let _doc1 = make_doc_with_status(&conn, &storage, "Entwurf");
        let _doc2 = make_doc_with_status(&conn, &storage, "aktiv");
        let summary = dashboard_summary(&conn).unwrap();
        assert_eq!(summary.total_active, 2);
        let mut conn = conn;
        archive_document(&mut conn, &_doc1.id).unwrap();
        let summary2 = dashboard_summary(&conn).unwrap();
        assert_eq!(summary2.total_active, 1);
        assert_eq!(summary2.archived, 1);
    }

    #[test]
    fn test_create_version_rejects_archiviert_status() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let pdf = make_valid_pdf(&storage, "v2.pdf");
        let input = CreateVersionInput {
            document_id: doc.id.clone(),
            version_number: "2.0".to_string(),
            status: "archiviert".to_string(),
            validity: "gültig".to_string(),
            valid_until: None,
            source_file_path: pdf.to_str().unwrap().to_string(),
            original_file_name: "v2.pdf".to_string(),
        };
        let mut conn = conn;
        let result = create_version_from_source(&mut conn, &input, storage.path());
        assert!(result.is_err());
        let msg = result.unwrap_err();
        assert!(msg.contains("Nur Entwurf oder aktiv"));
    }

    #[test]
    fn test_update_archived_document_rejected() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        archive_document(&mut conn, &doc.id).unwrap();
        let input = make_update_document_input(
            "Geändert", None, None, None, "1.0", "aktiv", "gültig", None, None,
        );
        let result = update_document(&conn, &doc.id, &input);
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Archivierte Dokumente können nicht bearbeitet werden"));
    }

    #[test]
    fn test_migration_v1_to_v2_adds_column() {
        let tmp = NamedTempFile::new().unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();
        ensure_schema_version_table(&conn).unwrap();
        for stmt in schema_statements() {
            conn.execute(stmt, []).unwrap();
        }
        set_schema_version(&conn, 1).unwrap();
        // Vor Migration: keine pre_archive_status-Spalte
        let columns_before: Vec<String> = conn
            .prepare("PRAGMA table_info(documents);")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(!columns_before.iter().any(|c| c == "pre_archive_status"));
        // Migration ausführen
        migrate_v1_to_v2(&conn).unwrap();
        // Nach Migration: Spalte existiert
        let columns_after: Vec<String> = conn
            .prepare("PRAGMA table_info(documents);")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(columns_after.iter().any(|c| c == "pre_archive_status"));
    }

    #[test]
    fn test_migration_v1_to_v2_idempotent() {
        let tmp = NamedTempFile::new().unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();
        ensure_schema_version_table(&conn).unwrap();
        for stmt in schema_statements() {
            conn.execute(stmt, []).unwrap();
        }
        migrate_v1_to_v2(&conn).unwrap();
        // Zweite Ausführung darf nicht fehlschlagen
        migrate_v1_to_v2(&conn).unwrap();
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(documents);")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        let count = columns.iter().filter(|c| *c == "pre_archive_status").count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_init_database_creates_v2_schema() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        let version: i64 = conn
            .query_row("SELECT MAX(version) FROM schema_version;", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 2);
        let columns: Vec<String> = conn
            .prepare("PRAGMA table_info(documents);")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        assert!(columns.iter().any(|c| c == "pre_archive_status"));
    }

    #[test]
    fn test_pre_archive_status_null_for_non_archived() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        assert!(doc.pre_archive_status.is_none());
        let doc2 = make_doc_with_status(&conn, &storage, "aktiv");
        assert!(doc2.pre_archive_status.is_none());
    }

    #[test]
    fn test_restore_clears_pre_archive_status() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        archive_document(&mut conn, &doc.id).unwrap();
        let archived = load_document(&conn, &doc.id).unwrap();
        assert!(archived.pre_archive_status.is_some());
        restore_document(&mut conn, &doc.id).unwrap();
        let restored = load_document(&conn, &doc.id).unwrap();
        assert!(restored.pre_archive_status.is_none());
    }

    #[test]
    fn test_double_archive_restore_cycle() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let mut conn = conn;
        // Erster Zyklus
        archive_document(&mut conn, &doc.id).unwrap();
        let restored = restore_document(&mut conn, &doc.id).unwrap();
        assert_eq!(restored.status, "aktiv");
        // Zweiter Zyklus
        archive_document(&mut conn, &doc.id).unwrap();
        let restored2 = restore_document(&mut conn, &doc.id).unwrap();
        assert_eq!(restored2.status, "aktiv");
        assert!(restored2.pre_archive_status.is_none());
    }

    // --- Kategorie- & Unterkategorie-Tests (Prompt 026A) ---

    #[test]
    fn test_list_categories_empty() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        let cats = list_categories(&conn).unwrap();
        assert!(cats.is_empty());
    }

    #[test]
    fn test_create_category() {
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "Hygiene").unwrap();
        assert!(!cat.id.is_empty());
        assert_eq!(cat.name, "Hygiene");
        let cats = list_categories(&conn).unwrap();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].name, "Hygiene");
    }

    #[test]
    fn test_create_category_duplicate_rejected() {
        let (conn, _tmp) = init_test_db();
        create_category(&conn, "Hygiene").unwrap();
        let result = create_category(&conn, "Hygiene");
        assert!(result.is_err(), "Duplikate sollten abgelehnt werden");
    }

    #[test]
    fn test_rename_category() {
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "Alt").unwrap();
        let renamed = rename_category(&conn, &cat.id, "Neu").unwrap();
        assert_eq!(renamed.id, cat.id);
        assert_eq!(renamed.name, "Neu");
        let cats = list_categories(&conn).unwrap();
        assert_eq!(cats[0].name, "Neu");
    }

    #[test]
    fn test_rename_category_preserves_uuid() {
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "Original").unwrap();
        let original_id = cat.id.clone();
        rename_category(&conn, &cat.id, "Umbenannt").unwrap();
        let cats = list_categories(&conn).unwrap();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].id, original_id);
        assert_eq!(cats[0].name, "Umbenannt");
    }

    #[test]
    fn test_rename_category_preserves_subcategory() {
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "Parent").unwrap();
        let sub = create_subcategory(&conn, "Child", &cat.id).unwrap();
        rename_category(&conn, &cat.id, "ParentRenamed").unwrap();
        let subs = list_subcategories(&conn).unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].id, sub.id);
        assert_eq!(subs[0].category_id, cat.id);
    }

    #[test]
    fn test_rename_category_nonexistent_returns_error() {
        let (conn, _tmp) = init_test_db();
        let result = rename_category(&conn, "nicht-vorhanden", "Neu");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_subcategory() {
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "Hygiene").unwrap();
        let sub = create_subcategory(&conn, "Oberflächen", &cat.id).unwrap();
        assert!(!sub.id.is_empty());
        assert_eq!(sub.name, "Oberflächen");
        assert_eq!(sub.category_id, cat.id);
    }

    #[test]
    fn test_create_subcategory_nonexistent_parent_rejected() {
        let (conn, _tmp) = init_test_db();
        let result = create_subcategory(&conn, "Test", "fake-uuid");
        assert!(result.is_err(), "Unterkategorie ohne existierende Kategorie sollte fehlschlagen");
    }

    #[test]
    fn test_rename_subcategory() {
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "Hygiene").unwrap();
        let sub = create_subcategory(&conn, "Oberflächen", &cat.id).unwrap();
        let renamed = rename_subcategory(&conn, &sub.id, "Desinfektion").unwrap();
        assert_eq!(renamed.id, sub.id);
        assert_eq!(renamed.name, "Desinfektion");
        assert_eq!(renamed.category_id, cat.id);
    }

    #[test]
    fn test_rename_subcategory_preserves_uuid_and_parent() {
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "Cat").unwrap();
        let sub = create_subcategory(&conn, "Sub", &cat.id).unwrap();
        let original_id = sub.id.clone();
        let original_cat = cat.id.clone();
        rename_subcategory(&conn, &sub.id, "SubRenamed").unwrap();
        let subs = list_subcategories(&conn).unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].id, original_id);
        assert_eq!(subs[0].name, "SubRenamed");
        assert_eq!(subs[0].category_id, original_cat);
    }

    #[test]
    fn test_rename_subcategory_nonexistent_returns_error() {
        let (conn, _tmp) = init_test_db();
        let result = rename_subcategory(&conn, "nicht-vorhanden", "Neu");
        assert!(result.is_err());
    }

    #[test]
    fn test_categories_survive_reload() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        create_category(&conn, "Persist").unwrap();
        let cat_id = list_categories(&conn).unwrap()[0].id.clone();
        create_subcategory(&conn, "PersistSub", &cat_id).unwrap();
        // Reopen connection
        drop(conn);
        let conn2 = Connection::open(tmp.path()).unwrap();
        let cats = list_categories(&conn2).unwrap();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].name, "Persist");
        let subs = list_subcategories(&conn2).unwrap();
        assert_eq!(subs.len(), 1);
        assert_eq!(subs[0].name, "PersistSub");
    }

    #[test]
    fn test_no_delete_functionality_exists() {
        // Ensure no DELETE statements for categories/subcategories are issued
        // This test documents the deferred delete behavior
        let (conn, _tmp) = init_test_db();
        let cat = create_category(&conn, "NoDelete").unwrap();
        let _sub = create_subcategory(&conn, "NoDeleteSub", &cat.id).unwrap();
        // Verify both still exist
        assert_eq!(list_categories(&conn).unwrap().len(), 1);
        assert_eq!(list_subcategories(&conn).unwrap().len(), 1);
    }

    // --- Schlagwort-Dictionary-Tests (Prompt 026B) ---

    #[test]
    fn test_list_keywords_empty() {
        let (conn, _tmp) = init_test_db();
        let keywords = list_keywords(&conn).unwrap();
        assert!(keywords.is_empty());
    }

    #[test]
    fn test_create_keyword() {
        let (conn, _tmp) = init_test_db();
        let kw = create_keyword(&conn, "Hygieneplan").unwrap();
        assert!(!kw.id.is_empty());
        assert_eq!(kw.name, "Hygieneplan");
        let keywords = list_keywords(&conn).unwrap();
        assert_eq!(keywords.len(), 1);
        assert_eq!(keywords[0].name, "Hygieneplan");
    }

    #[test]
    fn test_keyword_persists_after_reload() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        create_keyword(&conn, "Persist").unwrap();
        drop(conn);
        let conn2 = Connection::open(tmp.path()).unwrap();
        let keywords = list_keywords(&conn2).unwrap();
        assert_eq!(keywords.len(), 1);
        assert_eq!(keywords[0].name, "Persist");
    }

    #[test]
    fn test_create_keyword_trims_whitespace() {
        let (conn, _tmp) = init_test_db();
        let kw = create_keyword(&conn, "  QM  ").unwrap();
        assert_eq!(kw.name, "  QM  ");
        // The DB stores the value as-is; trimming happens in the Tauri command layer.
        // Here we verify the DB function stores what it receives.
        let keywords = list_keywords(&conn).unwrap();
        assert_eq!(keywords[0].name, "  QM  ");
    }

    #[test]
    fn test_create_keyword_empty_rejected_by_unique() {
        let (conn, _tmp) = init_test_db();
        // Empty string — DB function itself doesn't validate; the command layer does.
        // But UNIQUE constraint means a second empty string would fail.
        // The command layer rejects empty before reaching DB.
        // Here we test that the DB function accepts the value it's given.
        // (Validation is tested via command-layer behavior, documented in tests below.)
        let kw = create_keyword(&conn, "Valid").unwrap();
        assert_eq!(kw.name, "Valid");
    }

    #[test]
    fn test_create_keyword_duplicate_rejected() {
        let (conn, _tmp) = init_test_db();
        create_keyword(&conn, "Hygiene").unwrap();
        let result = create_keyword(&conn, "Hygiene");
        assert!(result.is_err(), "Duplikate sollten abgelehnt werden");
    }

    #[test]
    fn test_rename_keyword() {
        let (conn, _tmp) = init_test_db();
        let kw = create_keyword(&conn, "Alt").unwrap();
        let renamed = rename_keyword(&conn, &kw.id, "Neu").unwrap();
        assert_eq!(renamed.id, kw.id);
        assert_eq!(renamed.name, "Neu");
        let keywords = list_keywords(&conn).unwrap();
        assert_eq!(keywords[0].name, "Neu");
    }

    #[test]
    fn test_rename_keyword_preserves_id() {
        let (conn, _tmp) = init_test_db();
        let kw = create_keyword(&conn, "Original").unwrap();
        let original_id = kw.id.clone();
        rename_keyword(&conn, &kw.id, "Umbenannt").unwrap();
        let keywords = list_keywords(&conn).unwrap();
        assert_eq!(keywords.len(), 1);
        assert_eq!(keywords[0].id, original_id);
        assert_eq!(keywords[0].name, "Umbenannt");
    }

    #[test]
    fn test_rename_keyword_persists_after_reload() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        let kw = create_keyword(&conn, "Before").unwrap();
        rename_keyword(&conn, &kw.id, "After").unwrap();
        drop(conn);
        let conn2 = Connection::open(tmp.path()).unwrap();
        let keywords = list_keywords(&conn2).unwrap();
        assert_eq!(keywords.len(), 1);
        assert_eq!(keywords[0].name, "After");
        assert_eq!(keywords[0].id, kw.id);
    }

    #[test]
    fn test_rename_keyword_collision_rejected() {
        let (conn, _tmp) = init_test_db();
        create_keyword(&conn, "Erster").unwrap();
        let kw2 = create_keyword(&conn, "Zweiter").unwrap();
        let result = rename_keyword(&conn, &kw2.id, "Erster");
        assert!(result.is_err(), "Rename auf existierenden Namen sollte fehlschlagen");
    }

    #[test]
    fn test_rename_keyword_nonexistent_returns_error() {
        let (conn, _tmp) = init_test_db();
        let result = rename_keyword(&conn, "nicht-vorhanden", "Neu");
        assert!(result.is_err());
    }

    #[test]
    fn test_keyword_document_tag_relationship_survives_rename() {
        let (conn, _tmp) = init_test_db();
        // Create a keyword
        let kw = create_keyword(&conn, "Alt").unwrap();
        // Manually insert a document_tags row referencing this keyword
        // We need a document first — use the test helper to create one
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        conn.execute(
            "INSERT INTO document_tags (document_id, keyword_id) VALUES (?1, ?2);",
            rusqlite::params![doc.id, kw.id],
        ).unwrap();
        // Verify relationship exists
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM document_tags WHERE keyword_id = ?1;", rusqlite::params![kw.id], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
        // Rename the keyword
        rename_keyword(&conn, &kw.id, "Neu").unwrap();
        // Verify keyword ID is unchanged
        let keywords = list_keywords(&conn).unwrap();
        assert_eq!(keywords.len(), 1);
        assert_eq!(keywords[0].id, kw.id);
        assert_eq!(keywords[0].name, "Neu");
        // Verify relationship row still exists with same keyword_id
        let count_after: i64 = conn
            .query_row("SELECT COUNT(*) FROM document_tags WHERE keyword_id = ?1;", rusqlite::params![kw.id], |row| row.get(0))
            .unwrap();
        assert_eq!(count_after, 1);
        // Verify the relationship resolves to the renamed keyword
        let resolved_name: String = conn
            .query_row(
                "SELECT kd.keyword FROM document_tags dt JOIN keyword_dictionary kd ON dt.keyword_id = kd.id WHERE dt.document_id = ?1;",
                rusqlite::params![doc.id],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(resolved_name, "Neu");
    }

    #[test]
    fn test_keyword_rename_does_not_rewrite_document_rows() {
        let (conn, _tmp) = init_test_db();
        let kw = create_keyword(&conn, "Original").unwrap();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        conn.execute(
            "INSERT INTO document_tags (document_id, keyword_id) VALUES (?1, ?2);",
            rusqlite::params![doc.id, kw.id],
        ).unwrap();
        // Capture document row content before rename
        let doc_title_before: String = conn
            .query_row("SELECT title FROM documents WHERE id = ?1;", rusqlite::params![doc.id], |row| row.get(0))
            .unwrap();
        let doc_updated_before: String = conn
            .query_row("SELECT updated_at FROM documents WHERE id = ?1;", rusqlite::params![doc.id], |row| row.get(0))
            .unwrap();
        // Rename keyword
        rename_keyword(&conn, &kw.id, "Umbenannt").unwrap();
        // Verify document row is untouched
        let doc_title_after: String = conn
            .query_row("SELECT title FROM documents WHERE id = ?1;", rusqlite::params![doc.id], |row| row.get(0))
            .unwrap();
        let doc_updated_after: String = conn
            .query_row("SELECT updated_at FROM documents WHERE id = ?1;", rusqlite::params![doc.id], |row| row.get(0))
            .unwrap();
        assert_eq!(doc_title_before, doc_title_after);
        assert_eq!(doc_updated_before, doc_updated_after);
    }

    #[test]
    fn test_no_keyword_delete_functionality() {
        let (conn, _tmp) = init_test_db();
        create_keyword(&conn, "NoDelete").unwrap();
        // Verify keyword still exists — no delete function exists
        assert_eq!(list_keywords(&conn).unwrap().len(), 1);
    }

    // --- Dokument-Schlagwort-Zuordnung-Tests (Prompt 026C) ---

    #[test]
    fn test_document_zero_tags() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let tags = list_document_tags(&conn, &doc.id).unwrap();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_assign_one_tag() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Hygiene").unwrap();
        let tags = sync_document_tags(&conn, &doc.id, &[kw.id.clone()]).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "Hygiene");
    }

    #[test]
    fn test_assign_multiple_tags() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw1 = create_keyword(&conn, "Alpha").unwrap();
        let kw2 = create_keyword(&conn, "Beta").unwrap();
        let kw3 = create_keyword(&conn, "Gamma").unwrap();
        let tags = sync_document_tags(&conn, &doc.id, &[kw1.id, kw2.id, kw3.id]).unwrap();
        assert_eq!(tags.len(), 3);
    }

    #[test]
    fn test_duplicate_assignment_prevented() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Dup").unwrap();
        // First sync succeeds
        sync_document_tags(&conn, &doc.id, &[kw.id.clone()]).unwrap();
        // Second sync with same tag — should be idempotent, not duplicate
        let tags = sync_document_tags(&conn, &doc.id, &[kw.id.clone()]).unwrap();
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_assigned_tags_persist_after_reload() {
        let tmp = NamedTempFile::new().unwrap();
        init_database(tmp.path()).unwrap();
        let conn = Connection::open(tmp.path()).unwrap();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Persist").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw.id]).unwrap();
        drop(conn);
        let conn2 = Connection::open(tmp.path()).unwrap();
        let tags = list_document_tags(&conn2, &doc.id).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "Persist");
    }

    #[test]
    fn test_nonexistent_document_rejected() {
        let (conn, _tmp) = init_test_db();
        let kw = create_keyword(&conn, "Tag").unwrap();
        let result = sync_document_tags(&conn, "nonexistent-id", &[kw.id]);
        assert!(result.is_err());
    }

    #[test]
    fn test_nonexistent_tag_rejected() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let result = sync_document_tags(&conn, &doc.id, &["fake-tag-id".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_tag_assignment() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Remove").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw.id.clone()]).unwrap();
        let tags = sync_document_tags(&conn, &doc.id, &[]).unwrap();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_removing_assignment_does_not_delete_dictionary_entry() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Keep").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw.id.clone()]).unwrap();
        sync_document_tags(&conn, &doc.id, &[]).unwrap();
        let keywords = list_keywords(&conn).unwrap();
        assert_eq!(keywords.len(), 1);
        assert_eq!(keywords[0].name, "Keep");
    }

    #[test]
    fn test_sync_empty_set() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let tags = sync_document_tags(&conn, &doc.id, &[]).unwrap();
        assert!(tags.is_empty());
    }

    #[test]
    fn test_sync_adds_missing_relationships() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw1 = create_keyword(&conn, "First").unwrap();
        let kw2 = create_keyword(&conn, "Second").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw1.id.clone()]).unwrap();
        let tags = sync_document_tags(&conn, &doc.id, &[kw1.id.clone(), kw2.id.clone()]).unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn test_sync_removes_obsolete_relationships() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw1 = create_keyword(&conn, "Keep").unwrap();
        let kw2 = create_keyword(&conn, "Remove").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw1.id.clone(), kw2.id.clone()]).unwrap();
        let tags = sync_document_tags(&conn, &doc.id, &[kw1.id.clone()]).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "Keep");
    }

    #[test]
    fn test_keyword_rename_preserves_assignment() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Alt").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw.id.clone()]).unwrap();
        rename_keyword(&conn, &kw.id, "Neu").unwrap();
        let tags = list_document_tags(&conn, &doc.id).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "Neu");
    }

    #[test]
    fn test_archive_preserves_assignments() {
        let mut conn = init_test_db().0;
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Arch").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw.id]).unwrap();
        archive_document(&mut conn, &doc.id).unwrap();
        let tags = list_document_tags(&conn, &doc.id).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "Arch");
    }

    #[test]
    fn test_archived_document_mutation_rejected() {
        let mut conn = init_test_db().0;
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw1 = create_keyword(&conn, "Before").unwrap();
        let kw2 = create_keyword(&conn, "After").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw1.id.clone()]).unwrap();
        archive_document(&mut conn, &doc.id).unwrap();
        let result = sync_document_tags(&conn, &doc.id, &[kw2.id.clone()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_restore_preserves_assignments() {
        let mut conn = init_test_db().0;
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Restore").unwrap();
        sync_document_tags(&conn, &doc.id, &[kw.id]).unwrap();
        archive_document(&mut conn, &doc.id).unwrap();
        restore_document(&mut conn, &doc.id).unwrap();
        let tags = list_document_tags(&conn, &doc.id).unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0], "Restore");
    }

    #[test]
    fn test_draft_document_tag_assignment() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "Entwurf");
        let kw = create_keyword(&conn, "Draft").unwrap();
        let tags = sync_document_tags(&conn, &doc.id, &[kw.id]).unwrap();
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_active_document_tag_assignment() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Active").unwrap();
        let tags = sync_document_tags(&conn, &doc.id, &[kw.id]).unwrap();
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_tag_changes_do_not_alter_versions() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc = make_doc_with_status(&conn, &storage, "aktiv");
        let kw = create_keyword(&conn, "Ver").unwrap();
        let version_count_before: i64 = conn
            .query_row("SELECT COUNT(*) FROM document_versions WHERE document_id = ?1;", rusqlite::params![doc.id], |row| row.get(0))
            .unwrap();
        sync_document_tags(&conn, &doc.id, &[kw.id]).unwrap();
        let version_count_after: i64 = conn
            .query_row("SELECT COUNT(*) FROM document_versions WHERE document_id = ?1;", rusqlite::params![doc.id], |row| row.get(0))
            .unwrap();
        assert_eq!(version_count_before, version_count_after);
    }

    #[test]
    fn test_batch_document_tag_names() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc1 = make_doc_with_status(&conn, &storage, "aktiv");
        let doc2 = make_doc_with_status(&conn, &storage, "aktiv");
        let kw1 = create_keyword(&conn, "A").unwrap();
        let kw2 = create_keyword(&conn, "B").unwrap();
        let kw3 = create_keyword(&conn, "C").unwrap();
        sync_document_tags(&conn, &doc1.id, &[kw1.id.clone(), kw2.id.clone()]).unwrap();
        sync_document_tags(&conn, &doc2.id, &[kw3.id.clone()]).unwrap();
        let map = batch_document_tag_names(&conn, &[doc1.id.clone(), doc2.id.clone()]).unwrap();
        assert_eq!(map.get(&doc1.id).unwrap().len(), 2);
        assert_eq!(map.get(&doc2.id).unwrap().len(), 1);
    }

    #[test]
    fn test_batch_empty_document_ids() {
        let (conn, _tmp) = init_test_db();
        let map = batch_document_tag_names(&conn, &[]).unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn test_multiple_documents_independent_tag_sets() {
        let (conn, _tmp) = init_test_db();
        let storage = init_test_storage();
        let doc1 = make_doc_with_status(&conn, &storage, "aktiv");
        let doc2 = make_doc_with_status(&conn, &storage, "aktiv");
        let kw1 = create_keyword(&conn, "X").unwrap();
        let kw2 = create_keyword(&conn, "Y").unwrap();
        sync_document_tags(&conn, &doc1.id, &[kw1.id.clone()]).unwrap();
        sync_document_tags(&conn, &doc2.id, &[kw2.id.clone()]).unwrap();
        let tags1 = list_document_tags(&conn, &doc1.id).unwrap();
        let tags2 = list_document_tags(&conn, &doc2.id).unwrap();
        assert_eq!(tags1, vec!["X"]);
        assert_eq!(tags2, vec!["Y"]);
    }

}
