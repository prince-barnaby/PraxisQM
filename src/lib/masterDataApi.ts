import { invoke } from "./tauriInvoke";

export interface MasterDataItem {
  id: string;
  name: string;
}

export async function fetchResponsibilities(): Promise<MasterDataItem[]> {
  return invoke<MasterDataItem[]>("cmd_list_responsibilities");
}

export async function createResponsibility(name: string): Promise<MasterDataItem> {
  return invoke<MasterDataItem>("cmd_create_responsibility", { name });
}

export async function renameResponsibility(id: string, newName: string): Promise<MasterDataItem> {
  return invoke<MasterDataItem>("cmd_rename_responsibility", { id, newName });
}

export async function fetchQmAreas(): Promise<MasterDataItem[]> {
  return invoke<MasterDataItem[]>("cmd_list_qm_areas");
}

export async function createQmArea(name: string): Promise<MasterDataItem> {
  return invoke<MasterDataItem>("cmd_create_qm_area", { name });
}

export async function renameQmArea(id: string, newName: string): Promise<MasterDataItem> {
  return invoke<MasterDataItem>("cmd_rename_qm_area", { id, newName });
}
