import { useState } from "react";
import { Filter, ChevronDown } from "lucide-react";
import "./ArchiveFilters.css";

const FILTER_LABELS = [
  "Kategorie",
  "Unterkategorie",
  "Verantwortliche Person",
  "Archivierungszeitraum",
  "Status",
] as const;

export default function ArchiveFilters() {
  const [open, setOpen] = useState(false);

  return (
    <div className="pqm-archive-filters">
      <button
        type="button"
        className="pqm-archive-filters__toggle"
        aria-expanded={open}
        aria-controls="pqm-archive-filters-panel"
        onClick={() => setOpen((prev) => !prev)}
      >
        <Filter size={14} aria-hidden="true" />
        <span>Filter</span>
        <ChevronDown
          size={14}
          aria-hidden="true"
          className={
            "pqm-archive-filters__chevron" +
            (open ? " pqm-archive-filters__chevron--open" : "")
          }
        />
      </button>
      {open && (
        <div
          id="pqm-archive-filters-panel"
          className="pqm-archive-filters__panel"
          aria-label="Filterbereich – Platzhalter, keine aktive Filterung"
        >
          <div className="pqm-archive-filters__selects">
            {FILTER_LABELS.map((label) => (
              <div key={label} className="pqm-archive-filters__field">
                <label className="pqm-archive-filters__field-label">{label}</label>
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
