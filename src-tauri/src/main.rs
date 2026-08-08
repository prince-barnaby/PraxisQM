// PraxisQM – Tauri Haupteintrittspunkt
// Modul: Desktop Runtime
// Zweck: Startet die native Desktop-Anwendung, initialisiert die SQLite-Datenbank
//         und lädt die React-Oberfläche.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;

use database::{
    CategoryItem, CreateDocumentInput, CreateEmployeeInput, Document, Employee,
    MasterDataItem, SubcategoryItem, UpdateEmployeeInput,
};
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
            cmd_list_documents,
            cmd_get_document,
            cmd_get_document_by_number,
            cmd_create_document,
            cmd_list_categories,
            cmd_list_subcategories,
            cmd_select_pdf,
            cmd_open_pdf,
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

#[tauri::command]
fn cmd_list_documents(state: State<DbState>) -> Result<Vec<Document>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_documents(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_get_document(state: State<DbState>, id: String) -> Result<Document, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::load_document(&conn, &id).map_err(|e| {
        if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
            "Dokument nicht gefunden.".to_string()
        } else {
            e.to_string()
        }
    })
}

#[tauri::command]
fn cmd_get_document_by_number(state: State<DbState>, number: String) -> Result<Document, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::get_document_by_number(&conn, &number).map_err(|e| {
        if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
            "Dokument nicht gefunden.".to_string()
        } else {
            e.to_string()
        }
    })
}

#[tauri::command]
fn cmd_create_document(
    state: State<DbState>,
    app: tauri::AppHandle,
    input: CreateDocumentInput,
) -> Result<Document, String> {
    let source = std::path::Path::new(&input.source_file_path);
    database::validate_pdf(source)?;

    let storage_dir = database::document_storage_path(&app);
    let doc_id = uuid::Uuid::new_v4().to_string();

    let managed_file = database::copy_to_managed_storage(source, &storage_dir, &doc_id)?;

    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::create_document(&conn, &input, &managed_file) {
        Ok(doc) => Ok(doc),
        Err(e) => {
            database::remove_managed_file(&storage_dir, &managed_file);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn cmd_list_categories(state: State<DbState>) -> Result<Vec<CategoryItem>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_categories(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_list_subcategories(state: State<DbState>) -> Result<Vec<SubcategoryItem>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_subcategories(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_select_pdf(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri::api::dialog::FileDialogBuilder;

    let (tx, rx) = std::sync::mpsc::channel::<Option<std::path::PathBuf>>();
    FileDialogBuilder::new(&app)
        .add_filter("PDF-Dateien", &["pdf"])
        .pick_file(move |path| {
            let _ = tx.send(path);
        });

    match rx.recv() {
        Ok(Some(path)) => Ok(Some(path.to_string_lossy().to_string())),
        Ok(None) => Ok(None),
        Err(_) => Err("Dateiauswahl fehlgeschlagen.".to_string()),
    }
}

#[tauri::command]
fn cmd_open_pdf(
    state: State<DbState>,
    app: tauri::AppHandle,
    document_id: String,
) -> Result<(), String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    let doc = database::load_document(&conn, &document_id).map_err(|e| {
        if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
            "Dokument nicht gefunden.".to_string()
        } else {
            e.to_string()
        }
    })?;

    let file_path = doc.file_path.ok_or_else(|| "Keine Datei hinterlegt.".to_string())?;
    let storage_dir = database::document_storage_path(&app);
    let full_path = storage_dir.join(&file_path);

    if !full_path.exists() {
        return Err("Die Datei wurde nicht im Dokumentenspeicher gefunden.".to_string());
    }

    tauri::api::shell::open(&app, full_path.to_string_lossy().to_string(), None)
        .map_err(|_| "PDF konnte nicht geöffnet werden.".to_string())
}
