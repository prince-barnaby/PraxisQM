import { invoke } from "./tauriInvoke";

export interface Document {
  id: string;
  document_number: string;
  title: string;
  category_id: string | null;
  category_name: string | null;
  subcategory_id: string | null;
  subcategory_name: string | null;
  responsible_person_id: string | null;
  responsible_person_name: string | null;
  version: string;
  status: string;
  validity: string;
  valid_until: string | null;
  description: string | null;
  archived_at: string | null;
  file_name: string | null;
  file_path: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateDocumentInput {
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
}

export interface CategoryItem {
  id: string;
  name: string;
}

export interface SubcategoryItem {
  id: string;
  name: string;
  category_id: string;
}

export async function fetchDocuments(): Promise<Document[]> {
  return invoke<Document[]>("cmd_list_documents");
}

export async function fetchDocument(id: string): Promise<Document> {
  return invoke<Document>("cmd_get_document", { id });
}

export async function fetchDocumentByNumber(number: string): Promise<Document> {
  return invoke<Document>("cmd_get_document_by_number", { number });
}

export async function createDocument(input: CreateDocumentInput): Promise<Document> {
  return invoke<Document>("cmd_create_document", { input });
}

export async function fetchCategories(): Promise<CategoryItem[]> {
  return invoke<CategoryItem[]>("cmd_list_categories");
}

export async function fetchSubcategories(): Promise<SubcategoryItem[]> {
  return invoke<SubcategoryItem[]>("cmd_list_subcategories");
}

export async function selectPdf(): Promise<string | null> {
  return invoke<string | null>("cmd_select_pdf");
}

export async function openPdf(documentId: string): Promise<void> {
  return invoke<void>("cmd_open_pdf", { documentId });
}
