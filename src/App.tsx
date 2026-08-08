import { Routes, Route } from "react-router-dom";
import AppShell from "./components/AppShell";
import Startseite from "./pages/Startseite";
import Dokumente from "./pages/Dokumente";
import DokumentNeu from "./pages/DokumentNeu";
import Archiv from "./pages/Archiv";
import DokumentDetail from "./pages/DokumentDetail";
import DokumentBearbeiten from "./pages/DokumentBearbeiten";
import Mitarbeiter from "./pages/Mitarbeiter";
import MitarbeiterNeu from "./pages/MitarbeiterNeu";
import MitarbeiterBearbeiten from "./pages/MitarbeiterBearbeiten";
import Einstellungen from "./pages/Einstellungen";

export default function App() {
  return (
    <AppShell>
      <Routes>
        <Route path="/" element={<Startseite />} />
        <Route path="/dokumente" element={<Dokumente />} />
        <Route path="/dokumente/neu" element={<DokumentNeu />} />
        <Route path="/dokumente/:id" element={<DokumentDetail />} />
        <Route path="/dokumente/:id/bearbeiten" element={<DokumentBearbeiten />} />
        <Route path="/archiv" element={<Archiv />} />
        <Route path="/mitarbeiter" element={<Mitarbeiter />} />
        <Route path="/mitarbeiter/neu" element={<MitarbeiterNeu />} />
        <Route path="/mitarbeiter/:id/bearbeiten" element={<MitarbeiterBearbeiten />} />
        <Route path="/einstellungen" element={<Einstellungen />} />
      </Routes>
    </AppShell>
  );
}
