# ADR-027 -- Application Stack

## Status

Accepted

------------------------------------------------------------------------

## Datum

2026-08-05

------------------------------------------------------------------------

## Entscheidung

PraxisQM wird als vollständig lokale Desktop-Anwendung entwickelt.

Die technische Basis besteht aus:

  Ebene             Technologie
  ----------------- ---------------------
  UI                React
  Sprache           TypeScript
  Build-System      Vite
  Desktop Runtime   Tauri
  Datenbank         SQLite
  Styling           CSS + Design Tokens
  Icons             Lucide Icons

------------------------------------------------------------------------

## Ziel

PraxisQM soll:

-   vollständig offline funktionieren
-   keine Internetverbindung benötigen
-   auf Windows-PCs in Zahnarztpraxen laufen
-   DSGVO-konform lokal arbeiten
-   leicht wartbar sein
-   langfristig erweitert werden können

------------------------------------------------------------------------

## Begründung

Die Anwendung verarbeitet sensible Patientendaten sowie interne
QM-Dokumente.

Eine Cloud-Lösung widerspricht den Projektzielen und erhöht Komplexität,
Kosten und Datenschutzanforderungen.

SQLite bietet:

-   hohe Geschwindigkeit
-   keine Installation eines Datenbankservers
-   einfache Datensicherung
-   vollständigen Offlinebetrieb

Tauri ermöglicht:

-   native Desktop-Anwendungen
-   geringen Speicherverbrauch
-   moderne Weboberfläche
-   gute Performance

React und TypeScript bieten:

-   saubere Komponentenstruktur
-   gute Wartbarkeit
-   hohe Erweiterbarkeit
-   große Entwickler-Community

------------------------------------------------------------------------

## Nicht verwendete Technologien

Folgende Technologien wurden bewusst ausgeschlossen:

-   Flask
-   Django
-   Firebase
-   Supabase
-   Bolt Database
-   Electron
-   Cloud-Hosting
-   Webserver als Pflichtbestandteil

------------------------------------------------------------------------

## Architekturprinzipien

Die Anwendung folgt diesen Grundsätzen:

-   Offline First
-   Local First
-   Single Source of Truth
-   Modularer Aufbau
-   Trennung von UI, Logik und Daten
-   Keine Geschäftslogik in UI-Komponenten
-   Keine versteckten Abhängigkeiten

------------------------------------------------------------------------

## AI-Entwicklungsregeln

KI-Systeme (Bolt, ChatGPT oder andere) dürfen diese Architektur nicht
eigenständig ändern.

Abweichungen sind ausschließlich nach einer neuen ADR-Entscheidung
zulässig.

------------------------------------------------------------------------

## Konsequenzen

### Vorteile

-   maximale Datensicherheit
-   DSGVO-konforme lokale Speicherung
-   schneller Programmstart
-   geringe Hardwareanforderungen
-   einfache Installation
-   keine laufenden Cloudkosten

### Nachteile

-   keine automatische Synchronisation
-   keine Browser-Version
-   Updates müssen verteilt werden

Diese Nachteile werden bewusst akzeptiert.

------------------------------------------------------------------------

## Gültigkeit

Diese ADR besitzt Vorrang gegenüber älteren Dokumentationen, die
Python/Flask oder andere Architekturen beschreiben.

Bei Widersprüchen gilt ausschließlich diese Entscheidung.
