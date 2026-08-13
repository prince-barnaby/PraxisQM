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
    CategoryItem, CreateDocumentInput, CreateEmployeeInput, CreateVersionInput, DashboardSummary,
    Document, DocumentVersion, Employee, MasterDataItem, ReviewEntry, SubcategoryItem,
    UpdateDocumentInput, UpdateEmployeeInput,
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
            cmd_update_document,
            cmd_create_version,
            cmd_list_versions,
            cmd_archive_document,
            cmd_restore_document,
            cmd_list_archived_documents,
            cmd_list_categories,
            cmd_list_subcategories,
            cmd_create_category,
            cmd_rename_category,
            cmd_create_subcategory,
            cmd_rename_subcategory,
            cmd_list_keywords,
            cmd_create_keyword,
            cmd_rename_keyword,
            cmd_list_document_tags,
            cmd_sync_document_tags,
            cmd_batch_document_tags,
            cmd_select_pdf,
            cmd_dashboard_summary,
            cmd_review_list,
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

    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let doc = match database::create_document(&tx, &input, &managed_file) {
        Ok(doc) => doc,
        Err(e) => {
            let _ = tx.rollback();
            database::remove_managed_file(&storage_dir, &managed_file);
            return Err(e.to_string());
        }
    };
    if !input.tag_ids.is_empty() {
        if let Err(e) = database::sync_document_tags(&tx, &doc.id, &input.tag_ids) {
            let _ = tx.rollback();
            database::remove_managed_file(&storage_dir, &managed_file);
            return Err(e.to_string());
        }
    }
    match tx.commit() {
        Ok(()) => Ok(doc),
        Err(e) => {
            database::remove_managed_file(&storage_dir, &managed_file);
            Err(e.to_string())
        }
    }
}

/// Aktualisiert die Metadaten eines bestehenden Dokuments (nur DB-001).
/// Keine PDF-Ersetzung — dafür gibt cmd_create_version.
#[tauri::command]
fn cmd_update_document(
    state: State<DbState>,
    id: String,
    input: UpdateDocumentInput,
) -> Result<Document, String> {
    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    let doc = match database::update_document(&tx, &id, &input) {
        Ok(doc) => doc,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let _ = tx.rollback();
            return Err("Dokument nicht gefunden.".to_string());
        }
        Err(e) => {
            let _ = tx.rollback();
            return Err(e.to_string());
        }
    };
    if let Err(e) = database::sync_document_tags(&tx, &id, &input.tag_ids) {
        let _ = tx.rollback();
        return Err(e.to_string());
    }
    match tx.commit() {
        Ok(()) => Ok(doc),
        Err(e) => Err(e.to_string()),
    }
}

/// Erstellt eine neue Dokumentversion mit neuer PDF.
/// Die vorherige Version und deren PDF bleiben erhalten.
#[tauri::command]
fn cmd_create_version(
    state: State<DbState>,
    app: tauri::AppHandle,
    input: CreateVersionInput,
) -> Result<Document, String> {
    let storage_dir = database::document_storage_path(&app);
    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::create_version_from_source(&mut conn, &input, &storage_dir)
}

/// Lädt alle Versionen eines Dokuments.
#[tauri::command]
fn cmd_list_versions(state: State<DbState>, document_id: String) -> Result<Vec<DocumentVersion>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_versions(&conn, &document_id).map_err(|e| e.to_string())
}

