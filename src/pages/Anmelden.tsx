import { useState, type FormEvent } from "react";
import { useNavigate } from "react-router-dom";
import AuthScreen from "../components/auth/AuthScreen";
import { errorMessage, useAuth } from "../lib/AuthContext";

export default function Anmelden() {
  const { login } = useAuth();
  const navigate = useNavigate();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (submitting) return;
    if (!username.trim() || !password) {
      setError("Bitte Benutzername und Passwort eingeben.");
      return;
    }
    setError(null);
    setSubmitting(true);
    try {
      await login(username.trim(), password);
      setPassword("");
      navigate("/");
    } catch (err) {
      setPassword("");
      setError(errorMessage(err));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <AuthScreen
      title="Anmelden"
      subtitle="Melden Sie sich an, um Dokumente zu bearbeiten oder Einstellungen zu verwalten."
    >
      <form className="pqm-auth-form" onSubmit={handleSubmit} noValidate>
        <div className="pqm-auth-form__field">
          <label htmlFor="login-username">Benutzername</label>
          <input
            id="login-username"
            name="username"
            autoComplete="username"
            autoFocus
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            disabled={submitting}
          />
        </div>
        <div className="pqm-auth-form__field">
          <label htmlFor="login-password">Passwort</label>
          <input
            id="login-password"
            name="password"
            type="password"
            autoComplete="current-password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={submitting}
          />
        </div>
        {error && (
          <p className="pqm-auth-form__message pqm-auth-form__message--error" role="alert">
            {error}
          </p>
        )}
        <button
          type="submit"
          className="pqm-auth-form__submit"
          disabled={submitting}
          aria-busy={submitting}
        >
          {submitting ? "Anmeldung läuft …" : "Anmelden"}
        </button>
      </form>
    </AuthScreen>
  );
}
