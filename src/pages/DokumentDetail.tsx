import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { FileText, ChevronLeft } from "lucide-react";
import StatusBadge from "../components/documents/StatusBadge";
import DocumentMetadata from "../components/documents/DocumentMetadata";
import type { MetadataEntry } from "../components/documents/DocumentMetadata";
import DocumentActionBar from "../components/documents/DocumentActionBar";
import "./DokumentDetail.css";
import { fetchDocumentByNumber, openPdf, type Document } from "../lib/documentApi";

function statusToVariant(status: string): "success" | "neutral" {
  return status === "aktiv" ? "success" : "neutral";
}

export default function DokumentDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const documentNumber = id ?? "Unbekannt";

  const [doc, setDoc] = useState<Document | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    fetchDocumentByNumber(documentNumber)
      .then((d) => setDoc(d))
      .catch((err) => {
        setError(err instanceof Error ? err.message : String(err));
      })
      .finally(() => setLoading(false));
  }, [documentNumber]);

  const handleOpenPdf = async () => {
    if (!doc) return;
    try {
      await openPdf(doc.id);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    }
  };

  if (loading) {
    return (
      <div className="pqm-dokument-detail">
        <p>Dokument wird geladen …</p>
      </div>
    );
  }

  if (error || !doc) {
    return (
      <div className="pqm-dokument-detail">
        <a
          href="/dokumente"
          className="pqm-dokument-detail__back"
          aria-label="Zurück zu Dokumenten"
        >
          <ChevronLeft size={16} aria-hidden="true" />
          Zurück zu Dokumenten
        </a>
        <p className="pqm-dokument-detail__error" role="alert">
          {error ?? "Dokument konnte nicht geladen werden."}
        </p>
      </div>
    );
  }

  const metadata: MetadataEntry[] = [
    { label: "Dokumentnummer", value: doc.document_number, mono: true },
    { label: "Titel", value: doc.title },
    { label: "Kategorie", value: doc.category_name ?? "—" },
    { label: "Unterkategorie", value: doc.subcategory_name ?? "—" },
    { label: "Version", value: doc.version, mono: true },
    { label: "Status", value: doc.status },
    { label: "Verantwortliche Person", value: doc.responsible_person_name ?? "—" },
    { label: "Gültig bis", value: doc.valid_until ?? "—" },
    { label: "Letzte Änderung", value: doc.updated_at },
  ];

  return (
    <div className="pqm-dokument-detail">
      <a
        href="/dokumente"
        className="pqm-dokument-detail__back"
        aria-label="Zurück zu Dokumenten"
      >
        <ChevronLeft size={16} aria-hidden="true" />
        Zurück zu Dokumenten
      </a>

      <header className="pqm-dokument-detail__header">
        <div className="pqm-dokument-detail__header-main">
          <h2 className="pqm-dokument-detail__title">{doc.title}</h2>
          <span className="pqm-dokument-detail__number">{doc.document_number}</span>
        </div>
        <StatusBadge label={doc.status} variant={statusToVariant(doc.status)} />
      </header>

      <DocumentMetadata entries={metadata} />

      <section
        className="pqm-dokument-detail__card"
        aria-label="Dokumentbeschreibung"
      >
        <h3 className="pqm-dokument-detail__card-heading">Beschreibung</h3>
        <p className="pqm-dokument-detail__description">
          {doc.description ?? "Keine Beschreibung hinterlegt."}
        </p>
      </section>

      <section
        className="pqm-dokument-detail__card"
        aria-label="Angehängtes Dokument"
      >
        <h3 className="pqm-dokument-detail__card-heading">Dokumentdatei</h3>
        <div className="pqm-dokument-detail__attachment">
          <FileText size={28} aria-hidden="true" />
          <span className="pqm-dokument-detail__filename">{doc.file_name ?? "Keine Datei"}</span>
          <button
            type="button"
            className="pqm-dokument-detail__pdf-button"
            onClick={handleOpenPdf}
            disabled={!doc.file_path}
            aria-label="PDF öffnen"
          >
            PDF öffnen
          </button>
        </div>
      </section>

      <DocumentActionBar
        pdfFileName={doc.file_name ?? "—"}
        onEdit={() => navigate(`/dokumente/${doc.document_number}/bearbeiten`)}
      />
    </div>
  );
}
