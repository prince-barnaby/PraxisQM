import {
  LayoutDashboard,
  FileText,
  Archive,
  Users,
  Settings,
  ShieldCheck,
} from "lucide-react";
import "./Sidebar.css";

// PraxisQM – Sidebar
// Modul: Navigation
// Zweck: Statische Hauptnavigation links. Keine echte Navigation – nur Struktur.

const navItems = [
  { label: "Startseite", icon: LayoutDashboard },
  { label: "Dokumente", icon: FileText },
  { label: "Archiv", icon: Archive },
  { label: "Mitarbeiter", icon: Users },
  { label: "Einstellungen", icon: Settings },
];

export default function Sidebar() {
  return (
    <aside className="pqm-sidebar">
      <div className="pqm-sidebar__logo">
        <ShieldCheck size={28} color="var(--pqm-color-accent)" />
        <span className="pqm-sidebar__title">PraxisQM</span>
      </div>
      <nav className="pqm-sidebar__nav">
        {navItems.map(({ label, icon: Icon }) => (
          <div key={label} className="pqm-sidebar__item">
            <Icon size={20} />
            <span>{label}</span>
          </div>
        ))}
      </nav>
    </aside>
  );
}
