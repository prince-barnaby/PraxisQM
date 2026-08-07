import type { BadgeVariant } from "../documents/StatusBadge";
import StatusBadge from "../documents/StatusBadge";

export interface ArchiveRowData {
  id: string;
  documentNumber: string;
  title: string;
  category: string;
  subcategory: string;
  responsible: string;
  version: string;
  archivedAt: string;
  status: string;
  statusVariant: BadgeVariant;
}

interface ArchiveRowProps {
  entry: ArchiveRowData;
}

export default function ArchiveRow({ entry }: ArchiveRowProps) {
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTableRowElement>) => {
    if (e.key === "Enter") {
      e.preventDefault();
    }
  };

  return (
    <tr
      className="pqm-archive-row"
      tabIndex={0}
      role="link"
      aria-label={`Archiviertes Dokument ${entry.documentNumber} – ${entry.title}`}
      onKeyDown={handleKeyDown}
    >
      <td className="pqm-archive-row__number">{entry.documentNumber}</td>
      <td className="pqm-archive-row__title">{entry.title}</td>
      <td className="pqm-archive-row__category">{entry.category}</td>
      <td className="pqm-archive-row__subcategory">{entry.subcategory}</td>
      <td className="pqm-archive-row__responsible">{entry.responsible}</td>
      <td className="pqm-archive-row__version">{entry.version}</td>
      <td className="pqm-archive-row__archived-at">{entry.archivedAt}</td>
      <td className="pqm-archive-row__status">
        <StatusBadge label={entry.status} variant={entry.statusVariant} />
      </td>
    </tr>
  );
}
