# CHANGELOG

Alle wichtigen Änderungen an PraxisQM werden in dieser Datei dokumentiert.

## [0.9.28] - 08.08.2026

### Mitarbeiter-Lifecycle, inaktive Mitarbeitende und echte Filter (Prompt 017)

Vervollständigung des Mitarbeiterregisters mit echten Filtern,
klar aktiver/inaktiver Anzeige und Sicherstellung der QM-Historienrückverfolgbarkeit.

#### Lifecycle-Verhalten

- **Kein permanenter Löschzugriff:** Ehemalige Mitarbeitende bleiben dauerhaft
  im Mitarbeiterregister (DB-003). Keine cascade-Löschung, kein Entfernen
  historischer Mitarbeiterdatensätze.
- **Unabhängige Felder:** Aktivstatus (`is_active`) und Austrittsdatum
  (`departure_date`) bleiben unabhängig voneinander bearbeitbar. Keine
  automatische Verknüpfung zwischen den Feldern, da kanonisch nicht
  dokumentiert.
- **Statusanzeige:** Aktive Mitarbeitende zeigen "aktiv" (Success-Badge),
  inaktive zeigen "inaktiv" (Neutral-Badge). Inaktive bleiben sichtbar
  und werden nicht ins Archiv verschoben.
- **Austrittsdatum:** Leeres Austrittsdatum zeigt weiterhin "—" als neutrale
  Leeranzeige. Keine Platzhalterdaten.

#### Filter

- **Echte Filter statt Platzhalter:** EmployeeFilters von deaktivierten
  Platzhalter-Selects zu funktionsfähigen Filtern umgebaut.
- **Vier Filterkriterien:** Aktivstatus (aktiv/inaktiv/alle), Position
  (aus vorhandenen Mitarbeiterdaten abgeleitet), Verantwortungsposition
  (aus DB-015 geladen), QM-Bereich (aus DB-016 geladen).
- **AND-Semantik:** Mehrere aktive Filter kombinieren sich mit UND — nur
  Mitarbeitende, die allen Kriterien entsprechen, werden angezeigt.
- **UUID-basiert:** Verantwortungsposition/QM-Bereich-Filter matchen über
  kanonische UUIDs, nicht über Anzeigenamen. Renaming in Einstellungen
  ändert Filternamen sofort, ohne Beziehungen zu verlieren.
- **Zurücksetzen:** Reset-Button erscheint nur bei aktiven Filtern.
  Setzt alle Filter zurück und zeigt alle Mitarbeitenden wieder.
- **Zugänglichkeit:** Collapsible Filter-Sektion mit aria-expanded,
  nativen Select-Semantics, Labels für alle Felder.

#### Zähler

- **Mitarbeiter-Zähler zeigt gefilterte Trefferzahl** statt Datenbank-Gesamtzahl.
  Bei 10 Mitarbeitenden und 3 Treffern zeigt der Zähler "3 Mitarbeiter".

#### Leeranzeige

- **Filter ohne Treffer:** "Keine Mitarbeitenden entsprechen den ausgewählten
  Filtern" — neutral, mit Hinweis zum Zurücksetzen.
- **Leere Datenbank:** "Noch keine Mitarbeitenden erfasst" — bleibt
  unverändert für den Fall, dass noch keine Mitarbeitenden angelegt wurden.

#### Filter-Strategie

- **Frontend-Filterung** gewählt: Alle Mitarbeitenden werden bereits geladen,
  das Dataset ist klein, Beziehungen (responsibility_ids, qm_area_ids)
  sind im Employee-Modell vorhanden. Keine architektonische Regel erfordert
  Backend-Filterung. Keine unnötige Datenbankabfragekomplexität.

#### Frontend

- **EmployeeFilters.tsx:** Komplett neu implementiert mit useState für
  Collapsible, vier Select-Feldern, Reset-Button, NO_FILTERS-Konstante,
  EmployeeFilterValues-Typ, ActiveStatusFilter-Typ.
- **EmployeeFilters.css:** Neues Styling für aktive Selects, Reset-Button,
  collapsible Panel.
- **Mitarbeiter.tsx:** Lädt Mitarbeitende + Stammdaten parallel, leitet
  Positionsoptionen aus Mitarbeiterdaten ab, filtert mit useMemo, übergibt
  filteredEmpty an EmployeeList.
- **EmployeeList.tsx:** Neuer filteredEmpty-Prop für unterschiedliche
  Leeranzeige bei Filter ohne Treffer vs. leerer Datenbank.
- **EmployeeRow.tsx:** EmployeeRowData um responsibilityIds und qmAreaIds
  erweitert für UUID-basierte Filterung.
- **Mitarbeiter.css:** Veraltete .pqm-mitarbeiter__hint-Klasse entfernt.

#### Nicht implementiert (bewusst)

- Permanente Mitarbeiterlöschung (nicht angefordert, widerspricht
  QM-Historienrückverfolgbarkeit)
- Mitarbeiterspezifische Archivtabelle (nicht kanonisch dokumentiert)
- Datum-Filter für Eintritt/Austritt (nicht kanonisch dokumentiert, nicht
  in EmployeeFilters vorhanden)
- Authentifizierung/Benutzerkonten (nicht angefordert)
- Backend-Filterung (nicht erforderlich bei kleinem Dataset)

## [0.9.27] - 08.08.2026

### Mitarbeiterbearbeitung und transaktionale Zuordnungsaktualisierung (Prompt 016)

Implementierung der Bearbeitungsfunktion für bestehende Mitarbeiter,
einschließlich sicherer Aktualisierung der Many-to-Many-Zuordnungen
zu Verantwortungspositionen (DB-013) und QM-Bereichen (DB-014).

#### Backend (Rust / Tauri)

- **Neue Tauri Commands:**
  - `cmd_get_employee` — lädt einen einzelnen Mitarbeiter anhand UUID
    mit allen zugeordneten Verantwortungspositionen und QM-Bereichen
  - `cmd_update_employee` — aktualisiert einen Mitarbeiter transaktional
    (DB-003 Felder + DB-013/DB-014 Zuordnungen in einer Transaktion)
- **Neue Datenbankfunktionen:** `get_employee`, `update_employee`,
  `load_responsibility_ids`, `load_qm_area_ids`
- **Employee-Struct erweitert:** `responsibility_ids` und `qm_area_ids`
  als kanonische UUID-Arrays für Edit-Modus-Pre-Population
- **Transaktionssicherheit:** `update_employee` wird innerhalb einer
  SQLite-Transaktion ausgeführt — bei Fehler wird alles zurückgerollt
- **Validierung:** Eingabe-Validierung (Name, Vorname, Position,
  Eintrittsdatum erforderlich); ungültige Zuordnungs-IDs verursachen
  Transaktions-Rollback
- **Tests:** 17 neue Rust-Tests (update basic fields, UUID unchanged,
  updated_at changes, add/remove/replace/clear responsibilities,
  add/remove/replace/clear QM areas, simultaneous update, invalid ID
  rollback, failed update preserves fields, get_employee complete
  relationships, nonexistent employee error, update nonexistent error)

#### Frontend (React)

- **Neue Komponenten:**
  - `MitarbeiterBearbeiten` — Edit-Seite mit Pre-Population und Save/Cancel
  - `MitarbeiterBearbeiten.css` — Styling passend zu MitarbeiterNeu
- **Aktualisierte Komponenten:**
  - `EmployeeForm` — neuer optionale `initialValues`-Prop für Edit-Modus,
    neuer `submitLabel`-Prop, `useEffect` für Pre-Population
  - `EmployeeRow` — neuer Edit-Button (Bleistift-Icon) pro Zeile
  - `EmployeeList` — neue Aktionen-Spalte in Tabellenkopf
  - `employeeApi.ts` — `fetchEmployee` und `updateEmployee` hinzugefügt,
  `UpdateEmployeeInput`-Interface, `responsibility_ids`/`qm_area_ids`
  in `Employee`-Interface
  - `App.tsx` — Route `/mitarbeiter/:id/bearbeiten` hinzugefügt

#### Wiederverwendung

- `EmployeeForm` wird sowohl für Erstellen als auch Bearbeiten verwendet
  (keine Duplikation der Formularfelder)
