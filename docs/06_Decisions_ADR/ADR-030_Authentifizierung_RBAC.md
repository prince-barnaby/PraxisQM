# ADR-030 – Lokale Authentifizierung und rollenbasierte Zugriffskontrolle

## Status

Akzeptiert

## Datum

2026-10-28

## Kontext

PraxisQM verwaltet sensible QM-Dokumente einer Zahnarztpraxis. Bisher gab es
keine Authentifizierung — alle Funktionen waren für jeden zugänglich, der die
Anwendung bedient.

Die Anforderungen sehen drei Zugriffsstufen vor:

- **Gast** (nicht angemeldet): Lesezugriff auf veröffentlichte Dokumente, PDF-Anzeige.
- **Editor** (angemeldet): Alle Gast-Rechte + Dokumente erstellen, bearbeiten, versionieren, archivieren.
- **Admin** (angemeldet): Vollzugriff inkl. Benutzerverwaltung, Mitarbeiterverwaltung, Einstellungen.

ADR-002 (Offline-Betrieb) und ADR-027 (Application Stack) schließen
Cloud-Authentifizierung aus. Die Authentifizierung muss vollständig lokal
funktionieren.

ADR-004 trennt Benutzerkonten (DB-004) vom Mitarbeiterregister (DB-003).
Diese Trennung bleibt bestehen.

## Entscheidung

### 1. Lokale Passwort-Authentifizierung mit Argon2id

Passwörter werden mit Argon2id (OWASP-empfohlen) gehasht und als PHC-String
in der `password_hash`-Spalte der `users`-Tabelle gespeichert.

- Keine Klartext-Passwörter in der Datenbank.
- Keine fest codierten Administrator-Anmeldedaten.
- Die `argon2` Rust-K Crate wird verwendet (pure Rust, keine nativen Abhängigkeiten).

### 2. In-Memory Session-Verwaltung

Die Session wird als Rust-managed State (`AuthState`) gehalten, nicht in der
Datenbank persistiert. Die Session verfällt beim Schließen der Anwendung.

- `AuthState` enthält `Mutex<Option<Session>>` und Login-Versuch-Tracking.
- Bei jedem App-Start muss sich der Benutzer erneut anmelden.
- Gast-Modus (keine Session) ist der Standard nach App-Start.

### 3. Backend-enforcement der Autorisierung

Die Autorisierung wird im Rust-Backend erzwungen, nicht nur durch
Frontend-Sichtbarkeit. Jeder Tauri-Command prüft die Rolle vor Ausführung.

- `require_role(&session, min_role)` helper.
- Rollen-Rang: Gast (0) < Editor (1) < Admin (2).
- Bei fehlender Berechtigung: deutsche Fehlermeldung, keine Internals offenbart.

### 4. Phasenweise Aktivierung

Die Autorisierung wird in zwei Phasen aktiviert:

- **Phase 1 (diese ADR):** Auth-Infrastruktur, Passwort-Hashing, Session-Management,
  Auth-Commands. Enforcement-Code ist vorbereitet, aber noch nicht auf bestehende
  Commands angewendet, um die Entwicklung nicht zu blockieren.
- **Phase 2:** Frontend-Login-UI, dann Aktivierung des Enforcement auf allen
  bestehenden Commands.

Dies verhindert, dass eine teilweise implementierte Authentifizierung die
Anwendung unbrauchbar macht.

### 5. First-Run-Setup

Der erste Administrator-Account wird über einen Setup-Assistenten erstellt,
der nur aktiv wird, wenn die `users`-Tabelle leer ist.

- `cmd_needs_setup` prüft, ob Setup erforderlich ist.
- `cmd_setup_admin` erstellt den ersten Admin (nur bei leerer Tabelle).
- Nach dem Setup wird der Admin automatisch eingeloggt.
- Keine fest codierten Standard-Anmeldedaten.

### 6. Schutzmechanismen

- **Login-Sperre:** Nach 5 fehlgeschlagenen Versuchen innerhalb 60 Sekunden
  wird der Login für 120 Sekunden blockiert.
- **Letzter-Admin-Schutz:** Der letzte aktive Administrator kann nicht
  deaktiviert oder entfernt werden.
- **Deaktivierte Benutzer:** Verlieren sofort den Zugriff; `cmd_get_current_user`
  löscht die Session, wenn der Benutzer deaktiviert wurde.

### 7. Schema-Migration

Migration v2→v3 fügt die Spalte `password_hash TEXT` zur `users`-Tabelle hinzu.
Idempotent, nicht-destruktiv. Bestehende Daten bleiben erhalten.

## Konsequenzen

- Neue Abhängigkeit: `argon2` Crate (pure Rust, keine nativen Abhängigkeiten).
- Schema-Version wird von 2 auf 3 erhöht.
- `auth.rs`-Modul wird zum Tauri-Backend hinzugefügt.
- Frontend muss in Phase 2 Login-/Setup-Seiten und rollenbasierte Navigation implementieren.
- DB-004 Data Dictionary muss um `password_hash`-Feld ergänzt werden.

## Beziehung zu anderen ADRs

- **ADR-002:** Bestätigt — vollständig offline, keine Cloud-Authentifizierung.
- **ADR-004:** Bestätigt — Benutzerkonten bleiben vom Mitarbeiterregister getrennt.
- **ADR-027:** Bestätigt — kein Supabase, keine externen Auth-Dienste.
