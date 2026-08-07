import { useState } from "react";
import { Filter, ChevronDown } from "lucide-react";
import "./DocumentFilters.css";

const FILTER_LABELS = [
  "Kategorie",
  "Unterkategorie",
  "Status",
  "Verantwortliche Person",
  "Gültigkeit",
] as const;

export default function DocumentFilters() {
  const [open, setOpen] = useState(false);

  return (
    <div className="pqm-document-filters">
      <button
        type="button"
        className="pqm-document-filters__toggle"
        aria-expanded={open}
        aria-controls="pqm-document-filters-panel"
        onClick={() => setOpen((prev) => !prev)}
      >
        <Filter size={14} aria-hidden="true" />
        <span>Filter</span>
        <ChevronDown
          size={14}
          aria-hidden="true"
          className={
            "pqm-document-filters__chevron" +
            (open ? " pqm-document-filters__chevron--open" : "")
          }
        />
      </button>
      {open && (
        <div
          id="pqm-document-filters-panel"
          className="pqm-document-filters__panel"
          aria-label="Filterbereich – Platzhalter, keine aktive Filterung"
        >
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
      )}
    </div>
  );
}