- `MitarbeiterBearbeiten` folgt dem gleichen Layout wie `MitarbeiterNeu`
- Edit-Button verwendet lucide-react `Pencil`-Icon (bestehende
  Icon-Bibliothek)

#### Transaktionsstrategie

`cmd_update_employee` öffnet eine SQLite-Transaktion, führt alle
Updates aus (DB-003 Felder, DB-013 Zuordnungen, DB-014 Zuordnungen,
updated_at) und committet nur bei vollem Erfolg. Bei Fehler wird
die Transaktion zurückgerollt — keine teilweise aktualisierten
Mitarbeiterdatensätze.

#### Validierung

- Backend: trimmt Eingaben, leere Pflichtfelder abgelehnt,
  UNIQUE-Constraint-Fehler weitergegeben
- Frontend: gleiche Validierung wie Erstellen (Name, Vorname, Position,
  Eintrittsdatum erforderlich)
- Ungültige Zuordnungs-IDs: gesamte Transaktion schlägt fehl,
  Grunddaten bleiben unverändert

#### Nicht implementiert (bewusst)

- Mitarbeiterlöschung (nicht angefordert)
- Archivierung (nicht angefordert)
- Authentifizierung/Benutzerkonten (nicht angefordert)
- Aktivstatus/Austrittsdatum-Verknüpfung (keine kanonische Regel)
- Mitarbeiter-History/Versionierung (nicht angefordert)

## [0.9.26] - 07.08.2026

### Stammdatenverwaltung für Verantwortungspositionen und QM-Bereiche (Prompt 015)

Implementierung der CRUD-Stammdatenverwaltung für DB-015
(Verantwortungspositionen) und DB-016 (QMBereiche) innerhalb der
Einstellungen-Seite. Die Stammdatentabellen können nun über die UI
gefüllt werden, anstatt leer zu bleiben.

#### Backend (Rust / Tauri)

- **Neue Tauri Commands:**
  - `cmd_create_responsibility` — erstellt eine Verantwortungsposition
    (UUID + Zeitstempel im Backend, UNIQUE-Constraint geprüft)
  - `cmd_rename_responsibility` — benennt eine Verantwortungsposition um
    (gleiche UUID, updated_at aktualisiert, Zuordnungen bleiben erhalten)
  - `cmd_create_qm_area` — erstellt einen QM-Bereich
  - `cmd_rename_qm_area` — benennt einen QM-Bereich um
- **Neue Datenbankfunktionen:** `create_responsibility`,
  `rename_responsibility`, `create_qm_area`, `rename_qm_area`
- **Validierung:** Backend trimmt Eingaben, leere Bezeichnungen werden
  abgelehnt, UNIQUE-Constraint-Fehler werden an das Frontend weitergegeben
- **Bestehende Commands:** `cmd_list_responsibilities` und
  `cmd_list_qm_areas` werden weiterhin verwendet
- **Tests:** 12 neue Rust-Tests (create, list sorted, duplicate rejected,
  rename, rename preserves ID, rename preserves assignment — jeweils für
  Verantwortungspositionen und QM-Bereiche)

#### Frontend (React)

- **Neue Komponenten:**
  - `MasterDataSection` — wiederverwendbare Komponente für
    Stammdatenverwaltung (Anzeigen, Hinzufügen, Inline-Umbenennen)
  - `src/lib/tauriInvoke.ts` — gemeinsamer Tauri-v1-invoke-Helper
  (aus employeeApi.ts extrahiert)
  - `src/lib/masterDataApi.ts` — API-Wrapper für die vier neuen Commands
- **Aktualisierte Komponenten:**
  - `Einstellungen` — "Kategorien & Unterkategorien"-Bereich zeigt nun
    zwei MasterDataSection-Komponenten (Verantwortungspositionen und
    QM-Bereiche) statt eines Platzhalters
  - `employeeApi.ts` — verwendet nun den gemeinsamen tauriInvoke-Helper
- **Mitarbeiterformular:** Keine Code-Änderungen erforderlich — lädt
  Stammdaten weiterhin über fetchResponsibilities/fetchQmAreas, die nun
  die befüllten Tabellen anzeigen

#### Löschverhalten

Löschverhalten für DB-015 und DB-016 ist im Data Dictionary als
**deferred** markiert. Keine Löschfunktion implementiert. Keine
Lösch-Buttons in der UI. Kein kaskadierendes Löschen erfunden.

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.25] - 07.08.2026

### Falsche Tauri-Verfügbarkeitserkennung im Mitarbeiter-API behoben (Prompt 014C)

#### Ursache

`src/lib/employeeApi.ts` verwendete eine eigene Erkennungslogik, die
`window.__TAURI__?.invoke` prüfte, um festzustellen, ob die Anwendung in der
Tauri-Desktop-Umgebung läuft. In Tauri v1 wird die `window.__TAURI__`
Globalvariable jedoch nur befüllt, wenn in `tauri.conf.json` die Option
`withGlobalTauri: true` gesetzt ist — was in diesem Repository nicht der
Fall ist. Folglich schlug die Erkennung immer fehl, selbst innerhalb der
echten Tauri-Desktop-Anwendung, und der Mitarbeiter-API-Wrapper warf
fälschlich "Tauri nicht verfügbar – bitte als Desktop-App starten."

#### Korrektur

- `@tauri-apps/api` v1.6 Paket installiert (Tauri v1 Frontend-API)
- `invoke` wird nun aus `@tauri-apps/api/tauri` importiert (Tauri v1 Standard)
- Erkennungslogik geändert: prüft `window.__TAURI_IPC__` (in Tauri v1 immer
  vorhanden, unabhängig von `withGlobalTauri`)
- Browser-Modus (reiner Vite-Server ohne Tauri) zeigt weiterhin klare Meldung
  "Desktop-App erforderlich"
- Keine Mock-Daten, kein Browser-Fallback, keine Fehlerunterdrückung

#### Geänderte Dateien

- `package.json` — `@tauri-apps/api` v1.6 hinzugefügt
- `src/lib/employeeApi.ts` — komplett neu geschrieben mit korrekter
  Tauri v1 invoke-API und `__TAURI_IPC__`-Erkennung

#### Verifikation

- Frontend-Build: bestanden
- Runtime-Verifikation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `npm run tauri dev` verifiziert werden

## [0.9.24] - 07.08.2026

### Rust-Kompilierungsfehler behoben und automatische Stammdaten-Seeding entfernt (Prompt 014A)

#### Behobene Compiler-Fehler

1. **E0106 — `get_conn` Lifetime:** Die Hilfsfunktion `get_conn`, die einen
   `MutexGuard` aus einer `State`-Referenz zurückgab, konnte keine Lifetime
   ableiten. Die Funktion wurde entfernt; jeder Command sperrt die Verbindung
   nun direkt mit `state.0.lock()`.
2. **E0599 — `app.manage()` nicht gefunden:** Der `Manager`-Trait war nicht
   importiert. `use tauri::Manager` wurde hinzugefügt.
3. **E0596 — `conn` nicht mutierbar:** Der Transaktions-Test rief
   `conn.transaction()` auf einer nicht-mutablen `Connection` auf. Der Test
   deklariert `conn` nun als `mut`.
4. **E0603 — `seed_master_data_if_empty` privat:** Die Funktion war privat
   und wurde aus `main.rs` aufgerufen. Der Aufruf wurde entfernt (siehe unten).

#### Entfernung der automatischen Stammdaten-Seeding

Prompt 014 forderte, dass bei leeren Stammdatentabellen gestoppt und gemeldet
werden sollte, bevor erfundene Werte eingefügt werden. Die Implementierung
hat stattdessen erfundene Entwicklungswerte automatisch eingefügt.

**Korrektur:**
- `seed_master_data_if_empty` vollständig entfernt
- Startup-Aufruf in `main.rs` entfernt
- Die echte PraxisQM-Datenbank wird nicht mehr automatisch befüllt
- Leere DB-015/DB-016-Tabellen sind akzeptabel
- Das Frontend zeigt eine neutrale Meldung bei leeren Auswahllisten
- Test-Stammdaten werden nur in der temporären Testdatenbank eingefügt

