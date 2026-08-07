import { useParams, useNavigate } from "react-router-dom";
import { ChevronLeft } from "lucide-react";
import DocumentForm from "../components/documents/DocumentForm";
import "./DokumentBearbeiten.css";

export default function DokumentBearbeiten() {
  const { id } = useParams();
  const navigate = useNavigate();
  const documentNumber = id ?? "Unbekannt";

  return (
    <div className="pqm-dokument-bearbeiten">
      <button
        type="button"
        className="pqm-dokument-bearbeiten__back"
        onClick={() => navigate(`/dokumente/${documentNumber}`)}
        aria-label="Zurück zur Dokumentdetailansicht"
      >
        <ChevronLeft size={16} aria-hidden="true" />
        Zurück zum Dokument
      </button>

      <header className="pqm-dokument-bearbeiten__header">
        <h2 className="pqm-dokument-bearbeiten__title">Dokument bearbeiten</h2>
        <p className="pqm-dokument-bearbeiten__subtitle">
          Bearbeiten von „{documentNumber}" – Platzhalter, ohne Speicherung
        </p>
      </header>

      <DocumentForm
        mode="edit"
        documentNumber={documentNumber}
        pdfFileName="platzhalter-dokument.pdf"
        onCancel={() => navigate(`/dokumente/${documentNumber}`)}
      />
    </div>
  );
}
