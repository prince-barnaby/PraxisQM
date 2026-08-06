import { NavLink } from "react-router-dom";
import {
  LayoutDashboard,
  FileText,
  Archive,
  Users,
  Settings,
  ShieldCheck,
} from "lucide-react";
import type { LucideIcon } from "lucide-react";
import "./Sidebar.css";

interface NavItem {
  label: string;
  to: string;
  icon: LucideIcon;
}

const navItems: NavItem[] = [
  { label: "Startseite", to: "/", icon: LayoutDashboard },
  { label: "Dokumente", to: "/dokumente", icon: FileText },
  { label: "Archiv", to: "/archiv", icon: Archive },
  { label: "Mitarbeiter", to: "/mitarbeiter", icon: Users },
  { label: "Einstellungen", to: "/einstellungen", icon: Settings },
];

export default function Sidebar() {
  return (
    <aside className="pqm-sidebar">
      <div className="pqm-sidebar__logo">
        <ShieldCheck size={28} color="var(--pqm-color-accent)" />
        <span className="pqm-sidebar__title">PraxisQM</span>
      </div>
      <nav className="pqm-sidebar__nav">
        {navItems.map(({ label, to, icon: Icon }) => (
          <NavLink
            key={label}
            to={to}
            end={to === "/"}
            className="pqm-sidebar__item"
          >
            <Icon size={20} />
            <span>{label}</span>
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}
