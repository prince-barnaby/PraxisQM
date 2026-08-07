# Kapitel 4B

## DB-001 Documents – Felddefinitionen

| Feld | Typ | Beschreibung |
|---|---|---|
| Dokumentennummer | String | Automatisch vergeben, eindeutig, unveränderlich, wird niemals wiederverwendet (ADR-001) |
| Titel | String | Bezeichnung des Dokuments |
| Kategorie | String | Hauptkategorie |
| Unterkategorie | String | Unterkategorie |
| Verantwortliche Person | String | Zuständige Mitarbeitende |
| Version | String | Versionsnummer |
| Status | String | Bearbeitungsstatus (Entwurf, aktiv, archiviert) |
| Gültigkeit | String | Gültigkeitsstatus (gültig, läuft bald ab, abgelaufen) |
| Gültig bis | Date | Ablaufdatum der Gültigkeit |
| Beschreibung | Text | Beschreibender Text |
| Tags | Liste<String> | Schlagwörter |
| Archivierungsdatum | DateTime | Automatisch gesetzt, wenn ein Dokument archiviert wird. Leer für aktive Dokumente. Wird für Archiv-Sortierung und Archiv-Filterung (Archivierungszeitraum) verwendet. |

## DB-003 Employees – Felddefinitionen

| Feld | Typ | Beschreibung |
|---|---|---|
| Name | String | Nachname der Mitarbeitenden |
| Vorname | String | Vorname der Mitarbeitenden |
| Funktion | String | Position / Rolle in der Praxis (z. B. Zahnärztin, ZFA, Praxismanagerin) |
| Bereich | String | Abteilung / Bereich der Praxis (z. B. Behandlung, Empfang, Verwaltung) |
| Verantwortungsposition | Liste<String> | Multi-Value-Feld. Null, eine oder mehrere Verantwortungspositionen (z. B. QM-Beauftragte, Datenschutzbeauftragte). Many-to-many-Beziehung: employee ↔ Verantwortungsposition. |
| Zugeordneter QM-Bereich | Liste<String> | Multi-Value-Feld. Null, ein oder mehrere zugeordnete QM-Bereiche (z. B. Datenschutz, Patientendokumentation, Röntgeneinweisung). Many-to-many-Beziehung: employee ↔ QM-Bereich. |
| Aktivstatus | Boolean | Aktiv / inaktiv. Wird über StatusBadge dargestellt (success = aktiv, neutral = inaktiv). |
| E-Mail | String | Kontakt-E-Mail-Adresse |
| Telefonnummer | String | Kontakt-Telefonnummer |
| Eintrittsdatum | Date | Eintrittsdatum in die Praxis |
| Austrittsdatum | Date | Austrittsdatum aus der Praxis (leer / „—" bei aktiven Mitarbeitenden) |

### Multi-Value-Felder – Datenmodellregel

Die Felder **Verantwortungsposition** und **Zugeordneter QM-Bereich** sind Multi-Value-Felder.

- Ein Mitarbeiter kann **null, eine oder mehrere** Verantwortungspositionen haben.
- Ein Mitarbeiter kann **null, einem oder mehreren** QM-Bereichen zugeordnet sein.
- Beide Felder dürfen **nicht** als einzelner String oder als Single-Select-Wert modelliert werden.
- Das zukünftige DB-003 Employees-Datenmodell muss **Many-to-Many-Zuweisungen** unterstützen für:
  - employee ↔ Verantwortungsposition
  - employee ↔ QM-Bereich
- Datenbank-Verknüpfungstabellen (Join-Tables) werden noch nicht implementiert.
- In der aktuellen UI-Only-Implementierung werden mehrere Werte als klar getrennte Badges/Chips dargestellt.
- Die Platzhalterwerte sind Beispiele und keine festen Geschäftsregeln.
