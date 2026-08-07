import type { BadgeVariant } from "../documents/StatusBadge";
import StatusBadge from "../documents/StatusBadge";

export interface EmployeeRowData {
  id: string;
  lastName: string;
  firstName: string;
  role: string;
  department: string;
  responsibilityRoles: string[];
  qmAreas: string[];
  active: boolean;
  activeLabel: string;
  activeVariant: BadgeVariant;
  email: string;
  phone: string;
  entryDate: string;
  exitDate: string;
}

interface EmployeeRowProps {
  employee: EmployeeRowData;
}

export default function EmployeeRow({ employee: emp }: EmployeeRowProps) {
  return (
    <tr
      className="pqm-employee-row"
      tabIndex={0}
      aria-label={`${emp.lastName}, ${emp.firstName} – ${emp.role}`}
    >
      <td className="pqm-employee-row__name">{emp.lastName}</td>
      <td className="pqm-employee-row__firstname">{emp.firstName}</td>
      <td className="pqm-employee-row__role">{emp.role}</td>
      <td className="pqm-employee-row__department">{emp.department}</td>
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
      <td className="pqm-employee-row__email">{emp.email}</td>
      <td className="pqm-employee-row__phone">{emp.phone}</td>
      <td className="pqm-employee-row__entry-date">{emp.entryDate}</td>
      <td className="pqm-employee-row__exit-date">{emp.exitDate}</td>
    </tr>
  );
}
