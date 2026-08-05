# ADR-007 - Systemarchitektur

## Status
Beschlossen

## Entscheidung
PraxisQM Version 1.0 wird als lokale Desktop-Webanwendung mit modularer Architektur entwickelt.

Die Architektur besteht aus Frontend, Backend, SQLite-Datenbank, Dokumentenspeicher, Backup-Modul und Admin-Protokollierung.

## Begründung
Die Trennung der Module erhöht Wartbarkeit, Sicherheit und Erweiterbarkeit. Das Frontend darf keine direkte Datenbanklogik enthalten, damit Berechtigungen, Versionierung, Protokollierung und fachliche Regeln nicht umgangen werden können.

## Konsequenzen
- Frontend kommuniziert ausschließlich mit dem Backend.
- Das Backend enthält die Geschäftslogik.
- SQLite speichert Metadaten.
- Dateien liegen im lokalen Dokumentenspeicher.
- Version 1.0 wird nicht mobil optimiert.
