import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { ChevronLeft } from "lucide-react";
import DocumentForm from "../components/documents/DocumentForm";
import type { DocumentFormData } from "../components/documents/DocumentForm";
import {
  fetchCategories,
  fetchSubcategories,
  createDocument,
  selectPdf,
  type CategoryItem,
  type SubcategoryItem,
} from "../lib/documentApi";
import { fetchEmployees, type Employee } from "../lib/employeeApi";
import "./DokumentNeu.css";

export default function DokumentNeu() {
  const navigate = useNavigate();
  const [categories, setCategories] = useState<CategoryItem[]>([]);
  const [subcategories, setSubcategories] = useState<SubcategoryItem[]>([]);
  const [employees, setEmployees] = useState<{ id: string; name: string }[]>([]);

  useEffect(() => {
    Promise.all([fetchCategories(), fetchSubcategories(), fetchEmployees()])
      .then(([cats, subs, emps]) => {
        setCategories(cats);
        setSubcategories(subs);
        setEmployees(
          emps.map((e: Employee) => ({
            id: e.id,
            name: `${e.last_name}, ${e.first_name}`,
          })),
        );
      })
      .catch(() => {
        // Daten können leer sein, Formular bleibt nutzbar
      });
  }, []);

  const handleSubmit = async (data: DocumentFormData) => {
    await createDocument({
      title: data.title,
      category_id: data.category_id,
      subcategory_id: data.subcategory_id,
      responsible_person_id: data.responsible_person_id,
      version: data.version,
      status: data.status,
      validity: data.validity,
      valid_until: data.valid_until,
      description: data.description,
      source_file_path: data.source_file_path,
      original_file_name: data.original_file_name,
    });
    navigate("/dokumente");
  };

  return (
    <div className="pqm-dokument-neu">
      <button
        type="button"
        className="pqm-dokument-neu__back"
        onClick={() => navigate("/dokumente")}
        aria-label="Zurück zu Dokumenten"
      >
        <ChevronLeft size={16} aria-hidden="true" />
        Zurück zu Dokumenten
      </button>

      <header className="pqm-dokument-neu__header">
        <h2 className="pqm-dokument-neu__title">Neues Dokument</h2>
        <p className="pqm-dokument-neu__subtitle">
          Neues QM-Dokument anlegen – Metadaten und PDF werden dauerhaft gespeichert
        </p>
      </header>

      <DocumentForm
        mode="create"
        categories={categories}
        subcategories={subcategories}
        employees={employees}
        onSubmit={handleSubmit}
        onCancel={() => navigate("/dokumente")}
        onSelectPdf={selectPdf}
      />
    </div>
  );
}
