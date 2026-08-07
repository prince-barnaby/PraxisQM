import "./DocumentList.css";
import type { DocumentRowData } from "./DocumentRow";
import DocumentRow from "./DocumentRow";
import EmptyState from "./EmptyState";

interface DocumentListProps {
  documents: DocumentRowData[];
}

export default function DocumentList({ documents }: DocumentListProps) {
  if (documents.length === 0) {
    return (
      <EmptyState
        title="Keine Dokumente vorhanden"
        message="Es wurden noch keine Dokumente angelegt. Diese Ansicht ist ein Platzhalter."
      />
    );
  }

  return (
    <div className="pqm-document-list" role="region" aria-label="Dokumentenliste">
      <table className="pqm-document-list__table">
        <thead>
          <tr>
            <th scope="col">Dokumentennummer</th>
            <th scope="col">Titel</th>
            <th scope="col">Kategorie</th>
            <th scope="col">Unterkategorie</th>
            <th scope="col">Status</th>
            <th scope="col">Verantwortlich</th>
            <th scope="col">Gültigkeit</th>
            <th scope="col">Version</th>
          </tr>
        </thead>
        <tbody>
          {documents.map((doc) => (
            <DocumentRow key={doc.id} document={doc} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
