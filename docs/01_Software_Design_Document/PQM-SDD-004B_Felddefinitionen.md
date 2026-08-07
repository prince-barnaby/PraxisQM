# Kapitel 4B – Felddefinitionen

> **Status nach Prompt 012C:** Alle blockierenden Spezifikationslücken
> aufgelöst. Verantwortungspositionen und QM-Bereiche sind als
> normalisierte Masterdaten mit UUID-Referenzen in den Join-Tabellen
> dokumentiert. Siehe Abschnitt „Implementierungs-Readiness" am Ende
> für die Klassifizierung der Tabellen.

---

## Architekturentscheidungen (Prompt 012B)

### Primärschlüssel-Strategie

- Interne Datenbank-IDs verwenden **UUIDs**.
- Die menschenlesbare Dokumentennummer (z. B. PQM-0001) ist **nicht** der
  Datenbank-Primärschlüssel.
- Dokumentennummern bleiben eindeutig, unveränderlich und werden niemals
  wiederverwendet (ADR-001).
- Fremdschlüssel-Beziehungen referenzieren interne UUID-IDs, nicht
  Dokumentennummern.
- UUIDs werden nicht als benutzerseitige Dokumentkennzeichen angezeigt.

### Zeitstempel-Strategie

- Persistente Hauptentitäten verwenden technische Zeitstempel `created_at`
  und `updated_at`.
- `created_at` erfasst die Erstellung des Datenbankeintrags.
- `updated_at` erfasst die letzte Änderung des Datenbankeintrags.
- Diese Zeitstempel sind technische Metadaten und ersetzen **keine**
  domänenspezifischen Daten.
- Domänenspezifische Daten bleiben separat: Archivierungsdatum,
  Eintrittsdatum, Austrittsdatum, Gültigkeitsdaten, Backup-Zeitstempel,
  Versionszeitstempel.
- Reine Join-Tabellen erhalten **keine** `created_at` / `updated_at`
  Zeitstempel, es sei denn ein bestehendes Erfordernis rechtfertigt dies.

### Benutzerrollen

Genehmigte Anwendungsrollen:

- **Gast** — Lesezugriff ohne Login
- **Editor** — Bearbeitung von Dokumenten
- **Admin** — Vollzugriff

Diese Rollen sind **Software-Autorisierungsrollen**. Sie dürfen **nicht**
verwendet werden als: Mitarbeiter-Position, Verantwortungsposition, QM-Bereich
oder Dokumentkategorie.

### DB-011 Settings

DB-011 ist **reserviert / noch nicht implementiert**.

- Kein generisches Key/Value-Schema.
- Keine Persistierung von Anwendungsversion, Architektur-Informationen,
  Dokumentennummerierungs-Regeln oder Systeminformationen.
- Die Settings-UI darf Informationen und zukünftige Konfigurationsbereiche
  anzeigen, ohne dass diese Werte zu DB-011 gehören.
- SQLite-Implementierung darf nicht von DB-011 abhängen, bis konkret
  persistierte Einstellungen genehmigt sind.

### Backup- und Erinnerungsstatus

Backup-Funktionalität ist eine **spätere Implementierung**.

- Status-Enum-Werte für DB-010 Backups und DB-012 BackupReminders sind
  **nicht dokumentiert** und werden **nicht erfunden**.
- Die Felddefinitionen enthalten ein `status` Feld, dessen erlaubte Werte
  als **deferred** markiert sind.
- Die SQLite-Implementierung anderer Module wird durch die undefinierte
  Backup-Lebenszyklus nicht blockiert.

---

## DB-001 Documents

