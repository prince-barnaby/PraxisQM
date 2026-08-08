import { Users } from "lucide-react";
import "./EmployeeList.css";
import type { EmployeeRowData } from "./EmployeeRow";
import EmployeeRow from "./EmployeeRow";
import EmptyState from "../documents/EmptyState";

interface EmployeeListProps {
  employees: EmployeeRowData[];
  loading?: boolean;
  error?: string | null;
  filteredEmpty?: boolean;
}

export default function EmployeeList({ employees, loading, error, filteredEmpty = false }: EmployeeListProps) {
  if (loading) {
    return (
      <div className="pqm-employee-list pqm-employee-list--state">
        <p className="pqm-employee-list__message">Mitarbeiter werden geladen …</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="pqm-employee-list pqm-employee-list--state">
        <p className="pqm-employee-list__message pqm-employee-list__message--error">
          Fehler beim Laden der Mitarbeiter: {error}
        </p>
      </div>
    );
  }

  if (employees.length === 0) {
    return (
      <EmptyState
        icon={Users}
        title={filteredEmpty ? "Keine Mitarbeitenden entsprechen den ausgewählten Filtern" : "Noch keine Mitarbeitenden erfasst"}
        message={filteredEmpty ? "Aktuell gibt es keine Treffer für die gewählten Filterkriterien. Setzen Sie die Filter zurück, um alle Mitarbeitenden zu sehen." : "Es wurden noch keine Mitarbeitenden angelegt. Klicken Sie auf „Neuer Mitarbeiter\u201c, um einen Eintrag zu erstellen."}
      />
    );
  }

  return (
    <div className="pqm-employee-list" role="region" aria-label="Mitarbeiterliste">
      <table className="pqm-employee-list__table">
        <thead>
          <tr>
            <th scope="col">Name</th>
            <th scope="col">Vorname</th>
            <th scope="col">Position</th>
            <th scope="col">Verantwortungsposition</th>
            <th scope="col">QM-Bereich</th>
            <th scope="col">Status</th>
            <th scope="col">Eintritt</th>
            <th scope="col">Austritt</th>
            <th scope="col" aria-label="Aktionen"></th>
          </tr>
        </thead>
        <tbody>
          {employees.map((emp) => (
            <EmployeeRow key={emp.id} employee={emp} />
          ))}
        </tbody>
      </table>
    </div>
  );
}
