import { FileText, Pencil, Archive, FilePlus, RotateCcw } from "lucide-react";
import "./DocumentActionBar.css";

interface DocumentActionBarProps {
  pdfFileName: string;
  isArchived: boolean;
  onEdit?: () => void;
  onNewVersion?: () => void;
  onArchive?: () => void;
  onRestore?: () => void;
}

export default function DocumentActionBar({
  pdfFileName,
  isArchived,
  onEdit,
  onNewVersion,
  onArchive,
  onRestore,
}: DocumentActionBarProps) {
  return (
    <div
      className="pqm-action-bar"
      role="toolbar"
      aria-label="Dokument-Aktionen"
    >
      {!isArchived && (
        <>
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
            onClick={onArchive}
            disabled={!onArchive}
            aria-label="Dokument archivieren"
          >
            <Archive size={16} aria-hidden="true" />
            Archivieren
          </button>
        </>
      )}
      {isArchived && (
        <>
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
            className="pqm-action-bar__button pqm-action-bar__button--primary"
            onClick={onRestore}
            disabled={!onRestore}
            aria-label="Dokument wiederherstellen"
          >
            <RotateCcw size={16} aria-hidden="true" />
            Wiederherstellen
          </button>
        </>
      )}
    </div>
  );
}
