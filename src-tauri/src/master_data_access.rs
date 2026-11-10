// PraxisQM – Zugriffsschicht für Stammdaten (Verantwortungspositionen, QM-Bereiche)
// Modul: Stammdaten
// Zweck: Prüft die Rolle vor jedem Lesen oder Verändern dieser Stammdaten.

use crate::auth::{require_privileged, AuthState, ROLE_ADMIN, ROLE_EDITOR};
use crate::database::{self, MasterDataItem};
use rusqlite::Connection;

fn validated_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Bezeichnung darf nicht leer sein.".to_string());
    }
    Ok(trimmed.to_string())
}

fn map_rename_error(e: rusqlite::Error, not_found: &str) -> String {
    match e {
        rusqlite::Error::QueryReturnedNoRows => not_found.to_string(),
        other => other.to_string(),
    }
}

pub fn list_responsibilities(conn: &Connection, auth: &AuthState) -> Result<Vec<MasterDataItem>, String> {
    require_privileged(conn, auth, ROLE_EDITOR)?;
    database::list_responsibilities(conn).map_err(|e| e.to_string())
}

pub fn list_qm_areas(conn: &Connection, auth: &AuthState) -> Result<Vec<MasterDataItem>, String> {
    require_privileged(conn, auth, ROLE_EDITOR)?;
    database::list_qm_areas(conn).map_err(|e| e.to_string())
}

pub fn create_responsibility(conn: &Connection, auth: &AuthState, name: &str) -> Result<MasterDataItem, String> {
    require_privileged(conn, auth, ROLE_ADMIN)?;
    let name = validated_name(name)?;
    database::create_responsibility(conn, &name).map_err(|e| e.to_string())
}

pub fn rename_responsibility(
    conn: &Connection,
    auth: &AuthState,
    id: &str,
    new_name: &str,
) -> Result<MasterDataItem, String> {
    require_privileged(conn, auth, ROLE_ADMIN)?;
    let new_name = validated_name(new_name)?;
    database::rename_responsibility(conn, id, &new_name)
        .map_err(|e| map_rename_error(e, "Verantwortungsposition nicht gefunden."))
}

pub fn create_qm_area(conn: &Connection, auth: &AuthState, name: &str) -> Result<MasterDataItem, String> {
    require_privileged(conn, auth, ROLE_ADMIN)?;
    let name = validated_name(name)?;
    database::create_qm_area(conn, &name).map_err(|e| e.to_string())
}

