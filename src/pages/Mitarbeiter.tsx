import EmployeeToolbar from "../components/employees/EmployeeToolbar";
import EmployeeFilters from "../components/employees/EmployeeFilters";
import EmployeeList from "../components/employees/EmployeeList";
import type { EmployeeRowData } from "../components/employees/EmployeeRow";
import "./Mitarbeiter.css";

const PLACEHOLDER_EMPLOYEES: EmployeeRowData[] = [
  {
    id: "mock-1",
    lastName: "Müller",
    firstName: "Anna",
    role: "Zahnärztin",
    department: "Behandlung",
    responsibilityRoles: ["QM-Beauftragte", "Datenschutzbeauftragte"],
    qmAreas: ["Datenschutz", "Patientendokumentation", "Röntgeneinweisung"],
    active: true,
    activeLabel: "aktiv",
    activeVariant: "success",
    entryDate: "2019-03-01",
    exitDate: "—",
  },
  {
    id: "mock-2",
    lastName: "Schmidt",
    firstName: "Thomas",
    role: "ZFA",
    department: "Empfang",
    responsibilityRoles: ["Hygienebeauftragte"],
    qmAreas: ["Hygiene"],
    active: true,
    activeLabel: "aktiv",
    activeVariant: "success",
    entryDate: "2021-09-15",
    exitDate: "—",
  },
  {
    id: "mock-3",
    lastName: "Becker",
    firstName: "Julia",
    role: "Praxismanagerin",
    department: "Verwaltung",
    responsibilityRoles: [],
    qmAreas: [],
    active: false,
    activeLabel: "inaktiv",
    activeVariant: "neutral",
    entryDate: "2018-01-10",
    exitDate: "2024-06-30",
  },
];

export default function Mitarbeiter() {
  return (
    <div className="pqm-mitarbeiter">
      <EmployeeToolbar resultCount={PLACEHOLDER_EMPLOYEES.length} />
      <EmployeeFilters />
      <EmployeeList employees={PLACEHOLDER_EMPLOYEES} />
      <p className="pqm-mitarbeiter__hint">
        Hinweis: Alle Einträge sind Platzhalter. Filter und Aktionen sind nicht
        funktional. Das Mitarbeiterregister ist getrennt von der Benutzerverwaltung.
      </p>
    </div>
  );
}
