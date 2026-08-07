import { FileText, X } from "lucide-react";
import DocumentFormSection from "./DocumentFormSection";
import FormField from "./FormField";
import "./DocumentForm.css";

export type DocumentFormMode = "create" | "edit";

interface DocumentFormProps {
  mode: DocumentFormMode;
  documentNumber?: string;
  pdfFileName?: string;
  onSubmit?: () => void;
  onCancel?: () => void;
}

export default function DocumentForm({
  mode,
  documentNumber,
  pdfFileName,
  onCancel,
}: DocumentFormProps) {
  const isCreate = mode === "create";
  const submitLabel = isCreate ? "Dokument anlegen" : "Änderungen speichern";

  const numberValue = isCreate
    ? "wird automatisch vergeben"
    : documentNumber ?? "Unbekannt";

  const numberHint = isCreate
    ? "wird automatisch vergeben"
    : "Dokumentnummer ist unveränderlich";

  const fileText = isCreate
    ? "Noch keine Datei ausgewählt"
    : pdfFileName ?? "platzhalter-dokument.pdf";

  const fileHint = isCreate
    ? "Platzhalter – Datei-Upload folgt später"
    : "Platzhalter – Datei kann später ersetzt werden";

  const fileButtonLabel = isCreate ? "PDF auswählen" : "PDF ersetzen";

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
            defaultValue={isCreate ? undefined : "Platzhalter-Dokument"}
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
            <option selected={!isCreate}>Platzhalter</option>
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
            <option selected={!isCreate}>Platzhalter</option>
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
            defaultValue={isCreate ? undefined : "1.0"}
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
            <option selected={!isCreate}>aktiv</option>
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
            <option selected={!isCreate}>Platzhalter</option>
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
            defaultValue={
              isCreate
                ? undefined
                : "Platzhalter-Beschreibung für ein bestehendes Dokument."
            }
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
            defaultValue={isCreate ? undefined : "Platzhalter-Tag 1, Platzhalter-Tag 2"}
            aria-label="Schlagwörter – Platzhalter"
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
            disabled
            aria-label={`${fileButtonLabel} – Platzhalter, nicht funktional`}
          >
            {fileButtonLabel}
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
          disabled
          aria-label={`${submitLabel} – Platzhalter, nicht funktional`}
        >
          {submitLabel}
        </button>
      </div>
    </form>
  );
}


export default DocumentForm