import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { KeyRound, LogIn, LogOut } from "lucide-react";
import { errorMessage, useAuth } from "../../lib/AuthContext";
import "./SessionControls.css";

const ROLE_LABELS: Record<string, string> = {
  Admin: "Administrator",
  Editor: "Editor",
};

export default function SessionControls() {
  const { user, logout } = useAuth();
  const navigate = useNavigate();
  const [loggingOut, setLoggingOut] = useState(false);
  const [logoutError, setLogoutError] = useState<string | null>(null);

  async function handleLogout() {
    setLoggingOut(true);
    setLogoutError(null);
    try {
      await logout();
      navigate("/");
    } catch (err) {
      setLogoutError(errorMessage(err));
    } finally {
      setLoggingOut(false);
    }
  }

  if (!user) {
    return (
      <div className="pqm-session">
        <span className="pqm-session__badge pqm-session__badge--guest">Gastmodus</span>
        <Link to="/anmelden" className="pqm-session__button pqm-session__button--primary">
          <LogIn size={16} aria-hidden="true" />
          Anmelden
        </Link>
      </div>
    );
  }

  return (
    <div className="pqm-session">
      <span className="pqm-session__identity">
        <span className="pqm-session__username">{user.username}</span>
        <span className="pqm-session__badge">{ROLE_LABELS[user.role] ?? user.role}</span>
      </span>
      <Link to="/passwort" className="pqm-session__button">
        <KeyRound size={16} aria-hidden="true" />
        Passwort ändern
      </Link>
      <button
        type="button"
        className="pqm-session__button"
        onClick={handleLogout}
        disabled={loggingOut}
      >
        <LogOut size={16} aria-hidden="true" />
        {loggingOut ? "Abmeldung …" : "Abmelden"}
      </button>
      {logoutError && (
        <span className="pqm-session__error" role="alert">
          {logoutError}
        </span>
      )}
    </div>
  );
}
