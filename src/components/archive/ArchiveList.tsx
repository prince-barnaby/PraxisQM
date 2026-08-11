import "./ArchiveList.css";
import type { ArchiveRowData } from "./ArchiveRow";
import ArchiveRow from "./ArchiveRow";
import EmptyState from "../documents/EmptyState";

interface ArchiveListProps {
  entries: ArchiveRowData[];
  loading?: boolean;
}

export default function ArchiveList({ entries, loading }: ArchiveListProps) {
  if (loading) {
    return (
      <div className="pqm-archive-list" role="region" aria-label="Archivliste">
        <p className="pqm-archive-list__loading">Archiv wird geladen …</p>
      </div>
    );
  }

  if (entries.length === 0) {
    return (
      <EmptyState
        title="Keine archivierten Dokumente gefunden"
        message="Es wurden noch keine Dokumente archiviert oder die aktiven Filter ergeben keine Treffer."
      />
    );
  }

  return (
    <div className="pqm-archive-list" role="region" aria-label="Archivliste">
      <table className="pqm-archive-list__table">
        <thead>
          <tr>
            <th scope="col">Dokumentennummer</th>
            <th scope="col">Titel</th>
            <th scope="col">Kategorie</th>
            <th scope="col">Unterkategorie</th>
            <th scope="col">Verantwortlich</th>
            <th scope="col">Version</th>
            <th scope="col">Archivierungsdatum</th>
            <th scope="col">Status</th>
          </tr>
        </thead>
        <tbody>
          {entries.map((entry) => (
            <ArchiveRow key={entry.id} entry={entry} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
