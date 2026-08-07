import { useState } from "react";
import { Check, X } from "lucide-react";
import type { MasterDataItem } from "../../lib/employeeApi";
import DocumentFormSection from "../documents/DocumentFormSection";
import FormField from "../documents/FormField";
import "./EmployeeForm.css";

interface EmployeeFormProps {
  responsibilities: MasterDataItem[];
  qmAreas: MasterDataItem[];
  onSubmit: (data: EmployeeFormData) => void;
  onCancel: () => void;
  submitting?: boolean;
  error?: string | null;
}

export interface EmployeeFormData {
  last_name: string;
  first_name: string;
  position: string;
  is_active: boolean;
  hire_date: string | null;
  departure_date: string | null;
  responsibility_ids: string[];
  qm_area_ids: string[];
}

export default function EmployeeForm({
  responsibilities,
  qmAreas,
  onSubmit,
  onCancel,
  submitting = false,
  error = null,
}: EmployeeFormProps) {
  const [lastName, setLastName] = useState("");
  const [firstName, setFirstName] = useState("");
  const [position, setPosition] = useState("");
  const [isActive, setIsActive] = useState(true);
  const [hireDate, setHireDate] = useState("");
  const [departureDate, setDepartureDate] = useState("");
  const [selectedRespIds, setSelectedRespIds] = useState<string[]>([]);
  const [selectedAreaIds, setSelectedAreaIds] = useState<string[]>([]);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});

  const toggleResp = (id: string) => {
    setSelectedRespIds((prev) =>
      prev.includes(id) ? prev.filter((r) => r !== id) : [...prev, id],
    );
  };

  const toggleArea = (id: string) => {
    setSelectedAreaIds((prev) =>
      prev.includes(id) ? prev.filter((a) => a !== id) : [...prev, id],
    );
  };

  const validate = (): boolean => {
    const errors: Record<string, string> = {};
    if (!lastName.trim()) errors.lastName = "Name ist erforderlich";
    if (!firstName.trim()) errors.firstName = "Vorname ist erforderlich";
    if (!position.trim()) errors.position = "Position ist erforderlich";
    if (!hireDate.trim()) errors.hireDate = "Eintrittsdatum ist erforderlich";
    setValidationErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;
    onSubmit({
      last_name: lastName.trim(),
      first_name: firstName.trim(),
      position: position.trim(),
      is_active: isActive,
      hire_date: hireDate || null,
      departure_date: departureDate || null,
      responsibility_ids: selectedRespIds,
      qm_area_ids: selectedAreaIds,
    });
  };

  return (
    <form
      className="pqm-employee-form"
      aria-label="Neuen Mitarbeiter anlegen"
      onSubmit={handleSubmit}
    >
      <DocumentFormSection title="Allgemeine Daten">
        <FormField
          label="Name"
          htmlFor="emp-lastname"
          className="pqm-form-field--half"
        >
          <input
            id="emp-lastname"
            type="text"
            value={lastName}
            onChange={(e) => setLastName(e.target.value)}
            aria-label="Name"
            aria-invalid={!!validationErrors.lastName}
          />
          {validationErrors.lastName && (
            <span className="pqm-employee-form__error">{validationErrors.lastName}</span>
          )}
        </FormField>

        <FormField
          label="Vorname"
          htmlFor="emp-firstname"
          className="pqm-form-field--half"
        >
          <input
            id="emp-firstname"
            type="text"
            value={firstName}
            onChange={(e) => setFirstName(e.target.value)}
            aria-label="Vorname"
            aria-invalid={!!validationErrors.firstName}
          />
          {validationErrors.firstName && (
            <span className="pqm-employee-form__error">{validationErrors.firstName}</span>
          )}
        </FormField>

        <FormField
          label="Position"
          htmlFor="emp-position"
          hint="z. B. Zahnärztin, ZFA, Praxismanagerin"
          className="pqm-form-field--half"
        >
          <input
            id="emp-position"
            type="text"
            value={position}
            onChange={(e) => setPosition(e.target.value)}
            aria-label="Position"
            aria-invalid={!!validationErrors.position}
          />
          {validationErrors.position && (
            <span className="pqm-employee-form__error">{validationErrors.position}</span>
          )}
        </FormField>

        <FormField
          label="Aktivstatus"
          htmlFor="emp-active"
          className="pqm-form-field--compact"
        >
          <select
            id="emp-active"
            value={isActive ? "true" : "false"}
            onChange={(e) => setIsActive(e.target.value === "true")}
            aria-label="Aktivstatus"
          >
            <option value="true">aktiv</option>
            <option value="false">inaktiv</option>
          </select>
        </FormField>

        <FormField
          label="Eintrittsdatum"
          htmlFor="emp-hiredate"
          className="pqm-form-field--compact"
        >
          <input
            id="emp-hiredate"
            type="date"
            value={hireDate}
            onChange={(e) => setHireDate(e.target.value)}
            aria-label="Eintrittsdatum"
            aria-invalid={!!validationErrors.hireDate}
          />
          {validationErrors.hireDate && (
            <span className="pqm-employee-form__error">{validationErrors.hireDate}</span>
          )}
        </FormField>

        <FormField
          label="Austrittsdatum"
          htmlFor="emp-departuredate"
          hint="optional"
          className="pqm-form-field--compact"
        >
          <input
            id="emp-departuredate"
            type="date"
            value={departureDate}
            onChange={(e) => setDepartureDate(e.target.value)}
            aria-label="Austrittsdatum – optional"
          />
        </FormField>
      </DocumentFormSection>

      <DocumentFormSection title="Verantwortungspositionen">
        <p className="pqm-employee-form__section-hint">
          Wählen Sie eine oder mehrere Verantwortungspositionen aus. Mehrfachauswahl möglich.
        </p>
        <div className="pqm-employee-form__checkbox-list">
          {responsibilities.map((r) => (
            <label
              key={r.id}
              className="pqm-employee-form__checkbox-item"
            >
              <input
                type="checkbox"
                checked={selectedRespIds.includes(r.id)}
                onChange={() => toggleResp(r.id)}
              />
              <span className="pqm-employee-form__checkbox-label">{r.name}</span>
              {selectedRespIds.includes(r.id) && (
                <Check size={14} className="pqm-employee-form__checkbox-check" aria-hidden="true" />
              )}
            </label>
          ))}
          {responsibilities.length === 0 && (
            <p className="pqm-employee-form__empty-master">
              Keine Verantwortungspositionen vorhanden.
            </p>
          )}
        </div>
      </DocumentFormSection>

      <DocumentFormSection title="Zugeordnete QM-Bereiche">
        <p className="pqm-employee-form__section-hint">
          Wählen Sie einen oder mehrere QM-Bereiche aus. Mehrfachauswahl möglich.
        </p>
        <div className="pqm-employee-form__checkbox-list">
          {qmAreas.map((a) => (
            <label
              key={a.id}
              className="pqm-employee-form__checkbox-item"
            >
              <input
                type="checkbox"
                checked={selectedAreaIds.includes(a.id)}
                onChange={() => toggleArea(a.id)}
              />
              <span className="pqm-employee-form__checkbox-label">{a.name}</span>
              {selectedAreaIds.includes(a.id) && (
                <Check size={14} className="pqm-employee-form__checkbox-check" aria-hidden="true" />
              )}
            </label>
          ))}
          {qmAreas.length === 0 && (
            <p className="pqm-employee-form__empty-master">
              Keine QM-Bereiche vorhanden.
            </p>
          )}
        </div>
      </DocumentFormSection>

      {error && (
        <p className="pqm-employee-form__submit-error" role="alert">
          Fehler beim Speichern: {error}
        </p>
      )}

      <div className="pqm-employee-form__actions" role="group" aria-label="Formular-Aktionen">
        <button
          type="button"
          className="pqm-employee-form__cancel"
          onClick={onCancel}
          disabled={submitting}
          aria-label="Abbrechen"
        >
          <X size={16} aria-hidden="true" />
          Abbrechen
        </button>
        <button
          type="submit"
          className="pqm-employee-form__submit"
          disabled={submitting}
          aria-label="Mitarbeiter anlegen"
        >
          {submitting ? "Wird gespeichert …" : "Mitarbeiter anlegen"}
        </button>
      </div>
    </form>
  );
}
