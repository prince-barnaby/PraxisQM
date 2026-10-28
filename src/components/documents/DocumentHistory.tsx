import { Clock, FileText } from "lucide-react";
import { useState } from "react";
import type { DocumentVersion } from "../../lib/documentApi";
import { openDocumentVersionPdf } from "../../lib/documentApi";
import "./DocumentHistory.css";

interface DocumentHistoryProps {
  versions: DocumentVersion[];
}

export default function DocumentHistory({ versions }: DocumentHistoryProps) {
  const [openingId, setOpeningId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleOpen = async (versionId: string) => {
    setOpeningId(versionId);
    setError(null);
    try {
      await Promise.race([
        openDocumentVersionPdf(versionId),
        new Promise<never>((_, reject) =>
          setTimeout(
            () => reject(new Error("Das PDF konnte nicht geöffnet werden – Zeitüberschreitung.")),
            15000,
          ),
        ),
      ]);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setOpeningId(null);
    }
  };

  if (versions.length === 0) {
    return (
      <section className="pqm-document-history" aria-label="Versionshistorie">
        <h3 className="pqm-document-history__heading">
          <Clock size={16} aria-hidden="true" />
          Versionshistorie
        </h3>
        <p className="pqm-document-history__empty">Keine Versionen vorhanden.</p>
      </section>
    );
  }

  return (
    <section className="pqm-document-history" aria-label="Versionshistorie">
      <h3 className="pqm-document-history__heading">
        <Clock size={16} aria-hidden="true" />
        Versionshistorie
      </h3>
      {error && (
        <p className="pqm-document-history__error" role="alert">
          {error}
        </p>
      )}
      <ol className="pqm-document-history__timeline">
        {versions.map((v) => (
          <li
            key={v.id}
            className={`pqm-document-history__item${v.is_current ? " pqm-document-history__item--current" : ""}`}
          >
            <span className="pqm-document-history__dot" aria-hidden="true" />
            <div className="pqm-document-history__entry">
              <div className="pqm-document-history__entry-header">
                <span className="pqm-document-history__version">{v.version_number}</span>
                {v.is_current && (
                  <span className="pqm-document-history__current-badge">Aktuell</span>
                )}
                <button
                  type="button"
                  className="pqm-document-history__open-btn"
                  onClick={() => handleOpen(v.id)}
                  disabled={openingId === v.id}
                  aria-label={`PDF öffnen – Version ${v.version_number}`}
                >
                  <FileText size={14} aria-hidden="true" />
                  {openingId === v.id ? "Wird geöffnet …" : "Öffnen"}
                </button>
              </div>
              <dl className="pqm-document-history__meta">
                <div className="pqm-document-history__meta-row">
                  <dt>Status</dt>
                  <dd>{v.status}</dd>
                </div>
                <div className="pqm-document-history__meta-row">
                  <dt>Datum</dt>
                  <dd>{v.uploaded_at}</dd>
                </div>
                {v.valid_until && (
                  <div className="pqm-document-history__meta-row">
                    <dt>Gültig bis</dt>
                    <dd>{v.valid_until}</dd>
                  </div>
                )}
              </dl>
            </div>
          </li>
        ))}
      </ol>
    </section>
  );
}
