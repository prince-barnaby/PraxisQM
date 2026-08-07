import type { LucideIcon } from "lucide-react";
import { useNavigate } from "react-router-dom";
import type { BadgeVariant } from "./StatusBadge";
import StatusBadge from "./StatusBadge";

export interface DocumentRowData {
  id: string;
  documentNumber: string;
  title: string;
  category: string;
  subcategory: string;
  status: string;
  statusVariant: BadgeVariant;
  responsible: string;
  validity: string;
  validityVariant: BadgeVariant;
  version: string;
}

interface DocumentRowProps {
  document: DocumentRowData;
  statusIcon?: LucideIcon;
  validityIcon?: LucideIcon;
}

export default function DocumentRow({
  document: doc,
  statusIcon,
  validityIcon,
}: DocumentRowProps) {
  const navigate = useNavigate();

  const handleActivate = () => {
    navigate(`/dokumente/${doc.documentNumber}`);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTableRowElement>) => {
    if (e.key === "Enter") {
      e.preventDefault();
      handleActivate();
    }
  };

  return (
    <tr
      className="pqm-document-row"
      tabIndex={0}
      role="link"
      aria-label={`Dokument ${doc.documentNumber} – ${doc.title} öffnen`}
      onClick={handleActivate}
      onKeyDown={handleKeyDown}
    >
      <td className="pqm-document-row__number">{doc.documentNumber}</td>
      <td className="pqm-document-row__title">{doc.title}</td>
      <td className="pqm-document-row__category">{doc.category}</td>
      <td className="pqm-document-row__subcategory">{doc.subcategory}</td>
      <td className="pqm-document-row__status">
        <StatusBadge label={doc.status} variant={doc.statusVariant} icon={statusIcon} />
      </td>
      <td className="pqm-document-row__responsible">{doc.responsible}</td>
      <td className="pqm-document-row__validity">
        <StatusBadge label={doc.validity} variant={doc.validityVariant} icon={validityIcon} />
      </td>
      <td className="pqm-document-row__version">{doc.version}</td>
    </tr>
  );
}
