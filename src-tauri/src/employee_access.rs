// PraxisQM – Zugriffsschicht für Mitarbeiterdaten
// Modul: Mitarbeiterverwaltung
// Zweck: Prüft die Rolle vor jedem Lesen oder Verändern von Mitarbeiterdaten.

use crate::auth::{require_privileged, AuthState, ROLE_ADMIN, ROLE_EDITOR};
use crate::database::{self, CreateEmployeeInput, Employee, UpdateEmployeeInput};
use rusqlite::Connection;

fn map_missing(e: rusqlite::Error) -> String {
    match e {
        rusqlite::Error::QueryReturnedNoRows => "Mitarbeiter nicht gefunden.".to_string(),
        other => other.to_string(),
    }
}

pub fn list_employees(conn: &Connection, auth: &AuthState) -> Result<Vec<Employee>, String> {
    require_privileged(conn, auth, ROLE_EDITOR)?;
    database::list_employees(conn).map_err(|e| e.to_string())
}

pub fn get_employee(conn: &Connection, auth: &AuthState, id: &str) -> Result<Employee, String> {
    require_privileged(conn, auth, ROLE_EDITOR)?;
    database::get_employee(conn, id).map_err(map_missing)
}

pub fn create_employee(
    conn: &mut Connection,
    auth: &AuthState,
    input: &CreateEmployeeInput,
) -> Result<Employee, String> {
    require_privileged(conn, auth, ROLE_ADMIN)?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let employee = database::create_employee(&tx, input).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok(employee)
}

