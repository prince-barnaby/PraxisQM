import { invoke } from "./tauriInvoke";

export interface LoginResult {
  user_id: string;
  username: string;
  role: string;
}

export interface UserAccount {
  id: string;
  username: string;
  role: string;
  employee_id: string | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export async function needsSetup(): Promise<boolean> {
  return invoke<boolean>("cmd_needs_setup");
}

export async function setupAdmin(
  username: string,
  password: string,
): Promise<LoginResult> {
  return invoke<LoginResult>("cmd_setup_admin", { username, password });
}

export async function login(
  username: string,
  password: string,
): Promise<LoginResult> {
  return invoke<LoginResult>("cmd_login", { username, password });
}

export async function logout(): Promise<void> {
  return invoke<void>("cmd_logout");
}

export async function getCurrentUser(): Promise<UserAccount | null> {
  return invoke<UserAccount | null>("cmd_get_current_user");
}

export async function changePassword(
  currentPassword: string,
  newPassword: string,
): Promise<void> {
  return invoke<void>("cmd_change_password", {
    currentPassword,
    newPassword,
  });
}
