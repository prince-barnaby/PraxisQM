import { invoke } from "./tauriInvoke";

export interface DashboardSummary {
  total_active: number;
  valid: number;
  warning: number;
  expired: number;
  no_validity: number;
  archived: number;
  employees: number;
}

export interface ReviewEntry {
  id: string;
  document_number: string;
  title: string;
  valid_until: string | null;
  computed_validity: string;
  responsible_person_name: string | null;
  category_name: string | null;
}

export async function fetchDashboardSummary(): Promise<DashboardSummary> {
  return invoke<DashboardSummary>("cmd_dashboard_summary");
}

export async function fetchReviewList(): Promise<ReviewEntry[]> {
  return invoke<ReviewEntry[]>("cmd_review_list");
}
