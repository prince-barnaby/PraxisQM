import AppShell from "./components/AppShell";
import Startseite from "./pages/Startseite";

// PraxisQM – Anwendungskomponente
// Modul: Grundstruktur
// Zweck: Setzt die App-Shell und die Startseite zusammen.
export default function App() {
  return (
    <AppShell>
      <Startseite />
    </AppShell>
  );
}
