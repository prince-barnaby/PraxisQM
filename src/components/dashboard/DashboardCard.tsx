import type { LucideIcon } from "lucide-react";
import "./DashboardCard.css";

interface DashboardCardProps {
  icon: LucideIcon;
  title: string;
  value: string;
  description: string;
}

export default function DashboardCard({
  icon: Icon,
  title,
  value,
  description,
}: DashboardCardProps) {
  return (
    <section
      className="pqm-dashboard-card"
      role="region"
      aria-label={title}
    >
      <header className="pqm-dashboard-card__header">
        <span className="pqm-dashboard-card__icon" aria-hidden="true">
          <Icon size={24} />
        </span>
        <h3 className="pqm-dashboard-card__title">{title}</h3>
      </header>
      <div className="pqm-dashboard-card__value" aria-label={`${title} – Platzhalter`}>
        {value}
      </div>
      <p className="pqm-dashboard-card__description">{description}</p>
    </section>
  );
}
