import type { ReactNode } from "react";
import "./DashboardGrid.css";

interface DashboardGridProps {
  children: ReactNode;
  columns?: 1 | 2;
}

export default function DashboardGrid({ children, columns = 2 }: DashboardGridProps) {
  const className = columns === 1
    ? "pqm-dashboard-grid pqm-dashboard-grid--single"
    : "pqm-dashboard-grid";
  return (
    <div className={className}>
      {children}
    </div>
  );
}
