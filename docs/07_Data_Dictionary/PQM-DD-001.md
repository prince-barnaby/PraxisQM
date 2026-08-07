# Data Dictionary

## DB-001 Documents

Stammdaten eines QM-Dokuments.

| Feld | Typ | Beschreibung |
|---|---|---|
| Dokumentennummer | String | Automatisch vergeben, eindeutig, unveränderlich, wird niemals wiederverwendet (ADR-001) |
| Titel | String | Bezeichnung des Dokuments |
| Kategorie | String | Hauptkategorie |
| Unterkategorie | String | Unterkategorie |
| Verantwortliche Person | String | Zuständige Mitarbeitende |
| Version | String | Versionsnummer des Dokuments |
| Status | String | Bearbeitungsstatus (z. B. Entwurf, aktiv, archiviert) |
| Gültigkeit | String | Gültigkeitsstatus (gültig, läuft bald ab, abgelaufen) |
| Gültig bis | Date | Ablaufdatum der Gültigkeit |
| Beschreibung | Text | Beschreibender Text |
| Tags | Liste<String> | Schlagwörter |
| Archivierungsdatum | DateTime | Automatisch gesetzt, wenn ein Dokument archiviert wird. Leer für aktive Dokumente. Wird für Archiv-Sortierung und Archiv-Filterung verwendet. |
