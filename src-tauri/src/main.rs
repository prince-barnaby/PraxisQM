// PraxisQM – Tauri Haupteintrittspunkt
// Modul: Desktop Runtime
// Zweck: Startet die native Desktop-Anwendung, initialisiert die SQLite-Datenbank
//         und lädt die React-Oberfläche.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;

use database::{CreateEmployeeInput, Employee, MasterDataItem};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

struct DbState(Mutex<Connection>);

fn get_conn(state: &State<DbState>) -> std::sync::MutexGuard<'_, Connection> {
    state.0.lock().expect("Datenbank-Verbindung gesperrt")
}

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
            database::seed_master_data_if_empty(&conn)
                .expect("Stammdaten-Initialisierung fehlgeschlagen");
            app.manage(DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd_list_employees,
            cmd_create_employee,
            cmd_list_responsibilities,
            cmd_list_qm_areas,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten von PraxisQM");
}

#[tauri::command]
fn cmd_list_employees(state: State<DbState>) -> Result<Vec<Employee>, String> {
    let conn = get_conn(&state);
    database::list_employees(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_create_employee(
    input: CreateEmployeeInput,
    state: State<DbState>,
) -> Result<Employee, String> {
    let conn = get_conn(&state);
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
fn cmd_list_responsibilities(state: State<DbState>) -> Result<Vec<MasterDataItem>, String> {
    let conn = get_conn(&state);
    database::list_responsibilities(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_list_qm_areas(state: State<DbState>) -> Result<Vec<MasterDataItem>, String> {
    let conn = get_conn(&state);
    database::list_qm_areas(&conn).map_err(|e| e.to_string())
}