Stammdaten eines QM-Dokuments.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Dokumentennummer | document_number | String | Ja | Nein | — | Ja | Automatisch generiert | Menschenlesbare, unveränderliche Dokumentkennung (ADR-001) | Unveränderlich, wird nie wiederverwendet |
| Titel | title | String | Ja | Nein | — | Nein | — | Bezeichnung des Dokuments | Editierbar |
| Kategorie | category_id | UUID | Nein | Nein | DB-005 Categories | Nein | — | Fremdschlüssel zur Hauptkategorie | Änderbar |
| Unterkategorie | subcategory_id | UUID | Nein | Nein | DB-006 Subcategories | Nein | — | Fremdschlüssel zur Unterkategorie | Änderbar |
| Verantwortliche Person | responsible_person_id | UUID | Nein | Nein | DB-003 Employees | Nein | — | Zuständige Mitarbeitende | Änderbar |
| Version | version | String | Ja | Nein | — | Nein | — | Aktuelle Versionsnummer | Siehe DB-002 |
| Status | status | String (Enum) | Ja | Nein | — | Nein | — | Bearbeitungsstatus | Siehe Enum-Audit |
| Gültigkeit | validity | String (Enum) | Ja | Nein | — | Nein | — | Gültigkeitsstatus | Siehe Enum-Audit |
| Gültig bis | valid_until | Date | Nein | Nein | — | Nein | — | Ablaufdatum der Gültigkeit | Editierbar |
| Beschreibung | description | Text | Nein | Nein | — | Nein | — | Beschreibender Text | Editierbar |
| Archivierungsdatum | archived_at | DateTime | Nein | Nein | — | Nein | — | Automatisch gesetzt bei Archivierung. Leer für aktive Dokumente. | Wird gesetzt bei Archivierung, nicht manuell editierbar |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

### Tags

Tags werden über DB-008 DocumentTags als n:m-Beziehung zu DB-007
KeywordDictionary verwaltet. Das Feld `Tags` ist kein direktes Feld auf
DB-001, sondern eine abgeleitete n:m-Beziehung.

---

## DB-002 DocumentVersions

Alle Versionen eines Dokuments. Jede Version repräsentiert einen
Upload einer neuen Dateiversion.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Dokument | document_id | UUID | Ja | Nein | DB-001 Documents | Nein | — | Fremdschlüssel zum übergeordneten Dokument | Unveränderlich pro Version |
| Versionsnummer | version_number | String | Ja | Nein | — | Nein | — | Versionsnummer dieser Version | Unveränderlich nach Erstellung |
| Dateiname | file_name | String | Ja | Nein | — | Nein | — | Name der hochgeladenen Datei | Unveränderlich |
| Dateipfad | file_path | String | Ja | Nein | — | Nein | — | Relativer Pfad zur Datei im Dokumentenspeicher | Unveränderlich |
| Status | status | String (Enum) | Ja | Nein | — | Nein | — | Bearbeitungsstatus (gleiche Werte wie DB-001) | Siehe Enum-Audit |
| Gültigkeit | validity | String (Enum) | Ja | Nein | — | Nein | — | Gültigkeitsstatus (gleiche Werte wie DB-001) | Siehe Enum-Audit |
| Gültig bis | valid_until | Date | Nein | Nein | — | Nein | — | Ablaufdatum dieser Version | Editierbar |
| Hochgeladen von | uploaded_by | UUID | Nein | Nein | DB-004 Users | Nein | — | Benutzer, der die Version hochgeladen hat | Unveränderlich |
| Upload-Zeitpunkt | uploaded_at | DateTime | Ja | Nein | — | Nein | Automatisch | Zeitpunkt des Uploads | Unveränderlich |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

### Beziehung zu DB-001

- Kardinalität: 1 Dokument → n Versionen (1:n)
- Fremdschlüssel in DB-002 → DB-001 (`document_id`)
- Neue Versionen behalten die ursprüngliche Dokumentennummer (ADR-001)
- Archivieren statt Löschen (ADR-003)

---

## DB-003 Employees

