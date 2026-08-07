import { useCallback, useEffect, useState } from "react";
import { Plus, Pencil, Check, X } from "lucide-react";
import type { MasterDataItem } from "../../lib/masterDataApi";

interface MasterDataSectionProps {
  title: string;
  description: string;
  inputLabel: string;
  addButtonLabel: string;
  emptyMessage: string;
  loadingMessage: string;
  fetchItems: () => Promise<MasterDataItem[]>;
  createItem: (name: string) => Promise<MasterDataItem>;
  renameItem: (id: string, newName: string) => Promise<MasterDataItem>;
  duplicateHint: string;
}

interface EditState {
  id: string;
  value: string;
}

export default function MasterDataSection({
  title,
  description,
  inputLabel,
  addButtonLabel,
  emptyMessage,
  loadingMessage,
  fetchItems,
  createItem,
  renameItem,
  duplicateHint,
}: MasterDataSectionProps) {
  const [items, setItems] = useState<MasterDataItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newName, setNewName] = useState("");
  const [adding, setAdding] = useState(false);
  const [addError, setAddError] = useState<string | null>(null);
  const [editState, setEditState] = useState<EditState | null>(null);
  const [renameError, setRenameError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await fetchItems();
      setItems(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [fetchItems]);

  useEffect(() => {
    load();
  }, [load]);

  const handleAdd = async () => {
    const trimmed = newName.trim();
    if (!trimmed) return;
    setAdding(true);
    setAddError(null);
    try {
      const created = await createItem(trimmed);
      setItems((prev) => [...prev, created].sort((a, b) => a.name.localeCompare(b.name)));
      setNewName("");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      setAddError(msg.includes("UNIQUE") ? duplicateHint : msg);
    } finally {
      setAdding(false);
    }
  };

  const startEdit = (item: MasterDataItem) => {
    setRenameError(null);
    setEditState({ id: item.id, value: item.name });
  };

  const cancelEdit = () => {
    setEditState(null);
    setRenameError(null);
  };

  const handleSaveEdit = async () => {
    if (!editState) return;
    const trimmed = editState.value.trim();
    if (!trimmed) return;
    setRenameError(null);
    try {
      const renamed = await renameItem(editState.id, trimmed);
      setItems((prev) =>
        prev
          .map((it) => (it.id === renamed.id ? renamed : it))
          .sort((a, b) => a.name.localeCompare(b.name)),
      );
      setEditState(null);
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      setRenameError(msg.includes("UNIQUE") ? duplicateHint : msg);
    }
  };

  return (
    <div className="pqm-master-data">
      <header className="pqm-master-data__header">
        <h3 className="pqm-master-data__title">{title}</h3>
        <p className="pqm-master-data__description">{description}</p>
      </header>

      <div className="pqm-master-data__add-form">
        <label htmlFor={`md-input-${title}`} className="pqm-master-data__input-label">
          {inputLabel}
        </label>
        <div className="pqm-master-data__add-row">
          <input
            id={`md-input-${title}`}
            type="text"
            value={newName}
            onChange={(e) => setNewName(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !adding) handleAdd();
            }}
            disabled={adding}
            placeholder={inputLabel}
            aria-label={inputLabel}
          />
          <button
            type="button"
            className="pqm-master-data__add-btn"
            onClick={handleAdd}
            disabled={adding || !newName.trim()}
            aria-label={addButtonLabel}
          >
            <Plus size={16} aria-hidden="true" />
            {addButtonLabel}
          </button>
        </div>
        {addError && (
          <p className="pqm-master-data__error" role="alert">{addError}</p>
        )}
      </div>

      {loading ? (
        <p className="pqm-master-data__loading">{loadingMessage}</p>
      ) : error ? (
        <p className="pqm-master-data__error" role="alert">{error}</p>
      ) : items.length === 0 ? (
        <p className="pqm-master-data__empty">{emptyMessage}</p>
      ) : (
        <ul className="pqm-master-data__list">
          {items.map((item) => (
            <li key={item.id} className="pqm-master-data__item">
              {editState?.id === item.id ? (
                <div className="pqm-master-data__edit-row">
                  <input
                    type="text"
                    value={editState.value}
                    onChange={(e) => setEditState({ ...editState, value: e.target.value })}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") handleSaveEdit();
                      if (e.key === "Escape") cancelEdit();
                    }}
                    autoFocus
                    aria-label="Neue Bezeichnung"
                  />
                  <button
                    type="button"
                    className="pqm-master-data__icon-btn pqm-master-data__icon-btn--save"
                    onClick={handleSaveEdit}
                    aria-label="Speichern"
                  >
                    <Check size={16} aria-hidden="true" />
                  </button>
                  <button
                    type="button"
                    className="pqm-master-data__icon-btn pqm-master-data__icon-btn--cancel"
                    onClick={cancelEdit}
                    aria-label="Abbrechen"
                  >
                    <X size={16} aria-hidden="true" />
                  </button>
                </div>
              ) : (
                <div className="pqm-master-data__display-row">
                  <span className="pqm-master-data__item-name">{item.name}</span>
                  <button
                    type="button"
                    className="pqm-master-data__icon-btn"
                    onClick={() => startEdit(item)}
                    aria-label={`${item.name} umbenennen`}
                  >
                    <Pencil size={14} aria-hidden="true" />
                  </button>
                </div>
              )}
              {renameError && editState?.id === item.id && (
                <p className="pqm-master-data__error" role="alert">{renameError}</p>
              )}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
