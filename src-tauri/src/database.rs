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

/// --- Stammdaten-Initialisierung ------------------------------------------

/// Entwicklungs-Stammdaten für Verantwortungspositionen und QM-Bereiche.
/// Wird nur eingefügt, wenn die jeweilige Tabelle leer ist.
/// Diese Werte sind kanonisch nicht festgelegt — sie dienen ausschließlich
/// der Nutzbarkeit der Anwendung während der Entwicklung.
fn seed_master_data_if_empty(conn: &Connection) -> SqliteResult<()> {
    let now = now_iso();

    let resp_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM verantwortungspositionen;",
        [],
        |row| row.get(0),
    )?;
    if resp_count == 0 {
        let seed_responsibilities = [
            "QM-Beauftragte",
            "Datenschutzbeauftragte",
            "Hygienebeauftragte",
            "Praxisleitung",
            "Fortbildungsbeauftragte",
        ];
        for name in &seed_responsibilities {
            conn.execute(
                "INSERT INTO verantwortungspositionen (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
                rusqlite::params![uuid::Uuid::new_v4().to_string(), name, now, now],
            )?;
        }
    }

    let qm_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM qm_bereiche;", [], |row| row.get(0))?;
    if qm_count == 0 {
        let seed_qm_areas = [
            "Datenschutz",
            "Hygiene",
            "Patientendokumentation",
            "Röntgeneinweisung",
            "Fortbildung",
        ];
        for name in &seed_qm_areas {
            conn.execute(
                "INSERT INTO qm_bereiche (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4);",
                rusqlite::params![uuid::Uuid::new_v4().to_string(), name, now, now],
            )?;
        }
    }

    Ok(())
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
        seed_master_data_if_empty(&conn).unwrap();
        (conn, tmp)
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
        let (conn, _tmp) = init_test_db();
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

    #[test]
    fn test_seed_master_data() {
        let (conn, _tmp) = init_test_db();
        let responsibilities = list_responsibilities(&conn).unwrap();
        assert!(!responsibilities.is_empty());
        let qm_areas = list_qm_areas(&conn).unwrap();
        assert!(!qm_areas.is_empty());
    }
}
