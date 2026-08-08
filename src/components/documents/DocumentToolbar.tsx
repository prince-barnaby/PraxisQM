import { Plus, Search } from "lucide-react";
import { useNavigate } from "react-router-dom";
import "./DocumentToolbar.css";

interface DocumentToolbarProps {
  resultCount: number;
}

export default function DocumentToolbar({ resultCount }: DocumentToolbarProps) {
  const navigate = useNavigate();

  return (
    <header className="pqm-document-toolbar">
      <div className="pqm-document-toolbar__heading">
        <h2 className="pqm-document-toolbar__title">Dokumente</h2>
        <p className="pqm-document-toolbar__subtitle">
          Verwaltung und Übersicht aller Qualitätsmanagement-Dokumente
        </p>
      </div>
      <div className="pqm-document-toolbar__actions">
        <div
          className="pqm-document-toolbar__search"
          aria-label="Suchfeld – Platzhalter"
        >
          <Search size={16} aria-hidden="true" />
          <input
            type="text"
            placeholder="Suche – Platzhalter"
            disabled
            aria-label="Dokumentsuche (Platzhalter)"
          />
        </div>
        <button
          type="button"
          className="pqm-document-toolbar__button"
          onClick={() => navigate("/dokumente/neu")}
          aria-label="Neues Dokument erstellen"
        >
          <Plus size={18} aria-hidden="true" />
          Neues Dokument
        </button>
      </div>
      <p
        className="pqm-document-toolbar__count"
        aria-label={`Anzahl Treffer: ${resultCount}`}
      >
        {resultCount} Dokumente
      </p>
    </header>
  );
}
