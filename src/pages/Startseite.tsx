import { FileText, Users, Archive, MonitorSmartphone } from "lucide-react";
import DashboardGrid from "../components/dashboard/DashboardGrid";
import DashboardCard from "../components/dashboard/DashboardCard";
import "./Startseite.css";

export default function Startseite() {
  return (
    <div className="pqm-startseite">
      <header className="pqm-startseite__header">
        <h2 className="pqm-startseite__heading">Übersicht</h2>
        <p className="pqm-startseite__subtitle">
          Statusübersicht der Qualitätsmanagement-Dokumentation
        </p>
      </header>
      <DashboardGrid>
        <DashboardCard
          icon={FileText}
          title="Dokumente"
          value="—"
          description="Anzahl der verwalteten Dokumente. Platzhalter – wird mit Datenbankanbindung gefüllt."
        />
        <DashboardCard
          icon={Users}
          title="Mitarbeiter"
          value="—"
          description="Anzahl der erfassten Mitarbeitenden. Platzhalter – wird mit Datenbankanbindung gefüllt."
        />
        <DashboardCard
          icon={Archive}
          title="Archiv"
          value="—"
          description="Archivierte Dokumente. Platzhalter – wird mit Datenbankanbindung gefüllt."
        />
        <DashboardCard
          icon={MonitorSmartphone}
          title="Systemstatus"
          value="Platzhalter"
          description="Lokale System- und Speicherinformationen. Platzhalter – wird mit Systemprüfung gefüllt."
        />
      </DashboardGrid>
    </div>
  );
}
