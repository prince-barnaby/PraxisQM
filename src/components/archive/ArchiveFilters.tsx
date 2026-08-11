import { useState } from "react";
import { Filter, ChevronDown } from "lucide-react";
import "./ArchiveFilters.css";

export interface ArchiveFilterValues {
  category: string;
  subcategory: string;
  responsible: string;
  status: string;
}

interface ArchiveFiltersProps {
  categories: string[];
  subcategories: string[];
  responsibles: string[];
  statuses: string[];
  values: ArchiveFilterValues;
  onChange: (values: ArchiveFilterValues) => void;
}

export default function ArchiveFilters({
  categories,
  subcategories,
  responsibles,
  statuses,
  values,
  onChange,
}: ArchiveFiltersProps) {
  const [open, setOpen] = useState(false);

  const updateField = (field: keyof ArchiveFilterValues, value: string) => {
    onChange({ ...values, [field]: value });
  };

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
          aria-label="Filterbereich"
        >
          <div className="pqm-archive-filters__selects">
            <div className="pqm-archive-filters__field">
              <label className="pqm-archive-filters__field-label">Kategorie</label>
              <select
                aria-label="Kategorie filtern"
                value={values.category}
                onChange={(e) => updateField("category", e.target.value)}
              >
                <option value="">Alle</option>
                {categories.map((c) => (
                  <option key={c} value={c}>{c}</option>
                ))}
              </select>
            </div>
            <div className="pqm-archive-filters__field">
              <label className="pqm-archive-filters__field-label">Unterkategorie</label>
              <select
                aria-label="Unterkategorie filtern"
                value={values.subcategory}
                onChange={(e) => updateField("subcategory", e.target.value)}
              >
                <option value="">Alle</option>
                {subcategories.map((s) => (
                  <option key={s} value={s}>{s}</option>
                ))}
              </select>
            </div>
            <div className="pqm-archive-filters__field">
              <label className="pqm-archive-filters__field-label">Verantwortliche Person</label>
              <select
                aria-label="Verantwortliche Person filtern"
                value={values.responsible}
                onChange={(e) => updateField("responsible", e.target.value)}
              >
                <option value="">Alle</option>
                {responsibles.map((r) => (
                  <option key={r} value={r}>{r}</option>
                ))}
              </select>
            </div>
            <div className="pqm-archive-filters__field">
              <label className="pqm-archive-filters__field-label">Status</label>
              <select
                aria-label="Status filtern"
                value={values.status}
                onChange={(e) => updateField("status", e.target.value)}
              >
                <option value="">Alle</option>
                {statuses.map((s) => (
                  <option key={s} value={s}>{s}</option>
                ))}
              </select>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
