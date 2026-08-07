import "./ArchiveToolbar.css";

interface ArchiveToolbarProps {
  resultCount: number;
}

export default function ArchiveToolbar({ resultCount }: ArchiveToolbarProps) {
  return (
    <header className="pqm-archive-toolbar">
      <div className="pqm-archive-toolbar__heading">
        <h2 className="pqm-archive-toolbar__title">Archiv</h2>
        <p className="pqm-archive-toolbar__subtitle">
          Archivierte Qualitätsmanagement-Dokumente – nachvollziehbar, dauerhaft, ohne Wiederverwendung von Dokumentnummern
        </p>
      </div>
      <p
        className="pqm-archive-toolbar__count"
        aria-label={`Anzahl archivierte Dokumente: ${resultCount}`}
      >
        {resultCount} archivierte Dokumente
      </p>
    </header>
  );
}
