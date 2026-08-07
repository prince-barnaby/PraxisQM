import type { ReactNode } from "react";
import "./DocumentFormSection.css";

interface DocumentFormSectionProps {
  title: string;
  children: ReactNode;
}

export default function DocumentFormSection({
  title,
  children,
}: DocumentFormSectionProps) {
  return (
    <fieldset className="pqm-form-section">
      <legend className="pqm-form-section__title">{title}</legend>
      <div className="pqm-form-section__body">{children}</div>
    </fieldset>
  );
}
