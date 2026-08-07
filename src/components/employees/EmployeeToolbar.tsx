import { Plus } from "lucide-react";
import "./EmployeeToolbar.css";

interface EmployeeToolbarProps {
  resultCount: number;
}

export default function EmployeeToolbar({ resultCount }: EmployeeToolbarProps) {
  return (
    <header className="pqm-employee-toolbar">
      <div className="pqm-employee-toolbar__heading">
        <h2 className="pqm-employee-toolbar__title">Mitarbeiter</h2>
        <p className="pqm-employee-toolbar__subtitle">
          Mitarbeiterregister der Praxis – Übersicht der Mitarbeitenden, die als
          verantwortliche Personen eintragbar sind
        </p>
      </div>
      <div className="pqm-employee-toolbar__actions">
        <button
          type="button"
          className="pqm-employee-toolbar__button"
          disabled
          aria-label="Neuen Mitarbeiter anlegen – Platzhalter, nicht funktional"
        >
          <Plus size={18} aria-hidden="true" />
          Neuer Mitarbeiter
        </button>
      </div>
      <p
        className="pqm-employee-toolbar__count"
        aria-label={`Anzahl Mitarbeiter: ${resultCount}`}
      >
        {resultCount} Mitarbeiter
      </p>
    </header>
  );
}
