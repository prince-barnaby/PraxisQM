import { Search } from "lucide-react";
import SessionControls from "./auth/SessionControls";
import "./Header.css";

// PraxisQM – Header
// Modul: Navigation
// Zweck: Oberer Kopfbereich mit Anwendungstitel, Platzhalter-Suchleiste und Sitzungsstatus.

export default function Header() {
  return (
    <header className="pqm-header">
      <h1 className="pqm-header__title">Qualitätsmanagement</h1>
      <div className="pqm-header__search">
        <Search size={18} color="var(--pqm-color-neutral)" />
        <span className="pqm-header__search-placeholder">Suche (Platzhalter)</span>
      </div>
      <SessionControls />
    </header>
  );
}