**Zuvor erfundene Seed-Werte (zur getrennten Überprüfung):**
- Verantwortungspositionen: QM-Beauftragte, Datenschutzbeauftragte,
  Hygienebeauftragte, Praxisleitung, Fortbildungsbeauftragte
- QM-Bereiche: Datenschutz, Hygiene, Patientendokumentation,
  Röntgeneinweisung, Fortbildung

#### Test-Anpassungen

- `test_seed_master_data` entfernt
- `init_test_db` fügt nun explizite Test-Fixture-Daten über
  `insert_test_master_data` in die temporäre Testdatenbank ein
- Alle übrigen Mitarbeiter-Persistenz-Tests bleiben erhalten

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.23] - 07.08.2026

### Mitarbeiter-Persistenz End-to-End implementiert (Prompt 014)

Die Mitarbeiterverwaltung ist nun mit dem SQLite-Backend verbunden. Dies ist
die erste End-to-End-Persistenzfunktion in PraxisQM.

#### Backend (Rust / Tauri)

- **Tauri Commands hinzugefügt:**
  - `cmd_list_employees` — lädt alle Mitarbeiter mit Verantwortungspositionen
    und QM-Bereichen über JOINs
  - `cmd_create_employee` — erstellt Mitarbeiter und Zuordnungen in einer
    SQLite-Transaktion (Rollback bei Fehler)
  - `cmd_list_responsibilities` — lädt Verantwortungspositionen (Stammdaten)
  - `cmd_list_qm_areas` — lädt QM-Bereiche (Stammdaten)
- **Datenmodelle:** `Employee`, `MasterDataItem`, `CreateEmployeeInput`
  (mit serde Serialize/Deserialize)
- **Transaktionssicherheit:** `create_employee` verwendet eine Transaktion
  — bei ungültigen Zuordnungen werden alle Änderungen zurückgerollt
- **Stammdaten:** Keine automatische Seeding — DB-015/DB-016 bleiben leer bis
  kanonische Werte genehmigt sind (siehe Prompt 014A)
- **Verbindungspooling:** `DbState(Mutex<Connection>)` als Tauri State
- **UUID-Generierung:** Backend-seitig, nicht im Frontend
- **Tests:** 8 neue Rust-Tests für Mitarbeiter-Persistenz (create, list,
  zero/multiple assignments, transaction rollback, persisted assignments)

#### Frontend (React)

- **Neue Komponenten:**
  - `EmployeeForm` — Formular mit Multi-Select-Checkbox-Listen für
    Verantwortungspositionen und QM-Bereiche
  - `MitarbeiterNeu` — Seite zum Anlegen neuer Mitarbeiter
  - `src/lib/employeeApi.ts` — Tauri invoke API-Wrapper mit TypeScript-Typen
- **Aktualisierte Komponenten:**
  - `EmployeeRow` — `role` → `position`, `department` entfernt,
    `responsibilityRoles`/`qmAreas` als String-Arrays
  - `EmployeeList` — Spalten aktualisiert (Funktion → Position, Bereich →
    QM-Bereich), Loading/Error-State hinzugefügt, EmptyState mit deutscher
    Meldung
  - `EmployeeToolbar` — "Neuer Mitarbeiter"-Button aktiviert
  - `EmployeeFilters` — Filter-Labels aktualisiert
  - `Mitarbeiter` — lädt echte Daten vom Backend, ersetzt Placeholder-Daten
- **Routing:** Route `/mitarbeiter/neu` hinzugefügt
- **Validierung:** Name, Vorname, Position, Aktivstatus, Eintrittsdatum
  als Pflichtfelder; Austrittsdatum optional
- **ADR-004-Trennung:** Keine Benutzerkonten werden beim Anlegen eines
  Mitarbeiters erstellt

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.22] - 07.08.2026

### rusqlite::Error::InvalidQuery Unit-Variant-Fehler behoben (Prompt 013E)

`cargo test` schlug mit `error[E0618]: expected function, found rusqlite::Error`
fehl, weil `Error::InvalidQuery` in rusqlite 0.31 ein **Unit-Variant** (ohne
Datenfelder) ist und nicht mit einem String-Argument aufgerufen werden kann.

#### Ursache

In Prompt 013D wurde `Error::InvalidParameter(String)` durch
`Error::InvalidQuery(String)` ersetzt. `InvalidQuery` ist in rusqlite 0.31
aber ein Unit-Variant (`InvalidQuery,` — keine Klammern, keine Daten).
Der Aufruf `Error::InvalidQuery("...")` versucht, eine Funktion aufzurufen,
die keine ist — daher E0618.

#### Korrektur

Ersetzt durch `Error::SqliteFailure(ffi::Error::new(ffi::SQLITE_CONSTRAINT), Some(msg))`.
`SqliteFailure` ist die Standard-Variante für SQLite-Fehler und akzeptiert
ein `ffi::Error` (mit Fehlercode) und ein optionales `String`-Detail.
`SQLITE_CONSTRAINT` (Code 19) ist der passende SQLite-Fehlercode für eine
verletzte Integritätsbedingung — in diesem Fall: Foreign-Key-Enforcement
konnte nicht aktiviert/verifiziert werden.

#### Geänderte Dateien

- `src-tauri/src/database.rs`: Zeile 233 — `Error::InvalidQuery(String)`
  → `Error::SqliteFailure(ffi::Error::new(ffi::SQLITE_CONSTRAINT), Some(String))`

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.21] - 07.08.2026

### Rust-Compiler-Fehler in SQLite-Foundation behoben (Prompt 013D)

Die lokale Rust-Kompilierung schlug mit drei Fehlern und einer Warnung fehl,
alle in Prompt 013 eingeführtem Code.

#### Fehler und Ursachen

1. **`rusqlite::Error::InvalidParameter` (database.rs:233)** — Die Variante
   `InvalidParameter` existiert nicht in rusqlite 0.31. Die verfügbaren
   Invalid-Varianten sind `InvalidParameterName(String)`, `InvalidQuery(String)`,
   `InvalidPath(PathBuf)`, etc. Die Semantik des Aufrufs war ein allgemeiner
   Datenbankfehler (Foreign-Key-Enforcement fehlgeschlagen), kein Parametername-
   Fehler. Daher wurde `InvalidQuery(String)` gewählt — es ist die allgemeinste
   Invalid-Variante für einen SQL-bezogenen Fehler mit String-Beschreibung.

2. **`app.path().app_data_dir()` (database.rs:199-200)** — `path()` ist eine
   Tauri v2 API. In Tauri v1 heißt die Methode `path_resolver()` und ist eine
   inhärente Methode (über das `shared_app_impl!`-Makro), kein Trait-Method.

3. **`database_path(app.handle())` (main.rs:16)** — `app.handle()` gibt
   `AppHandle` (owned) zurück, aber `database_path` erwartet `&AppHandle`
   (Referenz). Fix: `&app.handle()`.

4. **Warnung: unused import `Manager`** — `Manager` wurde importiert für
   `path()`, aber in Tauri v1 ist `path_resolver()` eine inhärente Methode
   und benötigt den Trait nicht. Import entfernt.

#### Geänderte Dateien

- `src-tauri/src/database.rs`: `Manager`-Import entfernt, `path()` →
  `path_resolver()`, `InvalidParameter` → `InvalidQuery`
- `src-tauri/src/main.rs`: `app.handle()` → `&app.handle()`

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.20] - 07.08.2026

### Tauri v1 Icon-Set erstellt (Prompt 013C)

Der Tauri-Build schlug fehl, weil `tauri.conf.json` Icon-Dateien referenziert,
die nicht im Repository existierten. Der Tauri v1 Build-Prozess benötigt
`icons/icon.ico` zwingend für die Generierung der Windows-Ressourcendatei
(.rc/.res) während des Kompilierens.

#### Ursache

Die `tauri.conf.json` referenzierte drei Icon-Dateien unter `src-tauri/icons/`,
aber das Verzeichnis existierte nicht und keine Icon-Dateien waren im
Repository vorhanden. Es gab auch keine vorhandenen PraxisQM-Logo-Assets,
die hätten wiederverwendet werden können.

#### Korrektur

- Neue Icon-Dateien erstellt unter `src-tauri/icons/`:
  - `32x32.png` (PNG, 32×32)
  - `128x128.png` (PNG, 128×128)
  - `icon.ico` (ICO, multi-size: 256/128/64/48/32/16)
