import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles/tokens.css";
import "./styles/global.css";

// PraxisQM – Anwendungseintrittspunkt
// Modul: Grundstruktur
// Zweck: Bindet React in das DOM ein und lädt globale Styles.
ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
