# CHANGELOG

Alle wichtigen Änderungen an PraxisQM werden in dieser Datei dokumentiert.

## [0.9.2] - 06.08.2026

### Hinzugefügt

- Dashboard-Grundlayout auf der Startseite mit vier Statuskarten (Dokumente, Mitarbeiter, Archiv, Systemstatus)
- Wiederverwendbare Dashboard-Karten-Komponente (`src/components/dashboard/DashboardCard.tsx`)
- Responsive Dashboard-Grid-Komponente (`src/components/dashboard/DashboardGrid.tsx`)
- Alle Dashboard-Karten zeigen eindeutig gekennzeichnete Platzhalter-Werte, keine erfundenen Daten
- Accessibility-Eigenschaften (role, aria-label) für alle Karten
- Code Map und Changelog aktualisiert

### Geändert

- Startseite von Platzhalter auf Dashboard-Grundlayout umgestellt

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine echten Dokumentdaten
- Keine Suchfunktion, kein Upload, keine Benutzer- oder Rollenlogik
- Keine Archivierungslogik, keine Benachrichtigungslogik
- Keine Änderungen an Navigation, Header, Sidebar oder Routing

## [0.9.1] - 05.08.2026

### Hinzugefügt

- Technisches Grundgerüst der lokalen Desktop-Anwendung erstellt
- React + TypeScript + Vite Projektgerüst eingerichtet (`package.json`, `vite.config.ts`, `tsconfig.json`, `index.html`)
- Anwendungseintrittspunkt `src/main.tsx` und Anwendungskomponente `src/App.tsx` erstellt
- Statische App-Shell aus Sidebar, Header und zentralem Inhaltsbereich erstellt (`src/components/AppShell.tsx`)
- Statische Sidebar-Komponente mit Lucide-Icons und deutschsprachigen Navigationslabels (`src/components/Sidebar.tsx`)
- Statische Header-Komponente mit Platzhalter-Suchleiste (`src/components/Header.tsx`)
- Leere Startseite mit eindeutig gekennzeichnetem Platzhalter (`src/pages/Startseite.tsx`)
- Design Tokens als zentrale CSS-Variablen eingebunden (`src/styles/tokens.css`)
- Globales CSS auf Basis der Design Tokens erstellt (`src/styles/global.css`)
- Tauri-Projektstruktur gemäß ADR-027 vorbereitet (`src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/src/main.rs`, `src-tauri/build.rs`)
- Code Map (`docs/22_Code_Map.md`) für alle neuen Komponenten aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbanklogik, keine SQLite-Verbindung
- Kein Login, keine Rollen, keine Dokumentenverwaltung
- Keine Archivierung, kein Upload, kein Backup
- Keine echte Navigation oder Fachlogik
- Keine Cloud-Funktionen

## [0.9.0] - 04.07.2026

### Hinzugefügt

- Projektstruktur für GitHub angelegt
- README.md erstellt
- Software Design Document begonnen
- ADR-Ordner für Architekturentscheidungen angelegt
- Grundlegende Projektphilosophie dokumentiert
- Ordner für Mockups, Datenbank, Prompts, Tests und Releases angelegt