pub fn update_employee(
    conn: &mut Connection,
    auth: &AuthState,
    id: &str,
    input: &UpdateEmployeeInput,
) -> Result<Employee, String> {
    require_privileged(conn, auth, ROLE_ADMIN)?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let employee = database::update_employee(&tx, id, input).map_err(map_missing)?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok(employee)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{ROLE_ADMIN, ROLE_EDITOR, ROLE_GUEST};
    use crate::test_support::{init_test_db, insert_test_user, set_test_session};

    fn create_input() -> CreateEmployeeInput {
        CreateEmployeeInput {
            last_name: "Muster".to_string(),
            first_name: "Anna".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![],
        }
    }

    fn update_input() -> UpdateEmployeeInput {
        UpdateEmployeeInput {
            last_name: "Geändert".to_string(),
            first_name: "Anna".to_string(),
            position: None,
            is_active: true,
            hire_date: None,
            departure_date: None,
            responsibility_ids: vec![],
            qm_area_ids: vec![],
        }
    }

    fn seed_employee(conn: &Connection) -> String {
        database::create_employee(conn, &create_input()).unwrap().id
    }

    fn employee_count(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM employees;", [], |r| r.get(0)).unwrap()
    }

    fn last_name(conn: &Connection, id: &str) -> String {
        conn.query_row("SELECT last_name FROM employees WHERE id = ?1;", [id], |r| r.get(0))
            .unwrap()
    }

    fn setup_with_role(role: &str) -> (Connection, AuthState, String, tempfile::NamedTempFile) {
        let (conn, tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "u1", role, true);
        set_test_session(&auth, "u1", role);
        let emp_id = seed_employee(&conn);
        (conn, auth, emp_id, tmp)
    }

    #[test]
    fn test_guest_cannot_list_or_read_employees() {
        let (conn, auth, emp_id, _tmp) = setup_with_role(ROLE_GUEST);
        assert!(list_employees(&conn, &auth).is_err());
        assert!(get_employee(&conn, &auth, &emp_id).is_err());
    }

    #[test]
    fn test_anonymous_cannot_list_or_read_employees() {
        let (conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        let emp_id = seed_employee(&conn);
        assert!(list_employees(&conn, &auth).is_err());
        assert!(get_employee(&conn, &auth, &emp_id).is_err());
    }

    #[test]
    fn test_guest_cannot_create_or_update_employees() {
        let (mut conn, auth, emp_id, _tmp) = setup_with_role(ROLE_GUEST);
        assert!(create_employee(&mut conn, &auth, &create_input()).is_err());
        assert!(update_employee(&mut conn, &auth, &emp_id, &update_input()).is_err());
        assert_eq!(employee_count(&conn), 1);
        assert_eq!(last_name(&conn, &emp_id), "Muster");
    }

    #[test]
    fn test_editor_can_list_and_read_employees() {
        let (conn, auth, emp_id, _tmp) = setup_with_role(ROLE_EDITOR);
        assert_eq!(list_employees(&conn, &auth).unwrap().len(), 1);
        assert_eq!(get_employee(&conn, &auth, &emp_id).unwrap().id, emp_id);
    }

    #[test]
    fn test_editor_cannot_create_or_update_employees() {
        let (mut conn, auth, emp_id, _tmp) = setup_with_role(ROLE_EDITOR);
        assert!(create_employee(&mut conn, &auth, &create_input()).is_err());
        assert!(update_employee(&mut conn, &auth, &emp_id, &update_input()).is_err());
        assert_eq!(employee_count(&conn), 1);
        assert_eq!(last_name(&conn, &emp_id), "Muster");
    }

    #[test]
    fn test_admin_can_use_all_four_functions() {
        let (mut conn, auth, emp_id, _tmp) = setup_with_role(ROLE_ADMIN);
        assert_eq!(list_employees(&conn, &auth).unwrap().len(), 1);
        assert_eq!(get_employee(&conn, &auth, &emp_id).unwrap().id, emp_id);
        create_employee(&mut conn, &auth, &create_input()).unwrap();
        assert_eq!(employee_count(&conn), 2);
        update_employee(&mut conn, &auth, &emp_id, &update_input()).unwrap();
        assert_eq!(last_name(&conn, &emp_id), "Geändert");
    }

    #[test]
    fn test_missing_employee_returns_not_found() {
        let (conn, auth, _emp_id, _tmp) = setup_with_role(ROLE_EDITOR);
        let result = get_employee(&conn, &auth, "unbekannt");
        assert_eq!(result.unwrap_err(), "Mitarbeiter nicht gefunden.");
    }

    #[test]
    fn test_update_missing_employee_returns_not_found() {
        let (mut conn, auth, _emp_id, _tmp) = setup_with_role(ROLE_ADMIN);
        let result = update_employee(&mut conn, &auth, "unbekannt", &update_input());
        assert_eq!(result.unwrap_err(), "Mitarbeiter nicht gefunden.");
    }

    #[test]
    fn test_deactivated_admin_cannot_use_any_function() {
        let (mut conn, _tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "a1", ROLE_ADMIN, false);
        set_test_session(&auth, "a1", ROLE_ADMIN);
        let emp_id = seed_employee(&conn);
        assert!(list_employees(&conn, &auth).is_err());
        assert!(get_employee(&conn, &auth, &emp_id).is_err());
        assert!(create_employee(&mut conn, &auth, &create_input()).is_err());
        assert!(update_employee(&mut conn, &auth, &emp_id, &update_input()).is_err());
        assert_eq!(employee_count(&conn), 1);
        assert_eq!(last_name(&conn, &emp_id), "Muster");
    }

    #[test]
    fn test_demoted_admin_loses_admin_functions_immediately() {
        let (mut conn, auth, emp_id, _tmp) = setup_with_role(ROLE_ADMIN);
        conn.execute("UPDATE users SET role = ?1 WHERE id = ?2;", rusqlite::params![ROLE_EDITOR, "u1"])
            .unwrap();
        assert!(create_employee(&mut conn, &auth, &create_input()).is_err());
        assert!(update_employee(&mut conn, &auth, &emp_id, &update_input()).is_err());
        assert_eq!(employee_count(&conn), 1);
        assert_eq!(list_employees(&conn, &auth).unwrap().len(), 1);
    }
}
