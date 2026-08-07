import type { ReactNode } from "react";
import "./SettingsSection.css";

interface SettingsSectionProps {
  title: string;
  description: string;
  children: ReactNode;
}

export default function SettingsSection({
  title,
  description,
  children,
}: SettingsSectionProps) {
  return (
    <section className="pqm-settings-section">
      <header className="pqm-settings-section__header">
        <h2 className="pqm-settings-section__title">{title}</h2>
        <p className="pqm-settings-section__description">{description}</p>
      </header>
      <div className="pqm-settings-section__body">{children}</div>
    </section>
  );
}
