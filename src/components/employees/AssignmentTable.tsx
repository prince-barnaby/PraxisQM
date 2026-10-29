import { useState } from "react";
import type { MasterDataItem } from "../../lib/employeeApi";
import "./AssignmentTable.css";

interface AssignmentTableProps {
  id: string;
  label: string;
  items: MasterDataItem[];
  selectedIds: string[];
  onToggle: (id: string) => void;
  emptyMessage: string;
}

export default function AssignmentTable({
  id,
  label,
  items,
  selectedIds,
  onToggle,
  emptyMessage,
}: AssignmentTableProps) {
  const [searchTerm, setSearchTerm] = useState("");

  if (items.length === 0) {
    return <p className="pqm-assignment-table__empty">{emptyMessage}</p>;
  }

  const normalizedSearch = searchTerm.trim().toLocaleLowerCase();
  const visibleItems = normalizedSearch
    ? items.filter((item) => item.name.toLocaleLowerCase().includes(normalizedSearch))
    : items;

  return (
    <div className="pqm-assignment-table">
      <div className="pqm-assignment-table__toolbar">
        <label className="pqm-assignment-table__search-label" htmlFor={`${id}-search`}>
          Suchen
        </label>
        <input
          id={`${id}-search`}
          type="search"
          className="pqm-assignment-table__search"
          placeholder="Bezeichnung suchen"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
      </div>
      <div className="pqm-assignment-table__scroll">
        <table className="pqm-assignment-table__table" aria-label={label}>
          <thead>
            <tr>
              <th scope="col" className="pqm-assignment-table__check-col">
                <span className="pqm-assignment-table__sr-only">Auswahl</span>
              </th>
              <th scope="col">Bezeichnung</th>
            </tr>
          </thead>
          <tbody>
            {visibleItems.map((item) => {
              const selected = selectedIds.includes(item.id);
              return (
                <tr
                  key={item.id}
                  className={
                    selected
                      ? "pqm-assignment-table__row pqm-assignment-table__row--selected"
                      : "pqm-assignment-table__row"
                  }
                  onClick={(e) => {
                    if (!(e.target instanceof HTMLInputElement)) onToggle(item.id);
                  }}
                >
                  <td className="pqm-assignment-table__check-col">
                    <input
                      type="checkbox"
                      checked={selected}
                      onChange={() => onToggle(item.id)}
                      aria-label={item.name}
                    />
                  </td>
                  <td>{item.name}</td>
                </tr>
              );
            })}
            {visibleItems.length === 0 && (
              <tr>
                <td colSpan={2} className="pqm-assignment-table__no-results">
                  Keine passenden Einträge gefunden.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
