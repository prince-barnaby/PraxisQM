import { useCallback, useEffect, useMemo, useState } from "react";
import DocumentToolbar from "../components/documents/DocumentToolbar";
import DocumentFilters from "../components/documents/DocumentFilters";
import DocumentList from "../components/documents/DocumentList";
import type { DocumentRowData } from "../components/documents/DocumentRow";
import type { BadgeVariant } from "../components/documents/StatusBadge";
import {
  fetchCategories,
  fetchDocuments,
  fetchSubcategories,
  type CategoryItem,
  type Document,
  type SubcategoryItem,
} from "../lib/documentApi";
import { fetchEmployees, type Employee } from "../lib/employeeApi";
import "./Dokumente.css";

export type ValidityFilterValue = "all" | "gültig" | "läuft bald ab" | "abgelaufen" | "none";
type StatusFilterValue = "all" | "Entwurf" | "aktiv";

export interface DocumentFilterValues {
  categoryId: string;
  subcategoryId: string;
  responsiblePersonId: string;
  status: StatusFilterValue;
  validity: ValidityFilterValue;
}

const NO_DOCUMENT_FILTERS: DocumentFilterValues = {
  categoryId: "",
  subcategoryId: "",
  responsiblePersonId: "",
  status: "all",
  validity: "all",
};

function validityToVariant(validity: string | null): BadgeVariant {
  if (validity === null) return "neutral";
  if (validity === "gültig") return "success";
  if (validity === "läuft bald ab") return "warning";
  if (validity === "abgelaufen") return "error";
  return "neutral";
}

function statusToVariant(status: string): BadgeVariant {
  return status === "aktiv" ? "success" : "neutral";
}

function toRowData(doc: Document): DocumentRowData {
  return {
    id: doc.id,
    documentNumber: doc.document_number,
    title: doc.title,
    category: doc.category_name ?? "—",
    subcategory: doc.subcategory_name ?? "—",
    status: doc.status,
    statusVariant: statusToVariant(doc.status),
    responsible: doc.responsible_person_name ?? "—",
    validity: doc.computed_validity ?? "—",
    validityVariant: validityToVariant(doc.computed_validity),
    version: doc.version,
  };
}

function matchesDocument(
  document: Document,
  searchTerm: string,
  filters: DocumentFilterValues,
): boolean {
  const normalizedSearch = searchTerm.trim().toLocaleLowerCase();
  if (normalizedSearch) {
    const searchableText = [document.document_number, document.title, document.description ?? ""]
      .join(" ")
      .toLocaleLowerCase();
    if (!searchableText.includes(normalizedSearch)) return false;
  }
  if (filters.categoryId && document.category_id !== filters.categoryId) return false;
  if (filters.subcategoryId && document.subcategory_id !== filters.subcategoryId) return false;
  if (filters.responsiblePersonId && document.responsible_person_id !== filters.responsiblePersonId) return false;
  if (filters.status !== "all" && document.status !== filters.status) return false;
  if (filters.validity === "none" && document.computed_validity !== null) return false;
  if (filters.validity !== "all" && filters.validity !== "none" && document.computed_validity !== filters.validity) return false;
  return true;
}

export default function Dokumente() {
  const [documents, setDocuments] = useState<Document[]>([]);
  const [categories, setCategories] = useState<CategoryItem[]>([]);
  const [subcategories, setSubcategories] = useState<SubcategoryItem[]>([]);
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [filters, setFilters] = useState<DocumentFilterValues>(NO_DOCUMENT_FILTERS);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadDocuments = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [documentData, categoryData, subcategoryData, employeeData] = await Promise.all([
        fetchDocuments(),
        fetchCategories(),
        fetchSubcategories(),
        fetchEmployees(),
      ]);
      setDocuments(documentData);
      setCategories(categoryData);
      setSubcategories(subcategoryData);
      setEmployees(employeeData);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadDocuments();
  }, [loadDocuments]);

  const filteredSubcategories = useMemo(
    () => subcategories.filter((subcategory) => !filters.categoryId || subcategory.category_id === filters.categoryId),
    [filters.categoryId, subcategories],
  );

  const filteredDocuments = useMemo(
    () => documents.filter((document) => matchesDocument(document, searchTerm, filters)),
    [documents, filters, searchTerm],
  );

  const hasActiveRestrictions = searchTerm.trim() !== "" || filters.categoryId !== "" || filters.subcategoryId !== "" || filters.responsiblePersonId !== "" || filters.status !== "all" || filters.validity !== "all";
  const handleFilterChange = (nextFilters: DocumentFilterValues) => {
    const selectedSubcategory = subcategories.find((subcategory) => subcategory.id === nextFilters.subcategoryId);
    setFilters({
      ...nextFilters,
      subcategoryId: selectedSubcategory && selectedSubcategory.category_id === nextFilters.categoryId ? nextFilters.subcategoryId : "",
    });
  };
  const resetSearchAndFilters = () => {
    setSearchTerm("");
    setFilters(NO_DOCUMENT_FILTERS);
  };

  return (
    <div className="pqm-dokumente">
      <DocumentToolbar resultCount={filteredDocuments.length} searchTerm={searchTerm} onSearchTermChange={setSearchTerm} />
      <DocumentFilters
        values={filters}
        onChange={handleFilterChange}
        categories={categories.map((category) => ({ value: category.id, label: category.name }))}
        subcategories={filteredSubcategories.map((subcategory) => ({ value: subcategory.id, label: subcategory.name }))}
        responsiblePeople={employees.map((employee) => ({ value: employee.id, label: `${employee.first_name} ${employee.last_name}` }))}
      />
      {hasActiveRestrictions && (
        <button type="button" className="pqm-dokumente__reset" onClick={resetSearchAndFilters}>
          Suche und Filter zurücksetzen
        </button>
      )}
      {error && <p className="pqm-dokumente__error" role="alert">Fehler beim Laden der Dokumente: {error}</p>}
      <DocumentList
        documents={filteredDocuments.map(toRowData)}
        loading={loading}
        filteredEmpty={!loading && documents.length > 0 && filteredDocuments.length === 0}
      />
    </div>
  );
}
