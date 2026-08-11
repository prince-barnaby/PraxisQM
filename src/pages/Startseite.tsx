import { useCallback, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { FileText, Users, Archive, CircleCheck as CheckCircle, TriangleAlert as AlertTriangle, Circle as XCircle, Clock } from "lucide-react";
import DashboardGrid from "../components/dashboard/DashboardGrid";
import DashboardCard from "../components/dashboard/DashboardCard";
import ReviewList from "../components/dashboard/ReviewList";
import {
  fetchDashboardSummary,
  fetchReviewList,
  type DashboardSummary,
  type ReviewEntry,
} from "../lib/dashboardApi";
import "./Startseite.css";

export default function Startseite() {
  const navigate = useNavigate();
  const [summary, setSummary] = useState<DashboardSummary | null>(null);
  const [reviews, setReviews] = useState<ReviewEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadDashboard = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [s, r] = await Promise.all([fetchDashboardSummary(), fetchReviewList()]);
      setSummary(s);
      setReviews(r);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadDashboard();
  }, [loadDashboard]);

  const totalActive = summary?.total_active ?? "—";
  const employees = summary?.employees ?? "—";
  const archived = summary?.archived ?? "—";

  return (
    <div className="pqm-startseite">
      <header className="pqm-startseite__header">
        <h2 className="pqm-startseite__heading">Übersicht</h2>
        <p className="pqm-startseite__subtitle">
          Statusübersicht der Qualitätsmanagement-Dokumentation
        </p>
      </header>

      {error && (
        <p className="pqm-startseite__error" role="alert">
          Fehler beim Laden des Dashboards: {error}
        </p>
      )}

      <DashboardGrid>
        <DashboardCard
          icon={FileText}
          title="Dokumente"
          value={loading ? "…" : String(totalActive)}
          description="Aktive QM-Dokumente in der Verwaltung."
          onClick={() => navigate("/dokumente")}
        />
        <DashboardCard
          icon={Users}
          title="Mitarbeiter"
          value={loading ? "…" : String(employees)}
          description="Erfasste Mitarbeitenden im Mitarbeiterregister."
          onClick={() => navigate("/mitarbeiter")}
        />
        <DashboardCard
          icon={Archive}
          title="Archiv"
          value={loading ? "…" : String(archived)}
          description="Archivierte Dokumente."
          onClick={() => navigate("/archiv")}
        />
      </DashboardGrid>

      {summary && (summary.warning > 0 || summary.expired > 0) && (
        <div className="pqm-startseite__validity-cards">
          {summary.expired > 0 && (
            <DashboardCard
              icon={XCircle}
              title="Abgelaufen"
              value={String(summary.expired)}
              description="Dokumente mit überschrittenem Gültigkeitsdatum."
              onClick={() => navigate("/dokumente")}
            />
          )}
          {summary.warning > 0 && (
            <DashboardCard
              icon={AlertTriangle}
              title="Läuft bald ab"
              value={String(summary.warning)}
              description="Dokumente, die innerhalb von 30 Tagen ablaufen."
              onClick={() => navigate("/dokumente")}
            />
          )}
        </div>
      )}

      <section className="pqm-startseite__review-section">
        <div className="pqm-startseite__review-header">
          <h3 className="pqm-startseite__review-heading">
            <Clock size={20} aria-hidden="true" />
            Prüfungsbedürftige Dokumente
          </h3>
          {summary && summary.valid > 0 && (
            <span className="pqm-startseite__valid-count">
              <CheckCircle size={16} aria-hidden="true" />
              {summary.valid} gültig
            </span>
          )}
        </div>
        <ReviewList entries={reviews} loading={loading} />
      </section>
    </div>
  );
}
