import ArchiveToolbar from "../components/archive/ArchiveToolbar";
import ArchiveFilters from "../components/archive/ArchiveFilters";
import ArchiveList from "../components/archive/ArchiveList";
import type { ArchiveRowData } from "../components/archive/ArchiveRow";
import "./Archiv.css";

const PLACEHOLDER_ARCHIVE_ENTRIES: ArchiveRowData[] = [
  {
    id: "arch-mock-1",
    documentNumber: "PQM-0003",
    title: "Platzhalter-Dokument 3",
    category: "Platzhalter",
    subcategory: "Platzhalter",
    responsible: "Platzhalter",
    version: "2.1",
    archivedAt: "2026-06-15 10:30",
    status: "archiviert",
    statusVariant: "neutral",
  },
  {
    id: "arch-mock-2",
    documentNumber: "PQM-0007",
    title: "Platzhalter-Dokument 7",
    category: "Platzhalter",
    subcategory: "Platzhalter",
    responsible: "Platzhalter",
    version: "1.4",
    archivedAt: "2026-05-02 14:12",
    status: "archiviert",
    statusVariant: "neutral",
  },
  {
    id: "arch-mock-3",
    documentNumber: "PQM-0012",
    title: "Platzhalter-Dokument 12",
    category: "Platzhalter",
    subcategory: "Platzhalter",
    responsible: "Platzhalter",
    version: "3.0",
    archivedAt: "2026-04-18 09:05",
    status: "archiviert",
    statusVariant: "neutral",
  },
];

export default function Archiv() {
  return (
    <div className="pqm-archiv">
      <ArchiveToolbar resultCount={PLACEHOLDER_ARCHIVE_ENTRIES.length} />
      <ArchiveFilters />
      <ArchiveList entries={PLACEHOLDER_ARCHIVE_ENTRIES} />
      <p className="pqm-archiv__hint">
        Hinweis: Alle Einträge sind Platzhalter. Filter und Aktionen sind nicht funktional.
      </p>
    </div>
  );
}
