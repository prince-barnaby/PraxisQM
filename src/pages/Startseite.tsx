import { Info } from "lucide-react";
import "./Startseite.css";

// PraxisQM – Startseite
// Modul: Dashboard
// Zweck: Leere Startseite mit eindeutig gekennzeichnetem Platzhalter.

export default function Startseite() {
  return (
    <div className="pqm-startseite">
      <div className="pqm-startseite__placeholder">
        <Info size={48} color="var(--pqm-color-neutral)" />
        <h2>Startseite – Platzhalter</h2>
        <p>
          Dies ist ein Platzhalter. Die Dashboard-Inhalte werden in einem
          späteren Entwicklungsschritt implementiert.
        </p>
      </div>
    </div>
  );
}
