import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { ChevronLeft } from "lucide-react";
import EmployeeForm from "../components/employees/EmployeeForm";
import type { EmployeeFormData } from "../components/employees/EmployeeForm";
import {
  fetchResponsibilities,
  fetchQmAreas,
  createEmployee,
  type MasterDataItem,
} from "../lib/employeeApi";
import "./MitarbeiterNeu.css";

export default function MitarbeiterNeu() {
  const navigate = useNavigate();
  const [responsibilities, setResponsibilities] = useState<MasterDataItem[]>([]);
  const [qmAreas, setQmAreas] = useState<MasterDataItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [masterError, setMasterError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const [resp, areas] = await Promise.all([
          fetchResponsibilities(),
          fetchQmAreas(),
        ]);
        if (cancelled) return;
        setResponsibilities(resp);
        setQmAreas(areas);
      } catch (err) {
        if (cancelled) return;
        setMasterError(err instanceof Error ? err.message : String(err));
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const handleSubmit = async (data: EmployeeFormData) => {
    setSubmitting(true);
    setError(null);
    try {
      await createEmployee({
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

  return (
    <div className="pqm-mitarbeiter-neu">
      <button
        type="button"
        className="pqm-mitarbeiter-neu__back"
        onClick={() => navigate("/mitarbeiter")}
        aria-label="Zurück zu Mitarbeitern"
      >
        <ChevronLeft size={16} aria-hidden="true" />
        Zurück zu Mitarbeitern
      </button>

      <header className="pqm-mitarbeiter-neu__header">
        <h2 className="pqm-mitarbeiter-neu__title">Neuer Mitarbeiter</h2>
        <p className="pqm-mitarbeiter-neu__subtitle">
          Anlegen eines neuen Mitarbeitereintrags im Mitarbeiterregister
        </p>
      </header>

      {loading ? (
        <p className="pqm-mitarbeiter-neu__loading">Stammdaten werden geladen …</p>
      ) : masterError ? (
        <p className="pqm-mitarbeiter-neu__error" role="alert">
          Fehler beim Laden der Stammdaten: {masterError}
        </p>
      ) : (
        <EmployeeForm
          responsibilities={responsibilities}
          qmAreas={qmAreas}
          onSubmit={handleSubmit}
          onCancel={() => navigate("/mitarbeiter")}
          submitting={submitting}
          error={error}
        />
      )}
    </div>
  );
}
