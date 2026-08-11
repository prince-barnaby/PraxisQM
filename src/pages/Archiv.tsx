import { useCallback, useEffect, useMemo, useState } from "react";
import ArchiveToolbar from "../components/archive/ArchiveToolbar";
import ArchiveFilters from "../components/archive/ArchiveFilters";
import ArchiveList from "../components/archive/ArchiveList";
import type { ArchiveRowData } from "../components/archive/ArchiveRow";
import type { BadgeVariant } from "../components/documents/StatusBadge";
import { fetchArchivedDocuments, type Document } from "../lib/documentApi";
import "./Archiv.css";

function statusToVariant(status: string): BadgeVariant {
  switch (status) {
    case "aktiv":
      return "success";
    case "archiviert":
      return "neutral";
    case "Entwurf":
      return "neutral";
    default:
      return "neutral";
  }
}

function formatArchivedAt(archivedAt: string | null): string {
  if (!archivedAt) return "—";
  const secs = Number(archivedAt);
  if (Number.isNaN(secs) || secs === 0) return "—";
  return new Date(secs * 1000).toLocaleString("de-DE");
}

function toArchiveRowData(doc: Document): ArchiveRowData {
  return {
    id: doc.id,
    documentNumber: doc.document_number,
    title: doc.title,
    category: doc.category_name ?? "—",
    subcategory: doc.subcategory_name ?? "—",
    responsible: doc.responsible_person_name ?? "—",
    version: doc.version,
    archivedAt: formatArchivedAt(doc.archived_at),
    status: doc.status,
    statusVariant: statusToVariant(doc.status),
  };
}

export interface ArchiveFilterValues {
  category: string;
  subcategory: string;
  responsible: string;
  status: string;
}

export default function Archiv() {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filters, setFilters] = useState<ArchiveFilterValues>({
    category: "",
    subcategory: "",
    responsible: "",
    status: "",
  });

  const loadArchived = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const docs = await fetchArchivedDocuments();
      setDocuments(docs);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadArchived();
  }, [loadArchived]);

  const categories = useMemo(
    () => [...new Set(documents.map((d) => d.category_name).filter(Boolean))] as string[],
    [documents],
  );
  const subcategories = useMemo(
    () => [...new Set(documents.map((d) => d.subcategory_name).filter(Boolean))] as string[],
    [documents],
  );
  const responsibles = useMemo(
    () => [...new Set(documents.map((d) => d.responsible_person_name).filter(Boolean))] as string[],
    [documents],
  );
  const statuses = useMemo(
    () => [...new Set(documents.map((d) => d.status).filter(Boolean))] as string[],
    [documents],
  );

  const filtered = useMemo(() => {
    return documents.filter((doc) => {
      if (filters.category && doc.category_name !== filters.category) return false;
      if (filters.subcategory && doc.subcategory_name !== filters.subcategory) return false;
      if (filters.responsible && doc.responsible_person_name !== filters.responsible) return false;
      if (filters.status && doc.status !== filters.status) return false;
      return true;
    });
  }, [documents, filters]);

  const rows = filtered.map(toArchiveRowData);

  return (
    <div className="pqm-archiv">
      <ArchiveToolbar resultCount={rows.length} />
      {error && (
        <p className="pqm-archiv__error" role="alert">
          Fehler beim Laden des Archivs: {error}
        </p>
      )}
      <ArchiveFilters
        categories={categories}
        subcategories={subcategories}
        responsibles={responsibles}
        statuses={statuses}
        values={filters}
        onChange={setFilters}
      />
      <ArchiveList entries={rows} loading={loading} />
    </div>
  );
}
