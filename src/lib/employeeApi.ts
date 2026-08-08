import { invoke } from "./tauriInvoke";

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
  responsibility_ids: string[];
  qm_area_ids: string[];
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

export interface UpdateEmployeeInput {
  last_name: string;
  first_name: string;
  position: string | null;
  is_active: boolean;
  hire_date: string | null;
  departure_date: string | null;
  responsibility_ids: string[];
  qm_area_ids: string[];
}

export async function fetchEmployees(): Promise<Employee[]> {
  return invoke<Employee[]>("cmd_list_employees");
}

export async function fetchEmployee(id: string): Promise<Employee> {
  return invoke<Employee>("cmd_get_employee", { id });
}

export async function createEmployee(input: CreateEmployeeInput): Promise<Employee> {
  return invoke<Employee>("cmd_create_employee", { input });
}

export async function updateEmployee(id: string, input: UpdateEmployeeInput): Promise<Employee> {
  return invoke<Employee>("cmd_update_employee", { id, input });
}

export async function fetchResponsibilities(): Promise<MasterDataItem[]> {
  return invoke<MasterDataItem[]>("cmd_list_responsibilities");
}

export async function fetchQmAreas(): Promise<MasterDataItem[]> {
  return invoke<MasterDataItem[]>("cmd_list_qm_areas");
}
