import "./ArchiveList.css";
import type { ArchiveRowData } from "./ArchiveRow";
import ArchiveRow from "./ArchiveRow";
import EmptyState from "../documents/EmptyState";

interface ArchiveListProps {
  entries: ArchiveRowData[];
}

export default function ArchiveList({ entries }: ArchiveListProps) {
  if (entries.length === 0) {
    return (
      <EmptyState
        title="Noch keine archivierten Dokumente vorhanden"
        message="Es wurden noch keine Dokumente archiviert. Diese Ansicht ist ein Platzhalter."
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
