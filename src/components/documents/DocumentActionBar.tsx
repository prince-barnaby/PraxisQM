import { FileText, Pencil, Archive } from "lucide-react";
import "./DocumentActionBar.css";

interface DocumentActionBarProps {
  pdfFileName: string;
}

export default function DocumentActionBar({ pdfFileName }: DocumentActionBarProps) {
  return (
    <div
      className="pqm-action-bar"
      role="toolbar"
      aria-label="Dokument-Aktionen"
    >
      <button
        type="button"
        className="pqm-action-bar__button pqm-action-bar__button--primary"
        disabled
        aria-label="Bearbeiten – Platzhalter, nicht funktional"
      >
        <Pencil size={16} aria-hidden="true" />
        Bearbeiten
      </button>
      <button
        type="button"
        className="pqm-action-bar__button"
        disabled
        aria-label={`PDF öffnen – ${pdfFileName} – Platzhalter, nicht funktional`}
      >
        <FileText size={16} aria-hidden="true" />
        PDF öffnen
      </button>
      <button
        type="button"
        className="pqm-action-bar__button pqm-action-bar__button--danger"
        disabled
        aria-label="Archivieren – Platzhalter, nicht funktional"
      >
        <Archive size={16} aria-hidden="true" />
        Archivieren
      </button>
    </div>
  );
}
