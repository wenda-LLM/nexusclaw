import { createContext, useContext, useState, useEffect, type ReactNode } from 'react';
import type { User, AuthState } from '@/types';
import * as api from '@/lib/api';
import { gateway } from '@/lib/gateway';

interface AuthContextType extends AuthState {
  login: (email: string, password: string) => Promise<void>;
  register: (email: string, password: string, name: string) => Promise<void>;
  pair: (code: string) => Promise<void>;
  logout: () => void;
  updateUser: (user: Partial<User>) => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<AuthState>({
    user: null,
    token: null,
    isAuthenticated: false,
    isLoading: true,
  });

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      api.getCurrentUser()
        .then((user) => {
          setState({
            user,
            token,
            isAuthenticated: true,
            isLoading: false,
          });
          initGateway(token);
        })
        .catch(() => {
          localStorage.removeItem('token');
          setState({
            user: null,
            token: null,
            isAuthenticated: false,
            isLoading: false,
          });
        });
    } else {
      setState((prev) => ({ ...prev, isLoading: false }));
    }
  }, []);

  const initGateway = (token: string) => {
    const wsUrl = import.meta.env.VITE_WS_URL || window.location.origin;
    gateway.connect(wsUrl, token);
  };

  const login = async (email: string, password: string) => {
    const { user, token } = await api.login({ email, password });
    setState({ user, token, isAuthenticated: true, isLoading: false });
    initGateway(token);
  };

  const register = async (email: string, password: string, name: string) => {
    const { user, token } = await api.register({ email, password, name });
    setState({ user, token, isAuthenticated: true, isLoading: false });
    initGateway(token);
  };

  const pair = async (code: string) => {
    const { user, token } = await api.pairDevice(code);
    setState({ user, token, isAuthenticated: true, isLoading: false });
    initGateway(token);
  };

  const logout = () => {
    api.logout();
    gateway.disconnect();
    setState({ user: null, token: null, isAuthenticated: false, isLoading: false });
  };

  const updateUser = (userData: Partial<User>) => {
    if (state.user) {
      const updatedUser = { ...state.user, ...userData };
      setState((prev) => ({ ...prev, user: updatedUser }));
    }
  };

  return (
    <AuthContext.Provider value={{ ...state, login, register, pair, logout, updateUser }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
}