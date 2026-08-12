import { ChevronDown, Filter, RotateCcw } from "lucide-react";
import { useState } from "react";
import type { DocumentFilterValues, ValidityFilterValue } from "../../pages/Dokumente";
import "./DocumentFilters.css";

interface Option {
  value: string;
  label: string;
}

interface DocumentFiltersProps {
  values: DocumentFilterValues;
  onChange: (values: DocumentFilterValues) => void;
  categories: Option[];
  subcategories: Option[];
  responsiblePeople: Option[];
}

const VALIDITY_OPTIONS: { value: ValidityFilterValue; label: string }[] = [
  { value: "all", label: "Alle" },
  { value: "gültig", label: "gültig" },
  { value: "läuft bald ab", label: "läuft bald ab" },
  { value: "abgelaufen", label: "abgelaufen" },
  { value: "none", label: "kein Ablaufdatum" },
];

export default function DocumentFilters({
  values,
  onChange,
  categories,
  subcategories,
  responsiblePeople,
}: DocumentFiltersProps) {
  const [open, setOpen] = useState(false);
  const hasActiveFilters =
    values.categoryId !== "" ||
    values.subcategoryId !== "" ||
    values.responsiblePersonId !== "" ||
    values.status !== "all" ||
    values.validity !== "all";

  return (
    <div className="pqm-document-filters">
      <div className="pqm-document-filters__header">
        <button
          type="button"
          className="pqm-document-filters__toggle"
          aria-expanded={open}
          aria-controls="pqm-document-filters-panel"
          onClick={() => setOpen((previous) => !previous)}
        >
          <Filter size={14} aria-hidden="true" />
          <span>Filter</span>
          <ChevronDown
            size={14}
            aria-hidden="true"
            className={open ? "pqm-document-filters__chevron--open" : ""}
          />
        </button>
        {hasActiveFilters && (
          <button
            type="button"
            className="pqm-document-filters__reset"
            onClick={() => onChange({ categoryId: "", subcategoryId: "", responsiblePersonId: "", status: "all", validity: "all" })}
            aria-label="Dokumentfilter zurücksetzen"
          >
            <RotateCcw size={12} aria-hidden="true" />
            Zurücksetzen
          </button>
        )}
      </div>
      {open && (
        <div id="pqm-document-filters-panel" className="pqm-document-filters__active">
          <div className="pqm-document-filters__field">
            <label className="pqm-document-filters__field-label" htmlFor="pqm-filter-category">Kategorie</label>
            <select id="pqm-filter-category" value={values.categoryId} onChange={(event) => onChange({ ...values, categoryId: event.target.value, subcategoryId: event.target.value ? values.subcategoryId : "" })}>
              <option value="">Alle</option>
              {categories.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
            </select>
          </div>
          <div className="pqm-document-filters__field">
            <label className="pqm-document-filters__field-label" htmlFor="pqm-filter-subcategory">Unterkategorie</label>
            <select id="pqm-filter-subcategory" value={values.subcategoryId} onChange={(event) => onChange({ ...values, subcategoryId: event.target.value })} disabled={!values.categoryId}>
              <option value="">Alle</option>
              {subcategories.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
            </select>
          </div>
          <div className="pqm-document-filters__field">
            <label className="pqm-document-filters__field-label" htmlFor="pqm-filter-responsible">Verantwortliche Person</label>
            <select id="pqm-filter-responsible" value={values.responsiblePersonId} onChange={(event) => onChange({ ...values, responsiblePersonId: event.target.value })}>
              <option value="">Alle</option>
              {responsiblePeople.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
            </select>
          </div>
          <div className="pqm-document-filters__field">
            <label className="pqm-document-filters__field-label" htmlFor="pqm-filter-status">Status</label>
            <select id="pqm-filter-status" value={values.status} onChange={(event) => onChange({ ...values, status: event.target.value as DocumentFilterValues["status"] })}>
              <option value="all">Alle</option>
              <option value="Entwurf">Entwurf</option>
              <option value="aktiv">aktiv</option>
            </select>
          </div>
          <div className="pqm-document-filters__field">
            <label className="pqm-document-filters__field-label" htmlFor="pqm-filter-validity">Gültigkeit</label>
            <select id="pqm-filter-validity" value={values.validity} onChange={(event) => onChange({ ...values, validity: event.target.value as ValidityFilterValue })}>
              {VALIDITY_OPTIONS.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
            </select>
          </div>
        </div>
      )}
    </div>
  );
}
