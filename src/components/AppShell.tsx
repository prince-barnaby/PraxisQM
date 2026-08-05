import Sidebar from "./Sidebar";
import Header from "./Header";
import "./AppShell.css";

// PraxisQM – App-Shell
// Modul: Grundstruktur
// Zweck: Gesamtlayout aus Sidebar, Header und zentralem Inhaltsbereich.

interface AppShellProps {
  children: React.ReactNode;
}

export default function AppShell({ children }: AppShellProps) {
  return (
    <div className="pqm-app-shell">
      <Sidebar />
      <div className="pqm-app-shell__main">
        <Header />
        <main className="pqm-app-shell__content">{children}</main>
      </div>
    </div>
  );
}
