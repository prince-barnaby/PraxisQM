import type { ReactNode } from "react";
import "./DashboardGrid.css";

interface DashboardGridProps {
  children: ReactNode;
}

export default function DashboardGrid({ children }: DashboardGridProps) {
  return (
    <div className="pqm-dashboard-grid">
      {children}
    </div>
  );
}
