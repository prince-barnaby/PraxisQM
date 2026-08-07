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
| Dashboard | Startseite mit Übersicht, Statuskarten und Schnellzugriffen | `src/pages/`, `src/components/dashboard/` | aktiv |
| Navigation | Sidebar, Header, Seitenwechsel | `src/components/` | aktiv |
| Dokumente | Dokumentenverwaltung, Listen, Upload, Versionierung | `src/pages/`, `src/components/documents/` | vorbereitet |
| Archiv | Archivierte Dokumente und Wiederherstellung | `src/pages/`, `src/components/archive/` | vorbereitet |
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
| Sidebar | Navigation | `src/components/Sidebar.tsx` | `Sidebar` | Hauptnavigation links mit React-Router-NavLinks und Active-Highlighting | aktiv |
| Header | Navigation | `src/components/Header.tsx` | `Header` | Oberer Kopfbereich mit Titel und Platzhalter-Suchleiste | vorbereitet |
| Startseite | Dashboard | `src/pages/Startseite.tsx` | `Startseite` | Dashboard-Grundlayout mit vier Statuskarten | aktiv |
| Anwendungseintrittspunkt | Grundstruktur | `src/main.tsx` | `main` | React-DOM-Root, lädt globale Styles | aktiv |
| Anwendungskomponente | Grundstruktur | `src/App.tsx` | `App` | Setzt App-Shell und Startseite zusammen | aktiv |
| Design Tokens | Design System | `src/styles/tokens.css` | CSS-Variablen | Zentrale Farb-, Schrift- und Abstandsdefinitionen | aktiv |
| Globales CSS | Design System | `src/styles/global.css` | Reset + Base | Globale Basestyles auf Token-Basis | aktiv |
| Tauri-Konfiguration | Desktop Runtime | `src-tauri/tauri.conf.json` | – | Tauri-Anwendungskonfiguration | vorbereitet |
| Tauri-Cargo-Manifest | Desktop Runtime | `src-tauri/Cargo.toml` | – | Rust-Abhängigkeiten für Tauri | vorbereitet |
| Tauri-Haupteintrittspunkt | Desktop Runtime | `src-tauri/src/main.rs` | `main` | Startet native Desktop-Anwendung | vorbereitet |
| Tauri-Build-Skript | Desktop Runtime | `src-tauri/build.rs` | – | Tauri-Build-Hook | vorbereitet |
| Dashboard-Grid | Dashboard | `src/components/dashboard/DashboardGrid.tsx` | `DashboardGrid` | Responsive CSS-Grid-Wrapper für Statuskarten | aktiv |
| Dashboard-Karte | Dashboard | `src/components/dashboard/DashboardCard.tsx` | `DashboardCard` | Wiederverwendbare, klickbare Statuskarte (großes Icon, Titel, Wert, Beschreibung, Navigationspfeil, Hover-Animation) | aktiv |
| Dashboard-Karte „Dokumente“ | Dashboard | `src/pages/Startseite.tsx` | `DashboardCard` | Schnellübersicht Dokumente (Platzhalter-Wert) | vorbereitet |
| Dashboard-Karte „Mitarbeiter“ | Dashboard | `src/pages/Startseite.tsx` | `DashboardCard` | Schnellübersicht Mitarbeiter (Platzhalter-Wert) | vorbereitet |
| Dashboard-Karte „Archiv“ | Dashboard | `src/pages/Startseite.tsx` | `DashboardCard` | Schnellzugriff Archiv (Platzhalter-Wert) | vorbereitet |
| Dashboard-Karte „Systemstatus“ | Dashboard | `src/pages/Startseite.tsx` | `DashboardCard` | Anzeige lokaler Systeminformationen (Platzhalter-Wert) | vorbereitet |
| Button „Neues Dokument” | Dokumente | `src/components/documents/DocumentToolbar.tsx` | `DocumentToolbar` | Navigiert zum Dokument-Erstellungsformular (`/dokumente/neu`) | aktiv |
| Suchfeld Dokumente | Dokumente | `src/components/documents/DocumentToolbar.tsx` | `DocumentToolbar` | Filtert Dokumentenliste (Platzhalter, deaktiviert) | vorbereitet |
| Filterbereich Dokumente | Dokumente | `src/components/documents/DocumentFilters.tsx` | `DocumentFilters` | Einklappbarer Filterbereich mit fünf dokumentierten Filtern (Platzhalter, nicht funktional); Standard: eingeklappt | vorbereitet |
| Dokumentenliste | Dokumente | `src/components/documents/DocumentList.tsx` | `DocumentList` | Tabellarische Übersicht mit Mock-Platzhalter-Einträgen; sticky Tabellenkopf, optimierte Spaltengewichtung | vorbereitet |
| Dokumentenzeile | Dokumente | `src/components/documents/DocumentRow.tsx` | `DocumentRow` | Eine Tabellenzeile mit allen dokumentierten Dokumentfeldern; Hover-State, Keyboard-Focus, Monospace für Nummer/Version; komplette Zeile navigiert zur Detailansicht (`/dokumente/{nummer}`) | aktiv |
| Status-Badge | Dokumente | `src/components/documents/StatusBadge.tsx` | `StatusBadge` | Wiederverwendbare Badge für Status und Gültigkeit | aktiv |
| Empty State | Dokumente | `src/components/documents/EmptyState.tsx` | `EmptyState` | Platzhalter für leeren Zustand der Dokumentenliste | aktiv |
| Dokumente-Seite | Dokumente | `src/pages/Dokumente.tsx` | `Dokumente` | Statische Dokumentenübersicht mit Toolbar, Filtern und Liste | vorbereitet |
| Dokumentdetail-Seite | Dokumente | `src/pages/DokumentDetail.tsx` | `DokumentDetail` | Statische Dokumentdetailansicht mit Metadaten, Beschreibung, Datei, Tags, Aktionen und Historie | vorbereitet |
| Metadaten-Komponente | Dokumente | `src/components/documents/DocumentMetadata.tsx` | `DocumentMetadata` | Zweispaltige Metadaten-Tabelle (Platzhalter) | vorbereitet |
| Tag-Liste | Dokumente | `src/components/documents/TagList.tsx` | `TagList` | Anzeige von Schlagwörtern als Pills (Platzhalter) | vorbereitet |
| Aktionsleiste | Dokumente | `src/components/documents/DocumentActionBar.tsx` | `DocumentActionBar` | Aktion-Buttons: Bearbeiten (navigiert zur Bearbeitungsroute), PDF öffnen, Archivieren (Platzhalter, deaktiviert) | aktiv |
| Versionshistorie | Dokumente | `src/components/documents/DocumentHistory.tsx` | `DocumentHistory` | Einfache Timeline der Versionshistorie (Platzhalter) | vorbereitet |
| Dokumentformular | Dokumente | `src/components/documents/DocumentForm.tsx` | `DocumentForm` | Wiederverwendbares Formular für Erstellen (`create`) und Bearbeiten (`edit`); Felder: Dokumentnummer (read-only), Titel, Kategorie, Unterkategorie, Version, Status, Verantwortliche Person, Gültig bis, Beschreibung, Tags | vorbereitet |
| Formular-Abschnitt | Dokumente | `src/components/documents/DocumentFormSection.tsx` | `DocumentFormSection` | Abschnitts-Wrapper für Formularbereiche (fieldset/legend) | vorbereitet |
| Formular-Feld | Dokumente | `src/components/documents/FormField.tsx` | `FormField` | Wiederverwendbarer Field-Wrapper mit Label, Hint und Flex-Layout | vorbereitet |
| Neue-Dokument-Seite | Dokumente | `src/pages/DokumentNeu.tsx` | `DokumentNeu` | Statische Seite zum Anlegen eines neuen Dokuments; verwendet `DocumentForm` im Modus `create` | vorbereitet |
| Bearbeiten-Seite | Dokumente | `src/pages/DokumentBearbeiten.tsx` | `DokumentBearbeiten` | Statische Seite zum Bearbeiten eines bestehenden Dokuments; verwendet `DocumentForm` im Modus `edit` | vorbereitet |
| Button „Dokument archivieren“ | Dokumente / Archiv | `src/` | noch offen | Verschiebt Dokument ins Archiv | geplant |
| Archiv-Seite | Archiv | `src/pages/Archiv.tsx` | `Archiv` | Statische Archivübersicht mit Werkzeugleiste, einklappbarem Filterbereich und Tabelle archivierter Dokumente | vorbereitet |
| Archiv-Werkzeugleiste | Archiv | `src/components/archive/ArchiveToolbar.tsx` | `ArchiveToolbar` | Werkzeugleiste mit Seitentitel, Kurzbeschreibung und Archivzähler | vorbereitet |
| Archiv-Filter | Archiv | `src/components/archive/ArchiveFilters.tsx` | `ArchiveFilters` | Einklappbarer Filterbereich mit dokumentierten Archiv-Filtern: Kategorie, Unterkategorie, Verantwortliche Person, Archivierungszeitraum, Status | vorbereitet |
| Archiv-Liste | Archiv | `src/components/archive/ArchiveList.tsx` | `ArchiveList` | Tabellarische Übersicht archivierter Dokumente; zeigt `EmptyState` bei leerem Eingabearray | vorbereitet |
| Archiv-Zeile | Archiv | `src/components/archive/ArchiveRow.tsx` | `ArchiveRow` | Tabellenzeile mit archivspezifischen Feldern; Hover-State, Keyboard-Focus, visuelle Anzeige einer künftigen Archivdetailansicht | vorbereitet |
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
| Prompt 002 – Navigation | Navigation | `src/main.tsx`, `src/App.tsx`, `src/components/Sidebar.tsx`, `src/components/Sidebar.css`, `src/components/PlaceholderPage.tsx`, `src/components/PlaceholderPage.css`, `src/pages/Startseite.tsx`, `src/pages/Dokumente.tsx`, `src/pages/Archiv.tsx`, `src/pages/Mitarbeiter.tsx`, `src/pages/Einstellungen.tsx` | aktiv |
| Prompt 003 – Dashboard | Dashboard | `src/pages/Startseite.tsx`, `src/pages/Startseite.css`, `src/components/dashboard/DashboardGrid.tsx`, `src/components/dashboard/DashboardGrid.css`, `src/components/dashboard/DashboardCard.tsx`, `src/components/dashboard/DashboardCard.css` | aktiv |
| Prompt 004 – Dokumentenübersicht | Dokumente | `src/pages/Dokumente.tsx`, `src/pages/Dokumente.css`, `src/components/documents/DocumentToolbar.tsx`, `src/components/documents/DocumentToolbar.css`, `src/components/documents/DocumentFilters.tsx`, `src/components/documents/DocumentFilters.css`, `src/components/documents/DocumentList.tsx`, `src/components/documents/DocumentList.css`, `src/components/documents/DocumentRow.tsx`, `src/components/documents/StatusBadge.tsx`, `src/components/documents/StatusBadge.css`, `src/components/documents/EmptyState.tsx`, `src/components/documents/EmptyState.css` | vorbereitet |
| Prompt 005 – Archivgrundlage | Archiv | noch offen | geplant |
| Prompt 009 – Archivübersicht UI | Archiv | `src/pages/Archiv.tsx`, `src/pages/Archiv.css`, `src/components/archive/ArchiveToolbar.tsx`, `src/components/archive/ArchiveToolbar.css`, `src/components/archive/ArchiveFilters.tsx`, `src/components/archive/ArchiveFilters.css`, `src/components/archive/ArchiveList.tsx`, `src/components/archive/ArchiveList.css`, `src/components/archive/ArchiveRow.tsx` | vorbereitet |
| Prompt 005 – Dokumentdetailseite | Dokumente | `src/pages/DokumentDetail.tsx`, `src/pages/DokumentDetail.css`, `src/components/documents/DocumentMetadata.tsx`, `src/components/documents/DocumentMetadata.css`, `src/components/documents/TagList.tsx`, `src/components/documents/TagList.css`, `src/components/documents/DocumentActionBar.tsx`, `src/components/documents/DocumentActionBar.css`, `src/components/documents/DocumentHistory.tsx`, `src/components/documents/DocumentHistory.css`, `src/App.tsx` | vorbereitet |
| Prompt 007 – Neue-Dokument-Formular | Dokumente | `src/pages/DokumentNeu.tsx`, `src/pages/DokumentNeu.css`, `src/components/documents/DocumentForm.tsx`, `src/components/documents/DocumentForm.css`, `src/components/documents/DocumentFormSection.tsx`, `src/components/documents/DocumentFormSection.css`, `src/components/documents/FormField.tsx`, `src/components/documents/FormField.css`, `src/App.tsx`, `src/components/documents/DocumentToolbar.tsx`, `src/components/documents/DocumentToolbar.css` | vorbereitet |
| Prompt 008 – Dokument-Bearbeitungsformular | Dokumente | `src/pages/DokumentBearbeiten.tsx`, `src/pages/DokumentBearbeiten.css`, `src/components/documents/DocumentForm.tsx`, `src/components/documents/DocumentActionBar.tsx`, `src/components/documents/DocumentActionBar.css`, `src/pages/DokumentDetail.tsx`, `src/App.tsx` | vorbereitet |

