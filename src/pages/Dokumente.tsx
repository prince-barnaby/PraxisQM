import { useCallback, useEffect, useState } from "react";
import DocumentToolbar from "../components/documents/DocumentToolbar";
import DocumentFilters from "../components/documents/DocumentFilters";
import DocumentList from "../components/documents/DocumentList";
import type { DocumentRowData } from "../components/documents/DocumentRow";
import type { BadgeVariant } from "../components/documents/StatusBadge";
import { fetchDocuments, type Document } from "../lib/documentApi";
import "./Dokumente.css";

function validityToVariant(validity: string): BadgeVariant {
  switch (validity) {
    case "gültig":
      return "success";
    case "läuft bald ab":
      return "warning";
    case "abgelaufen":
      return "error";
    default:
      return "neutral";
  }
}

function statusToVariant(status: string): BadgeVariant {
  switch (status) {
    case "aktiv":
      return "success";
    case "Entwurf":
      return "neutral";
    case "archiviert":
      return "neutral";
    default:
      return "neutral";
  }
}

function toRowData(doc: Document): DocumentRowData {
  return {
    id: doc.id,
    documentNumber: doc.document_number,
    title: doc.title,
    category: doc.category_name ?? "—",
    subcategory: doc.subcategory_name ?? "—",
    status: doc.status,
    statusVariant: statusToVariant(doc.status),
    responsible: doc.responsible_person_name ?? "—",
    validity: doc.validity,
    validityVariant: validityToVariant(doc.validity),
    version: doc.version,
  };
}

export default function Dokumente() {
  const [documents, setDocuments] = useState<DocumentRowData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadDocuments = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const docs = await fetchDocuments();
      setDocuments(docs.map(toRowData));
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadDocuments();
  }, [loadDocuments]);

  return (
    <div className="pqm-dokumente">
      <DocumentToolbar resultCount={documents.length} />
      <DocumentFilters />
      {error && (
        <p className="pqm-dokumente__error" role="alert">
          Fehler beim Laden der Dokumente: {error}
        </p>
      )}
      <DocumentList documents={documents} loading={loading} />
    </div>
  );
}