- Design: QM-Monogramm in Weiß auf Primary-Blue-Hintergrund (#163A5F)
  mit Accent-Teal-Element (#008C8C), passend zu den PraxisQM-Brand-Farben
  aus dem Design-Token-System
- Keine Änderungen an tauri.conf.json, Dependencies oder Anwendungscode

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.19] - 07.08.2026

### tauri.conf.json auf Tauri v1-Schema korrigiert (Prompt 013B)

Die `tauri.conf.json` verwendete das Tauri v2-Konfigurationsschema
(Top-Level-Felder `app`, `bundle`, `identifier`, `frontendDist`, `devUrl`),
während alle Rust-Crates und das npm-CLI-Paket auf Tauri v1 ausgelegt sind.
Der Tauri v1 Build-Parser lehnt das `app`-Feld ab:
`unknown field 'app', expected one of '$schema', 'package', 'tauri', 'build', 'plugins'`.

#### Ursache

Die `tauri.conf.json` war im Tauri v2-Format geschrieben worden, obwohl
die gesamte übrige Projektkonfiguration (Cargo.toml, package.json, build.rs,
main.rs, database.rs) konsistent auf Tauri v1 ausgerichtet ist. Dies war ein
Konfigurationsfehler, keine absichtliche v2-Migration.

#### Korrektur

- `tauri.conf.json` auf Tauri v1-Schema umgestellt:
  - `build.frontendDist` → `build.distDir` (v1-Feldname)
  - `build.devUrl` → `build.devPath` (v1-Feldname)
  - Top-Level `app` entfernt; `windows` und `security` unter `tauri` verschoben
  - Top-Level `bundle` entfernt; `bundle` unter `tauri` verschoben
  - Top-Level `identifier` entfernt; `identifier` unter `tauri.bundle` verschoben
  - Top-Level `productName`/`version` unter `package` verschoben
- Keine Dependency-Änderungen
- Keine Änderungen an main.rs, build.rs, database.rs oder dem Frontend

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.18] - 07.08.2026

### tauri-build Dependency korrigiert (Prompt 013 Korrektur)

Die Dependency `tauri-build = "^1.6"` konnte nicht aufgelöst werden, da
`tauri-build` nie eine 1.6.x Veröffentlichung hatte. Die letzte v1.x
Version ist 1.5.6. Der `tauri` Crate selbst hat Versionen 1.6.0–1.8.3,
aber `tauri-build` blieb bei 1.5.x.

#### Korrektur

- `tauri-build`: `^1.6` → `^1.5` (kompatibel mit `tauri` 1.6.x–1.8.x)
- `tempfile` dev-dependency hinzugefügt (für Rust-Tests in database.rs)

#### Ursache

`tauri-build` folgte nicht der Versionsnummer des `tauri` Crates.
Während `tauri` 1.6–1.8 erreichte, blieb `tauri-build` bei 1.5.x.
Die `^1.6` Anforderung war von Anfang an nicht auflösbar.

#### Kompatibilität

- `tauri` 1.6.x ist kompatibel mit `tauri-build` 1.5.x (Build-Time-Helper,
  keine API-Änderungen im Config-Format zwischen 1.5 und 1.8)
- `rusqlite` 0.31, `uuid` 1.x, `tempfile` 3.x sind kompatibel mit Rust 1.97.1
- Kein Tauri Major-Version-Wechsel — das Projekt bleibt auf Tauri v1

#### Verifikation

- Frontend-Build: bestanden
- Rust-Compilation: nicht in dieser Umgebung verfügbar — muss lokal mit
  `cargo test` und `cargo build` verifiziert werden

## [0.9.17] - 07.08.2026

### Initiale SQLite-Foundation implementiert (Prompt 013)

Erste Persistenz-Implementierung für PraxisQM. Lokale SQLite-Datenbank
im Tauri-Backend, vollständig offline. Keine UI-Anbindung, keine
Backup-Funktionalität, keine Authentifizierung.

#### Abhängigkeiten

- `rusqlite` 0.31 (mit `bundled` Feature für statische SQLite-Einbindung)
- `uuid` 1.x (mit `v4` Feature für UUID-Generierung)

#### Implementierung

- **`src-tauri/src/database.rs`** — Datenbankmodul mit:
  - Kanonischem Schema für alle 13 Class-A Tabellen
  - Idempotenter Initialisierung (sicher bei wiederholtem Aufruf)
  - Schema-Versionierung via `schema_version` Tabelle
  - Foreign-Key-Enforcement (`PRAGMA foreign_keys = ON`)
  - 6 Unit-Tests (Schema-Erstellung, Idempotenz, Tabellen-Existenz, FK-Enforcement, UNIQUE-Constraints, Join-Tabellen-FKs)
- **`src-tauri/src/main.rs`** — Tauri-Setup-Hook initialisiert Datenbank beim Start
- **`src-tauri/Cargo.toml`** — rusqlite und uuid Dependencies hinzugefügt
- **`.gitignore`** — SQLite-Dateien (`*.sqlite`, `*.db`, WAL/SHM/Journal) ignoriert

#### Implementierte Tabellen (Class-A)

| DB-Nr | Tabellenname | Typ |
|---|---|---|
| DB-001 | documents | Haupttabelle |
| DB-002 | document_versions | Haupttabelle |
| DB-003 | employees | Haupttabelle |
| DB-004 | users | Haupttabelle |
| DB-005 | categories | Haupttabelle |
| DB-006 | subcategories | Haupttabelle |
| DB-007 | keyword_dictionary | Haupttabelle |
| DB-008 | document_tags | Join-Tabelle (Composite PK) |
| DB-009 | audit_log | Haupttabelle |
| DB-013 | employee_responsibilities | Join-Tabelle (Composite PK) |
| DB-014 | employee_qm_areas | Join-Tabelle (Composite PK) |
| DB-015 | verantwortungspositionen | Haupttabelle |
| DB-016 | qm_bereiche | Haupttabelle |

#### Bewusst nicht implementiert

- DB-010 Backups (Class B — Backup-Funktionalität später)
- DB-011 Settings (Class C — reserviert)
- DB-012 BackupReminders (Class B — später)

#### Foreign-Key-Verhalten

- `PRAGMA foreign_keys = ON` wird bei jeder Initialisierung aktiviert
- Kein CASCADE — wo Löschverhalten nicht dokumentiert ist, wird RESTRICT verwendet (SQLite-Standard)
- DB-013 referenziert: `employee_id` → employees, `responsibility_id` → verantwortungspositionen
- DB-014 referenziert: `employee_id` → employees, `qm_area_id` → qm_bereiche

#### Datenbank-Strategie

- **Location:** Tauri `app_data_dir` (betriebssystemspezifisches App-Data-Verzeichnis)
- **Dateiname:** `praxisqm.sqlite`
- **Initialisierung:** Idempotent — `CREATE TABLE IF NOT EXISTS` für alle Tabellen
- **Schema-Versionierung:** `schema_version` Tabelle mit Versionsnummer und Zeitstempel
- **WAL-Modus:** Nicht explizit aktiviert (Standard-Journal-Modus)

#### Verifikation

- Frontend-Build: bestanden (TypeScript + Vite)
- Rust-Compilation: nicht verfügbar in dieser Umgebung (kein Cargo-Toolchain)
- Rust-Tests: geschrieben, aber nicht ausführbar (kein Cargo-Toolchain)
- Rust-Code muss vor Deployment mit `cargo test` und `cargo build` verifiziert werden

### Dokumentation

- Code Map: Prompt 013 Eintrag hinzugefügt
- CHANGELOG: Version 0.9.17

### Ergebnis

**SQLite foundation implemented.** Rust-Compilation und Tests müssen in
einer Umgebung mit Rust-Toolchain verifiziert werden. Frontend-Build bestanden.

## [0.9.16] - 07.08.2026

### Mitarbeiter-Verantwortungspositionen und QM-Bereiche normalisiert (Prompt 012C)

Normalisierung der Employee-Multi-Value-Beziehungen von String-basierten
Join-Tabellen zu kanonischen Masterdaten-Tabellen mit UUID-Referenzen.
Dokumentation und Datenmodell-Spezifikation nur — keine Code-Änderungen.

