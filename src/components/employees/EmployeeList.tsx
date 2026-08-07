import { Users } from "lucide-react";
import "./EmployeeList.css";
import type { EmployeeRowData } from "./EmployeeRow";
import EmployeeRow from "./EmployeeRow";
import EmptyState from "../documents/EmptyState";

interface EmployeeListProps {
  employees: EmployeeRowData[];
}

export default function EmployeeList({ employees }: EmployeeListProps) {
  if (employees.length === 0) {
    return (
      <EmptyState
        icon={Users}
        title="Keine Mitarbeiter vorhanden"
        message="Es wurden noch keine Mitarbeitenden angelegt. Diese Ansicht ist ein Platzhalter."
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
            <th scope="col">Funktion</th>
            <th scope="col">Bereich</th>
            <th scope="col">Verantwortungsposition</th>
            <th scope="col">QM-Bereich</th>
            <th scope="col">Status</th>
            <th scope="col">Eintritt</th>
            <th scope="col">Austritt</th>
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
