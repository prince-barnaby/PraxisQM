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
- Übersichtlichkeit wird gegenüber dekorativem Design bevorzugut.

## Dokumentformular / Formularstandard

Die folgenden Regeln sind verbindlich für das Erstellen und Bearbeiten von Dokumenten.

### Wiederverwendung

- **Die Workflows zum Erstellen und Bearbeiten von Dokumenten müssen dieselben Formularkomponenten wiederverwenden, wo immer möglich.**
- Diese Regel ist verbindlich und gilt für alle zukünftigen Erweiterungen der Dokumentformulare.

### Layout

- Das Formular ist für Desktop optimiert mit klaren Abschnitten.
- Felder verwenden lesbare Beschriftungen und angemessenen Abstand.
- Zweispaltiges Layout für kurze Felder, einspaltiges Layout für lange Textfelder.
- Die Dokumentnummer ist schreibgeschützt und wird automatisch vergeben (Platzhalter: „wird automatisch vergeben").

### Felder

- Es werden ausschließlich dokumentierte Dokumentfelder verwendet: Dokumentnummer, Titel, Kategorie, Unterkategorie, Version, Status, Verantwortliche Person, Gültig bis, Beschreibung, Tags.
- Keine undocumented Felder hinzufügen.

### Datei-Upload

- Der Datei-Upload-Bereich ist klar abgesetzt und als Platzhalter markiert.
- Der Button „PDF auswählen" ist deaktiviert und öffnet keinen Datei-Dialog.

### Aktionen

- „Abbrechen" navigiert zur Dokumentenübersicht.
- „Dokument anlegen" ist deaktiviert und als Platzhalter markiert.
- Es wird kein erfolgreiches Speichern simuliert.

## Archivübersicht / Archiv-Tabellenstandard

Die folgenden Entscheidungen sind verbindlich für die Archivübersicht und gelten als UI-Standard für die Archivansicht in PraxisQM.

### Darstellung

- Das Archiv verwendet dieselbe **klassische tabellarische Desktop-Darstellung** wie die Dokumentenübersicht.
- Die Archivtabelle darf **nicht** durch Karten, Kacheln, Listen-Cards, Accordion-Elemente oder andere alternative Darstellungsformen ersetzt werden.
- Das visuelle Erscheinungsbild entspricht der Dokumentenübersicht: gleiche Spaltenstruktur-Prinzipien, dezente Farbgebung auf Basis der Design Tokens.

### Archiv-Prinzipien

- Archivierte Dokumente werden nicht gelöscht und Dokumentnummern werden niemals erneut vergeben.
- Das Archiv ist kein Papierkorb.
- Es gibt keinen Lösch-Button, keine permanente Löschaktion und kein Papierkorb-Icon.
- Wiederherstellung bleibt möglich und darf als deaktivierter Platzhalter-Button dargestellt werden.

### Tabellenkopf

- Der Tabellenkopf ist **sticky**, wie in der Dokumentenübersicht.

### Spalten

- Die Archivtabelle zeigt ausschließlich dokumentierte Felder: Dokumentennummer, Titel, Kategorie, Unterkategorie, Verantwortliche Person, Version, Archivierungsdatum, Status.
- Das Feld Archivierungsdatum ist vom Typ DateTime, wird automatisch beim Archivieren gesetzt und ist leer für aktive Dokumente.
- Keine undocumented Felder hinzufügen.

### Filterbereich

- Der Filterbereich ist **standardmäßig eingeklappt**.
- Im ausgeklappten Zustand werden die dokumentierten Archiv-Filter angezeigt: Kategorie, Unterkategorie, Verantwortliche Person, Archivierungszeitraum, Status.
- Die Filter bleiben nicht funktional, bis die Datenbankanbindung implementiert wird.

### Statusanzeige

- Der Archivstatus wird über die vorhandene `StatusBadge`-Komponente dargestellt (Variante `neutral` für archiviert).
- Es wird kein zweites archivspezifisches Badge-System eingeführt.

### Archivzähler

- Oberhalb der Tabelle wird ein Archivzähler angezeigt (z. B. „3 archivierte Dokumente").
- Der Zähler ist so implementiert, dass er später einen dynamischen Datenbankwert darstellen kann.

### Leerer Zustand

- Bei leerem Archiv wird die vorhandene `EmptyState`-Komponente angezeigt.
- Text: „Noch keine archivierten Dokumente vorhanden."

### Zeilen

- Archivzeilen haben einen **dezenten Hover-State** und **sichtbaren Keyboard-Focus**.
- Zeilen haben eine visuelle Anzeige, dass sie später eine Archivdetailansicht öffnen können.
- Es wird keine Archivdetail-Navigation implementiert, es sei denn, eine bestehende dokumentierte Route erfordert dies.

### Komponenten-Wiederverwendung

- Bestehende Tabellen-, Badge- und EmptyState-Komponenten werden wiederverwendet, wo immer möglich.
- Es werden keine redundanten Komponenten erstellt.

## Einstellungsoberfläche / Settings-Layout-Standard

Die folgenden Entscheidungen sind verbindlich für die Einstellungsseite und gelten als UI-Standard für die Einstellungsoberfläche in PraxisQM.

### Dokumentationspflicht

- Die Einstellungsoberfläche enthält nur dokumentierte Konfigurationsbereiche.
- Neue Einstellungsoptionen dürfen nicht ohne vorherige Dokumentation eingeführt werden.

### Darstellung

- Die Einstellungsseite verwendet ein **zweispaltiges Desktop-Layout** mit linker Navigationsleiste und rechtem Inhaltsbereich.
- Die linke Navigation listet alle dokumentierten Einstellungsbereiche als anklickbare Einträge.
- Der rechte Inhaltsbereich zeigt den aktuell ausgewählten Einstellungsbereich.
- Es werden **keine Dashboard-Karten** verwendet — die Seite fühlt sich wie eine Desktop-Einstellungsoberfläche an.

### Navigation

- Die Einstellungsnavigation ist eine vertikale Liste von Bereichsnamen mit Icons.
- Der aktive Bereich wird durch `aria-current="true"` und eine visuelle Hervorhebung markiert (Akzentfarbe links, Primary Blue Text).
- Navigation funktioniert über Mausklick und Tastatur.
- Die Navigation ist semantisch als `<nav>` mit `aria-label` markiert.

### Inhaltsbereich

- Jeder Einstellungsbereich zeigt einen Titel, eine Kurzbeschreibung und Platzhalter-Inhalt.
- Platzhalter-Inhalte sind eindeutig als Platzhalter gekennzeichnet.
- Es werden keine funktionalen Steuerelemente implementiert, es sei denn sie sind explizit erforderlich.

### Bereiche

- Die folgenden Einstellungsbereiche sind dokumentiert und werden angezeigt:
  - Allgemein
  - Dokumentennummerierung
  - Kategorien & Unterkategorien
  - Benutzerverwaltung
  - Backup
  - Systeminformationen
- Keine undocumented Bereiche hinzufügen.

### Dokumentennummerierung

- Die Dokumentennummerierung wird als **schreibgeschützte Platzhalterinformation** dargestellt.
- Es wird keine manuelle Bearbeitung bereits vergebener Dokumentennummern ermöglicht.
- ADR-001 bleibt gewahrt: Dokumentennummern sind automatisch, unveränderlich und werden niemals wiederverwendet.

### Kategorien

- Der Kategorien-Bereich zeigt einen Platzhalter für die zukünftige Verwaltung von Kategorien und Unterkategorien.
- Es wird kein CRUD implementiert.

### Benutzerverwaltung

- Die Benutzerverwaltung ist ein zukünftiger Einstiegspunkt und als Platzhalter markiert.
- **Benutzerverwaltung und Mitarbeiterregister sind getrennte Bereiche** (ADR-004).
- Benutzerkonten, Rollen und Berechtigungen werden nicht im Mitarbeiterregister geführt.

### Backup

- Der Backup-Bereich zeigt einen statischen Platzhalter.
- Es wird klar angezeigt, dass die Backup-Funktionalität nicht implementiert ist.
- Es werden keine Dateioperationen oder Zeitpläne implementiert.

### Systeminformationen

- Der Systeminformationsbereich zeigt schreibgeschützte Platzhalterinformationen:
  - PraxisQM-Version
  - Anwendungsarchitektur
  - Offline-Status
- Es werden nur dokumentierte Werte verwendet.
- Es werden keine erfundenen Umgebungsinformationen angezeigt.

### Desktop-first

- Das Layout ist für eine Desktop-Anwendung optimiert.
- Auf mobilen Viewports wird das Layout einspaltig; die Navigation erscheint oberhalb des Inhaltsbereichs.

Die folgenden Entscheidungen sind verbindlich für die Mitarbeiterübersicht und gelten als UI-Standard für das Mitarbeiterregister in PraxisQM.

### Trennungsregel

- **Das Mitarbeiterregister und die Benutzerverwaltung sind getrennte Bereiche.**
- **Die Mitarbeiterübersicht verwendet eine klassische tabellarische Desktop-Darstellung.**
- **Benutzerkonto-, Rollen- und Berechtigungsdaten dürfen nicht im Mitarbeiterregister vermischt werden.**
- Das Mitarbeiterregister dient ausschließlich der Verwaltung aller Mitarbeitenden der Praxis als verantwortliche Personen (ADR-004).
- Benutzerkonten, Login-Namen, Passwörter, Rollen und Berechtigungen werden nicht im Mitarbeiterregister geführt.

### Darstellung

- Die Mitarbeiterübersicht ist eine **klassische tabellarische Desktop-Ansicht**.
- Die Tabelle darf **nicht** durch Karten, Kacheln, Listen-Cards, Accordion-Elemente oder andere alternative Darstellungsformen ersetzt werden.
- Das visuelle Erscheinungsbild entspricht der Dokumenten- und Archivübersicht: gleiche Spaltenstruktur-Prinzipien, dezente Farbgebung auf Basis der Design Tokens.

### Tabellenkopf

- Der Tabellenkopf ist **sticky**, wie in der Dokumentenübersicht.

### Spalten

- Die Mitarbeitertabelle zeigt folgende Felder: Name, Vorname, Funktion, Bereich, Verantwortungsposition, QM-Bereich, Status, Eintrittsdatum, Austrittsdatum.
- Keine undocumented Felder hinzufügen.

### Multi-Value-Felder

- Die Felder **Verantwortungsposition** und **Zugeordneter QM-Bereich** sind Multi-Value-Felder.
- Ein Mitarbeiter kann null, eine oder mehrere Verantwortungspositionen haben.
- Ein Mitarbeiter kann null, einem oder mehreren QM-Bereichen zugeordnet sein.
- Beide Felder dürfen nicht als einzelner String oder als Single-Select-Wert dargestellt werden.
- In der Tabelle werden mehrere Werte als **kompakte Badges/Chips** dargestellt:
  - Verantwortungsposition: Primärfarbe (Primary Blue) mit hellem Hintergrund
  - QM-Bereich: Akzentfarbe (Accent Teal) mit hellem Hintergrund
- Bei null Werten wird ein neutraler Platzhalter („—") angezeigt.
- Die Tabelle bleibt lesbar und desktop-orientiert, auch wenn mehrere Werte vorhanden sind.
- Platzhalterwerte sind Beispiele und keine festen Geschäftsregeln.

### Filterbereich

- Der Filterbereich ist **standardmäßig eingeklappt**.
- Im ausgeklappten Zustand werden die dokumentierten Filter angezeigt: Funktion, Bereich, Aktivstatus.
- Die Filter bleiben nicht funktional, bis die Datenbankanbindung implementiert wird.

### Statusanzeige

- Der Aktivstatus wird über die vorhandene `StatusBadge`-Komponente dargestellt (Variante `success` für „aktiv", `neutral` für „inaktiv").
- Es wird kein zweites mitarbeiterspezifisches Badge-System eingeführt.
- Es werden keine weiteren Beschäftigungsstatus erfunden.

### Mitarbeiterzähler

- Oberhalb der Tabelle wird ein Mitarbeiterzähler angezeigt (z. B. „3 Mitarbeiter").
- Der Zähler ist so implementiert, dass er später einen dynamischen Datenbankwert darstellen kann.

### Leerer Zustand

- Bei leerem Mitarbeiterregister wird die vorhandene `EmptyState`-Komponente angezeigt.
- Text: „Keine Mitarbeiter vorhanden."

### Zeilen

- Mitarbeiterzeilen haben einen **dezenten Hover-State** und **sichtbaren Keyboard-Focus**.
- Zeilen haben eine visuelle Anzeige, dass sie später eine Mitarbeiterdetailansicht öffnen können.
- Es wird keine Mitarbeiterdetail-Navigation implementiert, es sei denn, eine bestehende dokumentierte Route erfordert dies.

### Button „Neuer Mitarbeiter"

- Der Button „Neuer Mitarbeiter" bleibt deaktiviert, da keine dokumentierte Route für das Mitarbeiter-Erstellungsformular existiert.
- Es wird kein Mitarbeiter-Erstellungsformular implementiert, es sei denn, es ist explizit dokumentiert und erforderlich.

### Komponenten-Wiederverwendung

- Bestehende Tabellen-, Badge- und EmptyState-Komponenten werden wiederverwendet, wo immer möglich.
- Es werden keine redundanten Komponenten erstellt.