#### Neue Tabellen

- **DB-015 Verantwortungspositionen** — Kanonische Masterdaten-Tabelle mit UUID-PK, eindeutiger Bezeichnung, created_at, updated_at. Zentral verwaltbare, wiederverwendbare organisatorische Stammdaten.
- **DB-016 QMBereiche** — Kanonische Masterdaten-Tabelle mit UUID-PK, eindeutiger Bezeichnung, created_at, updated_at. Zentral verwaltbare, wiederverwendbare organisatorische Stammdaten.

#### Refaktorierte Join-Tabellen

- **DB-013 EmployeeResponsibilities** — `responsibility` (String) → `responsibility_id` (UUID FK → DB-015). Composite PK (`employee_id`, `responsibility_id`).
- **DB-014 EmployeeQMAreas** — `qm_area` (String) → `qm_area_id` (UUID FK → DB-016). Composite PK (`employee_id`, `qm_area_id`).

#### Entfernt

- String-basierte Speicherung von Verantwortungsposition und QM-Bereich in Join-Tabellen (durch UUID-Referenzen ersetzt)
- Begründungsabschnitte „Warum keine separate Lookup-Tabelle?" (durch Masterdaten-Tabellen obsolet)

#### Dokumentiert

- Zentrale Verwaltung: zukünftige UI darf autorisierten Benutzern Erstellen, Umbenennen und Zuweisung mehrerer Einträge ermöglichen
- Löschverhalten für DB-015 und DB-016: deferred — nicht dokumentiert, kein kaskadierendes Löschen erfunden
- QM-Bereiche bleiben konzeptionell getrennt von Dokumentkategorien, Unterkategorien, Verantwortungspositionen, Mitarbeiterpositionen, Benutzerrollen und Benutzerberechtigungen

#### Konsistenz-Audit bestanden

- Verantwortungspositionen und QM-Bereiche sind normalisierte Masterdaten
- Join-Tabellen referenzieren UUIDs, keine String-Werte
- Namen existieren nur in ihren kanonischen Masterdaten-Entitäten
- Position bleibt getrennt von Verantwortungsposition
- QM-Bereich bleibt getrennt von Dokumentkategorie
- Keine komma-separierten Werte
- Kein Frontend geändert, kein SQLite implementiert

### Dokumentation

- PQM-SDD-004B (Felddefinitionen): DB-015, DB-016 hinzugefügt; DB-013, DB-014 refaktoriert; Beziehungsspezifikation, Readiness, Konsistenz-Audit aktualisiert
- PQM-DD-001 (Data Dictionary): DB-015, DB-016 hinzugefügt; DB-013, DB-014 refaktoriert; Begriffstabelle aktualisiert
- PQM-SDD-004A (Tabellenübersicht): Masterdaten-Tabellen und Readiness-Klassifizierung aktualisiert
- Code Map aktualisiert

### Ergebnis

**Initial SQLite foundation remains ready.** Alle Employee-Multi-Value-Beziehungen
sind normalisiert. Keine blockierenden Lücken für die initiale SQLite-Foundation.

## [0.9.15] - 07.08.2026

### Datenbankspezifikation abgeschlossen (Prompt 012B)

Auflösung aller blockierenden Spezifikationslücken aus Prompt 012.
Dokumentation und Datenmodell-Spezifikation nur — keine Code-Änderungen.

#### Architekturentscheidungen dokumentiert

- **Primärschlüssel:** UUID für alle Haupttabellen; Composite Key für Join-Tabellen. Dokumentennummer ist Unique-Feld, nicht Primärschlüssel.
- **Join-Tabellen genehmigt:** DB-013 EmployeeResponsibilities (Mitarbeiter ↔ Verantwortungsposition, n:m), DB-014 EmployeeQMAreas (Mitarbeiter ↔ QM-Bereich, n:m). Verantwortungsposition und QM-Bereich als String-Werte in Join-Tabellen (keine separaten Lookup-Tabellen, da keine Metadaten dokumentiert).
- **User ↔ Employee:** Optionale 1:1-Beziehung (FK in DB-004 → DB-003). Employee ohne User möglich. Kein automatisches Erstellen. ADR-004 gewahrt.
- **Benutzerrollen:** Gast, Editor, Admin als dokumentierte Enum-Werte für DB-004.
- **Zeitstempel:** `created_at` / `updated_at` auf allen Haupttabellen. Nicht auf reinen Join-Tabellen. Domänenspezifische Daten bleiben separat.
- **DB-011 Settings:** Reserviert / noch nicht implementiert. Kein Key/Value-Schema. Keine erfundenen Einstellungen.
- **Backup-Status:** Deferred — keine erfundenen Enum-Werte. DB-010 und DB-012 als Klasse B klassifiziert.

#### Vollständige Felddefinitionen

- DB-001 Documents: 13 Felder (inkl. UUID PK, archived_at, created_at, updated_at)
- DB-002 DocumentVersions: 12 Felder (inkl. file_name, file_path, uploaded_by, uploaded_at)
- DB-003 Employees: 9 Felder (inkl. UUID PK, is_active, hire_date, departure_date)
- DB-004 Users: 7 Felder (inkl. username, role, employee_id, is_active)
- DB-005 Categories: 4 Felder
- DB-006 Subcategories: 5 Felder
- DB-007 KeywordDictionary: 4 Felder
- DB-008 DocumentTags: 2 Felder (Composite PK)
- DB-009 AuditLog: 6 Felder (inkl. user_id, document_id, timestamp)
- DB-010 Backups: 8 Felder (Klasse B)
- DB-011 Settings: reserviert (Klasse C)
- DB-012 BackupReminders: 6 Felder (Klasse B)
- DB-013 EmployeeResponsibilities: 2 Felder (Composite PK)
- DB-014 EmployeeQMAreas: 2 Felder (Composite PK)

#### Implementierungs-Readiness

- **A (erforderlich):** DB-001, DB-002, DB-003, DB-004, DB-005, DB-006, DB-007, DB-008, DB-009, DB-013, DB-014
- **B (verschoben):** DB-010, DB-012
- **C (reserviert):** DB-011

#### Konsistenz-Audit bestanden

- UUID-Strategie konsistent
- Dokumentennummern unabhängig von Datenbank-IDs
- Employee-Multi-Value normalisiert (DB-013, DB-014)
- User und Employee getrennt (ADR-004)
- Gast/Editor/Admin nur Software-Rollen
- Archivierungsdatum dokumentiert
- Keine komma-separierten Werte
- DB-011 ohne erfundene Einstellungen
- Keine erfundenen Backup-Statuswerte
- Keine Cloud-Abhängigkeit
- Vollständig offline-fähig

### Dokumentation

- PQM-SDD-004B (Felddefinitionen) vollständig neu geschrieben mit allen Entscheidungen
- PQM-DD-001 (Data Dictionary) vollständig neu geschrieben
- PQM-SDD-004A (Tabellenübersicht) um Join-Tabellen und Readiness-Klassifizierung ergänzt
- Code Map aktualisiert

### Ergebnis

**Initial SQLite foundation may proceed.** Alle blockierenden Lücken für die
initiale SQLite-Foundation sind aufgelöst. Deferred Felder (Backup-Status,
AuditLog-Aktionen, Authentifizierungsdaten) blockieren die initiale Foundation nicht.

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine Migrationen, keine SQLite-Dateien
- Keine Code-Änderungen, keine Frontend-Änderungen
- Keine Implementierung von UUID-Generierung oder Zeitstempel-Logik
- Keine Authentifizierungslogik
- Keine Backup-Implementierung

## [0.9.14] - 07.08.2026

### Datenmodell-Audit (Prompt 012)

Vollständiger Audit der Datenbankspezifikation DB-001 bis DB-012. Dokumentation
und Datenmodell-Spezifikation nur — keine Code-Änderungen, keine Datenbankanbindung.

#### Korrigiert

- DB-003 Employees: Feld „Funktion" zu „Position" umbenannt (entspricht UI-Implementierung)
- DB-003 Employees: Feld „Bereich" entfernt (nicht Teil des dokumentierten Mitarbeitermodells)
- DB-003 Employees: Felder E-Mail und Telefonnummer als entfernt dokumentiert (bereits in 0.9.12 aus UI entfernt)

