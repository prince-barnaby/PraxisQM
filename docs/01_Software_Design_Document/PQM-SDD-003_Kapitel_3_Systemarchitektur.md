# Kapitel 3 - Systemarchitektur

**Dokumentcode:** PQM-SDD-003  
**Version:** 0.9.3  
**Status:** Beschlossen  
**Datum:** 04.07.2026

## 3.1 Architekturziel

PraxisQM wird als lokale Desktop-Webanwendung entwickelt. Version 1.0 konzentriert sich ausschließlich auf Arbeitsplatz-PCs im Praxisnetzwerk.

## 3.2 Hauptmodule

- Frontend / Benutzeroberfläche
- Backend / Geschäftslogik
- SQLite-Datenbank
- Dokumentenspeicher
- Backup-Modul
- Admin-Protokollierung

## 3.3 Architekturregeln

- Das Frontend greift niemals direkt auf die Datenbank zu.
- Alle fachlichen Regeln liegen im Backend.
- Dokumente werden im Dateisystem gespeichert.
- Metadaten werden in SQLite gespeichert.
- Version 1.0 ist Desktop-only.
- Module werden sauber getrennt entwickelt.

## 3.4 Datenfluss Dokument öffnen

```text
Benutzer klickt Dokument
↓
Frontend fragt Backend
↓
Backend prüft Dokumentstatus und aktuelle Version
↓
Backend liefert Datei aus
↓
PDF öffnet sich im Browser
```

## 3.5 Datenfluss Dokument hochladen

```text
Login
↓
Upload über Seitenpanel
↓
Backend prüft Rechte und Pflichtfelder
↓
Dokumentennummer wird erzeugt
↓
Datei wird gespeichert
↓
Metadaten werden in SQLite gespeichert
↓
Aktion wird protokolliert
↓
Dokument ist sofort sichtbar
```

## Entscheidungen

- D-021: Desktop-only in Version 1.0
- D-022: Modulare Entwicklung
- D-023: Kein direkter Datenbankzugriff durch das Frontend
- D-024: Dateien im Dokumentenspeicher, Metadaten in SQLite
- D-025: Fachliche Regeln im Backend
