# CHANGELOG

Alle wichtigen Änderungen an PraxisQM werden in dieser Datei dokumentiert.

## [0.9.11] - 07.08.2026

### Hinzugefügt

- Statische Mitarbeiterübersicht auf der Seite „Mitarbeiter" unter der Route `/mitarbeiter`
- Werkzeugleiste mit Seitentitel „Mitarbeiter", Kurzbeschreibung und Mitarbeiterzähler („3 Mitarbeiter")
- Button „Neuer Mitarbeiter" (deaktiviert, da keine dokumentierte Route für das Erstellungsformular existiert)
- Einklappbarer Filterbereich (Standard: eingeklappt) mit drei dokumentierten Filterkriterien: Funktion, Bereich, Aktivstatus (alle Platzhalter, nicht funktional)
- Tabellarische Desktop-Darstellung aller Mitarbeitenden mit dokumentierten Feldern: Name, Vorname, Funktion, Bereich, Status, E-Mail, Telefon, Eintrittsdatum, Austrittsdatum
- Drei klar gekennzeichnete Mock-Platzhalter-Einträge zur Veranschaulichung von Layout und Komponenten
- Wiederverwendbare Komponenten: `EmployeeToolbar`, `EmployeeFilters`, `EmployeeList`, `EmployeeRow`
- Bestehende `StatusBadge`-Komponente wiederverwendet (Variante `success` für „aktiv", `neutral` für „inaktiv")
- Bestehende `EmptyState`-Komponente wiederverwendet („Keine Mitarbeiter vorhanden")
- Mitarbeiterzeilen mit Hover-State, Keyboard-Focus und visueller Anzeige für spätere Detailnavigation
- Accessibility: `aria-expanded` für einklappbare Filter, `aria-label`, `scope`, Tabellen-Semantik

### Dokumentation

- UI Style Guide (005C) um Abschnitt „Mitarbeiterübersicht / Mitarbeiter-Tabellenstandard" ergänzt mit verbindlichen Regeln:
  - „Das Mitarbeiterregister und die Benutzerverwaltung sind getrennte Bereiche."
  - „Die Mitarbeiterübersicht verwendet eine klassische tabellarische Desktop-Darstellung."
  - „Benutzerkonto-, Rollen- und Berechtigungsdaten dürfen nicht im Mitarbeiterregister vermischt werden."
- Component Library (005D) um Mitarbeiter-Komponenten ergänzt
- Code Map aktualisiert

### Nicht enthalten (bewusst)

- Keine Datenbankanbindung, keine echten Mitarbeiterdaten
- Keine funktionierende Filterlogik
- Keine Mitarbeiterdetail-Navigation (keine dokumentierte Route vorhanden)
- Kein Mitarbeiter-Erstellungsformular (nicht dokumentiert)
- Keine Benutzerkonto-, Rollen- oder Berechtigungsdaten im Mitarbeiterregister
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