#### Hinzugefügt

- Vollständige Beziehungsspezifikation für alle dokumentierten Tabellen (Kardinalität, FK-Richtung, Lösch-/Archivierungsverhalten)
- Enum / Status-Audit für alle Status- und Enum-Felder
- Identifikatoren- und Zeitstempel-Audit (Primärschlüssel-Strategie, Dokumentennummer vs. ID, created_at/updated_at)
- Begriffliche Trennung dokumentiert: Position vs. Verantwortungsposition vs. QM-Bereich vs. Benutzerrolle vs. Benutzerberechtigung
- Archivierungsdatum (DateTime) vollständig in Felddefinitionen und Data Dictionary dokumentiert
- DB-011 Settings als „unresolved" dokumentiert — keine erfundenen Key/Value-Paare

#### Strukturelle Lücke gemeldet

- DB-003 Many-to-Many: Join-Tabellen DB-003a (EmployeeResponsibilities) und DB-003b (EmployeeQMAreas) vorgeschlagen — müssen vor SQLite-Implementierung freigegeben werden

#### Blockierende Spezifikationslücken dokumentiert

- Primärschlüssel-Strategie für alle Tabellen
- Felddefinitionen für DB-002, DB-004, DB-005, DB-006, DB-007, DB-008, DB-009, DB-010, DB-011, DB-012
- User ↔ Employee Referenz (optional, nicht dokumentiert)
- Enum-Definitionen: Benutzerrollen, Backup-Status, Erinnerungsstatus
- created_at / updated_at Zeitstempel für alle Tabellen
- Lösch-/Archivierungsverhalten für alle Tabellen außer DB-001

### Dokumentation

- PQM-SDD-004B (Felddefinitionen) um vollständigen Audit ergänzt
- PQM-DD-001 (Data Dictionary) aktualisiert und mit Referenzen auf SDD-004B versehen
- Code Map aktualisiert

### Ergebnis

**SQLite-Implementierung ist blockiert.** 16 blockierende Spezifikationslücken
müssen vor der Implementierung geklärt werden. Siehe PQM-SDD-004B für Details.

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine Migrationen, keine SQLite-Dateien
- Keine Code-Änderungen, keine Frontend-Änderungen
- Keine Implementierung von Repositorys oder Backend-Logik
- Keine erfundenen Felder, Defaults oder Geschäftsregeln

## [0.9.13] - 07.08.2026

### Hinzugefügt

- Statische Einstellungsübersicht auf der Seite „Einstellungen" unter der Route `/einstellungen`
- Zweispaltiges Desktop-Layout mit linker Navigationsleiste und rechtem Inhaltsbereich
- Sechs dokumentierte Einstellungsbereiche: Allgemein, Dokumentennummerierung, Kategorien & Unterkategorien, Benutzerverwaltung, Backup, Systeminformationen
- Einstellungsnavigation mit Icons, `aria-current` für aktiven Bereich, Keyboard-Focus
- Bereich „Dokumentennummerierung" als schreibgeschützte Platzhalterinformation (ADR-001: automatisch, unveränderlich, nie wiederverwendet)
- Bereich „Systeminformationen" mit PraxisQM-Version, Architektur und Offline-Status (ADR-002, ADR-027)
- Bereich „Backup" als statischer Platzhalter mit Warnung, dass die Funktionalität nicht implementiert ist
- Bereich „Benutzerverwaltung" als Platzhalter, getrennt vom Mitarbeiterregister (ADR-004)
- Responsive Layout: einspaltig auf mobilen Viewports
- Wiederverwendbare Komponenten: `SettingsNav`, `SettingsSection`
- Accessibility: semantische `<nav>`, `aria-current`, `aria-label`, Keyboard-Navigation

### Dokumentation

- UI Style Guide (005C) um Abschnitt „Einstellungsoberfläche / Settings-Layout-Standard" ergänzt mit verbindlichen Regeln:
  - „Die Einstellungsoberfläche enthält nur dokumentierte Konfigurationsbereiche."
  - „Neue Einstellungsoptionen dürfen nicht ohne vorherige Dokumentation eingeführt werden."
- Component Library (005D) um Einstellungs-Komponenten ergänzt
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine Persistenz
- Keine funktionalen Steuerelemente oder Konfigurationsänderungen
- Keine Authentifizierung oder Rollenlogik
- Keine Dateioperationen oder Backup-Scheduling
- Keine CRUD-Operationen für Kategorien
- Keine Benutzerkontoverwaltung
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Dokumentenmodul, Archivmodul, Mitarbeitermodul oder Routing bestehender Seiten

## [0.9.12] - 07.08.2026

### Entfernt

- Felder **E-Mail** und **Telefonnummer** vollständig aus dem Mitarbeiterregister entfernt
- Beide Felder aus der Mitarbeitertabelle, Platzhalterdaten, EmployeeRowData-Interface und CSS entfernt
- Beide Felder aus DB-003 Employees-Felddefinitionen im Data Dictionary (PQM-DD-001) entfernt
- Beide Felder aus DB-003 Employees-Felddefinitionen in PQM-SDD-004B entfernt
- UI Style Guide und Component Library entsprechend aktualisiert

### Geändert

- Die finalen DB-003 Employees-Felder sind: Name, Vorname, Position, Verantwortungsposition (Multi-Value), Zugeordneter QM-Bereich (Multi-Value), Aktivstatus, Eintrittsdatum, Austrittsdatum
- Spaltengewichtung der Mitarbeitertabelle für 9 Spalten angepasst
- Multi-Value-Implementierung für Verantwortungsposition und QM-Bereich bleibt unverändert

## [0.9.11] - 07.08.2026

### Hinzugefügt

