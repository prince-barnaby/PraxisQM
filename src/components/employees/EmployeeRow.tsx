import type { BadgeVariant } from "../documents/StatusBadge";
import StatusBadge from "../documents/StatusBadge";

export interface EmployeeRowData {
  id: string;
  lastName: string;
  firstName: string;
  role: string;
  department: string;
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
