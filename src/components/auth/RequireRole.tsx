import type { ReactNode } from "react";
import { Navigate } from "react-router-dom";
import { useAuth } from "../../lib/AuthContext";

interface RequireRoleProps {
  level: "editor" | "admin";
  children: ReactNode;
}

export default function RequireRole({ level, children }: RequireRoleProps) {
  const { isAuthenticated, isEditor, isAdmin } = useAuth();

  const allowed = level === "admin" ? isAdmin : isEditor;
  if (allowed) {
    return <>{children}</>;
  }

  if (!isAuthenticated) {
    return <Navigate to="/anmelden" replace />;
  }
  return <Navigate to="/" replace />;
}
