# PraxisQM – Code Map

## Zweck dieses Dokuments

Die Code Map ist der technische Lageplan der PraxisQM-Anwendung.

Sie dokumentiert, welches UI-Element, welche Funktion und welches Modul in welcher Datei umgesetzt ist. Dadurch bleibt der Code auch dann nachvollziehbar, wenn die Anwendung wächst oder später durch KI-gestützte Entwicklung erweitert wird.

Dieses Dokument muss bei neuen Modulen, Komponenten, Buttons, Dialogen, Services oder zentralen Funktionen aktualisiert werden.

---

## Status-Legende

| Status | Bedeutung |
|---|---|
| geplant | Funktion ist fachlich vorgesehen, aber noch nicht umgesetzt |
| vorbereitet | Datei oder Struktur existiert, Funktion ist aber noch nicht vollständig aktiv |
| in Entwicklung | Funktion wird aktuell umgesetzt |
| aktiv | Funktion ist umgesetzt und nutzbar |
| prüfen | Funktion existiert, muss aber getestet oder fachlich geprüft werden |
| überarbeiten | Funktion existiert, muss aber angepasst werden |
| archiviert | Funktion wurde ersetzt oder wird nicht mehr aktiv verwendet |

---

## Modulübersicht

| Modul | Zweck | Hauptordner | Status |
|---|---|---|---|
| Grundstruktur | App-Shell, Eintrittspunkt, Build-Konfiguration | `src/` | aktiv |
| Dashboard | Startseite mit Übersicht, Statuskarten und Schnellzugriffen | `src/pages/` | vorbereitet |
| Navigation | Sidebar, Header, Seitenwechsel | `src/components/` | vorbereitet |
| Dokumente | Dokumentenverwaltung, Listen, Upload, Versionierung | `src/` | geplant |
| Archiv | Archivierte Dokumente und Wiederherstellung | `src/` | geplant |
| Mitarbeiter | Mitarbeiterverwaltung, Rollen, Schulungen | `src/` | geplant |
| Einstellungen | Praxisdaten, Systemoptionen, Backup-Einstellungen | `src/` | geplant |
| Datenhaltung | lokale Speicherung und Datenmodell | `database/` / `src/` | geplant |
| Backup | Export, Import, Sicherung, Wiederherstellung | `src/` | geplant |
| Desktop Runtime | Tauri-Projektstruktur | `src-tauri/` | vorbereitet |
| Tests | Testfälle und Qualitätssicherung | `tests/` | geplant |

---

## UI-Elemente und Komponenten

| UI-Element | Modul | Datei | Funktion / Komponente | Zweck | Status |
|---|---|---|---|---|---|
| App-Shell | Grundstruktur | `src/components/AppShell.tsx` | `AppShell` | Grundlayout: Sidebar + Header + Inhaltsbereich | aktiv |
| Sidebar | Navigation | `src/components/Sidebar.tsx` | `Sidebar` | Hauptnavigation links (statisch, keine echte Navigation) | vorbereitet |
| Header | Navigation | `src/components/Header.tsx` | `Header` | Oberer Kopfbereich mit Titel und Platzhalter-Suchleiste | vorbereitet |
| Startseite | Dashboard | `src/pages/Startseite.tsx` | `Startseite` | Leere Startseite mit Platzhalter | vorbereitet |
| Anwendungseintrittspunkt | Grundstruktur | `src/main.tsx` | `main` | React-DOM-Root, lädt globale Styles | aktiv |
| Anwendungskomponente | Grundstruktur | `src/App.tsx` | `App` | Setzt App-Shell und Startseite zusammen | aktiv |
| Design Tokens | Design System | `src/styles/tokens.css` | CSS-Variablen | Zentrale Farb-, Schrift- und Abstandsdefinitionen | aktiv |
| Globales CSS | Design System | `src/styles/global.css` | Reset + Base | Globale Basestyles auf Token-Basis | aktiv |
| Tauri-Konfiguration | Desktop Runtime | `src-tauri/tauri.conf.json` | – | Tauri-Anwendungskonfiguration | vorbereitet |
| Tauri-Cargo-Manifest | Desktop Runtime | `src-tauri/Cargo.toml` | – | Rust-Abhängigkeiten für Tauri | vorbereitet |
| Tauri-Haupteintrittspunkt | Desktop Runtime | `src-tauri/src/main.rs` | `main` | Startet native Desktop-Anwendung | vorbereitet |
| Tauri-Build-Skript | Desktop Runtime | `src-tauri/build.rs` | – | Tauri-Build-Hook | vorbereitet |
| Dashboard-Karte „Dokumente“ | Dashboard | `src/` | noch offen | Schnellübersicht Dokumente | geplant |
| Dashboard-Karte „Mitarbeiter“ | Dashboard | `src/` | noch offen | Schnellübersicht Mitarbeiter | geplant |
| Dashboard-Karte „Archiv“ | Dashboard | `src/` | noch offen | Schnellzugriff Archiv | geplant |
| Dashboard-Karte „Systemstatus“ | Dashboard | `src/` | noch offen | Anzeige lokaler Systeminformationen | geplant |
| Button „Neues Dokument“ | Dokumente | `src/` | noch offen | Startet später den Dokumenten-Upload | geplant |
| Button „Dokument archivieren“ | Dokumente / Archiv | `src/` | noch offen | Verschiebt Dokument ins Archiv | geplant |
| Suchfeld Dokumente | Dokumente | `src/` | noch offen | Filtert Dokumentenliste | geplant |
| Mitarbeiterliste | Mitarbeiter | `src/` | noch offen | Zeigt Mitarbeitende der Praxis | geplant |
| Button „Mitarbeiter hinzufügen“ | Mitarbeiter | `src/` | noch offen | Öffnet Formular für neuen Mitarbeiter | geplant |
| Einstellungen-Seite | Einstellungen | `src/` | noch offen | System- und Praxiseinstellungen | geplant |

