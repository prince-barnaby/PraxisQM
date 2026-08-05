// PraxisQM – Tauri Haupteintrittspunkt
// Modul: Desktop Runtime
// Zweck: Startet die native Desktop-Anwendung und lädt die React-Oberfläche.
// Es sind noch keine Backend-Befehle definiert – dies ist das reine Grundgerüst.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten von PraxisQM");
}
