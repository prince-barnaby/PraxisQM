# ADR-028: Dokumentgültigkeits-Status und Review-Schwellwert

## Status

Angenommen — 11.08.2026

## Kontext

PraxisQM muss auf einen Blick zeigen, welche aktiven QM-Dokumente:

- aktuell gültig sind
- bald überprüft werden müssen
- abgelaufen sind

Die kanonische Dokumentation (Data Dictionary, SDD-004B) definiert drei Gültigkeits-Enum-Werte
(`gültig`, `läuft bald ab`, `abgelaufen`) und ein `valid_until`-Datum, aber keinen Schwellwert
für den Übergang von `gültig` zu `läuft bald ab`.

Die ursprüngliche Implementierung speicherte `validity` als manuell gepflegtes Enum-Feld,
ohne es jemals automatisch aus `valid_until` abzuleiten.

## Entscheidung

### 1. Schwellwert: 30 Kalendertage

Der Schwellwert für "läuft bald ab" ist **30 Kalendertage** vor `valid_until`.

Dieser Schwellwert ist **fix für v1**. Zukünftige Konfigurierbarkeit kann hinzugefügt werden,
ohne das Gültigkeitsmodell zu ändern.

### 2. Abgeleiteter Status (Runtime-Quelle der Wahrheit)

`valid_until` (DB-001) ist die **Runtime-Quelle der Wahrheit** für den aktuellen Gültigkeitsstatus.

Der Status wird zur Laufzeit berechnet aus:
- `valid_until`
- aktuellem lokalem Kalenderdatum
- dem fixen 30-Tage-Schwellwert

### 3. Persistierte `validity`-Kompatibilität

Das bestehende `validity TEXT NOT NULL`-Feld bleibt im Schema für Kompatibilität.
Es wird **nicht** als Runtime-Wahrheit verwendet. Runtime-Anzeige, Filterung und
zukünftige Erinnerungslogik verwenden ausschließlich das abgeleitete `computed_validity`.

### 4. NULL `valid_until`

Wenn `valid_until = NULL`, wird das Dokument **nicht überwacht**.
Es erhält keinen berechneten Gültigkeitsstatus (kein "gültig", kein vierter Enum-Wert).
In der Tabellenanzeige wird ein Gedankenstrich (`—`) gezeigt.

### 5. Datumsgrenzen

Kalenderdatum-Semantik (keine Tageszeit):

| Bedingung | Status |
|---|---|
| `today < (valid_until - 30 Tage)` | `gültig` |
| `today >= (valid_until - 30 Tage)` UND `today <= valid_until` | `läuft bald ab` |
| `today > valid_until` | `abgelaufen` |
| `valid_until = NULL` | kein Status (nicht überwacht) |

Das Dokument ist gültig **BIS** zum Ablaufdatum (inklusiv).

### 6. Lebenszyklus-Unabhängigkeit

`validity` und `status` (Bearbeitungsstatus) bleiben unabhängig.
Ein abgelaufenes Dokument kann `status = "aktiv"` behalten.
Kein automatisches Archivieren, keine automatische Neue-Version, keine PDF-Änderung.

### 7. Archivierte Dokumente

Archivierte Dokumente:
- behalten `valid_until` und historische `validity`-Daten
- werden von aktiven Warnungs-/Erinnerungs-Logiken ausgeschlossen
- `computed_validity` wird weiterhin berechnet (für Archiv-Anzeige), aber nicht für aktive Warnungen verwendet

### 8. Versionierung

DB-001 `valid_until` ist die Quelle für die aktuelle Gültigkeitsanzeige.
Jede DB-002-Version behält ihre eigenen `validity`/`valid_until`-Werte als historische Metadaten.
Neue Versionen folgen dem bestehenden Prompt-020-Verhalten für gelieferte Gültigkeitsdaten.
Vorgänger-DB-002-Zeilen werden nicht retroaktiv geändert.

## Konsequenzen

- `calculate_validity_status(valid_until, today)` ist die einzige kanonische Berechnungsfunktion
- Frontend verwendet ausschließlich `computed_validity` (nicht `validity`) für Anzeige und Filterung
- Keine Schema-Migration erforderlich — `validity`-Feld bleibt, `computed_validity` wird zur Laufzeit berechnet
- Deterministische Tests mit festem `today`-Parameter sind möglich
- Zukünftige Konfigurierbarkeit des Schwellwerts kann hinzugefügt werden, ohne die Berechnungsfunktion zu ändern