- Statische Mitarbeiterübersicht auf der Seite „Mitarbeiter" unter der Route `/mitarbeiter`
- Werkzeugleiste mit Seitentitel „Mitarbeiter", Kurzbeschreibung und Mitarbeiterzähler („3 Mitarbeiter")
- Button „Neuer Mitarbeiter" (deaktiviert, da keine dokumentierte Route für das Erstellungsformular existiert)
- Einklappbarer Filterbereich (Standard: eingeklappt) mit drei dokumentierten Filterkriterien: Funktion, Bereich, Aktivstatus (alle Platzhalter, nicht funktional)
- Tabellarische Desktop-Darstellung aller Mitarbeitenden mit dokumentierten Feldern: Name, Vorname, Funktion, Bereich, Verantwortungsposition, QM-Bereich, Status, Eintrittsdatum, Austrittsdatum
- Multi-Value-Felder **Verantwortungsposition** und **Zugeordneter QM-Bereich** als kompakte Badges/Chips dargestellt:
  - Verantwortungsposition: Primary Blue mit hellem Hintergrund
  - QM-Bereich: Accent Teal mit hellem Hintergrund
  - Bei null Werten wird ein neutraler Platzhalter („—") angezeigt
  - Ein Mitarbeiter mit mehreren Verantwortungspositionen und QM-Bereichen als Platzhalter-Beispiel
  - Ein Mitarbeiter mit einer einzelnen Verantwortungsposition und einem QM-Bereich als Platzhalter-Beispiel
  - Ein inaktiver Mitarbeiter ohne Verantwortungspositionen oder QM-Bereiche als Platzhalter-Beispiel
- Drei klar gekennzeichnete Mock-Platzhalter-Einträge zur Veranschaulichung von Layout, Multi-Value-Feldern und Komponenten
- Wiederverwendbare Komponenten: `EmployeeToolbar`, `EmployeeFilters`, `EmployeeList`, `EmployeeRow`
- Bestehende `StatusBadge`-Komponente wiederverwendet (Variante `success` für „aktiv", `neutral` für „inaktiv")
- Bestehende `EmptyState`-Komponente wiederverwendet („Keine Mitarbeiter vorhanden")
- Mitarbeiterzeilen mit Hover-State, Keyboard-Focus und visueller Anzeige für spätere Detailnavigation
- Accessibility: `aria-expanded` für einklappbare Filter, `aria-label`, `scope`, Tabellen-Semantik

### Datenmodell-Dokumentation

- DB-003 Employees-Felddefinitionen im Data Dictionary (PQM-DD-001) dokumentiert
- DB-003 Employees-Felddefinitionen in den Felddefinitionen (PQM-SDD-004B) dokumentiert
- Multi-Value-Felder **Verantwortungsposition** und **Zugeordneter QM-Bereich** als Many-to-Many-Beziehungen dokumentiert:
  - employee ↔ Verantwortungsposition
  - employee ↔ QM-Bereich
  - Beide Felder dürfen nicht als einzelner String oder Single-Select modelliert werden
  - Datenbank-Verknüpfungstabellen werden noch nicht implementiert
  - Platzhalterwerte sind Beispiele und keine festen Geschäftsregeln

### Dokumentation

- UI Style Guide (005C) um Abschnitt „Mitarbeiterübersicht / Mitarbeiter-Tabellenstandard" ergänzt mit verbindlichen Regeln:
  - „Das Mitarbeiterregister und die Benutzerverwaltung sind getrennte Bereiche."
  - „Die Mitarbeiterübersicht verwendet eine klassische tabellarische Desktop-Darstellung."
  - „Benutzerkonto-, Rollen- und Berechtigungsdaten dürfen nicht im Mitarbeiterregister vermischt werden."
- UI Style Guide (005C) um Abschnitt „Multi-Value-Felder" ergänzt
- Component Library (005D) um Mitarbeiter-Komponenten ergänzt, inkl. Multi-Value-Chip-Darstellung
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine echten Mitarbeiterdaten
- Keine funktionierende Filterlogik
- Keine Mitarbeiterdetail-Navigation (keine dokumentierte Route vorhanden)
- Kein Mitarbeiter-Erstellungsformular (nicht dokumentiert)
- Keine Benutzerkonto-, Rollen- oder Berechtigungsdaten im Mitarbeiterregister
- Keine Datenbank-Verknüpfungstabellen für Many-to-Many-Beziehungen
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Dokumentenmodul, Archivmodul oder Routing bestehender Seiten

## [0.9.10] - 07.08.2026

### Hinzugefügt

- Statische Archivübersicht auf der Seite „Archiv" unter der Route `/archiv`
- Werkzeugleiste mit Seitentitel „Archiv", Kurzbeschreibung und Archivzähler („3 archivierte Dokumente")
- Einklappbarer Filterbereich (Standard: eingeklappt) mit fünf dokumentierten Archiv-Filterkriterien: Kategorie, Unterkategorie, Verantwortliche Person, Archivierungszeitraum, Status (alle Platzhalter, nicht funktional)
- Tabellarische Desktop-Darstellung archivierter Dokumente mit dokumentierten Feldern: Dokumentennummer, Titel, Kategorie, Unterkategorie, Verantwortliche Person, Version, Archivierungsdatum, Status
- Drei klar gekennzeichnete Mock-Platzhalter-Einträge zur Veranschaulichung von Layout und Komponenten
- Wiederverwendbare Komponenten: `ArchiveToolbar`, `ArchiveFilters`, `ArchiveList`, `ArchiveRow`
- Bestehende `StatusBadge`-Komponente wiederverwendet (Variante `neutral` für „archiviert")
- Bestehende `EmptyState`-Komponente wiederverwendet („Noch keine archivierten Dokumente vorhanden")
- Archivzeilen mit Hover-State, Keyboard-Focus und `role="link"` für spätere Archivdetailansicht
- Accessibility: `aria-expanded` für einklappbare Filter, `aria-label`, `scope`, Tabellen-Semantik

### Dokumentation

- Feld `Archivierungsdatum` (Typ DateTime) in Data Dictionary und Felddefinitionen (PQM-SDD-004B) dokumentiert: automatisch beim Archivieren gesetzt, leer für aktive Dokumente, verwendet für Archiv-Sortierung und Archiv-Filterung
- UI Style Guide (005C) um Abschnitt „Archivübersicht / Archiv-Tabellenstandard" ergänzt mit verbindlichen Regeln:
  - „Das Archiv verwendet dieselbe klassische tabellarische Desktop-Darstellung wie die Dokumentenübersicht."
  - „Archivierte Dokumente werden nicht gelöscht und Dokumentnummern werden niemals erneut vergeben."
  - „Das Archiv ist kein Papierkorb."
  - Bestehende Tabellen-, Badge- und EmptyState-Komponenten werden wiederverwendet, wo immer möglich
- Component Library (005D) um Archiv-Komponenten ergänzt
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine echten Archivdaten
- Keine funktionierende Filterlogik oder Suche
- Keine Archivdetail-Navigation (keine dokumentierte Route vorhanden)
- Keine Wiederherstellungsfunktionalität – Wiederherstellung ist dokumentiert, aber nicht implementiert
- Kein Lösch-Button, keine permanente Löschaktion, kein Papierkorb-Icon
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Dokumentenübersicht, Detailseite, Formularen oder Routing bestehender Seiten

## [0.9.9] - 07.08.2026

### Hinzugefügt

- Statische Seite zum Bearbeiten eines bestehenden Dokuments unter der Route `/dokumente/:id/bearbeiten`
- `DocumentForm` wurde um den Edit-Modus erweitert (Props `documentNumber` und `pdfFileName` für edit mode)
- Bearbeiten-Seite zeigt Platzhalterwerte für ein bestehendes Dokument an (Titel, Kategorie, Version, Status, etc.)
- Dokumentnummer im Edit-Modus read-only und unveränderlich (aus Route-Parameter)
- Datei-Bereich zeigt eine Platzhalter-PDF mit „PDF ersetzen"-Button (deaktiviert)
- Zurück-Link zur Dokumentdetailansicht
- „Abbrechen" navigiert zur Detailansicht, „Änderungen speichern" ist deaktiviert

### Geändert

- `DocumentActionBar` akzeptiert jetzt eine `onEdit`-Prop, die den „Bearbeiten"-Button aktiviert und zur Bearbeitungsroute navigiert
- Dokumentdetailseite übergibt `onEdit`-Handler an `DocumentActionBar`
- Route `/dokumente/:id/bearbeiten` in App.tsx registriert

### Dokumentation

- Component Library (005D) um beide `DocumentForm`-Modi und `DocumentActionBar`-Props ergänzt
- Architektur-Regel dokumentiert: „Document creation and editing use the same DocumentForm component. Separate duplicate forms are not permitted."
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine Dateispeicherung, kein echter Datei-Upload oder -Ersatz
- Keine Speichern-Funktionalität — „Änderungen speichern" ist deaktiviert
- Keine Validierung, keine Dirty-State-Logik
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Dokumentenübersicht, Detailseite (visuell), Neue-Dokument-Formular oder Routing bestehender Seiten
- Create-Modus bleibt unverändert

## [0.9.8] - 07.08.2026

### Hinzugefügt

- Statische Seite zum Anlegen eines neuen Dokuments unter der Route `/dokumente/neu`
- Wiederverwendbare Formularkomponente `DocumentForm` mit Modus-Prop (`create` | `edit`) für spätere Wiederverwendung beim Bearbeiten
- `DocumentFormSection` als Abschnitts-Wrapper (fieldset/legend) für strukturierte Formularbereiche
- `FormField` als wiederverwendbarer Field-Wrapper mit Label, Hint und Flex-Layout
- Formularfelder ausschließlich für dokumentierte Felder: Dokumentnummer (read-only, „wird automatisch vergeben"), Titel, Kategorie, Unterkategorie, Version, Status, Verantwortliche Person, Gültig bis, Beschreibung, Tags
- Datei-Upload-Bereich mit PDF-Icon, Platzhalter-Text und deaktiviertem „PDF auswählen"-Button
- Formular-Aktionen: „Abbrechen" (navigiert zur Dokumentenübersicht) und „Dokument anlegen" (deaktiviert, Platzhalter)
- Zurück-Link zur Dokumentenübersicht
- Accessibility: labels, aria-labels, keyboard focus, disabled states

### Geändert

- Button „Neues Dokument" in der Dokumentenübersicht ist jetzt aktiv und navigiert zur Route `/dokumente/neu`
- Route `/dokumente/neu` vor `/dokumente/:id` registriert, damit der statische Pfad Vorrang hat

### Dokumentation

- UI Style Guide (005C) um Abschnitt „Dokumentformular / Formularstandard" ergänzt mit verbindlicher Regel: „Die Workflows zum Erstellen und Bearbeiten von Dokumenten müssen dieselben Formularkomponenten wiederverwenden, wo immer möglich."
- Component Library (005D) um `DocumentForm`, `DocumentFormSection`, `FormField` ergänzt; `DocumentToolbar` aktualisiert
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine Dateispeicherung, kein echter Datei-Upload
- Keine Speichern-Funktionalität — „Dokument anlegen" ist deaktiviert
- Keine Validierung, keine Filter, keine Suche
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Dokumentenübersicht oder Detailseite
- Keine neuen Mock-Daten

## [0.9.7] - 07.08.2026

### Geändert

- Dokumentenzeilen in der Dokumentenübersicht sind jetzt klickbar und navigieren zur Detailansicht (`/dokumente/{Dokumentennummer}`)
- Navigation funktioniert über Mausklick und Enter-Taste
- Zeilen haben `role="link"`, `aria-label` und Pointer-Cursor für Accessibility
- Bestehende Hover- und Focus-Stile bleiben unverändert
- Dokumentdetailseite liest den Route-Parameter (`:id`) und zeigt ihn als Dokumentennummer an
- Zurück-Link auf der Detailseite von „Zurück zur Übersicht" auf „Zurück zu Dokumenten" geändert
- Keine separaten „Öffnen"-Buttons hinzugefügt — die komplette Zeile ist das Navigationsziel

### Dokumentation

- UI Style Guide (005C) um verbindliche Navigationsregel ergänzt: „Die komplette Dokumenttabellenzeile ist das Navigationsziel zum Öffnen der Dokumentdetailansicht."
- Component Library (005D) — DocumentRow-Verhalten aktualisiert (Navigation aktiv)
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine echten Dokumentdaten
- Keine Änderungen am Tabellendesign oder den bestehenden Spalten
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Routing bestehender Seiten
- Keine Such-, Filter-, Bearbeiten- oder Archivierungsfunktionalität

## [0.9.6] - 07.08.2026

### Hinzugefügt

- Statische Dokumentdetailseite unter der Route `/dokumente/:id`
- Seitentitel mit Dokumenttitel, Dokumentennummer und Status-Badge
- Metadatenbereich mit zweispaltigem Layout für alle dokumentierten Felder: Dokumentnummer, Titel, Kategorie, Unterkategorie, Version, Status, Verantwortliche Person, Gültig bis, Letzte Änderung
- Beschreibungskarte mit Platzhaltertext
- Angehängte-Dokument-Karte mit PDF-Dateiname (Platzhalter), Dateityp-Icon und deaktiviertem „PDF öffnen"-Button
- Schlagwortliste mit Platzhalter-Tags
- Aktionsleiste mit deaktivierten Buttons: Bearbeiten, PDF öffnen, Archivieren
- Versionshistorie als einfache Timeline mit Platzhalter-Einträgen (Version 1.0 erstellt, 1.1 geändert, 1.2 freigegeben)
- Zurück-Link zur Dokumentenübersicht
- Wiederverwendbare Komponenten: `DocumentMetadata`, `TagList`, `DocumentActionBar`, `DocumentHistory`
- Bestehende `StatusBadge`-Komponente wiederverwendet
- Accessibility-Eigenschaften (role, aria-label) für alle relevanten Elemente
- Code Map und Changelog aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine echten Dokumentdaten
- Keine funktionierenden Aktionen (Bearbeiten, PDF öffnen, Archivieren)
- Keine echte Navigation aus der Dokumentenliste (Link ist statisch)
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Dokumentenübersicht, Routing bestehender Seiten
- Keine Änderungen an bestehenden Status- oder Gültigkeitsfarben

## [0.9.5] - 07.08.2026

### Geändert

- Filterbereich der Dokumentenübersicht ist jetzt einklappbar (Standard: eingeklappt) mit kompakter Schaltfläche, Filter-Icon und Pfeil-Indikator
- Dokumentenzähler von „Einträge" auf „Dokumente" geändert
- Tabellenkopf der Dokumentenliste ist sticky — Spaltenüberschriften bleiben beim vertikalen Scrollen sichtbar
- Spaltengewichtung für Desktop optimiert: Titel erhält den meisten Platz, Dokumentennummer/Status/Gültigkeit/Version kompakt, Kategorie/Unterkategorie/Verantwortlich mittel
- Dokumentenzeilen haben dezenten Hover-State und sichtbaren Keyboard-Focus (tabIndex)
- Dokumentennummern und Versionen in Monospace-Schrift dargestellt
- Kein Pointer-Cursor auf Zeilen, um keine nicht vorhandene Funktionalität vorzutäuschen

### Dokumentation

- UI Style Guide (005C) um Abschnitt „Dokumentenübersicht / Tabellenstandard" ergänzt — alle UX-Entscheidungen sind jetzt verbindlich dokumentiert
- Component Library (005D) um alle sechs Dokumenten-Komponenten mit neuem Verhalten ergänzt
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine funktionierende Filterlogik oder Suche
- Keine Datenbankanbindung
- Keine Dokumentendetailansicht oder Navigation aus Tabellenzeilen
- Keine Änderungen an Sidebar, Header, AppShell, Dashboard, Routing, bestehenden Status-/Gültigkeitsfarben, Dokumentfeldern oder Datenmodell

## [0.9.4] - 07.08.2026

### Hinzugefügt

- Statische Dokumentenübersicht auf der Seite „Dokumente“
- Werkzeugleiste mit Seitentitel, Beschreibung, deaktiviertem Suchfeld (Platzhalter) und Button „Neues Dokument“ (Platzhalter)
- Filterbereich mit fünf dokumentierten Kriterien: Kategorie, Unterkategorie, Status, Verantwortliche Person, Gültigkeit (alle als Platzhalter, nicht funktional)
- Dokumentenliste als Tabelle mit allen dokumentierten Feldern: Dokumentennummer, Titel, Kategorie, Unterkategorie, Status, Verantwortliche Person, Gültigkeit, Version
- Drei klar gekennzeichnete Mock-Platzhalter-Einträge zur Veranschaulichung von Layout und Komponenten
- Wiederverwendbare `StatusBadge`-Komponente für Status- und Gültigkeitsanzeige (success, warning, error, neutral)
- Wiederverwendbare `EmptyState`-Komponente für leere Listen
- Accessibility-Eigenschaften (role, aria-label, scope) für alle relevanten Elemente
- Code Map und Changelog aktualisiert

### Geändert

- Seite „Dokumente“ von Platzhalter auf statische Dokumentenübersicht umgestellt

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine echten Dokumentdaten
- Keine funktionierende Suche, keine funktionierenden Filter
- Kein Upload, keine Dokumenterstellung oder -bearbeitung
- Keine Archivierungs- oder Löschaktion
- Keine Rollen- oder Berechtigungslogik
- Keine Navigation aus einer Dokumentenzeile
- Keine Änderungen an Routing, Navigation, Header, Sidebar oder Dashboard

## [0.9.3] - 06.08.2026

### Geändert

- Dashboard-Karten visuell überarbeitet: größeres Icon in gefetteter Icon-Fläche, größerer Platzhalter-Wert, kompaktere Beschreibung
- Karten jetzt komplett klickbar (role=button, Tastatur-Fokus via tabIndex)
- Navigationspfeil in der unteren rechten Ecke jeder Karte, bei Hover hervorgehoben
- Subtile Hover-Animation (leichte Erhebung, Schatten, Akzent-Rahmen)
- Grid und Seitenlayout so angepasst, dass das 2×2-Raster auf einem normalen Desktop ohne vertikales Scrollen vollständig sichtbar ist
- Innenabstände reduziert, Hierarchie (Icon → Titel → Wert → Beschreibung) klarer strukturiert
- Voll responsive (zwei Spalten Desktop, eine Spalte mobil)

### Nicht enthalten (bewusst)

- Keine Navigationsfunktion hinter dem Klick (bewusst als Platzhalter belassen)
- Keine Datenbankanbindung, keine echten Daten
- Keine Änderungen an Routing, Navigation, Header oder Sidebar

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
