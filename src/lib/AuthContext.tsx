import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import * as authApi from "./authApi";

export interface AuthUser {
  username: string;
  role: string;
}

type AuthStatus = "loading" | "setup" | "ready" | "error";

interface AuthContextValue {
  status: AuthStatus;
  loadError: string | null;
  user: AuthUser | null;
  isAuthenticated: boolean;
  isEditor: boolean;
  isAdmin: boolean;
  retryInitialize: () => void;
  completeSetup: (username: string, password: string) => Promise<void>;
  login: (username: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function errorMessage(err: unknown): string {
  if (typeof err === "string") return err;
  if (err instanceof Error) return err.message;
  return "Unbekannter Fehler.";
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [status, setStatus] = useState<AuthStatus>("loading");
  const [loadError, setLoadError] = useState<string | null>(null);
  const [user, setUser] = useState<AuthUser | null>(null);
  const [attempt, setAttempt] = useState(0);

  useEffect(() => {
    let cancelled = false;

    async function initialize() {
      setStatus("loading");
      setLoadError(null);
      try {
        const needsSetup = await authApi.needsSetup();
        if (cancelled) return;
        if (needsSetup) {
          setUser(null);
          setStatus("setup");
          return;
        }
        const current = await authApi.getCurrentUser();
        if (cancelled) return;
        setUser(current ? { username: current.username, role: current.role } : null);
        setStatus("ready");
      } catch (err) {
        if (cancelled) return;
        setLoadError(errorMessage(err));
        setStatus("error");
      }
    }

    void initialize();
    return () => {
      cancelled = true;
    };
  }, [attempt]);

  const retryInitialize = useCallback(() => setAttempt((n) => n + 1), []);

  const completeSetup = useCallback(async (username: string, password: string) => {
    const result = await authApi.setupAdmin(username, password);
    setUser({ username: result.username, role: result.role });
    setStatus("ready");
  }, []);

  const login = useCallback(async (username: string, password: string) => {
    const result = await authApi.login(username, password);
    setUser({ username: result.username, role: result.role });
  }, []);

  const logout = useCallback(async () => {
    await authApi.logout();
    setUser(null);
  }, []);

  const value = useMemo<AuthContextValue>(
    () => ({
      status,
      loadError,
      user,
      isAuthenticated: user !== null,
      isEditor: user?.role === "Editor" || user?.role === "Admin",
      isAdmin: user?.role === "Admin",
      retryInitialize,
      completeSetup,
      login,
      logout,
    }),
    [status, loadError, user, retryInitialize, completeSetup, login, logout],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error("useAuth muss innerhalb von AuthProvider verwendet werden.");
  }
  return ctx;
}