Mitarbeiterregister der Praxis. Alle Mitarbeitenden, die als verantwortliche
Personen eintragbar sind. Getrennt von der Benutzerverwaltung (ADR-004).

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Name | last_name | String | Ja | Nein | — | Nein | — | Nachname der Mitarbeitenden | Editierbar |
| Vorname | first_name | String | Ja | Nein | — | Nein | — | Vorname der Mitarbeitenden | Editierbar |
| Position | position | String | Nein | Nein | — | Nein | — | Position / Rolle in der Praxis (z. B. Zahnärztin, ZFA) | Editierbar |
| Aktivstatus | is_active | Boolean | Ja | Nein | — | Nein | true | Aktiv / inaktiv | Editierbar |
| Eintrittsdatum | hire_date | Date | Nein | Nein | — | Nein | — | Eintrittsdatum in die Praxis | Editierbar |
| Austrittsdatum | departure_date | Date | Nein | Nein | — | Nein | — | Austrittsdatum (leer bei aktiven Mitarbeitenden) | Editierbar |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

### Entfernte Felder

- ~~Funktion~~ → umbenannt zu **Position**
- ~~Bereich~~ → entfernt
- ~~E-Mail~~ → entfernt (Mitarbeiterregister ist kein Kontaktverzeichnis)
- ~~Telefonnummer~~ → entfernt (Mitarbeiterregister ist kein Kontaktverzeichnis)

### Multi-Value-Felder

Verantwortungsposition und QM-Bereich sind **keine** direkten Felder auf
DB-003. Sie werden über Join-Tabellen verwaltet (siehe DB-013 und DB-014).

---

## DB-004 Users

Benutzerkonten mit Login und Software-Rollen. Getrennt vom
Mitarbeiterregister (ADR-004).

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Benutzername | username | String | Ja | Nein | — | Ja | — | Login-Name | Editierbar |
| Rolle | role | String (Enum) | Ja | Nein | — | Nein | — | Software-Autorisierungsrolle (Gast, Editor, Admin) | Editierbar |
| Mitarbeiter | employee_id | UUID | Nein | Nein | DB-003 Employees | Nein | — | Optionale Referenz zu einem Mitarbeiter | Editierbar |
| Aktivstatus | is_active | Boolean | Ja | Nein | — | Nein | true | Konto aktiv / inaktiv | Editierbar |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

### User ↔ Employee Beziehung

- Ein User **kann optional** genau einen Employee referenzieren.
- Ein Employee **kann ohne** User-Konto existieren.
- Employee enthält **keine** Authentifizierungsdaten.
- User-Rollen und Berechtigungen bleiben in DB-004.
- Employee-Organisationsinformationen bleiben in DB-003.
- Die Beziehung ist **optional** (ADR-004).
- Beim Erstellen eines Employees wird **kein** User automatisch erstellt.
- Beim Erstellen eines Users wird **kein** Employee automatisch erstellt.

### Authentifizierung

- Authentifizierungsdaten (Passwort, etc.) sind **nicht** in dieser
  Spezifikation definiert. Die genaue Authentifizierungsmethode wird in
  einer zukünftigen Spezifikation geklärt.
- DB-004 enthält kein Passwort-Feld in der aktuellen Spezifikation, da
  keine Authentifizierungsmethode dokumentiert ist.

---

## DB-005 Categories

Hauptkategorien für Dokumente.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Name | name | String | Ja | Nein | — | Ja | — | Kategoriename | Editierbar |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

---

## DB-006 Subcategories

Unterkategorien, jeweils einer Hauptkategorie zugeordnet.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Name | name | String | Ja | Nein | — | Nein | — | Unterkategoriename | Editierbar |
| Kategorie | category_id | UUID | Ja | Nein | DB-005 Categories | Nein | — | Fremdschlüssel zur übergeordneten Kategorie | Unveränderlich nach Erstellung |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

---

## DB-007 KeywordDictionary

Verwaltung der zulässigen Schlagwörter.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Schlagwort | keyword | String | Ja | Nein | — | Ja | — | Zulässiges Schlagwort | Editierbar |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

---

## DB-008 DocumentTags

