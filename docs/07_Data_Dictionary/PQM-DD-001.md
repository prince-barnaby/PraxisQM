# Data Dictionary

> **Status nach Prompt 012C:** Alle blockierenden Spezifikationslücken
> aufgelöst. Verantwortungspositionen und QM-Bereiche sind als
> normalisierte Masterdaten (DB-015, DB-016) mit UUID-Referenzen in
> den Join-Tabellen (DB-013, DB-014) dokumentiert.
> Siehe PQM-SDD-004B für die vollständigen Felddefinitionen,
> Beziehungsspezifikation, Enum-Audit und Implementierungs-Readiness.

## Architekturentscheidungen

- **Primärschlüssel:** UUID für alle Haupttabellen; Composite Key für Join-Tabellen.
- **Dokumentennummer:** Unique-Feld, nicht Primärschlüssel. Unveränderlich (ADR-001).
- **Zeitstempel:** `created_at` / `updated_at` auf allen Haupttabellen. Nicht auf reinen Join-Tabellen.
- **Benutzerrollen:** Gast, Editor, Admin (Software-Autorisierungsrollen).
- **DB-011 Settings:** Reserviert / noch nicht implementiert.

## DB-001 Documents

Stammdaten eines QM-Dokuments.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Dokumentennummer | document_number | String | Ja | Nein | — | Ja | Menschenlesbare, unveränderliche Kennung (ADR-001) |
| Titel | title | String | Ja | — | — | — | Bezeichnung |
| Kategorie | category_id | UUID | Nein | — | DB-005 | — | Hauptkategorie |
| Unterkategorie | subcategory_id | UUID | Nein | — | DB-006 | — | Unterkategorie |
| Verantwortliche Person | responsible_person_id | UUID | Nein | — | DB-003 | — | Zuständige Mitarbeitenden |
| Version | version | String | Ja | — | — | — | Aktuelle Versionsnummer |
| Status | status | String (Enum) | Ja | — | — | — | Entwurf, aktiv, archiviert |
| Gültigkeit | validity | String (Enum) | Ja | — | — | — | gültig, läuft bald ab, abgelaufen |
| Gültig bis | valid_until | Date | Nein | — | — | — | Ablaufdatum |
| Beschreibung | description | Text | Nein | — | — | — | Beschreibender Text |
| Archivierungsdatum | archived_at | DateTime | Nein | — | — | — | Automatisch bei Archivierung. Leer für aktive. |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

Tags werden über DB-008 (n:m) verwaltet.

## DB-002 DocumentVersions

Alle Versionen eines Dokuments.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Dokument | document_id | UUID | Ja | — | DB-001 | — | Übergeordnetes Dokument |
| Versionsnummer | version_number | String | Ja | — | — | — | Versionsnummer |
| Dateiname | file_name | String | Ja | — | — | — | Name der hochgeladenen Datei |
| Dateipfad | file_path | String | Ja | — | — | — | Pfad im Dokumentenspeicher |
| Status | status | String (Enum) | Ja | — | — | — | Entwurf, aktiv, archiviert |
| Gültigkeit | validity | String (Enum) | Ja | — | — | — | gültig, läuft bald ab, abgelaufen |
| Gültig bis | valid_until | Date | Nein | — | — | — | Ablaufdatum dieser Version |
| Hochgeladen von | uploaded_by | UUID | Nein | — | DB-004 | — | Hochladender Benutzer |
| Upload-Zeitpunkt | uploaded_at | DateTime | Ja | — | — | — | Zeitpunkt des Uploads |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

## DB-003 Employees

Mitarbeiterregister der Praxis. Getrennt von Benutzerverwaltung (ADR-004).

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Name | last_name | String | Ja | — | — | — | Nachname |
| Vorname | first_name | String | Ja | — | — | — | Vorname |
| Position | position | String | Nein | — | — | — | Position in der Praxis |
| Aktivstatus | is_active | Boolean | Ja | — | — | — | Aktiv / inaktiv |
| Eintrittsdatum | hire_date | Date | Nein | — | — | — | Eintrittsdatum |
| Austrittsdatum | departure_date | Date | Nein | — | — | — | Austrittsdatum (leer bei aktiven) |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

