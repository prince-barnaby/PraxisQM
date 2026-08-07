import { invoke as tauriInvoke } from "@tauri-apps/api/tauri";

export interface Employee {
  id: string;
  last_name: string;
  first_name: string;
  position: string | null;
  is_active: boolean;
  hire_date: string | null;
  departure_date: string | null;
  responsibilities: string[];
  qm_areas: string[];
}

export interface MasterDataItem {
  id: string;
  name: string;
}

export interface CreateEmployeeInput {
  last_name: string;
  first_name: string;
  position: string | null;
  is_active: boolean;
  hire_date: string | null;
  departure_date: string | null;
  responsibility_ids: string[];
  qm_area_ids: string[];
}

function isTauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_IPC__" in window;
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriAvailable()) {
    throw new Error("Desktop-App erforderlich – bitte als Tauri-Desktop-App starten.");
  }
  return tauriInvoke<T>(cmd, args);
}

export async function fetchEmployees(): Promise<Employee[]> {
  return invoke<Employee[]>("cmd_list_employees");
}

export async function createEmployee(input: CreateEmployeeInput): Promise<Employee> {
  return invoke<Employee>("cmd_create_employee", { input });
}

export async function fetchResponsibilities(): Promise<MasterDataItem[]> {
  return invoke<MasterDataItem[]>("cmd_list_responsibilities");
}

export async function fetchQmAreas(): Promise<MasterDataItem[]> {
  return invoke<MasterDataItem[]>("cmd_list_qm_areas");
}
