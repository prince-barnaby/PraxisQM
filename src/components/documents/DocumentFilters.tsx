import type { ValidityFilterValue } from "../../pages/Dokumente";
import "./DocumentFilters.css";

interface DocumentFiltersProps {
  validityFilter: ValidityFilterValue;
  onValidityFilterChange: (value: ValidityFilterValue) => void;
}

const VALIDITY_OPTIONS: { value: ValidityFilterValue; label: string }[] = [
  { value: "all", label: "Alle" },
  { value: "gültig", label: "gültig" },
  { value: "läuft bald ab", label: "läuft bald ab" },
  { value: "abgelaufen", label: "abgelaufen" },
  { value: "none", label: "kein Ablaufdatum" },
];

export default function DocumentFilters({
  validityFilter,
  onValidityFilterChange,
}: DocumentFiltersProps) {
  return (
    <div className="pqm-document-filters">
      <div className="pqm-document-filters__active">
        <div className="pqm-document-filters__field">
          <label className="pqm-document-filters__field-label">Gültigkeit</label>
          <select
            aria-label="Nach Gültigkeit filtern"
            value={validityFilter}
            onChange={(e) =>
              onValidityFilterChange(e.target.value as ValidityFilterValue)
            }
          >
            {VALIDITY_OPTIONS.map((opt) => (
              <option key={opt.value} value={opt.value}>
                {opt.label}
              </option>
            ))}
          </select>
        </div>
      </div>
    </div>
  );
}
