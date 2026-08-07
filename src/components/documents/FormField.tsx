import type { ReactNode } from "react";
import "./FormField.css";

interface FormFieldProps {
  label: string;
  htmlFor?: string;
  hint?: string;
  children: ReactNode;
  className?: string;
}

export default function FormField({
  label,
  htmlFor,
  hint,
  children,
  className,
}: FormFieldProps) {
  return (
    <div className={"pqm-form-field" + (className ? ` ${className}` : "")}>
      <label className="pqm-form-field__label" htmlFor={htmlFor}>
        {label}
      </label>
      {children}
      {hint && <p className="pqm-form-field__hint">{hint}</p>}
    </div>
  );
}
