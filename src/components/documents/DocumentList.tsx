import { FileText } from "lucide-react";
import "./DocumentList.css";
import type { DocumentRowData } from "./DocumentRow";
import DocumentRow from "./DocumentRow";
import EmptyState from "./EmptyState";

interface DocumentListProps {
  documents: DocumentRowData[];
  loading?: boolean;
}

export default function DocumentList({ documents, loading = false }: DocumentListProps) {
  if (loading) {
    return (
      <div className="pqm-document-list" role="region" aria-label="Dokumentenliste wird geladen">
        <p className="pqm-document-list__loading">Dokumente werden geladen …</p>
      </div>
    );
  }

  if (documents.length === 0) {
    return (
      <EmptyState
        icon={FileText}
        title="Noch keine Dokumente erfasst"
        message="Es wurden noch keine Dokumente angelegt. Klicken Sie auf „Neues Dokument“, um ein QM-Dokument zu erstellen."
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