Verknüpfung Dokument ↔ Schlagwort (n:m Join-Tabelle).

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| Dokument | document_id | UUID | Ja | Composite PK | DB-001 Documents | Nein | — | Fremdschlüssel zum Dokument | Unveränderlich nach Erstellung |
| Schlagwort | keyword_id | UUID | Ja | Composite PK | DB-007 KeywordDictionary | Nein | — | Fremdschlüssel zum Schlagwort | Unveränderlich nach Erstellung |

- Primärschlüssel: Composite Key aus (`document_id`, `keyword_id`)
- Keine `created_at` / `updated_at` (reine Join-Tabelle)

---

## DB-009 AuditLog

Protokoll wichtiger Aktionen.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Aktion | action | String | Ja | Nein | — | Nein | — | Art der protokollierten Aktion | Unveränderlich |
| Benutzer | user_id | UUID | Nein | Nein | DB-004 Users | Nein | — | Ausführender Benutzer (leer bei Gast-Aktionen) | Unveränderlich |
| Dokument | document_id | UUID | Nein | Nein | DB-001 Documents | Nein | — | Betroffenes Dokument (leer bei nicht-dokumentbezogenen Aktionen) | Unveränderlich |
| Details | details | Text | Nein | Nein | — | Nein | — | Zusätzliche Beschreibung der Aktion | Unveränderlich |
| Zeitstempel | timestamp | DateTime | Ja | Nein | — | Nein | Automatisch | Zeitpunkt der Aktion | Unveränderlich |

- `timestamp` ist der domänenspezifische Aktionszeitpunkt (nicht `created_at`).
- Kein `updated_at` (AuditLog-Einträge sind unveränderlich).
- Erlaubte Aktionswerte sind **nicht formal definiert** — siehe Enum-Audit.

---

## DB-010 Backups

Informationen über Sicherungen.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Backup-Name | backup_name | String | Ja | Nein | — | Nein | — | Bezeichnung der Sicherung | Unveränderlich |
| Dateipfad | file_path | String | Ja | Nein | — | Nein | — | Pfad zur Backup-Datei | Unveränderlich |
| Zeitstempel | backup_timestamp | DateTime | Ja | Nein | — | Nein | Automatisch | Zeitpunkt der Sicherung | Unveränderlich |
| Ausführender Benutzer | performed_by | UUID | Nein | Nein | DB-004 Users | Nein | — | Ausführender Benutzer | Unveränderlich |
| Status | status | String | Nein | Nein | — | Nein | — | Backup-Status (**deferred** — erlaubte Werte nicht dokumentiert) | Siehe Enum-Audit |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

---

## DB-011 Settings

**RESERVIERT / NOCH NICHT IMPLEMENTIERT.**

DB-011 hat keine ausreichend dokumentierten persistierten Einstellungen.

- Kein generisches Key/Value-Schema.
- Keine Persistierung von Anwendungsversion, Architektur-Informationen,
  Dokumentennummerierungs-Regeln oder Systeminformationen.
- Die Settings-UI darf Informationen anzeigen, ohne dass diese Werte zu
  DB-011 gehören.
- SQLite-Implementierung darf nicht von DB-011 abhängen.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| — | — | — | — | — | — | — | — | Reserviert für zukünftige persistierte Einstellungen | — |

---

## DB-012 BackupReminders

Verwaltung der Backup-Erinnerungen.

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Backup | backup_id | UUID | Nein | Nein | DB-010 Backups | Nein | — | Referenz zur zugehörigen Sicherung (optional) | Editierbar |
| Erinnerungsdatum | reminder_date | DateTime | Ja | Nein | — | Nein | — | Datum der Backup-Erinnerung | Editierbar |
| Status | status | String | Nein | Nein | — | Nein | — | Erinnerungsstatus (**deferred** — erlaubte Werte nicht dokumentiert) | Siehe Enum-Audit |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

---

## DB-013 EmployeeResponsibilities