pub fn rename_qm_area(
    conn: &Connection,
    auth: &AuthState,
    id: &str,
    new_name: &str,
) -> Result<MasterDataItem, String> {
    require_privileged(conn, auth, ROLE_ADMIN)?;
    let new_name = validated_name(new_name)?;
    database::rename_qm_area(conn, id, &new_name)
        .map_err(|e| map_rename_error(e, "QM-Bereich nicht gefunden."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{ROLE_ADMIN, ROLE_EDITOR, ROLE_GUEST};
    use crate::test_support::{init_test_db, insert_test_user, set_test_session};
    use rusqlite::Result as SqliteResult;

    struct Entity {
        table: &'static str,
        not_found: &'static str,
        seed: fn(&Connection, &str) -> SqliteResult<MasterDataItem>,
        list: fn(&Connection, &AuthState) -> Result<Vec<MasterDataItem>, String>,
        create: fn(&Connection, &AuthState, &str) -> Result<MasterDataItem, String>,
        rename: fn(&Connection, &AuthState, &str, &str) -> Result<MasterDataItem, String>,
    }

    fn entities() -> [Entity; 2] {
        [
            Entity {
                table: "verantwortungspositionen",
                not_found: "Verantwortungsposition nicht gefunden.",
                seed: database::create_responsibility,
                list: list_responsibilities,
                create: create_responsibility,
                rename: rename_responsibility,
            },
            Entity {
                table: "qm_bereiche",
                not_found: "QM-Bereich nicht gefunden.",
                seed: database::create_qm_area,
                list: list_qm_areas,
                create: create_qm_area,
                rename: rename_qm_area,
            },
        ]
    }

    fn row_count(conn: &Connection, table: &str) -> i64 {
        conn.query_row(&format!("SELECT COUNT(*) FROM {table};"), [], |r| r.get(0))
            .unwrap()
    }

    fn name_of(conn: &Connection, table: &str, id: &str) -> String {
        conn.query_row(&format!("SELECT name FROM {table} WHERE id = ?1;"), [id], |r| r.get(0))
            .unwrap()
    }

    fn setup_with_role(entity: &Entity, role: &str) -> (Connection, AuthState, String, tempfile::NamedTempFile) {
        let (conn, tmp) = init_test_db();
        let auth = AuthState::new();
        insert_test_user(&conn, "u1", role, true);
        set_test_session(&auth, "u1", role);
        let id = (entity.seed)(&conn, "Original").unwrap().id;
        (conn, auth, id, tmp)
    }

    fn assert_no_write_happened(conn: &Connection, entity: &Entity, id: &str) {
        assert_eq!(row_count(conn, entity.table), 1);
        assert_eq!(name_of(conn, entity.table, id), "Original");
    }

    #[test]
    fn test_guest_cannot_list_or_write() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_GUEST);
            assert!((entity.list)(&conn, &auth).is_err());
            assert!((entity.create)(&conn, &auth, "Neu").is_err());
            assert!((entity.rename)(&conn, &auth, &id, "Geändert").is_err());
            assert_no_write_happened(&conn, &entity, &id);
        }
    }

    #[test]
    fn test_anonymous_cannot_list_or_write() {
        for entity in entities() {
            let (conn, _tmp) = init_test_db();
            let auth = AuthState::new();
            let id = (entity.seed)(&conn, "Original").unwrap().id;
            assert!((entity.list)(&conn, &auth).is_err());
            assert!((entity.create)(&conn, &auth, "Neu").is_err());
            assert!((entity.rename)(&conn, &auth, &id, "Geändert").is_err());
            assert_no_write_happened(&conn, &entity, &id);
        }
    }

    #[test]
    fn test_editor_can_list_but_not_write() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_EDITOR);
            assert_eq!((entity.list)(&conn, &auth).unwrap().len(), 1);
            assert!((entity.create)(&conn, &auth, "Neu").is_err());
            assert!((entity.rename)(&conn, &auth, &id, "Geändert").is_err());
            assert_no_write_happened(&conn, &entity, &id);
        }
    }

    #[test]
    fn test_admin_can_list_create_and_rename() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_ADMIN);
            assert_eq!((entity.list)(&conn, &auth).unwrap().len(), 1);
            (entity.create)(&conn, &auth, "Neu").unwrap();
            assert_eq!(row_count(&conn, entity.table), 2);
            (entity.rename)(&conn, &auth, &id, "Geändert").unwrap();
            assert_eq!(name_of(&conn, entity.table, &id), "Geändert");
        }
    }

    #[test]
    fn test_rename_success_persists_new_name_and_returns_it() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_ADMIN);
            let renamed = (entity.rename)(&conn, &auth, &id, "  Geändert  ").unwrap();
            assert_eq!(renamed.id, id);
            assert_eq!(renamed.name, "Geändert");
            assert_eq!(name_of(&conn, entity.table, &id), "Geändert");
        }
    }

    #[test]
    fn test_rename_unknown_id_returns_error_and_changes_nothing() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_ADMIN);
            let err = (entity.rename)(&conn, &auth, "unbekannt", "Geändert").unwrap_err();
            assert_eq!(err, entity.not_found);
            assert_no_write_happened(&conn, &entity, &id);
        }
    }

    #[test]
    fn test_admin_name_is_trimmed_and_empty_name_rejected() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_ADMIN);
            let created = (entity.create)(&conn, &auth, "  Neu  ").unwrap();
            assert_eq!(created.name, "Neu");
            let err = (entity.create)(&conn, &auth, "   ").unwrap_err();
            assert_eq!(err, "Bezeichnung darf nicht leer sein.");
            let err = (entity.rename)(&conn, &auth, &id, "").unwrap_err();
            assert_eq!(err, "Bezeichnung darf nicht leer sein.");
            assert_eq!(row_count(&conn, entity.table), 2);
            assert_eq!(name_of(&conn, entity.table, &id), "Original");
        }
    }

    #[test]
    fn test_deactivated_admin_cannot_list_or_write() {
        for entity in entities() {
            let (conn, _tmp) = init_test_db();
            let auth = AuthState::new();
            insert_test_user(&conn, "a1", ROLE_ADMIN, false);
            set_test_session(&auth, "a1", ROLE_ADMIN);
            let id = (entity.seed)(&conn, "Original").unwrap().id;
            assert!((entity.list)(&conn, &auth).is_err());
            assert!((entity.create)(&conn, &auth, "Neu").is_err());
            assert!((entity.rename)(&conn, &auth, &id, "Geändert").is_err());
            assert_no_write_happened(&conn, &entity, &id);
        }
    }

    #[test]
    fn test_demoted_admin_loses_write_access_immediately() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_ADMIN);
            conn.execute("UPDATE users SET role = ?1 WHERE id = ?2;", rusqlite::params![ROLE_EDITOR, "u1"])
                .unwrap();
            assert_eq!((entity.list)(&conn, &auth).unwrap().len(), 1);
            assert!((entity.create)(&conn, &auth, "Neu").is_err());
            assert!((entity.rename)(&conn, &auth, &id, "Geändert").is_err());
            assert_no_write_happened(&conn, &entity, &id);
        }
    }

    #[test]
    fn test_deleted_user_cannot_list_or_write() {
        for entity in entities() {
            let (conn, auth, id, _tmp) = setup_with_role(&entity, ROLE_ADMIN);
            conn.execute("DELETE FROM users WHERE id = ?1;", ["u1"]).unwrap();
            assert!((entity.list)(&conn, &auth).is_err());
            assert!((entity.create)(&conn, &auth, "Neu").is_err());
            assert!((entity.rename)(&conn, &auth, &id, "Geändert").is_err());
            assert_no_write_happened(&conn, &entity, &id);
        }
    }
}
