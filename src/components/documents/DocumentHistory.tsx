import { Clock } from "lucide-react";
import "./DocumentHistory.css";

export interface HistoryEntry {
  id: string;
  label: string;
}

interface DocumentHistoryProps {
  entries: HistoryEntry[];
}

export default function DocumentHistory({ entries }: DocumentHistoryProps) {
  return (
    <section className="pqm-document-history" aria-label="Versionshistorie">
      <h3 className="pqm-document-history__heading">
        <Clock size={16} aria-hidden="true" />
        Versionshistorie
      </h3>
      <ol className="pqm-document-history__timeline">
        {entries.map((entry) => (
          <li key={entry.id} className="pqm-document-history__item">
            <span className="pqm-document-history__dot" aria-hidden="true" />
            <span className="pqm-document-history__label">{entry.label}</span>
          </li>
        ))}
      </ol>
    </section>
  );
}