Join-Tabelle: Employee ↔ Verantwortungsposition (n:m).

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| Mitarbeiter | employee_id | UUID | Ja | Composite PK | DB-003 Employees | Nein | — | Fremdschlüssel zum Mitarbeiter | Unveränderlich nach Erstellung |
| Verantwortungsposition | responsibility_id | UUID | Ja | Composite PK | DB-015 Verantwortungspositionen | Nein | — | Fremdschlüssel zur Verantwortungsposition-Masterdaten-Tabelle | Unveränderlich nach Erstellung |

- Primärschlüssel: Composite Key aus (`employee_id`, `responsibility_id`)
- Ein Mitarbeiter kann null, eine oder mehrere Verantwortungspositionen haben.
- Verantwortungspositionen sind **keine** Software-Rollen, **keine**
  Berechtigungen und **keine** Dokumentkategorien.
- Keine `created_at` / `updated_at` (reine Join-Tabelle)
- Die Join-Tabelle referenziert UUIDs der Masterdaten-Tabelle DB-015,
  keine String-Werte.

---

## DB-014 EmployeeQMAreas

Join-Tabelle: Employee ↔ QM-Bereich (n:m).

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| Mitarbeiter | employee_id | UUID | Ja | Composite PK | DB-003 Employees | Nein | — | Fremdschlüssel zum Mitarbeiter | Unveränderlich nach Erstellung |
| QM-Bereich | qm_area_id | UUID | Ja | Composite PK | DB-016 QMBereiche | Nein | — | Fremdschlüssel zur QM-Bereich-Masterdaten-Tabelle | Unveränderlich nach Erstellung |

- Primärschlüssel: Composite Key aus (`employee_id`, `qm_area_id`)
- Ein Mitarbeiter kann null, einem oder mehreren QM-Bereichen zugeordnet sein.
- QM-Bereiche sind **keine** Dokumentkategorien, **keine** Software-Rollen
  und **keine** Berechtigungen.
- Keine `created_at` / `updated_at` (reine Join-Tabelle)
- Die Join-Tabelle referenziert UUIDs der Masterdaten-Tabelle DB-016,
  keine String-Werte.

### Abgrenzung zu Dokumentkategorien

Ein QM-Bereich ist **keine** Dokumentkategorie (DB-005 Categories).
Dokumentkategorien strukturieren Dokumente. QM-Bereiche strukturieren
Mitarbeiter-Verantwortungsbereiche. Eine zukünftige Anforderung könnte
eine Abbildung zwischen QM-Bereichen und Dokumentkategorien definieren;
diese existiert aktuell nicht.

---

## DB-015 Verantwortungspositionen

Kanonische Masterdaten-Tabelle für Verantwortungspositionen.

Verantwortungspositionen sind zentral verwaltbare, wiederverwendbare
organisatorische Stammdaten. Sie unterstützen die zukünftige zentrale
Verwaltung (Erstellen, Umbenennen, mehreren Mitarbeitenden zuweisen).

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Bezeichnung | name | String | Ja | Nein | — | Ja | — | Bezeichnung der Verantwortungsposition | Editierbar |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

- Beispiele wie „Datenschutzbeauftragte" oder „Hygienebeauftragte" sind
  rein illustrativ und keine festen Werte.
- Keine zusätzlichen Metadaten-Felder, da nicht dokumentiert.
- Löschverhalten: **deferred** — nicht dokumentiert. Kein kaskadierendes
  Löschen erfunden.

---

## DB-016 QMBereiche

Kanonische Masterdaten-Tabelle für QM-Bereiche.

QM-Bereiche sind zentral verwaltbare, wiederverwendbare organisatorische
Stammdaten. Sie unterstützen die zukünftige zentrale Verwaltung (Erstellen,
Umbenennen, mehreren Mitarbeitenden zuweisen).