Verantwortungsposition und QM-Bereich werden über DB-013/DB-015 und DB-014/DB-016 verwaltet (normalisierte n:m-Beziehungen mit UUID-Referenzen).

### Entfernte Felder

- ~~Funktion~~ → **Position**
- ~~Bereich~~ → entfernt
- ~~E-Mail~~ → entfernt
- ~~Telefonnummer~~ → entfernt

### Begriffliche Trennung

| Begriff | Bedeutung | Tabelle |
|---|---|---|
| Position | Berufliche Rolle in der Praxis | DB-003 |
| Verantwortungsposition | QM-Verantwortung (Masterdaten) | DB-015 |
| QM-Bereich | Qualitätsmanagementbereich (Masterdaten) | DB-016 |
| Benutzerrolle | Software-Rolle (Gast, Editor, Admin) | DB-004 |
| Benutzerberechtigung | Einzelberechtigung | DB-004 |
| Dokumentkategorie | Dokument-Klassifizierung | DB-005 |

Eine Verantwortungsposition ist **keine** Software-Rolle.
Ein QM-Bereich ist **keine** Berechtigung und **keine** Dokumentkategorie.

## DB-004 Users

Benutzerkonten mit Login und Software-Rollen. Getrennt vom Mitarbeiterregister (ADR-004).

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Benutzername | username | String | Ja | — | — | Ja | Login-Name |
| Rolle | role | String (Enum) | Ja | — | — | — | Gast, Editor, Admin |
| Mitarbeiter | employee_id | UUID | Nein | — | DB-003 | — | Optionale Referenz zu Mitarbeiter |
| Aktivstatus | is_active | Boolean | Ja | — | — | — | Konto aktiv / inaktiv |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

### User ↔ Employee (ADR-004)

- Ein User kann optional genau einen Employee referenzieren.
- Ein Employee kann ohne User existieren.
- Employee enthält keine Authentifizierungsdaten.
- Die Beziehung ist optional.
- Kein automatisches Erstellen von User/Employee.

### Authentifizierung

Authentifizierungsdaten sind nicht spezifiziert. Die Methode wird zukünftig geklärt.

## DB-005 Categories

Hauptkategorien für Dokumente.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Name | name | String | Ja | — | — | Ja | Kategoriename |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

## DB-006 Subcategories

Unterkategorien, jeweils einer Hauptkategorie zugeordnet.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Name | name | String | Ja | — | — | — | Unterkategoriename |
| Kategorie | category_id | UUID | Ja | — | DB-005 | — | Übergeordnete Kategorie |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

## DB-007 KeywordDictionary

Zulässige Schlagwörter.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Schlagwort | keyword | String | Ja | — | — | Ja | Zulässiges Schlagwort |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

## DB-008 DocumentTags

Join-Tabelle: Dokument ↔ Schlagwort (n:m).

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| Dokument | document_id | UUID | Ja | Composite | DB-001 | — | Dokument |
| Schlagwort | keyword_id | UUID | Ja | Composite | DB-007 | — | Schlagwort |

Composite PK: (`document_id`, `keyword_id`). Keine Zeitstempel (reine Join-Tabelle).

## DB-009 AuditLog

Protokoll wichtiger Aktionen.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Aktion | action | String | Ja | — | — | — | Art der Aktion (Werte deferred) |
| Benutzer | user_id | UUID | Nein | — | DB-004 | — | Ausführender Benutzer |
| Dokument | document_id | UUID | Nein | — | DB-001 | — | Betroffenes Dokument |
| Details | details | Text | Nein | — | — | — | Zusätzliche Beschreibung |
| Zeitstempel | timestamp | DateTime | Ja | — | — | — | Zeitpunkt der Aktion |

Kein `updated_at` (Einträge unveränderlich). `timestamp` ist domänenspezifisch.

## DB-010 Backups

