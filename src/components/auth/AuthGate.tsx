import type { ReactNode } from "react";
import { useAuth } from "../../lib/AuthContext";
import AuthScreen from "./AuthScreen";
import Ersteinrichtung from "../../pages/Ersteinrichtung";

export default function AuthGate({ children }: { children: ReactNode }) {
  const { status, loadError, retryInitialize } = useAuth();

  if (status === "loading") {
    return <AuthScreen title="PraxisQM" subtitle="Die Anwendung wird geladen …">{null}</AuthScreen>;
  }

  if (status === "error") {
    return (
      <AuthScreen
        title="Start nicht möglich"
        subtitle="Die lokale Datenbank konnte nicht geprüft werden. Bitte versuchen Sie es erneut."
      >
        <p className="pqm-auth-form__message pqm-auth-form__message--error" role="alert">
          {loadError}
        </p>
        <button type="button" className="pqm-auth-form__submit" onClick={retryInitialize}>
          Erneut versuchen
        </button>
      </AuthScreen>
    );
  }

  if (status === "setup") {
    return <Ersteinrichtung />;
  }

  return <>{children}</>;
}
