# PraxisQM -- AI Development Rules

## 1. Grundregel

PraxisQM wird KI-gestützt entwickelt, aber nicht KI-gesteuert.

Die KI darf Code erzeugen, refaktorieren und Vorschläge machen.
Architekturentscheidungen, Funktionsumfang und fachliche Regeln werden
ausschließlich durch die Projektdokumentation und den
Projektverantwortlichen bestimmt.

## 2. Main Branch darf niemals gebrochen werden

Der Branch `main` ist immer stabil zu halten.

Es wird niemals direkt auf `main` entwickelt.

Neue Funktionen erfolgen immer auf eigenen Branches, z. B.:

-   `feature/dashboard`
-   `feature/login`
-   `feature/document-management`
-   `feature/archive`
-   `feature/settings`

Erst nach Prüfung, Test und Freigabe dürfen Änderungen nach `main`
übernommen werden.

## 3. Dokumentation ist verbindlich

Vor jeder Code-Erzeugung muss die KI die relevante Projektdokumentation
berücksichtigen, insbesondere:

-   `docs/01_Software_Design_Document`
-   `docs/02_Developer_Guide`
-   `docs/03_Design_System`
-   `docs/07_Data_Dictionary`
-   `docs/09_UI_Style_Guide`
-   `docs/12_Design_Tokens`
-   `docs/18_Requirements`
-   `docs/21_AI_Development_Rules.md`

Priorität bei Widersprüchen:

1.  ADR-Dokumente
2.  Requirements
3.  Software Design Document
4.  Developer Guide
5.  Design System / UI Style Guide
6.  Einzelprompt

## 4. Keine ungefragten Architekturentscheidungen

Die KI darf keine grundlegenden Architekturentscheidungen selbst
treffen.

Nicht ohne Freigabe ändern:

-   Datenmodell
-   Rollenmodell
-   Sicherheitskonzept
-   Offline-First-Prinzip
-   Archivierungslogik
-   Navigationsstruktur
-   Designsystem
-   Projektstruktur

## 5. Eine Aufgabe pro Prompt

Jeder Prompt behandelt genau eine klar abgegrenzte Aufgabe.

Nicht erlaubt:

-   „Baue die komplette App."
-   „Implementiere alle Module."
-   „Überarbeite alles."

## 6. Modularer Code

Jede Datei hat genau einen klaren Zweck.

Komponenten und Funktionen müssen verständlich benannt werden.

Große Dateien vermeiden. Wiederverwendbare Logik in eigene Module
auslagern.

## 7. Verständlicher Code

Jede wichtige Datei enthält:

-   Zweck der Datei
-   Zugehöriges Modul
-   wichtige Abhängigkeiten

Zentrale Funktionen werden kurz kommentiert.

## 8. Code Map pflegen

Neue UI-Elemente und Funktionen werden zusätzlich in:

`docs/22_Code_Map.md`

dokumentiert.

## 9. Deutsche Oberfläche

Die Benutzeroberfläche ist deutsch.

Code darf englische Bezeichner verwenden.

## 10. Offline First

Keine Cloud-Abhängigkeiten ohne ausdrückliche Anforderung.

Electron und lokale Datenhaltung sind vorgesehen.

## 11. Keine Echtdaten

Nur fiktive Testdaten verwenden.

## 12. Designsystem verwenden

Alle UI-Komponenten orientieren sich am Design System und den Design
Tokens.

## 13. Bestehenden Code respektieren

Vorhandener Code wird nur geändert, wenn dies ausdrücklich gefordert
ist.

## 14. Tests

Neue Funktionen sollen testbar aufgebaut werden.

## 15. Git-Workflow

1.  Änderung prüfen
2.  Lokal testen
3.  `git status`
4.  `git add .`
5.  `git commit -m "Aussagekräftige Nachricht"`
6.  `git push`

## 16. Keine Fake-Funktionalität

Platzhalter müssen eindeutig als Platzhalter gekennzeichnet sein.

## 17. Grenzen benennen

Wenn die KI unsicher ist oder ein Modul zu groß wird, muss sie dies
offen kommunizieren.

## 18. Ziel

PraxisQM soll langfristig wartbar, nachvollziehbar, erweiterbar und
professionell aufgebaut sein.
