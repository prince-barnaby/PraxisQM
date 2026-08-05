# PraxisQM – AI Development Rules

## 1. Grundregel

PraxisQM wird KI-gestützt entwickelt, aber nicht KI-gesteuert.

Die KI darf Code erzeugen, refaktorieren und Vorschläge machen. Architekturentscheidungen, Funktionsumfang und fachliche Regeln werden ausschließlich durch die Projektdokumentation und den Projektverantwortlichen bestimmt.

## 2. Main Branch darf niemals gebrochen werden

Der Branch `main` ist immer stabil zu halten.

Es wird niemals direkt auf `main` entwickelt.

Neue Funktionen, größere Änderungen und Experimente erfolgen immer auf eigenen Branches, zum Beispiel:

- `feature/dashboard`
- `feature/login`
- `feature/document-management`
- `feature/archive`
- `feature/settings`

Erst nach Prüfung, Test und Freigabe dürfen Änderungen nach `main` übernommen werden.

## 3. Dokumentation ist verbindlich

Vor jeder Code-Erzeugung muss die KI die relevante Projektdokumentation berücksichtigen, insbesondere:

- `docs/01_Software_Design_Document`
- `docs/02_Developer_Guide`
- `docs/03_Design_System`
- `docs/07_Data_Dictionary`
- `docs/09_UI_Style_Guide`
- `docs/12_Design_Tokens`
- `docs/18_Requirements`
- `docs/21_AI_Development_Rules.md`

Bei Widersprüchen gilt folgende Reihenfolge:

1. ADR-Dokumente
2. Requirements
3. Software Design Document
4. Developer Guide
5. Design System / UI Style Guide
6. Einzelprompt

## 4. Keine ungefragten Architekturentscheidungen

Die KI darf keine grundlegenden Architekturentscheidungen selbst treffen.

Wenn eine Anforderung unklar ist, muss sie nachfragen oder eine ausdrücklich gekennzeichnete Annahme treffen.

Nicht erlaubt sind ungefragte Änderungen an:

- Datenmodell
- Rollenmodell
- Sicherheitskonzept
- Offline-First-Prinzip
- Archivierungslogik
- Navigationsstruktur
- Designsystem
- Projektstruktur

## 5. Eine Aufgabe pro Prompt

Jeder Prompt behandelt genau eine klar abgegrenzte Aufgabe.

Nicht erlaubt:

- „Baue die komplette App.“
- „Implementiere alle Module.“
- „Überarbeite alles.“

Erlaubt:

- „Erstelle das Dashboard-Grundgerüst.“
- „Implementiere die Sidebar-Navigation.“
- „Erstelle die Dokumentenliste ohne Upload-Funktion.“
- „Ergänze den Archiv-Button für Dokumente.“

## 6. Modularer Code

Code muss modular aufgebaut werden.

Jede Datei hat genau einen klaren Zweck.

Funktionen und Komponenten müssen verständlich benannt sein.

Große Dateien sind zu vermeiden. Wiederverwendbare Logik gehört in eigene Module.

## 7. Code muss erklärbar bleiben

Jede wichtige Datei enthält am Anfang einen kurzen Kommentar zu Zweck, Modul und wichtigen Abhängigkeiten.

Zentrale Funktionen erhalten kurze, verständliche Kommentare.

## 8. Code Map pflegen

Bei neuen UI-Elementen, Komponenten oder Funktionen muss `docs/22_Code_Map.md` aktualisiert werden.

## 9. Deutsche Oberfläche

Die Benutzeroberfläche von PraxisQM ist deutschsprachig.

Code darf englische Bezeichner verwenden. UI-Texte, Labels, Hinweise und Fehlermeldungen sind deutsch.

## 10. Offline First

PraxisQM ist eine lokale Offline-Anwendung.

Die KI darf keine Cloudpflicht, externen Server oder Online-Abhängigkeit einbauen, wenn dies nicht ausdrücklich gefordert wird.

Tauri und lokale SQLite-Datenhaltung sind vorgesehen.

## 11. Keine sensiblen Echtdaten

Im Code, in Testdaten und in Beispielen dürfen keine echten Patientendaten, echten Mitarbeitendendaten oder echten Praxisinterna verwendet werden.

Testdaten müssen eindeutig fiktiv sein.

## 12. Designsystem verwenden

UI-Komponenten müssen sich am vorhandenen Designsystem orientieren.

Farben, Abstände, Typografie, Karten, Buttons, Tabellen und Statusanzeigen müssen aus den Designvorgaben beziehungsweise Design Tokens abgeleitet werden.

## 13. Bestehenden Code respektieren

Die KI darf bestehenden Code nicht unnötig überschreiben.

Bestehende Funktionen dürfen nur geändert werden, wenn der Prompt dies ausdrücklich verlangt.

## 14. Tests und Qualität

Neue Funktionen sollen testbar aufgebaut werden.

Die KI soll keine Funktion als „fertig“ betrachten, wenn sie offensichtlich nicht ausführbar, nicht eingebunden oder nicht erreichbar ist.

## 15. Git-Workflow

Nach jeder sinnvollen Arbeitseinheit gilt:

1. Änderung prüfen
2. lokal testen
3. `git status`
4. `git add .`
5. `git commit -m "Aussagekräftige Commit-Nachricht"`
6. `git push`

Commits sollen klein und nachvollziehbar bleiben.

## 16. Keine Fake-Funktionalität

Platzhalter sind erlaubt, müssen aber klar als Platzhalter erkennbar sein.

## 17. KI muss Grenzen benennen

Wenn die KI etwas nicht sicher weiß, muss sie das klar sagen.

Wenn ein Modul zu groß für einen Prompt ist, muss es in kleinere Schritte aufgeteilt werden.

Wenn Bolt.new mit der kostenlosen Version an Token-, Kontext- oder Projektgrenzen stößt, soll dies dokumentiert und bewertet werden, bevor ein Upgrade empfohlen wird.

## 18. Ziel

Ziel ist eine wartbare, verständliche, lokale PraxisQM-Anwendung.

Der Code soll nicht nur funktionieren, sondern langfristig nachvollziehbar, erweiterbar und prüfbar bleiben.
