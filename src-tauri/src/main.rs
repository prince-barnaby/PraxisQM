// PraxisQM – Tauri Haupteintrittspunkt
// Modul: Desktop Runtime
// Zweck: Startet die native Desktop-Anwendung, initialisiert die SQLite-Datenbank
//         und lädt die React-Oberfläche.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db_path = database::database_path(app.handle());
            match database::init_database(&db_path) {
                Ok(()) => {
                    println!("PraxisQM: SQLite-Datenbank initialisiert: {}", db_path.display());
                }
                Err(e) => {
                    eprintln!("PraxisQM: Fehler bei der Datenbankinitialisierung: {}", e);
                    std::process::exit(1);
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten von PraxisQM");
}
