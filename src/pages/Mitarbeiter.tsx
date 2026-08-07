import { useCallback, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import EmployeeToolbar from "../components/employees/EmployeeToolbar";
import EmployeeFilters from "../components/employees/EmployeeFilters";
import EmployeeList from "../components/employees/EmployeeList";
import type { EmployeeRowData } from "../components/employees/EmployeeRow";
import type { BadgeVariant } from "../components/documents/StatusBadge";
import { fetchEmployees, type Employee } from "../lib/employeeApi";
import "./Mitarbeiter.css";

function toRowData(emp: Employee): EmployeeRowData {
  return {
    id: emp.id,
    lastName: emp.last_name,
    firstName: emp.first_name,
    position: emp.position ?? "—",
    responsibilityRoles: emp.responsibilities,
    qmAreas: emp.qm_areas,
    active: emp.is_active,
    activeLabel: emp.is_active ? "aktiv" : "inaktiv",
    activeVariant: (emp.is_active ? "success" : "neutral") as BadgeVariant,
    entryDate: emp.hire_date ?? "—",
    exitDate: emp.departure_date ?? "—",
  };
}

export default function Mitarbeiter() {
  const navigate = useNavigate();
  const [employees, setEmployees] = useState<EmployeeRowData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadEmployees = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await fetchEmployees();
      setEmployees(data.map(toRowData));
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadEmployees();
  }, [loadEmployees]);

  return (
    <div className="pqm-mitarbeiter">
      <EmployeeToolbar
        resultCount={employees.length}
        onNewEmployee={() => navigate("/mitarbeiter/neu")}
      />
      <EmployeeFilters />
      <EmployeeList
        employees={employees}
        loading={loading}
        error={error}
      />
    </div>
  );
}
