# Kapitel 4A

## Tabellen

### Haupttabellen

- DB-001 Documents: Stammdaten eines Dokuments.
- DB-002 DocumentVersions: Alle Versionen eines Dokuments.
- DB-003 Employees: Alle Mitarbeiter der Praxis.
- DB-004 Users: Benutzer mit Login und Rollen.
- DB-005 Categories: Hauptkategorien.
- DB-006 Subcategories: Unterkategorien.
- DB-007 KeywordDictionary: Verwaltung der zulässigen Schlagwörter.
- DB-008 DocumentTags: Verknüpfung Dokument ↔ Schlagwort (n:m Join-Tabelle).
- DB-009 AuditLog: Protokoll wichtiger Aktionen.
- DB-010 Backups: Informationen über Sicherungen.
- DB-011 Settings: Programmeinstellungen (reserviert / noch nicht implementiert).
- DB-012 BackupReminders: Verwaltung der Backup-Erinnerungen.

### Join-Tabellen (Prompt 012B)

- DB-013 EmployeeResponsibilities: Verknüpfung Mitarbeiter ↔ Verantwortungsposition (n:m).
- DB-014 EmployeeQMAreas: Verknüpfung Mitarbeiter ↔ QM-Bereich (n:m).

### Primärschlüssel-Strategie

- Interne Datenbank-IDs verwenden UUIDs.
- Dokumentennummern sind Unique-Felder, nicht Primärschlüssel.
- Join-Tabellen verwenden Composite Keys.

### Implementierungs-Readiness

- **A (erforderlich für initiale SQLite-Foundation):** DB-001 bis DB-009, DB-013, DB-014
- **B (dokumentiert, Implementierung verschoben):** DB-010, DB-012
- **C (reserviert / zukünftig):** DB-011
