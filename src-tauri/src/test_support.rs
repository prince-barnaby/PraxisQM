// PraxisQM – Gemeinsame Testhilfen
// Modul: Tests
// Zweck: Stellt Testdatenbank, Testbenutzer und Testsitzungen für alle Modul-Tests bereit.

use crate::auth::{AuthState, Session};
use crate::database;
use rusqlite::Connection;
use tempfile::NamedTempFile;

pub fn init_test_db() -> (Connection, NamedTempFile) {
    let tmp = NamedTempFile::new().unwrap();
    database::init_database(tmp.path()).unwrap();
    let conn = Connection::open(tmp.path()).unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", []).unwrap();
    (conn, tmp)
}

pub fn insert_test_user(conn: &Connection, id: &str, role: &str, is_active: bool) {
    conn.execute(
        "INSERT INTO users (id, username, role, employee_id, is_active, password_hash, created_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, ?4, NULL, '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z');",
        rusqlite::params![id, format!("user-{}", id), role, is_active],
    )
    .unwrap();
}

pub fn set_test_session(auth: &AuthState, id: &str, role: &str) {
    auth.set_session(Session {
        user_id: id.to_string(),
        username: format!("user-{}", id),
        role: role.to_string(),
        login_at: 0,
    });
}
