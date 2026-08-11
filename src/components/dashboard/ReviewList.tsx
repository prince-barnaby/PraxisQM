import { CircleAlert as AlertCircle, Clock, ChevronRight } from "lucide-react";
import { useNavigate } from "react-router-dom";
import StatusBadge from "../documents/StatusBadge";
import type { BadgeVariant } from "../documents/StatusBadge";
import type { ReviewEntry } from "../../lib/dashboardApi";
import "./ReviewList.css";

function validityToVariant(validity: string): BadgeVariant {
  switch (validity) {
    case "abgelaufen":
      return "error";
    case "läuft bald ab":
      return "warning";
    default:
      return "neutral";
  }
}

function formatDate(iso: string | null): string {
  if (!iso) return "—";
  const parts = iso.split("-");
  if (parts.length !== 3) return iso;
  return `${parts[2]}.${parts[1]}.${parts[0]}`;
}

interface ReviewListProps {
  entries: ReviewEntry[];
  loading: boolean;
}

export default function ReviewList({ entries, loading }: ReviewListProps) {
  const navigate = useNavigate();

  if (loading) {
    return (
      <div className="pqm-review-list" role="region" aria-label="Review-Liste wird geladen">
        <p className="pqm-review-list__loading">Review-Liste wird geladen …</p>
      </div>
    );
  }

  if (entries.length === 0) {
    return (
      <div className="pqm-review-list pqm-review-list--empty" role="status">
        <div className="pqm-review-list__empty-icon" aria-hidden="true">
          <AlertCircle size={32} />
        </div>
        <p className="pqm-review-list__empty-text">
          Keine Dokumente benötigen aktuell eine Prüfung.
        </p>
      </div>
    );
  }

  return (
    <div className="pqm-review-list" role="region" aria-label="Dokumente, die eine Prüfung benötigen">
      {entries.map((entry) => (
        <button
          key={entry.id}
          className="pqm-review-list__item"
          onClick={() => navigate(`/dokumente/${entry.document_number}`)}
          aria-label={`Dokument ${entry.document_number} – ${entry.title} öffnen`}
        >
          <div className="pqm-review-list__item-main">
            <div className="pqm-review-list__item-header">
              <span className="pqm-review-list__item-number">{entry.document_number}</span>
              <StatusBadge
                label={entry.computed_validity}
                variant={validityToVariant(entry.computed_validity)}
                icon={entry.computed_validity === "abgelaufen" ? AlertCircle : Clock}
              />
            </div>
            <h4 className="pqm-review-list__item-title">{entry.title}</h4>
            <div className="pqm-review-list__item-meta">
              <span className="pqm-review-list__item-date">
                Gültig bis: {formatDate(entry.valid_until)}
              </span>
              {entry.responsible_person_name && (
                <span className="pqm-review-list__item-responsible">
                  {entry.responsible_person_name}
                </span>
              )}
              {entry.category_name && (
                <span className="pqm-review-list__item-category">
                  {entry.category_name}
                </span>
              )}
            </div>
          </div>
          <span className="pqm-review-list__item-arrow" aria-hidden="true">
            <ChevronRight size={20} />
          </span>
        </button>
      ))}
    </div>
  );
}
