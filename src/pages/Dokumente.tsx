import DocumentToolbar from "../components/documents/DocumentToolbar";
import DocumentFilters from "../components/documents/DocumentFilters";
import DocumentList from "../components/documents/DocumentList";
import type { DocumentRowData } from "../components/documents/DocumentRow";
import "./Dokumente.css";

const PLACEHOLDER_DOCUMENTS: DocumentRowData[] = [
  {
    id: "mock-1",
    documentNumber: "PQM-0001",
    title: "Platzhalter-Dokument 1",
    category: "Platzhalter",
    subcategory: "Platzhalter",
    status: "aktiv",
    statusVariant: "success",
    responsible: "Platzhalter",
    validity: "gültig",
    validityVariant: "success",
    version: "1.0",
  },
  {
    id: "mock-2",
    documentNumber: "PQM-0002",
    title: "Platzhalter-Dokument 2",
    category: "Platzhalter",
    subcategory: "Platzhalter",
    status: "Entwurf",
    statusVariant: "neutral",
    responsible: "Platzhalter",
    validity: "läuft bald ab",
    validityVariant: "warning",
    version: "0.9",
  },
  {
    id: "mock-3",
    documentNumber: "PQM-0003",
    title: "Platzhalter-Dokument 3",
    category: "Platzhalter",
    subcategory: "Platzhalter",
    status: "archiviert",
    statusVariant: "neutral",
    responsible: "Platzhalter",
    validity: "abgelaufen",
    validityVariant: "error",
    version: "2.1",
  },
];

export default function Dokumente() {
  return (
    <div className="pqm-dokumente">
      <DocumentToolbar resultCount={PLACEHOLDER_DOCUMENTS.length} />
      <DocumentFilters />
      <DocumentList documents={PLACEHOLDER_DOCUMENTS} />
      <p className="pqm-dokumente__hint">
        Hinweis: Alle Einträge sind Platzhalter. Suche, Filter und Aktionen sind nicht funktional.
      </p>
    </div>
  );
}
