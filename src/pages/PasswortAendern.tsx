import { useState, type FormEvent } from "react";
import { changePassword } from "../lib/authApi";
import { errorMessage } from "../lib/AuthContext";
import "./PasswortAendern.css";

const MIN_PASSWORD_LENGTH = 8;

export default function PasswortAendern() {
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  function clearFields() {
    setCurrentPassword("");
    setNewPassword("");
    setConfirmPassword("");
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (submitting) return;
    setError(null);
    setSuccess(null);

    if (!currentPassword) {
      setError("Bitte das aktuelle Passwort eingeben.");
      return;
    }
    if (newPassword.length < MIN_PASSWORD_LENGTH) {
      setError(`Das neue Passwort muss mindestens ${MIN_PASSWORD_LENGTH} Zeichen lang sein.`);
      return;
    }
    if (newPassword === currentPassword) {
      setError("Das neue Passwort muss sich vom aktuellen Passwort unterscheiden.");
      return;
    }
    if (newPassword !== confirmPassword) {
      setError("Die neuen Passwörter stimmen nicht überein.");
      return;
    }

    setSubmitting(true);
    try {
      await changePassword(currentPassword, newPassword);
      clearFields();
      setSuccess("Ihr Passwort wurde erfolgreich geändert.");
    } catch (err) {
      setCurrentPassword("");
      setError(errorMessage(err));
    } finally {
      setNewPassword("");
      setConfirmPassword("");
      setSubmitting(false);
    }
  }

  return (
    <div className="pqm-password-page">
      <h2 className="pqm-password-page__title">Passwort ändern</h2>
      <p className="pqm-password-page__subtitle">
        Bestätigen Sie zuerst Ihr aktuelles Passwort und legen Sie danach ein neues fest.
      </p>
      <form className="pqm-password-page__form" onSubmit={handleSubmit} noValidate>
        <div className="pqm-password-page__field">
          <label htmlFor="pw-current">Aktuelles Passwort</label>
          <input
            id="pw-current"
            type="password"
            autoComplete="current-password"
            value={currentPassword}
            onChange={(e) => setCurrentPassword(e.target.value)}
            disabled={submitting}
          />
        </div>
        <div className="pqm-password-page__field">
          <label htmlFor="pw-new">Neues Passwort</label>
          <input
            id="pw-new"
            type="password"
            autoComplete="new-password"
            value={newPassword}
            onChange={(e) => setNewPassword(e.target.value)}
            disabled={submitting}
            aria-describedby="pw-new-hint"
          />
          <p id="pw-new-hint" className="pqm-password-page__hint">
            Mindestens {MIN_PASSWORD_LENGTH} Zeichen.
          </p>
        </div>
        <div className="pqm-password-page__field">
          <label htmlFor="pw-confirm">Neues Passwort bestätigen</label>
          <input
            id="pw-confirm"
            type="password"
            autoComplete="new-password"
            value={confirmPassword}
            onChange={(e) => setConfirmPassword(e.target.value)}
            disabled={submitting}
          />
        </div>
        {error && (
          <p className="pqm-password-page__message pqm-password-page__message--error" role="alert">
            {error}
          </p>
        )}
        {success && (
          <p className="pqm-password-page__message pqm-password-page__message--success" role="status">
            {success}
          </p>
        )}
        <button type="submit" className="pqm-password-page__submit" disabled={submitting} aria-busy={submitting}>
          {submitting ? "Wird gespeichert …" : "Passwort ändern"}
        </button>
      </form>
    </div>
  );
}
