import type { LucideIcon } from "lucide-react";
import { ArrowRight } from "lucide-react";
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
    <article
      className="pqm-dashboard-card"
      role="button"
      tabIndex={0}
      aria-label={`${title} – ${description}`}
    >
      <div className="pqm-dashboard-card__icon" aria-hidden="true">
        <Icon size={40} />
      </div>
      <h3 className="pqm-dashboard-card__title">{title}</h3>
      <div
        className="pqm-dashboard-card__value"
        aria-label={`${title} – Platzhalter`}
      >
        {value}
      </div>
      <p className="pqm-dashboard-card__description">{description}</p>
      <span className="pqm-dashboard-card__arrow" aria-hidden="true">
        <ArrowRight size={18} />
      </span>
    </article>
  );
}
