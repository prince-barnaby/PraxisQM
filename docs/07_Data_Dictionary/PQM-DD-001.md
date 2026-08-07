# Data Dictionary

> **Status nach Audit (Prompt 012):** Die Felddefinitionen für DB-001 und DB-003
> sind dokumentiert. Alle anderen Tabellen (DB-002, DB-004–DB-012) haben
> **blockierende Spezifikationslücken**. Siehe PQM-SDD-004B für den vollständigen
> Audit-Bericht.

## DB-001 Documents

Stammdaten eines QM-Dokuments.

| Feld | Typ | Beschreibung |
|---|---|---|
| Dokumentennummer | String | Automatisch vergeben, eindeutig, unveränderlich, wird niemals wiederverwendet (ADR-001) |
| Titel | String | Bezeichnung des Dokuments |
| Kategorie | String | Hauptkategorie |
| Unterkategorie | String | Unterkategorie |
| Verantwortliche Person | String | Zuständige Mitarbeitenden |
| Version | String | Versionsnummer des Dokuments |
| Status | String | Bearbeitungsstatus (Entwurf, aktiv, archiviert) |
| Gültigkeit | String | Gültigkeitsstatus (gültig, läuft bald ab, abgelaufen) |
| Gültig bis | Date | Ablaufdatum der Gültigkeit |
| Beschreibung | Text | Beschreibender Text |
| Tags | Liste<String> | Schlagwörter |
| Archivierungsdatum | DateTime | Automatisch gesetzt, wenn ein Dokument archiviert wird. Leer für aktive Dokumente. Wird für Archiv-Sortierung und Archiv-Filterung verwendet. |

## DB-002 DocumentVersions

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B für ableitbare Felder und ungelöste Fragen.

## DB-003 Employees

Mitarbeiterregister der Praxis – alle Mitarbeitenden, die als verantwortliche Personen eintragbar sind. Getrennt von der Benutzerverwaltung (ADR-004).

| Feld | Typ | Beschreibung |
|---|---|---|
| Name | String | Nachname der Mitarbeitenden |
| Vorname | String | Vorname der Mitarbeitenden |
| Position | String | Position / Rolle in der Praxis (z. B. Zahnärztin, ZFA, Praxismanagerin) |
| Verantwortungsposition | Liste<String> | Multi-Value-Feld. Null, eine oder mehrere Verantwortungspositionen (z. B. QM-Beauftragte, Datenschutzbeauftragte, Hygienebeauftragte). Many-to-many-Beziehung: employee ↔ Verantwortungsposition. |
| Zugeordneter QM-Bereich | Liste<String> | Multi-Value-Feld. Null, ein oder mehrere zugeordnete QM-Bereiche (z. B. Datenschutz, Patientendokumentation, Röntgeneinweisung). Many-to-many-Beziehung: employee ↔ QM-Bereich. |
| Aktivstatus | Boolean | Aktiv / inaktiv. Wird über StatusBadge dargestellt (success = aktiv, neutral = inaktiv). |
| Eintrittsdatum | Date | Eintrittsdatum in die Praxis |
| Austrittsdatum | Date | Austrittsdatum aus der Praxis (leer / „—" bei aktiven Mitarbeitenden) |

### Entfernte Felder

- ~~Funktion~~ → umbenannt zu **Position**
- ~~Bereich~~ → entfernt
- ~~E-Mail~~ → entfernt
- ~~Telefonnummer~~ → entfernt

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

Für eine korrekte SQLite-Implementierung werden zwei zusätzliche
Verknüpfungstabellen benötigt:

1. **DB-003a EmployeeResponsibilities** — employee ↔ Verantwortungsposition (n:m)
2. **DB-003b EmployeeQMAreas** — employee ↔ QM-Bereich (n:m)

Diese Tabellen sind **vorgeschlagen** und müssen vor der SQLite-Implementierung
freigegeben werden. Siehe PQM-SDD-004B für Details.

### Trennung von Mitarbeiterregister und Benutzerverwaltung (ADR-004)

- DB-003 Employees und DB-004 Users sind **getrennte Tabellen**.
- Das Mitarbeiterregister enthält **keine** Authentifizierungsdaten.
- Benutzerrollen und Berechtigungen sind **keine** Mitarbeiter-Verantwortungspositionen.
- Eine eventuelle Referenz zwischen User und Employee ist **optional** und
  **nicht dokumentiert** — muss vor SQLite-Implementierung geklärt werden.

### Begriffliche Trennung

| Begriff | Bedeutung | Tabelle |
|---|---|---|
| Position | Berufliche Rolle in der Praxis (z. B. Zahnärztin, ZFA) | DB-003 |
| Verantwortungsposition | QM-Verantwortung (z. B. QM-Beauftragte, Datenschutzbeauftragte) | DB-003 (Multi-Value) |
| QM-Bereich | Zugeordneter Qualitätsmanagementbereich (z. B. Datenschutz, Hygiene) | DB-003 (Multi-Value) |
| Benutzerrolle | Software-Rolle für Zugriffskontrolle (z. B. Admin, Gast) | DB-004 |
| Benutzerberechtigung | Einzelfehmigung innerhalb einer Rolle | DB-004 |

Eine Verantwortungsposition ist **keine** Software-Rolle.
Ein QM-Bereich ist **keine** Berechtigung.

## DB-004 Users

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B für ableitbare Felder und ungelöste Fragen.

## DB-005 Categories

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B.

## DB-006 Subcategories

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B.

## DB-007 KeywordDictionary

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B.

## DB-008 DocumentTags

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B.

## DB-009 AuditLog

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B.

## DB-010 Backups

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B.

## DB-011 Settings

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> DB-011 ist **unresolved**. Es ist nicht ausreichend dokumentiert, welche Werte
> tatsächlich in DB-011 gespeichert werden sollen. DB-011 darf nicht mit
> erfundenen Key/Value-Paaren aufgefüllt werden. Siehe PQM-SDD-004B.

## DB-012 BackupReminders

> **Spezifikationslücke (BLOCKIEREND):** Keine Felddefinitionen dokumentiert.
> Siehe PQM-SDD-004B.

## Beziehungsspezifikation

Siehe PQM-SDD-004B für die vollständige Beziehungsspezifikation.

## Enum / Status-Audit

Siehe PQM-SDD-004B für den vollständigen Enum-Audit.

## Identifikatoren und Zeitstempel

Siehe PQM-SDD-004B für den vollständigen Identifikatoren-Audit.
