// PraxisQM – Tauri Haupteintrittspunkt
// Modul: Desktop Runtime
// Zweck: Startet die native Desktop-Anwendung, initialisiert die SQLite-Datenbank
//         und lädt die React-Oberfläche.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;

use database::{CreateEmployeeInput, Employee, MasterDataItem, UpdateEmployeeInput};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{Manager, State};

struct DbState(Mutex<Connection>);

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db_path = database::database_path(&app.handle());
            match database::init_database(&db_path) {
                Ok(()) => {
                    println!("PraxisQM: SQLite-Datenbank initialisiert: {}", db_path.display());
                }
                Err(e) => {
                    eprintln!("PraxisQM: Fehler bei der Datenbankinitialisierung: {}", e);
                    std::process::exit(1);
                }
            }
            let conn = Connection::open(&db_path).expect("Datenbankverbindung fehlgeschlagen");
            conn.execute("PRAGMA foreign_keys = ON;", [])
                .expect("Foreign-Key-Enforcement fehlgeschlagen");
            app.manage(DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd_list_employees,
            cmd_create_employee,
            cmd_list_responsibilities,
            cmd_list_qm_areas,
            cmd_create_responsibility,
            cmd_rename_responsibility,
            cmd_create_qm_area,
            cmd_rename_qm_area,
            cmd_get_employee,
            cmd_update_employee,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten von PraxisQM");
}

#[tauri::command]
fn cmd_list_employees(state: State<DbState>) -> Result<Vec<Employee>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_employees(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_create_employee(
    input: CreateEmployeeInput,
    state: State<DbState>,
) -> Result<Employee, String> {
    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    match database::create_employee(&tx, &input) {
        Ok(emp) => {
            tx.commit().map_err(|e| e.to_string())?;
            Ok(emp)
        }
        Err(e) => {
            let _ = tx.rollback();
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn cmd_get_employee(
    state: State<DbState>,
    id: String,
) -> Result<Employee, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::get_employee(&conn, &id) {
        Ok(emp) => Ok(emp),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Mitarbeiter nicht gefunden.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn cmd_update_employee(
    state: State<DbState>,
    id: String,
    input: UpdateEmployeeInput,
) -> Result<Employee, String> {
    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    match database::update_employee(&tx, &id, &input) {
        Ok(emp) => {
            tx.commit().map_err(|e| e.to_string())?;
            Ok(emp)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let _ = tx.rollback();
            Err("Mitarbeiter nicht gefunden.".to_string())
        }
        Err(e) => {
            let _ = tx.rollback();
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn cmd_list_responsibilities(state: State<DbState>) -> Result<Vec<MasterDataItem>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_responsibilities(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_list_qm_areas(state: State<DbState>) -> Result<Vec<MasterDataItem>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_qm_areas(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_create_responsibility(
    state: State<DbState>,
    name: String,
) -> Result<MasterDataItem, String> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Bezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::create_responsibility(&conn, &trimmed).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_rename_responsibility(
    state: State<DbState>,
    id: String,
    new_name: String,
) -> Result<MasterDataItem, String> {
    let trimmed = new_name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Bezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::rename_responsibility(&conn, &id, &trimmed).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_create_qm_area(
    state: State<DbState>,
    name: String,
) -> Result<MasterDataItem, String> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Bezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::create_qm_area(&conn, &trimmed).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_rename_qm_area(
    state: State<DbState>,
    id: String,
    new_name: String,
) -> Result<MasterDataItem, String> {
    let trimmed = new_name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Bezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::rename_qm_area(&conn, &id, &trimmed).map_err(|e| e.to_string())
}
