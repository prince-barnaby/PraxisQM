import type { ReactNode } from "react";
import { ShieldCheck } from "lucide-react";
import "./AuthScreen.css";

interface AuthScreenProps {
  title: string;
  subtitle: string;
  children: ReactNode;
}

export default function AuthScreen({ title, subtitle, children }: AuthScreenProps) {
  return (
    <div className="pqm-auth-screen">
      <section className="pqm-auth-screen__card">
        <div className="pqm-auth-screen__brand">
          <ShieldCheck size={28} color="var(--pqm-color-accent)" aria-hidden="true" />
          <span>PraxisQM</span>
        </div>
        <h1 className="pqm-auth-screen__title">{title}</h1>
        <p className="pqm-auth-screen__subtitle">{subtitle}</p>
        {children}
      </section>
    </div>
  );
}
