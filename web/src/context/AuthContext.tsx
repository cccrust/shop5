import { createContext, useContext, useState, useEffect, type ReactNode } from "react";
import { api, setToken, getToken } from "../api/client";
import type { User } from "../types";

interface AuthContextValue {
  user: User | null;
  loading: boolean;
  login: (username: string, password: string) => Promise<void>;
  register: (data: { username: string; display_name: string; email?: string; password: string; role?: string }) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const token = getToken();
    if (!token) {
      setLoading(false);
      return;
    }
    api.auth.me().then((u) => {
      setUser(u);
    }).catch(() => {
      setToken(null);
    }).finally(() => setLoading(false));
  }, []);

  const login = async (username: string, password: string) => {
    const res = await api.auth.login(username, password);
    setToken(res.token);
    setUser(res.user);
  };

  const register = async (data: { username: string; display_name: string; email?: string; password: string; role?: string }) => {
    const res = await api.auth.register(data);
    setToken(res.token);
    setUser(res.user);
  };

  const logout = async () => {
    try { await api.auth.logout(); } catch { /* ignore */ }
    setToken(null);
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ user, loading, login, register, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
