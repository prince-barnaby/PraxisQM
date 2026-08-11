import type { LucideIcon } from "lucide-react";
import { ArrowRight } from "lucide-react";
import "./DashboardCard.css";

interface DashboardCardProps {
  icon: LucideIcon;
  title: string;
  value: string;
  description: string;
  onClick?: () => void;
  to?: string;
}

export default function DashboardCard({
  icon: Icon,
  title,
  value,
  description,
  onClick,
  to,
}: DashboardCardProps) {
  const handleClick = () => {
    if (onClick) {
      onClick();
    } else if (to) {
      window.location.hash = to;
    }
  };

  const interactive = Boolean(onClick || to);

  return (
    <article
      className="pqm-dashboard-card"
      role={interactive ? "button" : undefined}
      tabIndex={interactive ? 0 : undefined}
      aria-label={`${title} – ${description}`}
      onClick={interactive ? handleClick : undefined}
      onKeyDown={
        interactive
          ? (e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                handleClick();
              }
            }
          : undefined
      }
    >
      <div className="pqm-dashboard-card__icon" aria-hidden="true">
        <Icon size={40} />
      </div>
      <h3 className="pqm-dashboard-card__title">{title}</h3>
      <div className="pqm-dashboard-card__value" aria-label={`${title} – ${value}`}>
        {value}
      </div>
      <p className="pqm-dashboard-card__description">{description}</p>
      {interactive && (
        <span className="pqm-dashboard-card__arrow" aria-hidden="true">
          <ArrowRight size={18} />
        </span>
      )}
    </article>
  );
}
