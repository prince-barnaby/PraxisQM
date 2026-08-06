import { Routes, Route } from "react-router-dom";
import AppShell from "./components/AppShell";
import Startseite from "./pages/Startseite";
import Dokumente from "./pages/Dokumente";
import Archiv from "./pages/Archiv";
import Mitarbeiter from "./pages/Mitarbeiter";
import Einstellungen from "./pages/Einstellungen";

export default function App() {
  return (
    <AppShell>
      <Routes>
        <Route path="/" element={<Startseite />} />
        <Route path="/dokumente" element={<Dokumente />} />
        <Route path="/archiv" element={<Archiv />} />
        <Route path="/mitarbeiter" element={<Mitarbeiter />} />
        <Route path="/einstellungen" element={<Einstellungen />} />
      </Routes>
    </AppShell>
  );
}
