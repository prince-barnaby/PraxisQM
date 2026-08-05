import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// PraxisQM – Vite-Konfiguration
// Modul: Build-System
// Zweck: Konfiguriert Vite für die lokale React-/TypeScript-Desktop-Anwendung.
export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2021",
    minify: "esbuild",
    sourcemap: false,
  },
});
