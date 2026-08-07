# Kapitel 4B

## DB-001 Documents – Felddefinitionen

| Feld | Typ | Beschreibung |
|---|---|---|
| Dokumentennummer | String | Automatisch vergeben, eindeutig, unveränderlich, wird niemals wiederverwendet (ADR-001) |
| Titel | String | Bezeichnung des Dokuments |
| Kategorie | String | Hauptkategorie |
| Unterkategorie | String | Unterkategorie |
| Verantwortliche Person | String | Zuständige Mitarbeitenden |
| Version | String | Versionsnummer |
| Status | String | Bearbeitungsstatus (Entwurf, aktiv, archiviert) |
| Gültigkeit | String | Gültigkeitsstatus (gültig, läuft bald ab, abgelaufen) |
| Gültig bis | Date | Ablaufdatum der Gültigkeit |
| Beschreibung | Text | Beschreibender Text |
| Tags | Liste<String> | Schlagwörter |
| Archivierungsdatum | DateTime | Automatisch gesetzt, wenn ein Dokument archiviert wird. Leer für aktive Dokumente. Wird für Archiv-Sortierung und Archiv-Filterung (Archivierungszeitraum) verwendet. |

## DB-002 DocumentVersions – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Aus den vorhandenen Dokumenten lassen sich folgende Felder ableiten,
> aber die vollständige Definition ist nicht ausreichend dokumentiert.

| Feld | Typ | Beschreibung |
|---|---|---|
| Dokumentennummer | String | Fremdschlüssel zu DB-001 Documents. Neue Versionen behalten die ursprüngliche Dokumentennummer (ADR-001). |
| Version | String | Versionsnummer dieser Version |
| Status | String | Bearbeitungsstatus (Entwurf, aktiv, archiviert) |
| Gültigkeit | String | Gültigkeitsstatus (gültig, läuft bald ab, abgelaufen) |
| Gültig bis | Date | Ablaufdatum der Gültigkeit |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Dateiname / Dateipfad (nicht dokumentiert)
- Upload-Zeitpunkt (nicht dokumentiert)
- Hochladende Person (nicht dokumentiert)
- Beziehung zu DB-001 (Kardinalität dokumentiert als 1:n, aber Fremdschlüssel-Strategie nicht dokumentiert)

## DB-003 Employees – Felddefinitionen

