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

declare global {
  interface Window {
    __TAURI__?: {
      invoke: <T = unknown>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
    };
  }
}

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!window.__TAURI__?.invoke) {
    throw new Error("Tauri nicht verfügbar – bitte als Desktop-App starten.");
  }
  return window.__TAURI__.invoke<T>(cmd, args);
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
