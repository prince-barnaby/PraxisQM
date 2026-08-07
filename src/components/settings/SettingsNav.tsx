import type { LucideIcon } from "lucide-react";
import "./SettingsNav.css";

export interface SettingsNavItem {
  id: string;
  label: string;
  icon: LucideIcon;
}

interface SettingsNavProps {
  items: SettingsNavItem[];
  activeId: string;
  onSelect: (id: string) => void;
}

export default function SettingsNav({
  items,
  activeId,
  onSelect,
}: SettingsNavProps) {
  return (
    <nav
      className="pqm-settings-nav"
      aria-label="Einstellungsbereiche"
    >
      <ul className="pqm-settings-nav__list">
        {items.map((item) => {
          const isActive = item.id === activeId;
          const Icon = item.icon;
          return (
            <li key={item.id}>
              <button
                type="button"
                className={
                  "pqm-settings-nav__item" +
                  (isActive ? " pqm-settings-nav__item--active" : "")
                }
                aria-current={isActive ? "true" : undefined}
                onClick={() => onSelect(item.id)}
              >
                <Icon size={16} aria-hidden="true" />
                <span>{item.label}</span>
              </button>
            </li>
          );
        })}
      </ul>
    </nav>
  );
}
