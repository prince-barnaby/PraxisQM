# PraxisQM

> Status: 🚧 Active Development

Ein vollständig offlinefähiges Qualitätsmanagementsystem für Zahnarztpraxen.

---

# Ziel

PraxisQM soll die Verwaltung aller QM-Dokumente einer Zahnarztpraxis vereinfachen.

## Schwerpunkte

- Offlinebetrieb
- Dokumentenverwaltung
- Rollen- und Benutzerverwaltung
- Automatische Dokumentennummerierung
- Archivierung
- PDF-Vorschau
- Suchfunktion
- Modernes Desktop-Design

---

# Projektstatus

**Aktuelle Phase**

✅ Architektur abgeschlossen

**Nächster Schritt**

➡ Umsetzung der Anwendung

---

# Technologie

## Frontend

- React
- TypeScript
- Vite

## Desktop-Anwendung

- Tauri
- Rust als Tauri-Runtime

## Datenbank

- SQLite

## Styling und Icons

- CSS
- Design Tokens
- Lucide React

## Versionsverwaltung

- Git
- GitHub

---

# Architekturgrundsatz

PraxisQM wird als lokale Desktop-Anwendung entwickelt.

Die Anwendung arbeitet vollständig offline und verwendet keine verpflichtenden Cloud-Dienste.

Die verbindliche Stack-Entscheidung ist dokumentiert in:

```text
docs/06_Decisions_ADR/ADR-027_Application_Stack.md
```

---

# Projektstruktur

```text
assets/
database/
diagrams/
docs/
mockups/
prompts/
releases/
source/
tests/
```

---

# Dokumentation

Die vollständige Projektdokumentation befindet sich im Ordner `docs/`.

Enthalten sind unter anderem:

- Software Design Document
- Administrator Guide
- Developer Guide
- Design System
- Navigation Specification
- Component Library
- Decision Records (ADR)
- Data Dictionary
- AI Development Rules
- Code Map

---

# Entwicklungsprinzipien

- Offline First
- Never Break Main
- Architecture before Code
- Clean Code
- Modular Development
- Documentation First
- Small Pull Requests

---

# Branch-Strategie

```text
main      → immer lauffähig
feature/* → neue Funktionen
bugfix/*  → Fehlerbehebungen
docs/*    → Dokumentation
```

---

# Build Status

- ✅ Architecture Complete
- ⬜ Sprint 1
- ⬜ Sprint 2
- ⬜ Sprint 3
- ⬜ Release Candidate
- ⬜ Version 1.0

---

# Lizenz

Private Project

Nicht zur öffentlichen Nutzung bestimmt.