---

## Änderungsprotokoll Code Map

| Datum | Änderung | Autor |
|---|---|---|
| 2026-07-06 | Initiale Code Map erstellt | Saskia / ChatGPT |
| 2026-08-05 | Code Map für technisches Grundgerüst aktualisiert (App-Shell, Sidebar, Header, Startseite, Design Tokens, Tauri-Struktur) | Saskia / Bolt |
| 2026-08-06 | Code Map für Prompt 002 (Navigation: React Router, NavLink, Platzhalter-Seiten) aktualisiert | Saskia / Bolt |
| 2026-08-06 | Code Map für Prompt 003 (Dashboard-Grundlayout, DashboardGrid, DashboardCard) aktualisiert | Saskia / Bolt |
| 2026-08-06 | Code Map für Dashboard-Visual-Improvement (klickbare Karten, Navigationspfeil, Hover-Animation, kompakteres Layout) aktualisiert | Saskia / Bolt |
| 2026-08-07 | Code Map für Prompt 004 (Dokumentenübersicht UI: DocumentToolbar, DocumentFilters, DocumentList, DocumentRow, StatusBadge, EmptyState) aktualisiert | Saskia / Bolt |
| 2026-08-07 | Code Map für Prompt 004A (Dokumentenübersicht UX-Verfeinerung: einklappbare Filter, sticky Tabellenkopf, Spaltengewichtung, Zeilen-Hover/Focus, Dokumentenzähler) aktualisiert | Saskia / Bolt |
| 2026-08-07 | Code Map für Prompt 005 (Dokumentdetailseite: DokumentDetail, DocumentMetadata, TagList, DocumentActionBar, DocumentHistory, Route `/dokumente/:id`) aktualisiert | Saskia / Bolt |
| 2026-08-07 | Code Map für Prompt 006 (Navigation Dokumentenliste → Detailansicht: DocumentRow onClick/Enter, useParams in DokumentDetail, Zurück-Link) aktualisiert | Saskia / Bolt |
| 2026-08-07 | Code Map für Prompt 007 (Neue-Dokument-Formular: DokumentNeu, DocumentForm, DocumentFormSection, FormField, Route `/dokumente/neu`, Button „Neues Dokument" aktiviert) aktualisiert | Saskia / Bolt |
| 2026-08-07 | Code Map für Prompt 008 (Dokument-Bearbeitungsformular: DokumentBearbeiten, DocumentForm edit-Modus, DocumentActionBar onEdit, Route `/dokumente/:id/bearbeiten`) aktualisiert | Saskia / Bolt |
| 2026-08-07 | Code Map für Prompt 009 (Archivübersicht UI: Archiv, ArchiveToolbar, ArchiveFilters, ArchiveList, ArchiveRow; Feld Archivierungsdatum dokumentiert) aktualisiert | Saskia / Bolt |

---

## Pflege-Regeln

1. Jede neue Komponente wird in dieser Datei eingetragen.
2. Jeder neue Button bekommt einen Eintrag.
3. Jede zentrale Funktion bekommt einen Eintrag.
4. Wenn Bolt Code erzeugt, muss geprüft werden, ob die Code Map angepasst werden muss.
5. Datei- und Funktionsnamen müssen nach der Umsetzung aktualisiert werden.
6. Platzhalter wie „noch offen“ werden ersetzt, sobald konkrete Dateien existieren.
7. Die Code Map darf nicht veralten, da sie als Orientierung für Entwicklung, Review und spätere Wartung dient.
