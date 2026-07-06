# PraxisQM – Code Map

## Zweck dieses Dokuments

Die Code Map ist der technische Lageplan der PraxisQM-Anwendung.

Sie dokumentiert, welches UI-Element, welche Funktion und welches Modul in welcher Datei umgesetzt ist. Dadurch bleibt der Code auch dann nachvollziehbar, wenn die Anwendung wächst oder später durch KI-gestützte Entwicklung erweitert wird.

Dieses Dokument muss bei neuen Modulen, Komponenten, Buttons, Dialogen, Services oder zentralen Funktionen aktualisiert werden.

---

## Status-Legende

| Status | Bedeutung |
|---|---|
| geplant | Funktion ist fachlich vorgesehen, aber noch nicht umgesetzt |
| vorbereitet | Datei oder Struktur existiert, Funktion ist aber noch nicht vollständig aktiv |
| in Entwicklung | Funktion wird aktuell umgesetzt |
| aktiv | Funktion ist umgesetzt und nutzbar |
| prüfen | Funktion existiert, muss aber getestet oder fachlich geprüft werden |
| überarbeiten | Funktion existiert, muss aber angepasst werden |
| archiviert | Funktion wurde ersetzt oder wird nicht mehr aktiv verwendet |

---

## Modulübersicht

| Modul | Zweck | Hauptordner | Status |
|---|---|---|---|
| Dashboard | Startseite mit Übersicht, Statuskarten und Schnellzugriffen | `source/` | geplant |
| Navigation | Sidebar, Header, Seitenwechsel | `source/` | geplant |
| Dokumente | Dokumentenverwaltung, Listen, Upload, Versionierung | `source/` | geplant |
| Archiv | Archivierte Dokumente und Wiederherstellung | `source/` | geplant |
| Mitarbeiter | Mitarbeiterverwaltung, Rollen, Schulungen | `source/` | geplant |
| Einstellungen | Praxisdaten, Systemoptionen, Backup-Einstellungen | `source/` | geplant |
| Datenhaltung | lokale Speicherung und Datenmodell | `database/` / `source/` | geplant |
| Backup | Export, Import, Sicherung, Wiederherstellung | `source/` | geplant |
| Tests | Testfälle und Qualitätssicherung | `tests/` | geplant |

---

## UI-Elemente und Komponenten

| UI-Element | Modul | Datei | Funktion / Komponente | Zweck | Status |
|---|---|---|---|---|---|
| App-Shell | Grundstruktur | `source/` | noch offen | Grundlayout der Anwendung | geplant |
| Sidebar | Navigation | `source/` | noch offen | Hauptnavigation links | geplant |
| Header | Navigation | `source/` | noch offen | Oberer Seitenbereich mit Titel und Status | geplant |
| Dashboard-Karte „Dokumente“ | Dashboard | `source/` | noch offen | Schnellübersicht Dokumente | geplant |
| Dashboard-Karte „Mitarbeiter“ | Dashboard | `source/` | noch offen | Schnellübersicht Mitarbeiter | geplant |
| Dashboard-Karte „Archiv“ | Dashboard | `source/` | noch offen | Schnellzugriff Archiv | geplant |
| Dashboard-Karte „Systemstatus“ | Dashboard | `source/` | noch offen | Anzeige lokaler Systeminformationen | geplant |
| Button „Neues Dokument“ | Dokumente | `source/` | noch offen | Startet später den Dokumenten-Upload | geplant |
| Button „Dokument archivieren“ | Dokumente / Archiv | `source/` | noch offen | Verschiebt Dokument ins Archiv | geplant |
| Suchfeld Dokumente | Dokumente | `source/` | noch offen | Filtert Dokumentenliste | geplant |
| Mitarbeiterliste | Mitarbeiter | `source/` | noch offen | Zeigt Mitarbeitende der Praxis | geplant |
| Button „Mitarbeiter hinzufügen“ | Mitarbeiter | `source/` | noch offen | Öffnet Formular für neuen Mitarbeiter | geplant |
| Einstellungen-Seite | Einstellungen | `source/` | noch offen | System- und Praxiseinstellungen | geplant |

---

## Services und zentrale Logik

| Service / Funktion | Datei | Zweck | Verwendet von | Status |
|---|---|---|---|---|
| Dokument speichern | noch offen | Speichert Dokumentmetadaten lokal | Dokumente | geplant |
| Dokument archivieren | noch offen | Markiert Dokument als archiviert | Dokumente / Archiv | geplant |
| Dokument wiederherstellen | noch offen | Holt Dokument aus dem Archiv zurück | Archiv | geplant |
| Dokument suchen | noch offen | Filtert Dokumente nach Suchbegriff | Dokumente | geplant |
| Mitarbeiter speichern | noch offen | Speichert Mitarbeiterdaten lokal | Mitarbeiter | geplant |
| Backup erstellen | noch offen | Erstellt lokale Sicherung | Einstellungen / Backup | geplant |
| Backup wiederherstellen | noch offen | Importiert lokale Sicherung | Einstellungen / Backup | geplant |
| Systemstatus prüfen | noch offen | Prüft lokale App- und Speicherinformationen | Dashboard | geplant |

---

## Datenmodelle

| Datenmodell | Datei | Zweck | Status |
|---|---|---|---|
| Document | noch offen | Struktur für QM-Dokumente | geplant |
| Employee | noch offen | Struktur für Mitarbeitende | geplant |
| Role | noch offen | Rollen und Berechtigungen | geplant |
| TrainingRecord | noch offen | Schulungs- und Kenntnisnachweise | geplant |
| ArchiveEntry | noch offen | Archivierte Objekte | geplant |
| BackupRecord | noch offen | Sicherungen und Wiederherstellungen | geplant |

---

## Prompt-Verknüpfung

| Prompt | Zielmodul | Ergebnisdateien | Status |
|---|---|---|---|
| Prompt 001 – Projektgrundgerüst | Grundstruktur | noch offen | geplant |
| Prompt 002 – Navigation | Navigation | noch offen | geplant |
| Prompt 003 – Dashboard | Dashboard | noch offen | geplant |
| Prompt 004 – Dokumentenübersicht | Dokumente | noch offen | geplant |
| Prompt 005 – Archivgrundlage | Archiv | noch offen | geplant |

---

## Änderungsprotokoll Code Map

| Datum | Änderung | Autor |
|---|---|---|
| 2026-07-06 | Initiale Code Map erstellt | Saskia / ChatGPT |

---

## Pflege-Regeln

1. Jede neue Komponente wird in dieser Datei eingetragen.
2. Jeder neue Button bekommt einen Eintrag.
3. Jede zentrale Funktion bekommt einen Eintrag.
4. Wenn Bolt Code erzeugt, muss geprüft werden, ob die Code Map angepasst werden muss.
5. Datei- und Funktionsnamen müssen nach der Umsetzung aktualisiert werden.
6. Platzhalter wie „noch offen“ werden ersetzt, sobald konkrete Dateien existieren.
7. Die Code Map darf nicht veralten, da sie als Orientierung für Entwicklung, Review und spätere Wartung dient.