/// Archiviert ein Dokument (Lifecycle, keine Löschung).
#[tauri::command]
fn cmd_archive_document(state: State<DbState>, id: String) -> Result<Document, String> {
    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::archive_document(&mut conn, &id) {
        Ok(doc) => Ok(doc),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Dokument nicht gefunden.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Stellt ein archiviertes Dokument wieder her.
#[tauri::command]
fn cmd_restore_document(state: State<DbState>, id: String) -> Result<Document, String> {
    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::restore_document(&mut conn, &id) {
        Ok(doc) => Ok(doc),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Dokument nicht gefunden.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Lädt alle archivierten Dokumente.
#[tauri::command]
fn cmd_list_archived_documents(state: State<DbState>) -> Result<Vec<Document>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_archived_documents(&conn).map_err(|e| e.to_string())
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
fn cmd_create_category(
    state: State<DbState>,
    name: String,
) -> Result<CategoryItem, String> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Kategoriebezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::create_category(&conn, &trimmed).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_rename_category(
    state: State<DbState>,
    id: String,
    new_name: String,
) -> Result<CategoryItem, String> {
    let trimmed = new_name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Kategoriebezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::rename_category(&conn, &id, &trimmed) {
        Ok(item) => Ok(item),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Kategorie nicht gefunden.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn cmd_create_subcategory(
    state: State<DbState>,
    name: String,
    category_id: String,
) -> Result<SubcategoryItem, String> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Unterkategoriebezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::create_subcategory(&conn, &trimmed, &category_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_rename_subcategory(
    state: State<DbState>,
    id: String,
    new_name: String,
) -> Result<SubcategoryItem, String> {
    let trimmed = new_name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Unterkategoriebezeichnung darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::rename_subcategory(&conn, &id, &trimmed) {
        Ok(item) => Ok(item),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Unterkategorie nicht gefunden.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn cmd_list_keywords(state: State<DbState>) -> Result<Vec<MasterDataItem>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::list_keywords(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_create_keyword(
    state: State<DbState>,
    name: String,
) -> Result<MasterDataItem, String> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Schlagwort darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::create_keyword(&conn, &trimmed).map_err(|e| e.to_string())
}

#[tauri::command]
fn cmd_rename_keyword(
    state: State<DbState>,
    id: String,
    new_name: String,
) -> Result<MasterDataItem, String> {
    let trimmed = new_name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Schlagwort darf nicht leer sein.".to_string());
    }
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::rename_keyword(&conn, &id, &trimmed) {
        Ok(item) => Ok(item),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Schlagwort nicht gefunden.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn cmd_list_document_tags(
    state: State<DbState>,
    document_id: String,
) -> Result<Vec<String>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    match database::list_document_tags(&conn, &document_id) {
        Ok(tags) => Ok(tags),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Dokument nicht gefunden.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn cmd_sync_document_tags(
    state: State<DbState>,
    document_id: String,
    tag_ids: Vec<String>,
) -> Result<Vec<String>, String> {
    let mut conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    match database::sync_document_tags(&tx, &document_id, &tag_ids) {
        Ok(tags) => {
            tx.commit().map_err(|e| e.to_string())?;
            Ok(tags)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let _ = tx.rollback();
            Err("Dokument nicht gefunden.".to_string())
        }
        Err(rusqlite::Error::SqliteFailure(_, msg)) => {
            let _ = tx.rollback();
            Err(msg.unwrap_or_else(|| "Schlagwort-Zuordnung fehlgeschlagen.".to_string()))
        }
        Err(e) => {
            let _ = tx.rollback();
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn cmd_batch_document_tags(
    state: State<DbState>,
    document_ids: Vec<String>,
) -> Result<std::collections::HashMap<String, Vec<String>>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::batch_document_tag_names(&conn, &document_ids).map_err(|e| e.to_string())
}

/// Lädt die Dashboard-Zusammenfassung (Zähler für aktive/archivierte Dokumente, Mitarbeiter).
#[tauri::command]
fn cmd_dashboard_summary(state: State<DbState>) -> Result<DashboardSummary, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::dashboard_summary(&conn).map_err(|e| e.to_string())
}

/// Lädt die Review-Liste (aktive Dokumente, die Aufmerksamkeit erfordern).
#[tauri::command]
fn cmd_review_list(state: State<DbState>) -> Result<Vec<ReviewEntry>, String> {
    let conn = state.0.lock().expect("Datenbank-Verbindung gesperrt");
    database::review_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_select_pdf(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri::api::dialog::FileDialogBuilder;

    let (tx, rx) = std::sync::mpsc::channel::<Option<std::path::PathBuf>>();
    FileDialogBuilder::new()
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

