# ADR-029: Dokument-Lifecycle und Archiv-Wiederherstellung

## Status

Angenommen — 11.08.2026

## Kontext

PraxisQM verwaltet QM-Dokumente mit einem Bearbeitungsstatus (`status` in DB-001).
Die kanonische Dokumentation (Data Dictionary, SDD-004B) definiert drei Status-Werte:

- `Entwurf`
- `aktiv`
- `archiviert`

Die ursprüngliche Implementierung hatte zwei Fehler im Lifecycle:

1. **`archive_document` überschrieb `status` mit `archiviert`**, ohne den vorherigen
   Status zu sichern. Der Pre-Archive-Status (Entwurf oder aktiv) ging verloren.

2. **`restore_document` setzte `status` immer auf `aktiv`**, unabhängig vom
   ursprünglichen Status. Ein archiviertes Entwurf-Dokument wurde fälschlich als
   aktiv wiederhergestellt.

Zudem war `archiviert` als Option im normalen Bearbeitungsformular auswählbar,
was dem kanonischen Lifecycle-Modell widerspricht: Archivierung ist eine
dedizierte Operation, kein gewöhnlicher Statuswechsel.

## Entscheidung

### 1. Kanonische Non-Archived Lifecycle-Status

Die kanonischen Non-Archived-Lifecycle-Status sind:

- `Entwurf`
- `aktiv`

`archiviert` ist ein Archive-Lifecycle-State, kein Bearbeitungsstatus.

### 2. `archiviert` nur über dedizierte Archiv-Operation

`archiviert` darf **nur** über die kanonische `archive_document`-Operation
erreicht werden, niemals über gewöhnliche Metadaten-Bearbeitung.

Das Bearbeitungsformular bietet `archiviert` nicht als Status-Option an.
Das Backend weist `archiviert` als Status bei `create_document`,
`update_document` und `create_version` mit einem Constraint-Fehler zurück.

### 3. Manuelle Lifecycle-Übergänge für Non-Archived Dokumente

Erlaubte manuelle Übergänge (über Bearbeitung):

- `Entwurf` → `aktiv`
- `aktiv` → `Entwurf`

Beide Richtungen sind erlaubt. Das Backend validiert nur, dass der neue Status
`Entwurf` oder `aktiv` ist (nicht `archiviert`).

### 4. Archiv-Übergänge

- `Entwurf` → `archiviert` (über `archive_document`)
- `aktiv` → `archiviert` (über `archive_document`)

### 5. Restore-Übergänge

Restore muss das Dokument auf seinen exakten Pre-Archive-Status zurücksetzen:

- `Entwurf` → `archiviert` → `Entwurf`
- `aktiv` → `archiviert` → `aktiv`

### 6. `pre_archive_status`-Spalte (DB-001)

Neue nullable `TEXT`-Spalte `pre_archive_status` in der `documents`-Tabelle.

- Existiert ausschließlich zur Erhaltung des Non-Archived-Lifecycle-Status
  während Archivierung/Wiederherstellung.
- Wird bei `archive_document` gesetzt (`= status` vor Archivierung).
- Wird bei `restore_document` gelesen und anschließend auf `NULL` gesetzt.
- Für Legacy-Dokumente (vor dieser Migration archiviert) bleibt sie `NULL`.

### 7. Archiv-Verhalten

`archive_document`:

1. Validiere: Dokument existiert
2. Validiere: Dokument ist nicht bereits archiviert (`archived_at IS NULL`)
3. Speichere aktuellen `status` in `pre_archive_status`
4. Setze `status = 'archiviert'`
5. Setze `archived_at = current_timestamp`
6. Atomar in einer Transaktion

### 8. Restore-Verhalten

`restore_document`:

1. Validiere: Dokument existiert
2. Validiere: Dokument ist archiviert (`archived_at IS NOT NULL`)
3. Lese `pre_archive_status`
4. Wenn `pre_archive_status` `Entwurf` oder `aktiv` enthält:
   - Setze `status = pre_archive_status`
   - Setze `archived_at = NULL`
   - Setze `pre_archive_status = NULL`
   - Atomar in einer Transaktion
5. Wenn `pre_archive_status` `NULL` oder ungültig (Legacy-Dokument):
   - Return Controlled-Error (kein automatisches Restore als `aktiv`)
   - Fehlermeldung: "Der vorherige Status kann nicht automatisch ermittelt werden."

### 9. Legacy-Archivierte Dokumente

Bestehende archivierte Dokumente, die vor dieser Migration erstellt wurden,
haben keinen zuverlässigen historischen Pre-Archive-Status.

- `pre_archive_status` bleibt `NULL` für diese Dokumente
- Keine Fakturierung eines `pre_archive_status`-Werts während Migration
- Restore eines Legacy-Dokuments mit `NULL pre_archive_status` gibt einen
  kontrollierten Fehler zurück
- Kein automatisches Restore als `aktiv`

### 10. DB-002 Historische Version-Status

DB-002 `document_versions.status` wird durch Archivierung/Wiederherstellung
nicht geändert. Historische Version-Status sind unveränderliche Metadaten.

### 11. Active-List-Semantik

Die Active-Liste (nicht-archivierte Dokumente) bleibt basiert auf
`archived_at IS NULL`, nicht auf `status = 'aktiv'`.

Daher bleiben sowohl `Entwurf`- als auch `aktiv`-Dokumente in der
Non-Archived-Dokumentenliste.

### 12. Dashboard-Zähler

Dashboard-Dokumentzähler bleiben basiert auf nicht-archivierten Dokumenten
(`archived_at IS NULL`), nicht nur auf `status = 'aktiv'`.

### 13. Gültigkeit/Erinnerung

Validity- und Erinnerungs-Verhalten aus ADR-028 / Prompt 023 bleibt unverändert.

## Konsequenzen

- Schema-Migration v1→v2: `ALTER TABLE documents ADD COLUMN pre_archive_status TEXT`
- `archive_document` sichert Pre-Archive-Status in `pre_archive_status`
- `restore_document` stellt Pre-Archive-Status aus `pre_archive_status` wieder her
- Backend weist `archiviert` als Status bei Erstellung/Bearbeitung/Neue-Version zurück
- Frontend-Bearbeitungsformular bietet `archiviert` nicht als Status-Option an
- Legacy-archivierte Dokumente können nicht automatisch wiederhergestellt werden
- DB-002-Version-Status werden durch Archivierung/Wiederherstellung nicht geändert
- `SCHEMA_VERSION` wird von 1 auf 2 erhöht