---

## Services und zentrale Logik

| Service / Funktion | Datei | Zweck | Verwendet von | Status |
|---|---|---|---|---|
| Dokument speichern | noch offen | Speichert Dokumentmetadaten lokal | Dokumente | geplant |
| Dokument archivieren | noch offen | Markiert Dokument als archiviert | Dokumente / Archiv | geplant |
| Dokument wiederherstellen | noch offen | Holt Dokument aus dem Archiv zurück | Archiv | geplant |
| Dokument suchen | noch offen | Filtert Dokumente nach Suchbegriff | Dokumente | geplant |
| Mitarbeiter speichern | noch offen | Speichert Mitarbeiterdaten lokal | Mitarbeiter | geplant |
| Backup erstellen | noch offen | Erstellt lokale Sicherung | Einstellungen / Backup | geplant |
| Backup wiederherstellen | noch offen | Importiert lokale Sicherung | Einstellungen / Backup | geplant |
| Systemstatus prüfen | noch offen | Prüft lokale App- und Speicherinformationen | Dashboard | geplant |

---

## Datenmodelle

| Datenmodell | Datei | Zweck | Status |
|---|---|---|---|
| Document | noch offen | Struktur für QM-Dokumente | geplant |
| Employee | noch offen | Struktur für Mitarbeitende | geplant |
| Role | noch offen | Rollen und Berechtigungen | geplant |
| TrainingRecord | noch offen | Schulungs- und Kenntnisnachweise | geplant |
| ArchiveEntry | noch offen | Archivierte Objekte | geplant |
| BackupRecord | noch offen | Sicherungen und Wiederherstellungen | geplant |

---

## Prompt-Verknüpfung

| Prompt | Zielmodul | Ergebnisdateien | Status |
|---|---|---|---|
| Prompt 001 – Projektgrundgerüst | Grundstruktur | `package.json`, `vite.config.ts`, `tsconfig.json`, `index.html`, `src/main.tsx`, `src/App.tsx`, `src/components/AppShell.tsx`, `src/components/Sidebar.tsx`, `src/components/Header.tsx`, `src/pages/Startseite.tsx`, `src/styles/tokens.css`, `src/styles/global.css`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `src-tauri/src/main.rs`, `src-tauri/build.rs` | aktiv |
| Prompt 002 – Navigation | Navigation | noch offen | geplant |
| Prompt 003 – Dashboard | Dashboard | noch offen | geplant |
| Prompt 004 – Dokumentenübersicht | Dokumente | noch offen | geplant |
| Prompt 005 – Archivgrundlage | Archiv | noch offen | geplant |

---

## Änderungsprotokoll Code Map

| Datum | Änderung | Autor |
|---|---|---|
| 2026-07-06 | Initiale Code Map erstellt | Saskia / ChatGPT |
| 2026-08-05 | Code Map für technisches Grundgerüst aktualisiert (App-Shell, Sidebar, Header, Startseite, Design Tokens, Tauri-Struktur) | Saskia / Bolt |

---

## Pflege-Regeln

1. Jede neue Komponente wird in dieser Datei eingetragen.
2. Jeder neue Button bekommt einen Eintrag.
3. Jede zentrale Funktion bekommt einen Eintrag.
4. Wenn Bolt Code erzeugt, muss geprüft werden, ob die Code Map angepasst werden muss.
5. Datei- und Funktionsnamen müssen nach der Umsetzung aktualisiert werden.
6. Platzhalter wie „noch offen“ werden ersetzt, sobald konkrete Dateien existieren.
7. Die Code Map darf nicht veralten, da sie als Orientierung für Entwicklung, Review und spätere Wartung dient.