Informationen über Sicherungen. **Klasse B — Implementierung verschoben.**

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Backup-Name | backup_name | String | Ja | — | — | — | Bezeichnung der Sicherung |
| Dateipfad | file_path | String | Ja | — | — | — | Pfad zur Backup-Datei |
| Zeitstempel | backup_timestamp | DateTime | Ja | — | — | — | Zeitpunkt der Sicherung |
| Ausführender Benutzer | performed_by | UUID | Nein | — | DB-004 | — | Ausführender Benutzer |
| Status | status | String | Nein | — | — | — | Backup-Status (deferred) |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

## DB-011 Settings

**RESERVIERT / NOCH NICHT IMPLEMENTIERT.** Klasse C.

Keine persistierten Einstellungen dokumentiert. Kein Key/Value-Schema.
SQLite-Implementierung darf nicht von DB-011 abhängen.

## DB-012 BackupReminders

Verwaltung der Backup-Erinnerungen. **Klasse B — Implementierung verschoben.**

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Backup | backup_id | UUID | Nein | — | DB-010 | — | Referenz zur Sicherung |
| Erinnerungsdatum | reminder_date | DateTime | Ja | — | — | — | Datum der Erinnerung |
| Status | status | String | Nein | — | — | — | Erinnerungsstatus (deferred) |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

## DB-013 EmployeeResponsibilities

Join-Tabelle: Mitarbeiter ↔ Verantwortungsposition (n:m). Referenziert UUIDs der Masterdaten-Tabelle DB-015.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| Mitarbeiter | employee_id | UUID | Ja | Composite | DB-003 | — | Mitarbeiter |
| Verantwortungsposition | responsibility_id | UUID | Ja | Composite | DB-015 | — | Verantwortungsposition (UUID-Referenz) |

Composite PK: (`employee_id`, `responsibility_id`). Keine Zeitstempel (reine Join-Tabelle).

## DB-014 EmployeeQMAreas

Join-Tabelle: Mitarbeiter ↔ QM-Bereich (n:m). Referenziert UUIDs der Masterdaten-Tabelle DB-016.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| Mitarbeiter | employee_id | UUID | Ja | Composite | DB-003 | — | Mitarbeiter |
| QM-Bereich | qm_area_id | UUID | Ja | Composite | DB-016 | — | QM-Bereich (UUID-Referenz) |

Composite PK: (`employee_id`, `qm_area_id`). Keine Zeitstempel.

## DB-015 Verantwortungspositionen

Kanonische Masterdaten für Verantwortungspositionen. Zentral verwaltbare,
wiederverwendbare organisatorische Stammdaten.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Bezeichnung | name | String | Ja | — | — | Ja | Bezeichnung der Verantwortungsposition |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

Löschverhalten: deferred — nicht dokumentiert.

## DB-016 QMBereiche

Kanonische Masterdaten für QM-Bereiche. Zentral verwaltbare,
wiederverwendbare organisatorische Stammdaten.

| Feld | Technischer Name | Typ | Required | PK | FK | Unique | Bedeutung |
|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Interner Primärschlüssel |
| Bezeichnung | name | String | Ja | — | — | Ja | Bezeichnung des QM-Bereichs |
| Erstellt am | created_at | DateTime | Ja | — | — | — | Technisch |
| Geändert am | updated_at | DateTime | Ja | — | — | — | Technisch |

Löschverhalten: deferred — nicht dokumentiert. QM-Bereiche bleiben
konzeptionell getrennt von Dokumentkategorien (DB-005), Unterkategorien
(DB-006), Verantwortungspositionen (DB-015), Mitarbeiterpositionen (DB-003
`position`), Benutzerrollen (DB-004 `role`) und Benutzerberechtigungen.

## Beziehungsspezifikation

Siehe PQM-SDD-004B für die vollständige Beziehungsspezifikation.

## Enum / Status-Audit

Siehe PQM-SDD-004B für den vollständigen Enum-Audit.

## Implementierungs-Readiness

Siehe PQM-SDD-004B für die Klassifizierung (A / B / C).
