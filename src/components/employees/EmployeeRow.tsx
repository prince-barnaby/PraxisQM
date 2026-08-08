import { useNavigate } from "react-router-dom";
import { Pencil } from "lucide-react";
import type { BadgeVariant } from "../documents/StatusBadge";
import StatusBadge from "../documents/StatusBadge";

export interface EmployeeRowData {
  id: string;
  lastName: string;
  firstName: string;
  position: string;
  responsibilityRoles: string[];
  qmAreas: string[];
  active: boolean;
  activeLabel: string;
  activeVariant: BadgeVariant;
  entryDate: string;
  exitDate: string;
}

interface EmployeeRowProps {
  employee: EmployeeRowData;
}

export default function EmployeeRow({ employee: emp }: EmployeeRowProps) {
  const navigate = useNavigate();

  const handleEdit = () => {
    navigate(`/mitarbeiter/${emp.id}/bearbeiten`);
  };

  return (
    <tr
      className="pqm-employee-row"
      tabIndex={0}
      aria-label={`${emp.lastName}, ${emp.firstName} – ${emp.position}`}
    >
      <td className="pqm-employee-row__name">{emp.lastName}</td>
      <td className="pqm-employee-row__firstname">{emp.firstName}</td>
      <td className="pqm-employee-row__role">{emp.position}</td>
      <td className="pqm-employee-row__responsibility-roles">
        {emp.responsibilityRoles.length > 0 ? (
          <div className="pqm-employee-row__chips">
            {emp.responsibilityRoles.map((r) => (
              <span key={r} className="pqm-employee-chip pqm-employee-chip--role">
                {r}
              </span>
            ))}
          </div>
        ) : (
          <span className="pqm-employee-row__empty-multi">—</span>
        )}
      </td>
      <td className="pqm-employee-row__qm-areas">
        {emp.qmAreas.length > 0 ? (
          <div className="pqm-employee-row__chips">
            {emp.qmAreas.map((a) => (
              <span key={a} className="pqm-employee-chip pqm-employee-chip--area">
                {a}
              </span>
            ))}
          </div>
        ) : (
          <span className="pqm-employee-row__empty-multi">—</span>
        )}
      </td>
      <td className="pqm-employee-row__status">
        <StatusBadge label={emp.activeLabel} variant={emp.activeVariant} />
      </td>
      <td className="pqm-employee-row__entry-date">{emp.entryDate}</td>
      <td className="pqm-employee-row__exit-date">{emp.exitDate}</td>
      <td className="pqm-employee-row__actions">
        <button
          type="button"
          className="pqm-employee-row__edit-btn"
          onClick={handleEdit}
          aria-label={`${emp.lastName}, ${emp.firstName} bearbeiten`}
          title="Mitarbeiter bearbeiten"
        >
          <Pencil size={14} aria-hidden="true" />
        </button>
      </td>
    </tr>
  );
}
