import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { ArrowLeft, FileText, FilePlus } from "lucide-react";
import DocumentFormSection from "../components/documents/DocumentFormSection";
import FormField from "../components/documents/FormField";
import {
  fetchDocumentByNumber,
  createVersion,
  selectPdf,
  type Document,
  type CreateVersionInput,
} from "../lib/documentApi";
import "./DokumentNeueVersion.css";

export default function DokumentNeueVersion() {
  const { id } = useParams();
  const navigate = useNavigate();
  const documentNumber = id ?? "Unbekannt";

  const [doc, setDoc] = useState<Document | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [versionNumber, setVersionNumber] = useState("");
  const [status, setStatus] = useState("Entwurf");
  const [validity, setValidity] = useState("gültig");
  const [validUntil, setValidUntil] = useState("");
  const [selectedPdfPath, setSelectedPdfPath] = useState<string | null>(null);
  const [selectedPdfName, setSelectedPdfName] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    fetchDocumentByNumber(documentNumber)
      .then((d) => {
        setDoc(d);
        setStatus(d.status);
        setValidity(d.validity);
        setValidUntil(d.valid_until ?? "");
      })
      .catch((err) => {
        setError(err instanceof Error ? err.message : String(err));
      })
      .finally(() => setLoading(false));
  }, [documentNumber]);

  const handleSelectPdf = async () => {
    try {
      const result = await selectPdf();
      if (result) {
        setSelectedPdfPath(result);
        const name = result.split(/[\\/]/).pop() ?? "dokument.pdf";
        setSelectedPdfName(name);
      }
    } catch (err) {
      setFormError(err instanceof Error ? err.message : String(err));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!doc) return;

    if (!versionNumber.trim()) {
      setFormError("Bitte geben Sie eine Versionsnummer ein.");
      return;
    }
    if (!selectedPdfPath) {
      setFormError("Bitte wählen Sie eine PDF-Datei aus.");
      return;
    }

    setSubmitting(true);
    setFormError(null);
    try {
      const input: CreateVersionInput = {
        document_id: doc.id,
        version_number: versionNumber.trim(),
        status,
        validity,
        valid_until: validUntil || null,
        source_file_path: selectedPdfPath,
        original_file_name: selectedPdfName ?? "dokument.pdf",
      };
      await createVersion(input);
      navigate(`/dokumente/${documentNumber}`);
    } catch (err) {
      setFormError(err instanceof Error ? err.message : String(err));
    } finally {
      setSubmitting(false);
    }
  };

  if (loading) {
    return (
      <div className="pqm-neue-version">
        <p className="pqm-neue-version__loading">Dokument wird geladen …</p>
      </div>
    );
  }

  if (error || !doc) {
    return (
      <div className="pqm-neue-version">
        <button
          type="button"
          className="pqm-neue-version__back"
          onClick={() => navigate("/dokumente")}
        >
          <ArrowLeft size={14} aria-hidden="true" />
          Zurück zu Dokumenten
        </button>
        <p className="pqm-neue-version__error" role="alert">
          {error ?? "Dokument nicht gefunden."}
        </p>
      </div>
    );
  }

  const fileText = selectedPdfName ?? "Noch keine Datei ausgewählt";
  const fileHint = selectedPdfPath
    ? "PDF ausgewählt – wird beim Speichern in den Dokumentenspeicher kopiert"
    : "Wählen Sie eine PDF-Datei für die neue Version aus";

  return (
    <div className="pqm-neue-version">
      <button
        type="button"
        className="pqm-neue-version__back"
        onClick={() => navigate(`/dokumente/${documentNumber}`)}
      >
        <ArrowLeft size={14} aria-hidden="true" />
        Zurück zum Dokument
      </button>
      <header className="pqm-neue-version__header">
        <h2 className="pqm-neue-version__title">Neue Version erstellen</h2>
        <p className="pqm-neue-version__subtitle">
          {doc.title} ({documentNumber}) – Aktuelle Version: {doc.version}
        </p>
      </header>

      <form className="pqm-neue-version__form" onSubmit={handleSubmit}>
        <DocumentFormSection title="Versionsinformationen">
          <FormField label="Versionsnummer" htmlFor="ver-version" className="pqm-form-field--compact">
            <input
              id="ver-version"
              type="text"
              placeholder="z.B. 2.0"
              value={versionNumber}
              onChange={(e) => setVersionNumber(e.target.value)}
              aria-label="Versionsnummer"
              required
              autoFocus
            />
          </FormField>

          <FormField label="Status" htmlFor="ver-status" className="pqm-form-field--compact">
            <select
              id="ver-status"
              value={status}
              onChange={(e) => setStatus(e.target.value)}
              aria-label="Status"
            >
              <option value="Entwurf">Entwurf</option>
              <option value="aktiv">aktiv</option>
              <option value="archiviert">archiviert</option>
            </select>
          </FormField>

          <FormField label="Gültigkeit" htmlFor="ver-validity" className="pqm-form-field--compact">
            <select
              id="ver-validity"
              value={validity}
              onChange={(e) => setValidity(e.target.value)}
              aria-label="Gültigkeitsstatus"
            >
              <option value="gültig">gültig</option>
              <option value="läuft bald ab">läuft bald ab</option>
              <option value="abgelaufen">abgelaufen</option>
            </select>
          </FormField>

          <FormField label="Gültig bis" htmlFor="ver-valid-until" className="pqm-form-field--compact">
            <input
              id="ver-valid-until"
              type="date"
              value={validUntil}
              onChange={(e) => setValidUntil(e.target.value)}
              aria-label="Gültig bis"
            />
          </FormField>
        </DocumentFormSection>

        <DocumentFormSection title="Dokumentdatei">
          <div className="pqm-neue-version__file-area">
            <FileText size={32} aria-hidden="true" />
            <div className="pqm-neue-version__file-info">
              <p className="pqm-neue-version__file-text">{fileText}</p>
              <p className="pqm-neue-version__file-hint">{fileHint}</p>
            </div>
            <button
              type="button"
              className="pqm-neue-version__file-button"
              onClick={handleSelectPdf}
              aria-label="PDF auswählen"
            >
              PDF auswählen
            </button>
          </div>
        </DocumentFormSection>

        {formError && (
          <p className="pqm-neue-version__error" role="alert">
            {formError}
          </p>
        )}

        <div className="pqm-neue-version__actions">
          <button
            type="submit"
            className="pqm-neue-version__submit"
            disabled={submitting}
          >
            <FilePlus size={16} aria-hidden="true" />
            {submitting ? "Wird gespeichert …" : "Neue Version speichern"}
          </button>
          <button
            type="button"
            className="pqm-neue-version__cancel"
            onClick={() => navigate(`/dokumente/${documentNumber}`)}
          >
            Abbrechen
          </button>
        </div>
      </form>
    </div>
  );
}