| Feld | Typ | Beschreibung |
|---|---|---|
| Name | String | Nachname der Mitarbeitenden |
| Vorname | String | Vorname der Mitarbeitenden |
| Position | String | Position / Rolle in der Praxis (z. B. Zahnärztin, ZFA, Praxismanagerin) |
| Verantwortungsposition | Liste<String> | Multi-Value-Feld. Null, eine oder mehrere Verantwortungspositionen (z. B. QM-Beauftragte, Datenschutzbeauftragte). Many-to-many-Beziehung: employee ↔ Verantwortungsposition. |
| Zugeordneter QM-Bereich | Liste<String> | Multi-Value-Feld. Null, ein oder mehrere zugeordnete QM-Bereiche (z. B. Datenschutz, Patientendokumentation, Röntgeneinweisung). Many-to-many-Beziehung: employee ↔ QM-Bereich. |
| Aktivstatus | Boolean | Aktiv / inaktiv. Wird über StatusBadge dargestellt (success = aktiv, neutral = inaktiv). |
| Eintrittsdatum | Date | Eintrittsdatum in die Praxis |
| Austrittsdatum | Date | Austrittsdatum aus der Praxis (leer / „—" bei aktiven Mitarbeitenden) |

### Entfernte Felder

- ~~Funktion~~ → umbenannt zu **Position** (entspricht aktueller UI-Implementierung)
- ~~Bereich~~ → entfernt (nicht Teil des dokumentierten Mitarbeitermodells)
- ~~E-Mail~~ → entfernt (Mitarbeiterregister ist kein Kontaktverzeichnis)
- ~~Telefonnummer~~ → entfernt (Mitarbeiterregister ist kein Kontaktverzeichnis)

### Multi-Value-Felder – Datenmodellregel

Die Felder **Verantwortungsposition** und **Zugeordneter QM-Bereich** sind Multi-Value-Felder.

- Ein Mitarbeiter kann **null, eine oder mehrere** Verantwortungspositionen haben.
- Ein Mitarbeiter kann **null, einem oder mehreren** QM-Bereichen zugeordnet sein.
- Beide Felder dürfen **nicht** als einzelner String oder als Single-Select-Wert modelliert werden.
- Das zukünftige DB-003 Employees-Datenmodell muss **Many-to-Many-Zuweisungen** unterstützen für:
  - employee ↔ Verantwortungsposition
  - employee ↔ QM-Bereich
- Datenbank-Verknüpfungstabellen (Join-Tables) werden noch nicht implementiert.
- In der aktuellen UI-Only-Implementierung werden mehrere Werte als klar getrennte Badges/Chips dargestellt.
- Die Platzhalterwerte sind Beispiele und keine festen Geschäftsregeln.

### Strukturelle Lücke – Many-to-Many (BLOCKIEREND)

Die aktuelle Schema-Dokumentation modelliert Verantwortungsposition und QM-Bereich
als `Liste<String>` auf DB-003. Dies ist eine UI-Ebene-Beschreibung, keine
relationale Modellierung.

Für eine korrekte SQLite-Implementierung werden zwei zusätzliche
Verknüpfungstabellen benötigt:

1. **DB-003a EmployeeResponsibilities** — employee ↔ Verantwortungsposition (n:m)
2. **DB-003b EmployeeQMAreas** — employee ↔ QM-Bereich (n:m)

**Begründung:** Die bestehende Schema-Dokumentation ist unzureichend, weil
`Liste<String>` in einer relationalen Datenbank nicht als Feldtyp existiert.
Die Many-to-Many-Beziehung erfordert Join-Tabellen.

**Minimaler Vorschlag:**
- DB-003a: `employee_id` (FK → DB-003), `responsibility_role` (String)
- DB-003b: `employee_id` (FK → DB-003), `qm_area` (String)

Diese Tabellen sind **vorgeschlagen** und müssen vor der SQLite-Implementierung
freigegeben werden.

## DB-004 Users – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Aus ADR-004 und SDD-004A lassen sich folgende Felder ableiten:

| Feld | Typ | Beschreibung |
|---|---|---|
| Benutzername | String | Login-Name (nicht explizit dokumentiert, aber aus „Login" ableitbar) |
| Rolle | String | Benutzerrolle (nicht explizit dokumentiert) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Referenz zu DB-003 Employees (ADR-004 erwähnt Trennung, aber optionale Referenz nicht dokumentiert)
- Passwort / Authentifizierungsdaten (nicht dokumentiert)
- Zugehörige Mitarbeiter-ID (ob ein Benutzerkonto einem Mitarbeiter zugeordnet ist, ist nicht dokumentiert)
- Erstellungszeitpunkt (nicht dokumentiert)
- Aktivstatus (nicht dokumentiert)

### Trennung von Mitarbeiterregister und Benutzerverwaltung (ADR-004)

- DB-003 Employees und DB-004 Users sind **getrennte Tabellen**.
- Nicht jeder Mitarbeiter benötigt ein Benutzerkonto.
- Das Mitarbeiterregister enthält **keine** Authentifizierungsdaten.
- Benutzerrollen und Berechtigungen sind **keine** Mitarbeiter-Verantwortungspositionen.
- Eine eventuelle Referenz zwischen User und Employee ist **optional** und
  **nicht dokumentiert** — muss vor SQLite-Implementierung geklärt werden.

## DB-005 Categories – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.

| Feld | Typ | Beschreibung |
|---|---|---|
| Kategorie | String | Hauptkategoriename (aus DB-001 ableitbar) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Beschreibung (nicht dokumentiert)
- Sortierung / Reihenfolge (nicht dokumentiert)
- Erstellungs-/Änderungszeitpunkt (nicht dokumentiert)

## DB-006 Subcategories – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.

| Feld | Typ | Beschreibung |
|---|---|---|
| Unterkategorie | String | Unterkategoriename (aus DB-001 ableitbar) |
| Kategorie | String | Fremdschlüssel zu DB-005 Categories (ableitbar) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Fremdschlüssel-Strategie zu DB-005 (Kardinalität 1:n ableitbar, aber FK-Feld nicht dokumentiert)
- Sortierung / Reihenfolge (nicht dokumentiert)
- Erstellungs-/Änderungszeitpunkt (nicht dokumentiert)

## DB-007 KeywordDictionary – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.

| Feld | Typ | Beschreibung |
|---|---|---|
| Schlagwort | String | Zulässiges Schlagwort (aus SDD-004A ableitbar) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Status (aktiv / inaktiv — nicht dokumentiert)
- Erstellungs-/Änderungszeitpunkt (nicht dokumentiert)

## DB-008 DocumentTags – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.

| Feld | Typ | Beschreibung |
|---|---|---|
| Dokument | String | Fremdschlüssel zu DB-001 Documents (ableitbar) |
| Schlagwort | String | Fremdschlüssel zu DB-007 KeywordDictionary (ableitbar) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Fremdschlüssel-Strategie (Kardinalität n:m ableitbar, aber FK-Felder nicht dokumentiert)
- Composite Key vs. Surrogate Key (nicht dokumentiert)

## DB-009 AuditLog – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Aus SDD-003 („Aktion wird protokolliert") ableitbar:

| Feld | Typ | Beschreibung |
|---|---|---|
| Aktion | String | Art der protokollierten Aktion (ableitbar) |
| Zeitstempel | DateTime | Zeitpunkt der Aktion (ableitbar) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Referenz zu DB-004 Users (welcher Benutzer hat die Aktion ausgeführt — nicht dokumentiert)
- Referenz zu DB-001 Documents (welches Dokument betroffen ist — nicht dokumentiert)
- Details / Beschreibung der Aktion (nicht dokumentiert)
- Art der protokollierten Aktionen (nicht definiert)

## DB-010 Backups – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.

| Feld | Typ | Beschreibung |
|---|---|---|
| Backup-Name | String | Bezeichnung der Sicherung (ableitbar) |
| Zeitstempel | DateTime | Zeitpunkt der Sicherung (ableitbar) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Dateipfad / Speicherort (nicht dokumentiert)
- Backup-Status (erfolgreich / fehlgeschlagen — nicht dokumentiert)
- Größe (nicht dokumentiert)
- Ausführender Benutzer (nicht dokumentiert)

## DB-011 Settings – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> DB-011 existiert nur als Tabellenname in SDD-004A.

Es konnten keine konkret dokumentierten, persistierten Einstellungen gefunden werden.

Die Einstellungsoberfläche (Prompt 011) definiert sechs UI-Bereiche:
Allgemein, Dokumentennummerierung, Kategorien, Benutzerverwaltung, Backup,
Systeminformationen. Diese sind jedoch **UI-Bereiche**, keine DB-011-Einträge.

- **Dokumentennummerierung** — Architekturkonstante (ADR-001), keine editierbare Einstellung.
- **Systeminformationen** — Anwendungs-Metadaten (Version, Architektur), keine persistierten Einstellungen.
- **Kategorien** — Verwaltung über DB-005/DB-006, nicht über DB-011.
- **Benutzerverwaltung** — Verwaltung über DB-004, nicht über DB-011.
- **Backup** — Konfiguration über DB-010/DB-012, nicht über DB-011.
- **Allgemein** — Keine konkret dokumentierten Einstellungen gefunden.

**Bewertung:** DB-011 ist **unresolved**. Es ist nicht ausreichend dokumentiert,
welche Werte tatsächlich in DB-011 gespeichert werden sollen. DB-011 darf
nicht mit erfundenen Key/Value-Paaren aufgefüllt werden.

## DB-012 BackupReminders – Felddefinitionen

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.

| Feld | Typ | Beschreibung |
|---|---|---|
| Erinnerungsdatum | DateTime | Datum der Backup-Erinnerung (ableitbar) |

**Nicht aufgelöste Felder:**
- Interner Primärschlüssel (ID-Strategie nicht dokumentiert)
- Status (erledigt / offen — nicht dokumentiert)
- Referenz zu DB-010 Backups (nicht dokumentiert)
- Wiederholungsintervall (nicht dokumentiert)
- Erstellungszeitpunkt (nicht dokumentiert)

## Beziehungsspezifikation

### Dokumentierte Beziehungen

| Beziehung | Kardinalität | Fremdschlüssel-Richtung | Lösch-/Archivierungsverhalten |
|---|---|---|---|
| DB-001 Documents ↔ DB-002 DocumentVersions | 1:n | FK in DB-002 → DB-001 | Archivieren statt Löschen (ADR-003). Neue Versionen behalten Dokumentennummer (ADR-001). |
| DB-001 Documents ↔ DB-005 Categories | n:1 | FK in DB-001 → DB-005 | Nicht dokumentiert. |
| DB-005 Categories ↔ DB-006 Subcategories | 1:n | FK in DB-006 → DB-005 | Nicht dokumentiert. |
| DB-001 Documents ↔ DB-007 Tags (via DB-008) | n:m | Join-Tabelle DB-008 | Nicht dokumentiert. |
| DB-003 Employees ↔ DB-004 Users | optional 1:1 | Nicht dokumentiert | ADR-004: Konzepte sind getrennt. Referenz ist optional. |
| DB-003 Employees ↔ Verantwortungsposition | n:m | Join-Tabelle erforderlich (DB-003a vorgeschlagen) | Nicht dokumentiert. |
| DB-003 Employees ↔ QM-Bereich | n:m | Join-Tabelle erforderlich (DB-003b vorgeschlagen) | Nicht dokumentiert. |
| DB-009 AuditLog ↔ DB-004 Users | n:1 | FK in DB-009 → DB-004 (vermutet) | Nicht dokumentiert. |
| DB-009 AuditLog ↔ DB-001 Documents | n:1 | FK in DB-009 → DB-001 (vermutet) | Nicht dokumentiert. |
| DB-010 Backups ↔ DB-012 BackupReminders | 1:n | FK in DB-012 → DB-010 (vermutet) | Nicht dokumentiert. |

### Archivierungsverhalten

- ADR-003: Dokumente werden archiviert, nicht gelöscht.
- Archivierungsdatum (DateTime) wird automatisch gesetzt bei Archivierung.
- Archivierte Dokumente verschwinden aus der Gast-Suche.
- Admins können archivierte Dokumente einsehen.
- Wiederherstellung bleibt möglich.
- Kein kaskadierendes Löschen dokumentiert.

## Enum / Status-Audit

| Feld | Tabelle | Dokumentierte Werte | Status |
|---|---|---|---|
| Status | DB-001 Documents | Entwurf, aktiv, archiviert | Dokumentiert in SDD-004B |
| Gültigkeit | DB-001 Documents | gültig, läuft bald ab, abgelaufen | Dokumentiert in SDD-004B |
| Aktivstatus | DB-003 Employees | aktiv, inaktiv (Boolean) | Dokumentiert |
| Status | DB-002 DocumentVersions | (vermutet wie DB-001) | **Nicht dokumentiert** |
| Rolle | DB-004 Users | Nicht definiert | **BLOCKIEREND** |
| Backup-Status | DB-010 Backups | Nicht definiert | **BLOCKIEREND** |
| Erinnerungsstatus | DB-012 BackupReminders | Nicht definiert | **BLOCKIEREND** |

## Identifikatoren und Zeitstempel

| Konzept | Dokumentation | Status |
|---|---|---|
| Primärschlüssel-Strategie | Nicht dokumentiert | **BLOCKIEREND** |
| Dokumentennummer vs. interne ID | Dokumentennummer ist eindeutig und unveränderlich (ADR-001). Ob sie Primärschlüssel ist, ist nicht dokumentiert. | **BLOCKIEREND** |
| created_at / updated_at | Nicht dokumentiert für任何 Tabelle | **BLOCKIEREND** |
| Archivierungszeitstempel | DateTime, automatisch bei Archivierung, leer für aktive | Dokumentiert (DB-001) |
| Versionsnummer | String, neue Versionen behalten ursprüngliche Dokumentennummer (ADR-001) | Dokumentiert |

## Zusammenfassung der Spezifikationslücken

### BLOCKIERENDE Lücken

1. **Primärschlüssel-Strategie** — Für keine Tabelle ist dokumentiert, ob UUID, Autoincrement oder Dokumentennummer als Primärschlüssel verwendet wird.
2. **DB-002 DocumentVersions** — Keine Felddefinitionen.
3. **DB-004 Users** — Keine Felddefinitionen. Authentifizierungsdaten, Rollen-Definitionen und Employee-Referenz unklar.
4. **DB-005 Categories** — Keine Felddefinitionen.
5. **DB-006 Subcategories** — Keine Felddefinitionen.
6. **DB-007 KeywordDictionary** — Keine Felddefinitionen.
7. **DB-008 DocumentTags** — Keine Felddefinitionen.
8. **DB-009 AuditLog** — Keine Felddefinitionen.
9. **DB-010 Backups** — Keine Felddefinitionen.
10. **DB-011 Settings** — Keine Felddefinitionen; unklar, welche Werte persistiert werden.
11. **DB-012 BackupReminders** — Keine Felddefinitionen.
12. **Many-to-Many für DB-003** — Join-Tabellen DB-003a und DB-003b vorgeschlagen, müssen freigegeben werden.
13. **User ↔ Employee Referenz** — Ob und wie ein Benutzerkonto einem Mitarbeiter referenziert, ist nicht dokumentiert.
14. **Enum-Definitionen** — Benutzerrollen (DB-004), Backup-Status (DB-010), Erinnerungsstatus (DB-012) sind nicht definiert.
15. **created_at / updated_at** — Für keine Tabelle dokumentiert.
16. **Lösch-/Archivierungsverhalten** — Nur für DB-001 dokumentiert (ADR-003). Für alle anderen Tabellen undefiniert.
