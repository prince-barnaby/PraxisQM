import { useState, useEffect, type FormEvent } from "react";
import { FileText, X } from "lucide-react";
import DocumentFormSection from "./DocumentFormSection";
import FormField from "./FormField";
import "./DocumentForm.css";

export type DocumentFormMode = "create" | "edit";

export interface CategoryOption {
  id: string;
  name: string;
}

export interface SubcategoryOption {
  id: string;
  name: string;
  category_id: string;
}

export interface EmployeeOption {
  id: string;
  name: string;
}

interface DocumentFormProps {
  mode: DocumentFormMode;
  documentNumber?: string;
  pdfFileName?: string;
  categories?: CategoryOption[];
  subcategories?: SubcategoryOption[];
  employees?: EmployeeOption[];
  onSubmit?: (data: DocumentFormData) => Promise<void>;
  onCancel?: () => void;
  onSelectPdf?: () => Promise<string | null>;
}

export interface DocumentFormData {
  title: string;
  category_id: string | null;
  subcategory_id: string | null;
  responsible_person_id: string | null;
  version: string;
  status: string;
  validity: string;
  valid_until: string | null;
  description: string | null;
  source_file_path: string;
  original_file_name: string;
}

export default function DocumentForm({
  mode,
  documentNumber,
  pdfFileName,
  categories = [],
  subcategories = [],
  employees = [],
  onSubmit,
  onCancel,
  onSelectPdf,
}: DocumentFormProps) {
  const isCreate = mode === "create";
  const submitLabel = isCreate ? "Dokument anlegen" : "Änderungen speichern";

  const numberValue = isCreate
    ? "wird automatisch vergeben"
    : documentNumber ?? "Unbekannt";

  const numberHint = isCreate
    ? "wird automatisch vergeben"
    : "Dokumentnummer ist unveränderlich";

  const [title, setTitle] = useState("");
  const [categoryId, setCategoryId] = useState("");
  const [subcategoryId, setSubcategoryId] = useState("");
  const [responsiblePersonId, setResponsiblePersonId] = useState("");
  const [version, setVersion] = useState("1.0");
  const [status, setStatus] = useState("Entwurf");
  const [validity, setValidity] = useState("gültig");
  const [validUntil, setValidUntil] = useState("");
  const [description, setDescription] = useState("");
  const [selectedPdfPath, setSelectedPdfPath] = useState<string | null>(null);
  const [selectedPdfName, setSelectedPdfName] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fileText = selectedPdfName ?? (isCreate ? "Noch keine Datei ausgewählt" : pdfFileName ?? "—");
  const fileHint = selectedPdfPath
    ? "PDF ausgewählt – wird beim Speichern in den Dokumentenspeicher kopiert"
    : isCreate
      ? "Wählen Sie eine PDF-Datei aus"
      : "Datei kann später ersetzt werden";

  const filteredSubcategories = categoryId
    ? subcategories.filter((s) => s.category_id === categoryId)
    : subcategories;

  const handleSelectPdf = async () => {
    if (!onSelectPdf) return;
    try {
      const path = await onSelectPdf();
      if (path) {
        setSelectedPdfPath(path);
        const name = path.split(/[/\\]/).pop() ?? "Unbenannt.pdf";
        setSelectedPdfName(name);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!onSubmit) return;

    if (!title.trim()) {
      setError("Bitte geben Sie einen Titel ein.");
      return;
    }
    if (isCreate && !selectedPdfPath) {
      setError("Bitte wählen Sie eine PDF-Datei aus.");
      return;
    }

    setSubmitting(true);
    setError(null);
    try {
      await onSubmit({
        title: title.trim(),
        category_id: categoryId || null,
        subcategory_id: subcategoryId || null,
        responsible_person_id: responsiblePersonId || null,
        version: version.trim() || "1.0",
        status,
        validity,
        valid_until: validUntil || null,
        description: description.trim() || null,
        source_file_path: selectedPdfPath ?? "",
        original_file_name: selectedPdfName ?? "",
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSubmitting(false);
    }
  };

  useEffect(() => {
    if (error) {
      const timer = setTimeout(() => setError(null), 5000);
      return () => clearTimeout(timer);
    }
  }, [error]);

  return (
    <form
      className="pqm-document-form"
      aria-label={isCreate ? "Neues Dokument anlegen" : "Dokument bearbeiten"}
      onSubmit={handleSubmit}
    >
      {error && (
        <p className="pqm-document-form__error" role="alert">
          {error}
        </p>
      )}
      <DocumentFormSection title="Allgemeine Daten">
        <FormField
          label="Dokumentnummer"
          htmlFor="doc-number"
          hint={numberHint}
          className="pqm-form-field--readonly"
        >
          <input
            id="doc-number"
            type="text"
            value={numberValue}
            readOnly
            disabled
            aria-label={`Dokumentnummer – ${numberHint}`}
          />
        </FormField>

        <FormField
          label="Titel"
          htmlFor="doc-title"
          className="pqm-form-field--full"
        >
          <input
            id="doc-title"
            type="text"
            placeholder="Dokumenttitel"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            aria-label="Dokumenttitel"
            required
          />
        </FormField>

        <FormField
          label="Kategorie"
          htmlFor="doc-category"
          className="pqm-form-field--half"
        >
          <select
            id="doc-category"
            value={categoryId}
            onChange={(e) => {
              setCategoryId(e.target.value);
              setSubcategoryId("");
            }}
            aria-label="Kategorie"
          >
            <option value="">Bitte wählen …</option>
            {categories.map((c) => (
              <option key={c.id} value={c.id}>{c.name}</option>
            ))}
          </select>
        </FormField>

        <FormField
          label="Unterkategorie"
          htmlFor="doc-subcategory"
          className="pqm-form-field--half"
        >
          <select
            id="doc-subcategory"
            value={subcategoryId}
            onChange={(e) => setSubcategoryId(e.target.value)}
            aria-label="Unterkategorie"
            disabled={!categoryId}
          >
            <option value="">Bitte wählen …</option>
            {filteredSubcategories.map((s) => (
              <option key={s.id} value={s.id}>{s.name}</option>
            ))}
          </select>
        </FormField>

        <FormField
          label="Version"
          htmlFor="doc-version"
          className="pqm-form-field--compact"
        >
          <input
            id="doc-version"
            type="text"
            placeholder="1.0"
            value={version}
            onChange={(e) => setVersion(e.target.value)}
            aria-label="Version"
            required
          />
        </FormField>

        <FormField
          label="Status"
          htmlFor="doc-status"
          className="pqm-form-field--compact"
        >
          <select
            id="doc-status"
            value={status}
            onChange={(e) => setStatus(e.target.value)}
            aria-label="Status"
          >
            <option value="Entwurf">Entwurf</option>
            <option value="aktiv">aktiv</option>
            <option value="archiviert">archiviert</option>
          </select>
        </FormField>

        <FormField
          label="Verantwortliche Person"
          htmlFor="doc-responsible"
          className="pqm-form-field--half"
        >
          <select
            id="doc-responsible"
            value={responsiblePersonId}
            onChange={(e) => setResponsiblePersonId(e.target.value)}
            aria-label="Verantwortliche Person"
          >
            <option value="">Bitte wählen …</option>
            {employees.map((emp) => (
              <option key={emp.id} value={emp.id}>{emp.name}</option>
            ))}
          </select>
        </FormField>

        <FormField
          label="Gültig bis"
          htmlFor="doc-validity"
          className="pqm-form-field--compact"
        >
          <input
            id="doc-validity"
            type="date"
            value={validUntil}
            onChange={(e) => setValidUntil(e.target.value)}
            aria-label="Gültig bis"
          />
        </FormField>

        <FormField
          label="Gültigkeit"
          htmlFor="doc-validity-status"
          className="pqm-form-field--compact"
        >
          <select
            id="doc-validity-status"
            value={validity}
            onChange={(e) => setValidity(e.target.value)}
            aria-label="Gültigkeitsstatus"
          >
            <option value="gültig">gültig</option>
            <option value="läuft bald ab">läuft bald ab</option>
            <option value="abgelaufen">abgelaufen</option>
          </select>
        </FormField>
      </DocumentFormSection>

      <DocumentFormSection title="Beschreibung">
        <FormField
          label="Beschreibung"
          htmlFor="doc-description"
          className="pqm-form-field--full"
        >
          <textarea
            id="doc-description"
            rows={5}
            placeholder="Beschreibung des Dokuments …"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            aria-label="Dokumentbeschreibung"
          />
        </FormField>
      </DocumentFormSection>

      <DocumentFormSection title="Dokumentdatei">
        <div className="pqm-document-form__file-area">
          <FileText size={32} aria-hidden="true" />
          <div className="pqm-document-form__file-info">
            <p className="pqm-document-form__file-text">
              {fileText}
            </p>
            <p className="pqm-document-form__file-hint">
              {fileHint}
            </p>
          </div>
          <button
            type="button"
            className="pqm-document-form__file-button"
            onClick={handleSelectPdf}
            disabled={!onSelectPdf}
            aria-label={isCreate ? "PDF auswählen" : "PDF ersetzen"}
          >
            {isCreate ? "PDF auswählen" : "PDF ersetzen"}
          </button>
        </div>
      </DocumentFormSection>

      <div className="pqm-document-form__actions" role="group" aria-label="Formular-Aktionen">
        <button
          type="button"
          className="pqm-document-form__cancel"
          onClick={onCancel}
          aria-label="Abbrechen"
        >
          <X size={16} aria-hidden="true" />
          Abbrechen
        </button>
        <button
          type="submit"
          className="pqm-document-form__submit"
          disabled={submitting || !onSubmit}
          aria-label={submitLabel}
        >
          {submitting ? "Wird gespeichert …" : submitLabel}
        </button>
      </div>
    </form>
  );
}
