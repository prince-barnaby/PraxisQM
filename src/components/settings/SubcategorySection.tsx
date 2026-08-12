import { useCallback, useEffect, useState } from "react";
import { Plus, Pencil, Check, X } from "lucide-react";
import type { MasterDataItem } from "../../lib/masterDataApi";
import type { SubcategoryItem } from "../../lib/documentApi";
import "./MasterDataSection.css";

interface SubcategorySectionProps {
  title: string;
  description: string;
  inputLabel: string;
  categoryLabel: string;
  addButtonLabel: string;
  emptyMessage: string;
  loadingMessage: string;
  duplicateHint: string;
  fetchSubcategories: () => Promise<SubcategoryItem[]>;
  fetchCategories: () => Promise<MasterDataItem[]>;
  createSubcategory: (name: string, categoryId: string) => Promise<SubcategoryItem>;
  renameSubcategory: (id: string, newName: string) => Promise<SubcategoryItem>;
}

interface EditState {
  id: string;
  value: string;
}

export default function SubcategorySection({
  title,
  description,
  inputLabel,
  categoryLabel,
  addButtonLabel,
  emptyMessage,
  loadingMessage,
  duplicateHint,
  fetchSubcategories,
  fetchCategories,
  createSubcategory,
  renameSubcategory,
}: SubcategorySectionProps) {
  const [subcategories, setSubcategories] = useState<SubcategoryItem[]>([]);
  const [categories, setCategories] = useState<MasterDataItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newName, setNewName] = useState("");
  const [selectedCategoryId, setSelectedCategoryId] = useState("");
  const [adding, setAdding] = useState(false);
  const [addError, setAddError] = useState<string | null>(null);
  const [editState, setEditState] = useState<EditState | null>(null);
  const [renameError, setRenameError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [subs, cats] = await Promise.all([fetchSubcategories(), fetchCategories()]);
      setSubcategories(subs);
      setCategories(cats);
      if (cats.length > 0 && !selectedCategoryId) {
        setSelectedCategoryId(cats[0].id);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fetchSubcategories, fetchCategories]);

  useEffect(() => {
    load();
  }, [load]);

  const categoryName = (catId: string): string =>
    categories.find((c) => c.id === catId)?.name ?? "—";

  const handleAdd = async () => {
    const trimmed = newName.trim();
    if (!trimmed || !selectedCategoryId) return;
    setAdding(true);
    setAddError(null);
    try {
      const created = await createSubcategory(trimmed, selectedCategoryId);
      setSubcategories((prev) =>
        [...prev, created].sort((a, b) => a.name.localeCompare(b.name)),
      );
      setNewName("");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      setAddError(msg.includes("UNIQUE") ? duplicateHint : msg);
    } finally {
      setAdding(false);
    }
  };

  const startEdit = (item: SubcategoryItem) => {
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
      const renamed = await renameSubcategory(editState.id, trimmed);
      setSubcategories((prev) =>
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
        <label
          htmlFor={`md-cat-${title}`}
          className="pqm-master-data__input-label"
        >
          {categoryLabel}
        </label>
        <select
          id={`md-cat-${title}`}
          className="pqm-master-data__category-select"
          value={selectedCategoryId}
          onChange={(e) => setSelectedCategoryId(e.target.value)}
          disabled={adding || categories.length === 0}
          aria-label={categoryLabel}
        >
          {categories.length === 0 && (
            <option value="">Zuerst Kategorie anlegen</option>
          )}
          {categories.map((cat) => (
            <option key={cat.id} value={cat.id}>
              {cat.name}
            </option>
          ))}
        </select>

        <label
          htmlFor={`md-input-${title}`}
          className="pqm-master-data__input-label"
        >
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
            disabled={adding || categories.length === 0}
            placeholder={inputLabel}
            aria-label={inputLabel}
          />
          <button
            type="button"
            className="pqm-master-data__add-btn"
            onClick={handleAdd}
            disabled={adding || !newName.trim() || !selectedCategoryId}
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
      ) : subcategories.length === 0 ? (
        <p className="pqm-master-data__empty">{emptyMessage}</p>
      ) : (
        <ul className="pqm-master-data__list">
          {subcategories.map((item) => (
            <li key={item.id} className="pqm-master-data__item">
              {editState?.id === item.id ? (
                <div className="pqm-master-data__edit-row">
                  <input
                    type="text"
                    value={editState.value}
                    onChange={(e) =>
                      setEditState({ ...editState, value: e.target.value })
                    }
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
                  <span className="pqm-master-data__item-name">
                    {item.name}
                  </span>
                  <span className="pqm-master-data__parent-hint">
                    {categoryName(item.category_id)}
                  </span>
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
