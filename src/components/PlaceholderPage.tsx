import type { LucideIcon } from "lucide-react";
import "./PlaceholderPage.css";

interface PlaceholderPageProps {
  icon: LucideIcon;
  title: string;
  message: string;
}

export default function PlaceholderPage({
  icon: Icon,
  title,
  message,
}: PlaceholderPageProps) {
  return (
    <div className="pqm-placeholder">
      <div className="pqm-placeholder__card">
        <Icon size={48} color="var(--pqm-color-neutral)" />
        <h2 className="pqm-placeholder__title">{title}</h2>
        <p className="pqm-placeholder__message">{message}</p>
      </div>
    </div>
  );
}
