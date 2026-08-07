# UI Style Guide 005C - Komponentenverwendung

## Dokumentenübersicht / Tabellenstandard

Die folgenden Entscheidungen sind verbindlich für die Dokumentenübersicht und gelten als UI-Standard für tabellarische Ansichten in PraxisQM.

### Darstellung

- Die Dokumentenübersicht ist eine **klassische tabellarische Desktop-Ansicht**.
- Die Tabelle darf **nicht** durch Karten, Kacheln, Listen-Cards, Accordion-Elemente oder andere alternative Darstellungsformen ersetzt werden.
- Das visuelle Erscheinungsbild der Tabelle ist grundsätzlich erhalten: bestehende Spaltenstruktur, dezente Farbgebung auf Basis der Design Tokens.

### Tabellenkopf

- Der Tabellenkopf ist **sticky**: beim vertikalen Scrollen innerhalb der Dokumentenliste bleiben die Spaltenüberschriften sichtbar.
- Das horizontale Scrollverhalten der Tabelle wird dadurch nicht beeinträchtigt.

### Filterbereich

- Der Filterbereich ist **standardmäßig eingeklappt**.
- Im eingeklappten Zustand wird eine kompakte Schaltfläche „Filter" mit Lucide-Filtericon angezeigt, die klar signalisiert, dass der Bereich geöffnet werden kann.
- Im ausgeklappten Zustand werden die fünf dokumentierten Filter angezeigt: Kategorie, Unterkategorie, Status, Verantwortliche Person, Gültigkeit.
- Die Filter bleiben nicht funktional, bis die Datenbankanbindung implementiert wird.

### Status und Gültigkeit

- Status und Gültigkeit werden visuell über **Badges** mit dezenten Farbakkenten auf Basis der vorhandenen Design Tokens dargestellt.
- Verwendete Varianten: success (gültig), warning (läuft bald ab), error (abgelaufen), neutral (Entwurf / archiviert).

### Dokumentenzähler

- Oberhalb der Tabelle wird ein Dokumentenzähler angezeigt (z. B. „3 Dokumente").
- Der Zähler ist so implementiert, dass er später einen dynamischen Datenbankwert darstellen kann.
- Es werden keine zusätzlichen Statistiken angezeigt.

### Spaltengewichtung (Desktop)

| Spalte | Gewichtung |
|---|---|
| Dokumentennummer | kompakt |
| Titel | erhält den meisten verfügbaren Platz |
| Kategorie | mittel |
| Unterkategorie | mittel |
| Status | kompakt |
| Verantwortliche Person | mittel |
| Gültigkeit | kompakt |
| Version | kompakt |

- Die Titelspalte erhält den größten verfügbaren Raum.
- Die Tabelle bleibt responsiv und unterstützt horizontales Scrollen auf schmalen Viewports.

### Dokumentennummern

- Dokumentennummern werden in **Monospace-Schrift** dargestellt, um sie visuell von normalen Textinhalten abzuheben.
- Es wird keine neue Schriftart hinzugefügt; die Monospace-Familie der bestehenden Schriftumgebung wird verwendet.

### Dokumentenzeilen

- Dokumentenzeilen haben einen **dezenten Hover-State** (leichte Hintergrundänderung).
- Zeilen haben einen **sichtbaren Keyboard-Focus** (Akzent-Umrandung).
- Die komplette Dokumentenzeile ist das **Navigationsziel** zum Öffnen der Dokumentdetailansicht.
- Navigation funktioniert über **Mausklick** und **Enter-Taste**.
- Die Route wird aus der Dokumentennummer gebildet (z. B. PQM-0001 → `/dokumente/PQM-0001`).
- Es werden **keine separaten „Öffnen"-Buttons** in der Tabelle hinzugefügt.
- Zeilen haben `role="link"` und einen Pointer-Cursor, da die Navigation jetzt funktional ist.
- Bestehende Hover- und Focus-Stile bleiben erhalten.

### Navigation zur Detailansicht

- **Die komplette Dokumenttabellenzeile ist das Navigationsziel zum Öffnen der Dokumentdetailansicht.**
- Diese Regel ist verbindlich und gilt für alle zukünftigen Erweiterungen der Dokumentenliste.

### Desktop-first

- Das Layout ist für eine Desktop-Anwendung optimiert.
- Die Dokumentenliste soll auf einem normalen Praxis-PC gut scanbar sein.
- Übersichtlichkeit wird gegenüber dekorativem Design bevorzugt.