| Feldname | Technischer Name | Typ | Required | PK | FK | Unique | Default | Bedeutung | Lifecycle |
|---|---|---|---|---|---|---|---|---|---|
| ID | id | UUID | Ja | Ja | — | Ja | Generiert | Interner Primärschlüssel | Unveränderlich |
| Bezeichnung | name | String | Ja | Nein | — | Ja | — | Bezeichnung des QM-Bereichs | Editierbar |
| Erstellt am | created_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Erstellungszeitpunkt | Automatisch |
| Geändert am | updated_at | DateTime | Ja | Nein | — | Nein | Automatisch | Technischer Änderungszeitpunkt | Automatisch bei Änderung |

- Beispiele wie „Datenschutz" oder „Hygiene" sind rein illustrativ und
  keine festen Werte.
- Keine zusätzlichen Metadaten-Felder, da nicht dokumentiert.
- Löschverhalten: **deferred** — nicht dokumentiert. Kein kaskadierendes
  Löschen erfunden.

### Abgrenzung zu anderen Konzepten

Ein QM-Bereich ist konzeptionell getrennt von:
- Dokumentkategorien (DB-005 Categories)
- Unterkategorien (DB-006 Subcategories)
- Verantwortungspositionen (DB-015)
- Mitarbeiterpositionen (DB-003 `position`)
- Benutzerrollen (DB-004 `role`)
- Benutzerberechtigungen

Es wird **keine** Abbildung zwischen QM-Bereichen und Dokumentkategorien
definiert, es sei denn eine zukünftige Anforderung fordert dies explizit.

---

## Beziehungsspezifikation

| Beziehung | Kardinalität | FK-Richtung | Lösch-/Archivierungsverhalten |
|---|---|---|---|
| DB-001 ↔ DB-002 | 1:n | FK in DB-002 → DB-001 | Archivieren statt Löschen (ADR-003). Neue Versionen behalten Dokumentennummer (ADR-001). |
| DB-001 ↔ DB-005 | n:1 | FK in DB-001 → DB-005 | Nicht dokumentiert. Kein kaskadierendes Löschen. |
| DB-005 ↔ DB-006 | 1:n | FK in DB-006 → DB-005 | Nicht dokumentiert. Kein kaskadierendes Löschen. |
| DB-001 ↔ DB-007 (via DB-008) | n:m | Join-Tabelle DB-008 | Nicht dokumentiert. |
| DB-003 ↔ DB-004 | optional 1:1 | FK in DB-004 → DB-003 | ADR-004: Konzepte sind getrennt. Beziehung ist optional. Kein kaskadierendes Löschen. |
| DB-003 ↔ DB-015 (via DB-013) | n:m | Join-Tabelle DB-013 (FK → DB-003, FK → DB-015) | Kein kaskadierendes Löschen dokumentiert. |
| DB-003 ↔ DB-016 (via DB-014) | n:m | Join-Tabelle DB-014 (FK → DB-003, FK → DB-016) | Kein kaskadierendes Löschen dokumentiert. |
| DB-009 ↔ DB-004 | n:1 | FK in DB-009 → DB-004 | AuditLog-Einträge sind unveränderlich. |
| DB-009 ↔ DB-001 | n:1 | FK in DB-009 → DB-001 | AuditLog-Einträge sind unveränderlich. |
| DB-010 ↔ DB-012 | 1:n | FK in DB-012 → DB-010 | Nicht dokumentiert. |
| DB-010 ↔ DB-004 | n:1 | FK in DB-010 → DB-004 | Nicht dokumentiert. |
| DB-002 ↔ DB-004 | n:1 | FK in DB-002 → DB-004 | Nicht dokumentiert. |
| DB-001 ↔ DB-003 | n:1 | FK in DB-001 → DB-003 | Verantwortliche Person. Nicht dokumentiert. |

### Archivierungsverhalten

- ADR-003: Dokumente werden archiviert, nicht gelöscht.
- Archivierungsdatum (`archived_at`) wird automatisch gesetzt bei
  Archivierung. Leer für aktive Dokumente.
