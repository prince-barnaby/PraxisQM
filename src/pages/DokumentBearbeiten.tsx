import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { ArrowLeft } from "lucide-react";
import DocumentForm, {
  type CategoryOption,
  type SubcategoryOption,
  type EmployeeOption,
  type DocumentFormInitialValues,
} from "../components/documents/DocumentForm";
import {
  fetchDocumentByNumber,
  fetchCategories,
  fetchSubcategories,
  updateDocument,
  selectPdf,
  type Document,
  type UpdateDocumentInput,
} from "../lib/documentApi";
import { fetchEmployees } from "../lib/employeeApi";
import "./DokumentBearbeiten.css";

export default function DokumentBearbeiten() {
  const { id } = useParams();
  const navigate = useNavigate();
  const documentNumber = id ?? "Unbekannt";

  const [doc, setDoc] = useState<Document | null>(null);
  const [categories, setCategories] = useState<CategoryOption[]>([]);
  const [subcategories, setSubcategories] = useState<SubcategoryOption[]>([]);
  const [employees, setEmployees] = useState<EmployeeOption[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    setError(null);
    Promise.all([
      fetchDocumentByNumber(documentNumber),
      fetchCategories(),
      fetchSubcategories(),
      fetchEmployees(),
    ])
      .then(([d, cats, subs, emps]) => {
        setDoc(d);
        setCategories(cats);
        setSubcategories(subs);
        setEmployees(
          emps.map((e) => ({
            id: e.id,
            name: `${e.last_name}, ${e.first_name}`,
          }))
        );
      })
      .catch((err) => {
        setError(err instanceof Error ? err.message : String(err));
      })
      .finally(() => setLoading(false));
  }, [documentNumber]);

  const handleSubmit = async (data: {
    title: string;
    category_id: string | null;
    subcategory_id: string | null;
    responsible_person_id: string | null;
    version: string;
    status: string;
    validity: string;
    valid_until: string | null;
    description: string | null;
    source_file_path: string;
    original_file_name: string;
  }) => {
    if (!doc) return;
    const input: UpdateDocumentInput = {
      title: data.title,
      category_id: data.category_id,
      subcategory_id: data.subcategory_id,
      responsible_person_id: data.responsible_person_id,
      version: data.version,
      status: data.status,
      validity: data.validity,
      valid_until: data.valid_until,
      description: data.description,
      source_file_path: data.source_file_path || null,
      original_file_name: data.original_file_name || null,
    };
    await updateDocument(doc.id, input);
    navigate(`/dokumente/${documentNumber}`);
  };

  const handleSelectPdf = async () => {
    return await selectPdf();
  };

  const initialValues: DocumentFormInitialValues | undefined = doc
    ? {
        title: doc.title,
        category_id: doc.category_id,
        subcategory_id: doc.subcategory_id,
        responsible_person_id: doc.responsible_person_id,
        version: doc.version,
        status: doc.status,
        validity: doc.validity,
        valid_until: doc.valid_until,
        description: doc.description,
        file_name: doc.file_name,
      }
    : undefined;

  if (loading) {
    return (
      <div className="pqm-dokument-bearbeiten">
        <p className="pqm-dokument-bearbeiten__loading">Dokument wird geladen …</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="pqm-dokument-bearbeiten">
        <button
          type="button"
          className="pqm-dokument-bearbeiten__back"
          onClick={() => navigate("/dokumente")}
        >
          <ArrowLeft size={14} aria-hidden="true" />
          Zurück zu Dokumenten
        </button>
        <p className="pqm-dokument-bearbeiten__error" role="alert">
          {error}
        </p>
      </div>
    );
  }

  if (!doc) {
    return (
      <div className="pqm-dokument-bearbeiten">
        <button
          type="button"
          className="pqm-dokument-bearbeiten__back"
          onClick={() => navigate("/dokumente")}
        >
          <ArrowLeft size={14} aria-hidden="true" />
          Zurück zu Dokumenten
        </button>
        <p className="pqm-dokument-bearbeiten__error" role="alert">
          Dokument nicht gefunden.
        </p>
      </div>
    );
  }

  return (
    <div className="pqm-dokument-bearbeiten">
      <button
        type="button"
        className="pqm-dokument-bearbeiten__back"
        onClick={() => navigate(`/dokumente/${documentNumber}`)}
      >
        <ArrowLeft size={14} aria-hidden="true" />
        Zurück zum Dokument
      </button>
      <header className="pqm-dokument-bearbeiten__header">
        <h2 className="pqm-dokument-bearbeiten__title">Dokument bearbeiten</h2>
        <p className="pqm-dokument-bearbeiten__subtitle">
          {doc.title} ({documentNumber})
        </p>
      </header>
      <DocumentForm
        mode="edit"
        documentNumber={documentNumber}
        categories={categories}
        subcategories={subcategories}
        employees={employees}
        initialValues={initialValues}
        onSubmit={handleSubmit}
        onCancel={() => navigate(`/dokumente/${documentNumber}`)}
        onSelectPdf={handleSelectPdf}
      />
    </div>
  );
}
