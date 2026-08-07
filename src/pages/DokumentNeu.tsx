import { useNavigate } from "react-router-dom";
import { ChevronLeft } from "lucide-react";
import DocumentForm from "../components/documents/DocumentForm";
import "./DokumentNeu.css";

export default function DokumentNeu() {
  const navigate = useNavigate();

  return (
    <div className="pqm-dokument-neu">
      <button
        type="button"
        className="pqm-dokument-neu__back"
        onClick={() => navigate("/dokumente")}
        aria-label="Zurück zu Dokumenten"
      >
        <ChevronLeft size={16} aria-hidden="true" />
        Zurück zu Dokumenten
      </button>

      <header className="pqm-dokument-neu__header">
        <h2 className="pqm-dokument-neu__title">Neues Dokument</h2>
        <p className="pqm-dokument-neu__subtitle">
          Anlegen eines neuen QM-Dokuments – Platzhalter, ohne Speicherung
        </p>
      </header>

      <DocumentForm
        mode="create"
        onCancel={() => navigate("/dokumente")}
      />
    </div>
  );
}