- Archivierte Dokumente verschwinden aus der Gast-Suche.
- Admins können archivierte Dokumente einsehen.
- Wiederherstellung bleibt möglich.
- Kein kaskadierendes Löschen für andere Tabellen dokumentiert.

---

## Enum / Status-Audit

| Feld | Tabelle | Erlaubte Werte | Status |
|---|---|---|---|
| Status | DB-001 Documents | Entwurf, aktiv, archiviert | Dokumentiert |
| Gültigkeit | DB-001 Documents | gültig, läuft bald ab, abgelaufen | Dokumentiert |
| Status | DB-002 DocumentVersions | Entwurf, aktiv, archiviert (gleiche Werte wie DB-001) | Dokumentiert |
| Gültigkeit | DB-002 DocumentVersions | gültig, läuft bald ab, abgelaufen (gleiche Werte wie DB-001) | Dokumentiert |
| Aktivstatus | DB-003 Employees | true / false (Boolean) | Dokumentiert |
| Rolle | DB-004 Users | Gast, Editor, Admin | Dokumentiert (Prompt 012B) |
| Aktivstatus | DB-004 Users | true / false (Boolean) | Dokumentiert |
| Aktion | DB-009 AuditLog | **Nicht formal definiert** | **Deferred** — nicht blockierend für initiale SQLite-Foundation |
| Status | DB-010 Backups | **Nicht dokumentiert** | **Deferred** — nicht blockierend für initiale SQLite-Foundation |
| Status | DB-012 BackupReminders | **Nicht dokumentiert** | **Deferred** — nicht blockierend für initiale SQLite-Foundation |

---

## Identifikatoren und Zeitstempel

| Konzept | Entscheidung | Status |
|---|---|---|
| Primärschlüssel-Strategie | UUID für alle Haupttabellen; Composite Key für Join-Tabellen | Dokumentiert (Prompt 012B) |
| Dokumentennummer vs. interne ID | Dokumentennummer ist Unique-Feld, nicht Primärschlüssel. Interne UUID ist Primärschlüssel. | Dokumentiert (Prompt 012B) |
| created_at / updated_at | Technische Zeitstempel auf allen Haupttabellen. Nicht auf reinen Join-Tabellen. | Dokumentiert (Prompt 012B) |
| Archivierungszeitstempel | `archived_at` (DateTime), domänenspezifisch, nur auf DB-001 | Dokumentiert |
| Versionsnummer | String, neue Versionen behalten ursprüngliche Dokumentennummer (ADR-001) | Dokumentiert |
| Upload-Zeitpunkt | `uploaded_at` (DateTime), domänenspezifisch auf DB-002 | Dokumentiert |
| Aktionszeitstempel | `timestamp` (DateTime), domänenspezifisch auf DB-009 | Dokumentiert |
| Backup-Zeitstempel | `backup_timestamp` (DateTime), domänenspezifisch auf DB-010 | Dokumentiert |

---

## Implementierungs-Readiness-Klassifizierung

