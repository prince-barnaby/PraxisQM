import { useCallback, useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { FileText, ChevronLeft } from "lucide-react";
import StatusBadge from "../components/documents/StatusBadge";
import type { BadgeVariant } from "../components/documents/StatusBadge";
import DocumentMetadata from "../components/documents/DocumentMetadata";
import type { MetadataEntry } from "../components/documents/DocumentMetadata";
import DocumentActionBar from "../components/documents/DocumentActionBar";
import DocumentHistory from "../components/documents/DocumentHistory";
import {
  fetchDocumentByNumber,
  listVersions,
  archiveDocument,
  restoreDocument,
  fetchDocumentTags,
  type Document,
  type DocumentVersion,
} from "../lib/documentApi";
import TagList from "../components/documents/TagList";
import "./DokumentDetail.css";

function statusToVariant(status: string): "success" | "neutral" {
  return status === "aktiv" ? "success" : "neutral";
}

function validityToVariant(validity: string | null): BadgeVariant {
  if (validity === null) return "neutral";
  switch (validity) {
    case "gültig":
      return "success";
    case "läuft bald ab":
      return "warning";
    case "abgelaufen":
      return "error";
    default:
      return "neutral";
  }
}

export default function DokumentDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const documentNumber = id ?? "Unbekannt";

  const [doc, setDoc] = useState<Document | null>(null);
  const [versions, setVersions] = useState<DocumentVersion[]>([]);
  const [tags, setTags] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);
  const [actionLoading, setActionLoading] = useState(false);

  const loadAll = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const d = await fetchDocumentByNumber(documentNumber);
      setDoc(d);
      const [v, t] = await Promise.all([listVersions(d.id), fetchDocumentTags(d.id)]);
      setVersions(v);
      setTags(t);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [documentNumber]);

  useEffect(() => {
    loadAll();
  }, [loadAll]);

  const handleArchive = useCallback(async () => {
    if (!doc) return;
    const confirmed = window.confirm(
      `Möchten Sie das Dokument "${doc.title}" (${doc.document_number}) wirklich archivieren?\n\nArchivierte Dokumente verschwinden aus der aktiven Übersicht, bleiben aber vollständig erhalten und können wiederhergestellt werden.`,
    );
    if (!confirmed) return;
    setActionLoading(true);
    setActionError(null);
    try {
      await archiveDocument(doc.id);
      await loadAll();
    } catch (err) {
      setActionError(err instanceof Error ? err.message : String(err));
    } finally {
      setActionLoading(false);
    }
  }, [doc, loadAll]);

  const handleRestore = useCallback(async () => {
    if (!doc) return;
    const confirmed = window.confirm(
      `Möchten Sie das Dokument "${doc.title}" (${doc.document_number}) wiederherstellen?\n\nDas Dokument erscheint wieder in der aktiven Übersicht.`,
    );
    if (!confirmed) return;
    setActionLoading(true);
    setActionError(null);
    try {
      await restoreDocument(doc.id);
      await loadAll();
    } catch (err) {
      setActionError(err instanceof Error ? err.message : String(err));
    } finally {
      setActionLoading(false);
    }
  }, [doc, loadAll]);

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

  const isArchived = doc.status === "archiviert";

  const metadata: MetadataEntry[] = [
    { label: "Dokumentnummer", value: doc.document_number, mono: true },
    { label: "Titel", value: doc.title },
    { label: "Kategorie", value: doc.category_name ?? "—" },
    { label: "Unterkategorie", value: doc.subcategory_name ?? "—" },
    { label: "Version", value: doc.version, mono: true },
    { label: "Status", value: doc.status },
    { label: "Verantwortliche Person", value: doc.responsible_person_name ?? "—" },
    { label: "Gültigkeit", value: doc.computed_validity ?? "—" },
    { label: "Gültig bis", value: doc.valid_until ?? "—" },
    { label: "Letzte Änderung", value: doc.updated_at },
  ];

  if (isArchived && doc.archived_at) {
    const secs = Number(doc.archived_at);
    if (!Number.isNaN(secs) && secs > 0) {
      metadata.splice(metadata.length - 1, 0, {
        label: "Archivierungsdatum",
        value: new Date(secs * 1000).toLocaleString("de-DE"),
      });
    }
  }

  return (
    <div className="pqm-dokument-detail">
      <a
        href={isArchived ? "/archiv" : "/dokumente"}
        className="pqm-dokument-detail__back"
        aria-label={isArchived ? "Zurück zum Archiv" : "Zurück zu Dokumenten"}
      >
        <ChevronLeft size={16} aria-hidden="true" />
        {isArchived ? "Zurück zum Archiv" : "Zurück zu Dokumenten"}
      </a>

      <header className="pqm-dokument-detail__header">
        <div className="pqm-dokument-detail__header-main">
          <h2 className="pqm-dokument-detail__title">{doc.title}</h2>
          <span className="pqm-dokument-detail__number">{doc.document_number}</span>
        </div>
        <StatusBadge label={doc.status} variant={statusToVariant(doc.status)} />
        {doc.computed_validity && (
          <StatusBadge
            label={doc.computed_validity}
            variant={validityToVariant(doc.computed_validity)}
          />
        )}
      </header>

      <DocumentMetadata entries={metadata} />

      <TagList tags={tags} />

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
            disabled
            aria-label="PDF öffnen – noch nicht implementiert"
          >
            PDF öffnen
          </button>
        </div>
      </section>

      <DocumentHistory versions={versions} />

      {actionError && (
        <p className="pqm-dokument-detail__error" role="alert">
          {actionError}
        </p>
      )}

      <DocumentActionBar
        pdfFileName={doc.file_name ?? "—"}
        isArchived={isArchived}
        onEdit={!isArchived ? () => navigate(`/dokumente/${doc.document_number}/bearbeiten`) : undefined}
        onNewVersion={!isArchived ? () => navigate(`/dokumente/${doc.document_number}/neue-version`) : undefined}
        onArchive={!isArchived && !actionLoading ? handleArchive : undefined}
        onRestore={isArchived && !actionLoading ? handleRestore : undefined}
      />
    </div>
  );
}
