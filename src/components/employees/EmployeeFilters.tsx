import { useState } from "react";
import { Filter, ChevronDown } from "lucide-react";
import "./EmployeeFilters.css";

const FILTER_LABELS = [
  "Position",
  "Verantwortungsposition",
  "QM-Bereich",
  "Aktivstatus",
] as const;

export default function EmployeeFilters() {
  const [open, setOpen] = useState(false);

  return (
    <div className="pqm-employee-filters">
      <button
        type="button"
        className="pqm-employee-filters__toggle"
        aria-expanded={open}
        aria-controls="pqm-employee-filters-panel"
        onClick={() => setOpen((prev) => !prev)}
      >
        <Filter size={14} aria-hidden="true" />
        <span>Filter</span>
        <ChevronDown
          size={14}
          aria-hidden="true"
          className={
            "pqm-employee-filters__chevron" +
            (open ? " pqm-employee-filters__chevron--open" : "")
          }
        />
      </button>
      {open && (
        <div
          id="pqm-employee-filters-panel"
          className="pqm-employee-filters__panel"
          aria-label="Filterbereich – Platzhalter, keine aktive Filterung"
        >
          <div className="pqm-employee-filters__selects">
            {FILTER_LABELS.map((label) => (
              <div key={label} className="pqm-employee-filters__field">
                <label className="pqm-employee-filters__field-label">{label}</label>
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
