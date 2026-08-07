import { invoke as tauriInvoke } from "@tauri-apps/api/tauri";

export function isTauriAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_IPC__" in window;
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauriAvailable()) {
    throw new Error("Desktop-App erforderlich – bitte als Tauri-Desktop-App starten.");
  }
  return tauriInvoke<T>(cmd, args);
}
