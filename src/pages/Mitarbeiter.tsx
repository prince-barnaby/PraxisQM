import { useCallback, useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import EmployeeToolbar from "../components/employees/EmployeeToolbar";
import EmployeeFilters, {
  type EmployeeFilterValues,
  NO_FILTERS,
} from "../components/employees/EmployeeFilters";
import EmployeeList from "../components/employees/EmployeeList";
import type { EmployeeRowData } from "../components/employees/EmployeeRow";
import type { BadgeVariant } from "../components/documents/StatusBadge";
import {
  fetchEmployees,
  fetchResponsibilities,
  fetchQmAreas,
  type Employee,
  type MasterDataItem,
} from "../lib/employeeApi";
import "./Mitarbeiter.css";

function toRowData(emp: Employee): EmployeeRowData {
  return {
    id: emp.id,
    lastName: emp.last_name,
    firstName: emp.first_name,
    position: emp.position ?? "—",
    responsibilityRoles: emp.responsibilities,
    qmAreas: emp.qm_areas,
    responsibilityIds: emp.responsibility_ids,
    qmAreaIds: emp.qm_area_ids,
    active: emp.is_active,
    activeLabel: emp.is_active ? "aktiv" : "inaktiv",
    activeVariant: (emp.is_active ? "success" : "neutral") as BadgeVariant,
    entryDate: emp.hire_date ?? "—",
    exitDate: emp.departure_date ?? "—",
  };
}

function matchesFilters(
  emp: EmployeeRowData,
  filters: EmployeeFilterValues,
): boolean {
  if (filters.activeStatus === "active" && !emp.active) return false;
  if (filters.activeStatus === "inactive" && emp.active) return false;
  if (filters.position && emp.position !== filters.position) return false;
  if (
    filters.responsibilityId &&
    !emp.responsibilityIds.includes(filters.responsibilityId)
  )
    return false;
  if (filters.qmAreaId && !emp.qmAreaIds.includes(filters.qmAreaId))
    return false;
  return true;
}

export default function Mitarbeiter() {
  const navigate = useNavigate();
  const [allEmployees, setAllEmployees] = useState<EmployeeRowData[]>([]);
  const [responsibilities, setResponsibilities] = useState<MasterDataItem[]>([]);
  const [qmAreas, setQmAreas] = useState<MasterDataItem[]>([]);
  const [filters, setFilters] = useState<EmployeeFilterValues>(NO_FILTERS);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadEmployees = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [empData, respData, areaData] = await Promise.all([
        fetchEmployees(),
        fetchResponsibilities(),
        fetchQmAreas(),
      ]);
      setAllEmployees(empData.map(toRowData));
      setResponsibilities(respData);
      setQmAreas(areaData);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadEmployees();
  }, [loadEmployees]);

  const positionOptions = useMemo(() => {
    const unique = new Map<string, string>();
    for (const emp of allEmployees) {
      if (emp.position && emp.position !== "—") {
        unique.set(emp.position, emp.position);
      }
    }
    return Array.from(unique.entries()).map(([value, label]) => ({ value, label }));
  }, [allEmployees]);

  const filteredEmployees = useMemo(() => {
    return allEmployees.filter((emp) => matchesFilters(emp, filters));
  }, [allEmployees, filters]);

  const hasActiveFilters =
    filters.activeStatus !== "all" ||
    filters.position !== "" ||
    filters.responsibilityId !== "" ||
    filters.qmAreaId !== "";

  return (
    <div className="pqm-mitarbeiter">
      <EmployeeToolbar
        resultCount={filteredEmployees.length}
        onNewEmployee={() => navigate("/mitarbeiter/neu")}
      />
      <EmployeeFilters
        values={filters}
        onChange={setFilters}
        positions={positionOptions}
        responsibilities={responsibilities.map((r) => ({ value: r.id, label: r.name }))}
        qmAreas={qmAreas.map((a) => ({ value: a.id, label: a.name }))}
      />
      <EmployeeList
        employees={filteredEmployees}
        loading={loading}
        error={error}
        filteredEmpty={hasActiveFilters && allEmployees.length > 0 && filteredEmployees.length === 0}
      />
    </div>
  );
}
