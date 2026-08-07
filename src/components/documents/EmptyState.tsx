import { FileX } from "lucide-react";
import "./EmptyState.css";

interface EmptyStateProps {
  icon?: typeof FileX;
  title: string;
  message: string;
}

export default function EmptyState({
  icon: Icon = FileX,
  title,
  message,
}: EmptyStateProps) {
  return (
    <div className="pqm-empty-state" role="status">
      <Icon size={40} aria-hidden="true" />
      <h3 className="pqm-empty-state__title">{title}</h3>
      <p className="pqm-empty-state__message">{message}</p>
    </div>
  );
}
