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
const SCHEMA_VERSION: i64 = 1;

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
    pub description: Option<String>,
    pub archived_at: Option<String>,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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
                d.created_at, d.updated_at
         FROM documents d
         LEFT JOIN categories c ON c.id = d.category_id
         LEFT JOIN subcategories s ON s.id = d.subcategory_id
         LEFT JOIN employees e ON e.id = d.responsible_person_id
         LEFT JOIN (
             SELECT document_id, file_name, file_path
             FROM document_versions
             WHERE id IN (SELECT MIN(id) FROM document_versions GROUP BY document_id)
         ) dv ON dv.document_id = d.id
         ORDER BY d.document_number;",
    )?;

    let docs = stmt
        .query_map([], |row| {
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
                valid_until: row.get(12)?,
                description: row.get(13)?,
                archived_at: row.get(14)?,
                file_name: row.get(15)?,
                file_path: row.get(16)?,
                created_at: row.get(17)?,
                updated_at: row.get(18)?,
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
                d.created_at, d.updated_at
         FROM documents d
         LEFT JOIN categories c ON c.id = d.category_id
         LEFT JOIN subcategories s ON s.id = d.subcategory_id
         LEFT JOIN employees e ON e.id = d.responsible_person_id
         LEFT JOIN (
             SELECT document_id, file_name, file_path
             FROM document_versions
             WHERE id IN (SELECT MIN(id) FROM document_versions GROUP BY document_id)
         ) dv ON dv.document_id = d.id
         {}",
        where_clause
    );

    conn.query_row(&sql, params, |row| {
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
            valid_until: row.get(12)?,
            description: row.get(13)?,
            archived_at: row.get(14)?,
            file_name: row.get(15)?,
            file_path: row.get(16)?,
            created_at: row.get(17)?,
            updated_at: row.get(18)?,
        })
    })
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

}
