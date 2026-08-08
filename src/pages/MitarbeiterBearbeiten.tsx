import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { ChevronLeft } from "lucide-react";
import EmployeeForm from "../components/employees/EmployeeForm";
import type { EmployeeFormData } from "../components/employees/EmployeeForm";
import {
  fetchEmployee,
  updateEmployee,
  fetchResponsibilities,
  fetchQmAreas,
  type Employee,
  type MasterDataItem,
} from "../lib/employeeApi";
import "./MitarbeiterBearbeiten.css";

export default function MitarbeiterBearbeiten() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [employee, setEmployee] = useState<Employee | null>(null);
  const [responsibilities, setResponsibilities] = useState<MasterDataItem[]>([]);
  const [qmAreas, setQmAreas] = useState<MasterDataItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);

  useEffect(() => {
    if (!id) return;
    let cancelled = false;
    (async () => {
      try {
        const [emp, resp, areas] = await Promise.all([
          fetchEmployee(id),
          fetchResponsibilities(),
          fetchQmAreas(),
        ]);
        if (cancelled) return;
        setEmployee(emp);
        setResponsibilities(resp);
        setQmAreas(areas);
      } catch (err) {
        if (cancelled) return;
        setLoadError(err instanceof Error ? err.message : String(err));
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, [id]);

  const handleSubmit = async (data: EmployeeFormData) => {
    if (!id) return;
    setSubmitting(true);
    setError(null);
    try {
      await updateEmployee(id, {
        last_name: data.last_name,
        first_name: data.first_name,
        position: data.position || null,
        is_active: data.is_active,
        hire_date: data.hire_date || null,
        departure_date: data.departure_date || null,
        responsibility_ids: data.responsibility_ids,
        qm_area_ids: data.qm_area_ids,
      });
      navigate("/mitarbeiter");
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      setSubmitting(false);
    }
  };

  const initialValues: EmployeeFormData | undefined = employee
    ? {
        last_name: employee.last_name,
        first_name: employee.first_name,
        position: employee.position ?? "",
        is_active: employee.is_active,
        hire_date: employee.hire_date ?? "",
        departure_date: employee.departure_date ?? "",
        responsibility_ids: employee.responsibility_ids,
        qm_area_ids: employee.qm_area_ids,
      }
    : undefined;

  return (
    <div className="pqm-mitarbeiter-bearbeiten">
      <button
        type="button"
        className="pqm-mitarbeiter-bearbeiten__back"
        onClick={() => navigate("/mitarbeiter")}
        aria-label="Zurück zu Mitarbeitern"
      >
        <ChevronLeft size={16} aria-hidden="true" />
        Zurück zu Mitarbeitern
      </button>

      <header className="pqm-mitarbeiter-bearbeiten__header">
        <h2 className="pqm-mitarbeiter-bearbeiten__title">Mitarbeiter bearbeiten</h2>
        <p className="pqm-mitarbeiter-bearbeiten__subtitle">
          Bearbeiten eines bestehenden Mitarbeitereintrags
        </p>
      </header>

      {loading ? (
        <p className="pqm-mitarbeiter-bearbeiten__loading">
          Mitarbeiterdaten werden geladen …
        </p>
      ) : loadError ? (
        <p className="pqm-mitarbeiter-bearbeiten__error" role="alert">
          {loadError}
        </p>
      ) : (
        <EmployeeForm
          responsibilities={responsibilities}
          qmAreas={qmAreas}
          onSubmit={handleSubmit}
          onCancel={() => navigate("/mitarbeiter")}
          submitting={submitting}
          error={error}
          initialValues={initialValues}
          submitLabel="Änderungen speichern"
          ariaLabel="Mitarbeiter bearbeiten"
        />
      )}
    </div>
  );
}
