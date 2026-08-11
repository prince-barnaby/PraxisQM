import { FileText, Pencil, Archive, FilePlus } from "lucide-react";
import "./DocumentActionBar.css";

interface DocumentActionBarProps {
  pdfFileName: string;
  onEdit?: () => void;
  onNewVersion?: () => void;
}

export default function DocumentActionBar({ pdfFileName, onEdit, onNewVersion }: DocumentActionBarProps) {
  return (
    <div
      className="pqm-action-bar"
      role="toolbar"
      aria-label="Dokument-Aktionen"
    >
      <button
        type="button"
        className="pqm-action-bar__button pqm-action-bar__button--primary"
        onClick={onEdit}
        disabled={!onEdit}
        aria-label="Bearbeiten"
      >
        <Pencil size={16} aria-hidden="true" />
        Bearbeiten
      </button>
      <button
        type="button"
        className="pqm-action-bar__button pqm-action-bar__button--primary"
        onClick={onNewVersion}
        disabled={!onNewVersion}
        aria-label="Neue Version erstellen"
      >
        <FilePlus size={16} aria-hidden="true" />
        Neue Version
      </button>
      <button
        type="button"
        className="pqm-action-bar__button"
        disabled
        aria-label={`PDF öffnen – noch nicht implementiert – ${pdfFileName}`}
      >
        <FileText size={16} aria-hidden="true" />
        PDF öffnen
      </button>
      <button
        type="button"
        className="pqm-action-bar__button pqm-action-bar__button--danger"
        disabled
        aria-label="Archivieren – noch nicht implementiert"
      >
        <Archive size={16} aria-hidden="true" />
        Archivieren
      </button>
    </div>
  );
}
