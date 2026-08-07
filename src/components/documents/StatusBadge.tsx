import type { LucideIcon } from "lucide-react";
import "./StatusBadge.css";

export type BadgeVariant = "success" | "warning" | "error" | "neutral";

interface StatusBadgeProps {
  label: string;
  variant?: BadgeVariant;
  icon?: LucideIcon;
}

export default function StatusBadge({
  label,
  variant = "neutral",
  icon: Icon,
}: StatusBadgeProps) {
  return (
    <span
      className={`pqm-status-badge pqm-status-badge--${variant}`}
      role="status"
    >
      {Icon && (
        <span className="pqm-status-badge__icon" aria-hidden="true">
          <Icon size={12} />
        </span>
      )}
      {label}
    </span>
  );
}
