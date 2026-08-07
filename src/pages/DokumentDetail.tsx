import { FileText, ChevronLeft } from "lucide-react";
import StatusBadge from "../components/documents/StatusBadge";
import DocumentMetadata from "../components/documents/DocumentMetadata";
import type { MetadataEntry } from "../components/documents/DocumentMetadata";
import TagList from "../components/documents/TagList";
import DocumentActionBar from "../components/documents/DocumentActionBar";
import DocumentHistory from "../components/documents/DocumentHistory";
import type { HistoryEntry } from "../components/documents/DocumentHistory";
import "./DokumentDetail.css";

const PLACEHOLDER_TITLE = "Platzhalter-Dokument 1";
const PLACEHOLDER_NUMBER = "PQM-0001";
const PLACEHOLDER_PDF = "platzhalter-dokument-1.pdf";

const METADATA_ENTRIES: MetadataEntry[] = [
  { label: "Dokumentnummer", value: PLACEHOLDER_NUMBER, mono: true },
  { label: "Titel", value: PLACEHOLDER_TITLE },
  { label: "Kategorie", value: "Platzhalter" },
  { label: "Unterkategorie", value: "Platzhalter" },
  { label: "Version", value: "1.0", mono: true },
  { label: "Status", value: "aktiv" },
  { label: "Verantwortliche Person", value: "Platzhalter" },
  { label: "Gültig bis", value: "Platzhalter" },
  { label: "Letzte Änderung", value: "Platzhalter" },
];

const PLACEHOLDER_TAGS = [
  "Platzhalter-Tag 1",
  "Platzhalter-Tag 2",
  "Platzhalter-Tag 3",
];

const HISTORY_ENTRIES: HistoryEntry[] = [
  { id: "h1", label: "Version 1.0 erstellt" },
  { id: "h2", label: "Version 1.1 geändert" },
  { id: "h3", label: "Version 1.2 freigegeben" },
];

export default function DokumentDetail() {
  return (
    <div className="pqm-dokument-detail">
      <a
        href="/dokumente"
        className="pqm-dokument-detail__back"
        aria-label="Zurück zur Dokumentenübersicht"
      >
        <ChevronLeft size={16} aria-hidden="true" />
        Zurück zur Übersicht
      </a>

      <header className="pqm-dokument-detail__header">
        <div className="pqm-dokument-detail__header-main">
          <h2 className="pqm-dokument-detail__title">{PLACEHOLDER_TITLE}</h2>
          <span className="pqm-dokument-detail__number">{PLACEHOLDER_NUMBER}</span>
        </div>
        <StatusBadge label="aktiv" variant="success" />
      </header>

      <DocumentMetadata entries={METADATA_ENTRIES} />

      <section
        className="pqm-dokument-detail__card"
        aria-label="Dokumentbeschreibung"
      >
        <h3 className="pqm-dokument-detail__card-heading">Beschreibung</h3>
        <p className="pqm-dokument-detail__description">
          Dies ist ein Platzhalter-Text für die Dokumentbeschreibung. Die
          eigentliche Beschreibung wird später aus den Dokumentmetadaten
          geladen. Dieser Text dient ausschließlich der Darstellung des
          Layouts und der Komponenten.
        </p>
      </section>

      <section
        className="pqm-dokument-detail__card"
        aria-label="Angehängtes Dokument"
      >
        <h3 className="pqm-dokument-detail__card-heading">Dokumentdatei</h3>
        <div className="pqm-dokument-detail__attachment">
          <FileText size={28} aria-hidden="true" />
          <span className="pqm-dokument-detail__filename">{PLACEHOLDER_PDF}</span>
          <button
            type="button"
            className="pqm-dokument-detail__pdf-button"
            disabled
            aria-label="PDF öffnen – Platzhalter, nicht funktional"
          >
            PDF öffnen
          </button>
        </div>
      </section>

      <TagList tags={PLACEHOLDER_TAGS} />

      <DocumentActionBar pdfFileName={PLACEHOLDER_PDF} />

      <DocumentHistory entries={HISTORY_ENTRIES} />

      <p className="pqm-dokument-detail__hint">
        Hinweis: Alle Inhalte sind Platzhalter. Aktionen sind nicht funktional.
      </p>
    </div>
  );
}
