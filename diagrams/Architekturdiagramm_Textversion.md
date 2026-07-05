# Architekturdiagramm - Textversion

```text
Benutzer
  |
  v
Desktop-Browser / Frontend
  |
  v
Backend / Geschäftslogik
  |--------------------|
  v                    v
SQLite-Datenbank       Dokumentenspeicher
  |                    |
  v                    v
Metadaten              PDF/DOCX/XLSX-Dateien

Backend -> Backup-Modul -> Backup-Ordner
Backend -> Admin-Protokoll -> SQLite/logs
```
