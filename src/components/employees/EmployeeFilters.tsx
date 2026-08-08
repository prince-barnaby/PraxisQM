import { useState } from "react";
import { Filter, ChevronDown, RotateCcw } from "lucide-react";
import "./EmployeeFilters.css";

export type ActiveStatusFilter = "all" | "active" | "inactive";

export interface EmployeeFilterValues {
  activeStatus: ActiveStatusFilter;
  position: string;
  responsibilityId: string;
  qmAreaId: string;
}

export const NO_FILTERS: EmployeeFilterValues = {
  activeStatus: "all",
  position: "",
  responsibilityId: "",
  qmAreaId: "",
};

interface Option {
  value: string;
  label: string;
}

interface EmployeeFiltersProps {
  values: EmployeeFilterValues;
  onChange: (values: EmployeeFilterValues) => void;
  positions: Option[];
  responsibilities: Option[];
  qmAreas: Option[];
}

export default function EmployeeFilters({
  values,
  onChange,
  positions,
  responsibilities,
  qmAreas,
}: EmployeeFiltersProps) {
  const [open, setOpen] = useState(false);
  const hasActiveFilters =
    values.activeStatus !== "all" ||
    values.position !== "" ||
    values.responsibilityId !== "" ||
    values.qmAreaId !== "";

  return (
    <div className="pqm-employee-filters">
      <div className="pqm-employee-filters__header">
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
        {hasActiveFilters && (
          <button
            type="button"
            className="pqm-employee-filters__reset"
            onClick={() => onChange(NO_FILTERS)}
            aria-label="Filter zurücksetzen"
          >
            <RotateCcw size={12} aria-hidden="true" />
            Zurücksetzen
          </button>
        )}
      </div>
      {open && (
        <div
          id="pqm-employee-filters-panel"
          className="pqm-employee-filters__panel"
        >
          <div className="pqm-employee-filters__selects">
            <div className="pqm-employee-filters__field">
              <label
                className="pqm-employee-filters__field-label"
                htmlFor="pqm-filter-active"
              >
                Aktivstatus
              </label>
              <select
                id="pqm-filter-active"
                value={values.activeStatus}
                onChange={(e) =>
                  onChange({ ...values, activeStatus: e.target.value as ActiveStatusFilter })
                }
                aria-label="Nach Aktivstatus filtern"
              >
                <option value="all">Alle</option>
                <option value="active">aktiv</option>
                <option value="inactive">inaktiv</option>
              </select>
            </div>

            <div className="pqm-employee-filters__field">
              <label
                className="pqm-employee-filters__field-label"
                htmlFor="pqm-filter-position"
              >
                Position
              </label>
              <select
                id="pqm-filter-position"
                value={values.position}
                onChange={(e) => onChange({ ...values, position: e.target.value })}
                aria-label="Nach Position filtern"
              >
                <option value="">Alle</option>
                {positions.map((p) => (
                  <option key={p.value} value={p.value}>
                    {p.label}
                  </option>
                ))}
              </select>
            </div>

            <div className="pqm-employee-filters__field">
              <label
                className="pqm-employee-filters__field-label"
                htmlFor="pqm-filter-responsibility"
              >
                Verantwortungsposition
              </label>
              <select
                id="pqm-filter-responsibility"
                value={values.responsibilityId}
                onChange={(e) => onChange({ ...values, responsibilityId: e.target.value })}
                aria-label="Nach Verantwortungsposition filtern"
              >
                <option value="">Alle</option>
                {responsibilities.map((r) => (
                  <option key={r.value} value={r.value}>
                    {r.label}
                  </option>
                ))}
              </select>
            </div>

            <div className="pqm-employee-filters__field">
              <label
                className="pqm-employee-filters__field-label"
                htmlFor="pqm-filter-qm-area"
              >
                QM-Bereich
              </label>
              <select
                id="pqm-filter-qm-area"
                value={values.qmAreaId}
                onChange={(e) => onChange({ ...values, qmAreaId: e.target.value })}
                aria-label="Nach QM-Bereich filtern"
              >
                <option value="">Alle</option>
                {qmAreas.map((a) => (
                  <option key={a.value} value={a.value}>
                    {a.label}
                  </option>
                ))}
              </select>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
