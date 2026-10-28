import { useState, type FormEvent } from "react";
import AuthScreen from "../components/auth/AuthScreen";
import { errorMessage, useAuth } from "../lib/AuthContext";

const MIN_PASSWORD_LENGTH = 8;

export default function Ersteinrichtung() {
  const { completeSetup } = useAuth();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [passwordConfirm, setPasswordConfirm] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (submitting) return;
    if (!username.trim()) {
      setError("Bitte einen Benutzernamen eingeben.");
      return;
    }
    if (password.length < MIN_PASSWORD_LENGTH) {
      setError(`Das Passwort muss mindestens ${MIN_PASSWORD_LENGTH} Zeichen lang sein.`);
      return;
    }
    if (password !== passwordConfirm) {
      setError("Die Passwörter stimmen nicht überein.");
      return;
    }
    setError(null);
    setSubmitting(true);
    try {
      await completeSetup(username.trim(), password);
    } catch (err) {
      setError(errorMessage(err));
    } finally {
      setPassword("");
      setPasswordConfirm("");
      setSubmitting(false);
    }
  }

  return (
    <AuthScreen
      title="Ersteinrichtung"
      subtitle="Vor der ersten Nutzung muss ein Administrator-Konto angelegt werden. Dieses Konto verwaltet Mitarbeiter und Einstellungen."
    >
      <form className="pqm-auth-form" onSubmit={handleSubmit} noValidate>
        <div className="pqm-auth-form__field">
          <label htmlFor="setup-username">Benutzername</label>
          <input
            id="setup-username"
            name="username"
            autoComplete="username"
            autoFocus
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            disabled={submitting}
          />
        </div>
        <div className="pqm-auth-form__field">
          <label htmlFor="setup-password">Passwort</label>
          <input
            id="setup-password"
            name="password"
            type="password"
            autoComplete="new-password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            disabled={submitting}
            aria-describedby="setup-password-hint"
          />
          <p id="setup-password-hint" className="pqm-auth-form__hint">
            Mindestens {MIN_PASSWORD_LENGTH} Zeichen.
          </p>
        </div>
        <div className="pqm-auth-form__field">
          <label htmlFor="setup-password-confirm">Passwort bestätigen</label>
          <input
            id="setup-password-confirm"
            name="passwordConfirm"
            type="password"
            autoComplete="new-password"
            value={passwordConfirm}
            onChange={(e) => setPasswordConfirm(e.target.value)}
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
          {submitting ? "Konto wird angelegt …" : "Administrator anlegen"}
        </button>
      </form>
    </AuthScreen>
  );
}
