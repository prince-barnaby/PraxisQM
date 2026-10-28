import { Routes, Route } from "react-router-dom";
import AppShell from "./components/AppShell";
import AuthGate from "./components/auth/AuthGate";
import RequireRole from "./components/auth/RequireRole";
import Startseite from "./pages/Startseite";
import Dokumente from "./pages/Dokumente";
import DokumentNeu from "./pages/DokumentNeu";
import Archiv from "./pages/Archiv";
import DokumentDetail from "./pages/DokumentDetail";
import DokumentBearbeiten from "./pages/DokumentBearbeiten";
import DokumentNeueVersion from "./pages/DokumentNeueVersion";
import Mitarbeiter from "./pages/Mitarbeiter";
import MitarbeiterNeu from "./pages/MitarbeiterNeu";
import MitarbeiterBearbeiten from "./pages/MitarbeiterBearbeiten";
import Einstellungen from "./pages/Einstellungen";
import Anmelden from "./pages/Anmelden";
import PasswortAendern from "./pages/PasswortAendern";

export default function App() {
  return (
    <AuthGate>
      <AppShell>
        <Routes>
          <Route path="/" element={<Startseite />} />
          <Route path="/anmelden" element={<Anmelden />} />
          <Route path="/dokumente" element={<Dokumente />} />
          <Route path="/dokumente/:id" element={<DokumentDetail />} />
          <Route path="/archiv" element={<RequireRole level="editor"><Archiv /></RequireRole>} />
          <Route path="/dokumente/neu" element={<RequireRole level="editor"><DokumentNeu /></RequireRole>} />
          <Route path="/dokumente/:id/bearbeiten" element={<RequireRole level="editor"><DokumentBearbeiten /></RequireRole>} />
          <Route path="/dokumente/:id/neue-version" element={<RequireRole level="editor"><DokumentNeueVersion /></RequireRole>} />
          <Route path="/passwort" element={<RequireRole level="editor"><PasswortAendern /></RequireRole>} />
          <Route path="/mitarbeiter" element={<RequireRole level="admin"><Mitarbeiter /></RequireRole>} />
          <Route path="/mitarbeiter/neu" element={<RequireRole level="admin"><MitarbeiterNeu /></RequireRole>} />
          <Route path="/mitarbeiter/:id/bearbeiten" element={<RequireRole level="admin"><MitarbeiterBearbeiten /></RequireRole>} />
          <Route path="/einstellungen" element={<RequireRole level="admin"><Einstellungen /></RequireRole>} />
        </Routes>
      </AppShell>
    </AuthGate>
  );
}