| Tabelle | Klasse | Begründung |
|---|---|---|
| DB-001 Documents | **A** | Kern-Dokumentenverwaltung. Erforderlich für initiale SQLite-Foundation. |
| DB-002 DocumentVersions | **A** | Versionierung ist Kernfunktionalität. Erforderlich für initiale SQLite-Foundation. |
| DB-003 Employees | **A** | Mitarbeiterregister ist erforderlich für Verantwortlichkeitszuordnung in Dokumenten. |
| DB-004 Users | **A** | Benutzerverwaltung ist erforderlich für Authentifizierung und Autorisierung. |
| DB-005 Categories | **A** | Kategorien sind erforderlich für Dokumentenklassifizierung. |
| DB-006 Subcategories | **A** | Unterkategorien sind erforderlich für Dokumentenklassifizierung. |
| DB-007 KeywordDictionary | **A** | Schlagwörter sind erforderlich für Dokumentenverschlagwortung. |
| DB-008 DocumentTags | **A** | Join-Tabelle für Dokument-Tags. Erforderlich für n:m-Beziehung. |
| DB-009 AuditLog | **A** | Protokollierung ist Kernfunktionalität (SDD-003). |
| DB-010 Backups | **B** | Backup-Funktionalität ist spätere Implementierung. Status-Enum deferred. |
| DB-011 Settings | **C** | Reserviert. Keine persistierten Einstellungen dokumentiert. |
| DB-012 BackupReminders | **B** | Backup-Erinnerungen sind spätere Implementierung. Status-Enum deferred. |
| DB-013 EmployeeResponsibilities | **A** | Join-Tabelle für Mitarbeiter-Verantwortungspositionen. Erforderlich für n:m-Beziehung. |
| DB-014 EmployeeQMAreas | **A** | Join-Tabelle für Mitarbeiter-QM-Bereiche. Erforderlich für n:m-Beziehung. |
| DB-015 Verantwortungspositionen | **A** | Kanonische Masterdaten für Verantwortungspositionen. Erforderlich für normalisierte n:m-Beziehung. |
| DB-016 QMBereiche | **A** | Kanonische Masterdaten für QM-Bereiche. Erforderlich für normalisierte n:m-Beziehung. |

### Klassifizierung

- **A — Erforderlich für initiale SQLite-Foundation:** DB-001, DB-002, DB-003, DB-004, DB-005, DB-006, DB-007, DB-008, DB-009, DB-013, DB-014, DB-015, DB-016
- **B — Dokumentiert, Implementierung kann verschoben werden:** DB-010, DB-012
- **C — Reserviert / zukünftige Spezifikation:** DB-011

---

## Konsistenz-Audit

- [x] UUID-Strategie ist konsistent für alle Haupttabellen
- [x] Dokumentennummern bleiben unabhängig von Datenbank-IDs
- [x] Employee-Multi-Value-Beziehungen sind normalisiert (DB-013 → DB-015, DB-014 → DB-016)
- [x] Verantwortungspositionen und QM-Bereiche sind kanonische Masterdaten mit UUID-PK
- [x] Join-Tabellen referenzieren UUIDs, keine String-Werte
- [x] Namen existieren nur in ihren kanonischen Masterdaten-Entitäten
- [x] User und Employee bleiben getrennt (ADR-004)
- [x] Gast / Editor / Admin sind nur Software-Rollen
- [x] Archivierungsdatum bleibt dokumentiert
- [x] Keine komma-separierten relationalen Werte
- [x] DB-011 enthält keine erfundenen Einstellungen
- [x] Keine erfundenen Backup-Lebenszykluswerte
- [x] Keine Cloud-Abhängigkeit vorhanden
- [x] SQLite-Design bleibt vollständig offline-fähig

---

## Verbleibende ungelöste Felder

| Feld | Tabelle | Status | Blockierend für initiale Foundation? |
|---|---|---|---|
| Authentifizierungsdaten | DB-004 Users | Nicht spezifiziert (keine Methode dokumentiert) | **Nein** — das `employee_id` Feld und die Rolle sind definiert. Authentifizierung kann später hinzugefügt werden. |
| Aktion-Enum-Werte | DB-009 AuditLog | Deferred | **Nein** — `action` als String ist ausreichend für initiale Foundation. |
| Backup-Status-Enum | DB-010 Backups | Deferred | **Nein** — DB-010 ist Klasse B. |
| Erinnerungsstatus-Enum | DB-012 BackupReminders | Deferred | **Nein** — DB-012 ist Klasse B. |
| DB-011 Settings-Inhalt | DB-011 Settings | Reserviert | **Nein** — DB-011 ist Klasse C. |
| Löschverhalten DB-015 Verantwortungspositionen | DB-015 | Deferred | **Nein** — Löschverhalten kann später definiert werden. |
| Löschverhalten DB-016 QMBereiche | DB-016 | Deferred | **Nein** — Löschverhalten kann später definiert werden. |
