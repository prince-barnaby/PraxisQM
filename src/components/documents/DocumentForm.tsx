import { FileText, X } from "lucide-react";
import DocumentFormSection from "./DocumentFormSection";
import FormField from "./FormField";
import "./DocumentForm.css";

export type DocumentFormMode = "create" | "edit";

interface DocumentFormProps {
  mode: DocumentFormMode;
  onSubmit?: () => void;
  onCancel?: () => void;
}

export default function DocumentForm({ mode, onCancel }: DocumentFormProps) {
  const isCreate = mode === "create";
  const submitLabel = isCreate ? "Dokument anlegen" : "Änderungen speichern";

  return (
    <form
      className="pqm-document-form"
      aria-label={isCreate ? "Neues Dokument anlegen" : "Dokument bearbeiten"}
      onSubmit={(e) => e.preventDefault()}
    >
      <DocumentFormSection title="Allgemeine Daten">
        <FormField
          label="Dokumentnummer"
          htmlFor="doc-number"
          hint="wird automatisch vergeben"
          className="pqm-form-field--readonly"
        >
          <input
            id="doc-number"
            type="text"
            value="wird automatisch vergeben"
            readOnly
            disabled
            aria-label="Dokumentnummer – wird automatisch vergeben"
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
            aria-label="Dokumenttitel"
          />
        </FormField>

        <FormField
          label="Kategorie"
          htmlFor="doc-category"
          className="pqm-form-field--half"
        >
          <select id="doc-category" disabled aria-label="Kategorie – Platzhalter">
            <option value="">Bitte wählen …</option>
            <option disabled>Platzhalter</option>
          </select>
        </FormField>

        <FormField
          label="Unterkategorie"
          htmlFor="doc-subcategory"
          className="pqm-form-field--half"
        >
          <select
            id="doc-subcategory"
            disabled
            aria-label="Unterkategorie – Platzhalter"
          >
            <option value="">Bitte wählen …</option>
            <option disabled>Platzhalter</option>
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
            aria-label="Version"
          />
        </FormField>

        <FormField
          label="Status"
          htmlFor="doc-status"
          className="pqm-form-field--compact"
        >
          <select id="doc-status" disabled aria-label="Status – Platzhalter">
            <option value="">Bitte wählen …</option>
            <option disabled>Platzhalter</option>
          </select>
        </FormField>

        <FormField
          label="Verantwortliche Person"
          htmlFor="doc-responsible"
          className="pqm-form-field--half"
        >
          <select
            id="doc-responsible"
            disabled
            aria-label="Verantwortliche Person – Platzhalter"
          >
            <option value="">Bitte wählen …</option>
            <option disabled>Platzhalter</option>
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
            aria-label="Gültig bis"
          />
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
            aria-label="Dokumentbeschreibung"
          />
        </FormField>
      </DocumentFormSection>

      <DocumentFormSection title="Schlagwörter">
        <FormField
          label="Tags"
          htmlFor="doc-tags"
          hint="Platzhalter – Tag-Verwaltung folgt später"
          className="pqm-form-field--full"
        >
          <input
            id="doc-tags"
            type="text"
            placeholder="Platzhalter-Tag, Platzhalter-Tag 2 …"
            aria-label="Schlagwörter – Platzhalter"
          />
        </FormField>
      </DocumentFormSection>

      <DocumentFormSection title="Dokumentdatei">
        <div className="pqm-document-form__file-area">
          <FileText size={32} aria-hidden="true" />
          <div className="pqm-document-form__file-info">
            <p className="pqm-document-form__file-text">
              Noch keine Datei ausgewählt
            </p>
            <p className="pqm-document-form__file-hint">
              Platzhalter – Datei-Upload folgt später
            </p>
          </div>
          <button
            type="button"
            className="pqm-document-form__file-button"
            disabled
            aria-label="PDF auswählen – Platzhalter, nicht funktional"
          >
            PDF auswählen
          </button>
        </div>
      </DocumentFormSection>

      <div className="pqm-document-form__actions" role="group" aria-label="Formular-Aktionen">
        <button
          type="button"
          className="pqm-document-form__cancel"
          onClick={onCancel}
          aria-label="Abbrechen und zur Dokumentenübersicht zurückkehren"
        >
          <X size={16} aria-hidden="true" />
          Abbrechen
        </button>
        <button
          type="submit"
          className="pqm-document-form__submit"
          disabled
          aria-label={`${submitLabel} – Platzhalter, nicht funktional`}
        >
          {submitLabel}
        </button>
      </div>
    </form>
  );
}
