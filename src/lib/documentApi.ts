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
  computed_validity: string | null;
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

export interface UpdateDocumentInput {
  title: string;
  category_id: string | null;
  subcategory_id: string | null;
  responsible_person_id: string | null;
  version: string;
  status: string;
  validity: string;
  valid_until: string | null;
  description: string | null;
}

export interface DocumentVersion {
  id: string;
  document_id: string;
  version_number: string;
  file_name: string;
  status: string;
  validity: string;
  valid_until: string | null;
  uploaded_at: string;
  created_at: string;
  is_current: boolean;
}

export interface CreateVersionInput {
  document_id: string;
  version_number: string;
  status: string;
  validity: string;
  valid_until: string | null;
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

export async function updateDocument(id: string, input: UpdateDocumentInput): Promise<Document> {
  return invoke<Document>("cmd_update_document", { id, input });
}

export async function createVersion(input: CreateVersionInput): Promise<Document> {
  return invoke<Document>("cmd_create_version", { input });
}

export async function listVersions(documentId: string): Promise<DocumentVersion[]> {
  return invoke<DocumentVersion[]>("cmd_list_versions", { documentId });
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

export async function createCategory(name: string): Promise<CategoryItem> {
  return invoke<CategoryItem>("cmd_create_category", { name });
}

export async function renameCategory(id: string, newName: string): Promise<CategoryItem> {
  return invoke<CategoryItem>("cmd_rename_category", { id, newName });
}

export async function createSubcategory(name: string, categoryId: string): Promise<SubcategoryItem> {
  return invoke<SubcategoryItem>("cmd_create_subcategory", { name, categoryId });
}

export async function renameSubcategory(id: string, newName: string): Promise<SubcategoryItem> {
  return invoke<SubcategoryItem>("cmd_rename_subcategory", { id, newName });
}

export async function selectPdf(): Promise<string | null> {
  return invoke<string | null>("cmd_select_pdf");
}

export async function archiveDocument(id: string): Promise<Document> {
  return invoke<Document>("cmd_archive_document", { id });
}

export async function restoreDocument(id: string): Promise<Document> {
  return invoke<Document>("cmd_restore_document", { id });
}

export async function fetchArchivedDocuments(): Promise<Document[]> {
  return invoke<Document[]>("cmd_list_archived_documents");
}
