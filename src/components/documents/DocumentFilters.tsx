import { Filter } from "lucide-react";
import "./DocumentFilters.css";

const FILTER_LABELS = [
  "Kategorie",
  "Unterkategorie",
  "Status",
  "Verantwortliche Person",
  "Gültigkeit",
] as const;

export default function DocumentFilters() {
  return (
    <div
      className="pqm-document-filters"
      aria-label="Filterbereich – Platzhalter, keine aktive Filterung"
    >
      <div className="pqm-document-filters__label">
        <Filter size={14} aria-hidden="true" />
        <span>Filter</span>
      </div>
      <div className="pqm-document-filters__selects">
        {FILTER_LABELS.map((label) => (
          <div key={label} className="pqm-document-filters__field">
            <label className="pqm-document-filters__field-label">{label}</label>
            <select
              disabled
              aria-label={`${label} – Filter (Platzhalter, nicht funktional)`}
              defaultValue=""
            >
              <option value="">Alle</option>
              <option disabled>Platzhalter</option>
            </select>
          </div>
        ))}
      </div>
    </div>
  );
}
