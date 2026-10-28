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
import { useAuth } from "../lib/AuthContext";
import "./Sidebar.css";

interface NavItem {
  label: string;
  to: string;
  icon: LucideIcon;
  requires: "guest" | "editor" | "admin";
}

const navItems: NavItem[] = [
  { label: "Startseite", to: "/", icon: LayoutDashboard, requires: "guest" },
  { label: "Dokumente", to: "/dokumente", icon: FileText, requires: "guest" },
  { label: "Archiv", to: "/archiv", icon: Archive, requires: "editor" },
  { label: "Mitarbeiter", to: "/mitarbeiter", icon: Users, requires: "admin" },
  { label: "Einstellungen", to: "/einstellungen", icon: Settings, requires: "admin" },
];

export default function Sidebar() {
  const { isEditor, isAdmin } = useAuth();
  const visibleItems = navItems.filter(
    (item) =>
      item.requires === "guest" ||
      (item.requires === "editor" && isEditor) ||
      (item.requires === "admin" && isAdmin),
  );

  return (
    <aside className="pqm-sidebar">
      <div className="pqm-sidebar__logo">
        <ShieldCheck size={28} color="var(--pqm-color-accent)" />
        <span className="pqm-sidebar__title">PraxisQM</span>
      </div>
      <nav className="pqm-sidebar__nav">
        {visibleItems.map(({ label, to, icon: Icon }) => (

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
