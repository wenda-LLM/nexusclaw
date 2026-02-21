export interface User {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'user';
  avatar?: string;
}

export interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

export interface LoginCredentials {
  email: string;
  password: string;
}

export interface RegisterData {
  email: string;
  password: string;
  name: string;
}

export interface GatewayMessage {
  method: string;
  params?: Record<string, unknown>;
  id?: string;
}

export interface GatewayResponse<T = unknown> {
  ok?: T;
  error?: string;
}

export interface PresenceEntry {
  host: string;
  mode: string;
  roles: string[];
  scopes: string[];
  platform?: string;
  deviceFamily?: string;
  modelIdentifier?: string;
  version?: string;
  lastInputSeconds?: number;
  reason?: string;
  updatedAt?: string;
}

export interface GatewaySessionRow {
  key: string;
  label?: string;
  displayName?: string;
  kind: string;
  updatedAt?: string;
  inputTokens?: number;
  outputTokens?: number;
  thinkingLevel?: string;
  verboseLevel?: string;
  reasoningLevel?: string;
  modelProvider?: string;
}

export interface SessionsListResult {
  sessions: GatewaySessionRow[];
  count: number;
  path: string;
}

export interface CronJob {
  id: string;
  name: string;
  schedule: string;
  message: string;
  sessionKey: string;
  enabled: boolean;
  nextRunAt?: string;
  lastRunAt?: string;
}

export interface CronStatus {
  enabled: boolean;
  nextWakeAtMs?: number;
}

export interface Tenant {
  id: string;
  name: string;
  createdAt: string;
  userCount: number;
}

export interface TenantUser {
  id: string;
  email: string;
  name: string;
  role: 'admin' | 'member';
  createdAt: string;
}

export interface Tool {
  name: string;
  description: string;
  category: string;
  enabled: boolean;
  parameters?: unknown;
}

export interface Skill {
  key: string;
  name: string;
  description: string;
  enabled: boolean;
  apiKey?: string;
  config?: Record<string, string>;
}

export interface McpServer {
  id: string;
  name: string;
  url: string;
  description?: string;
  status: 'connected' | 'disconnected';
  tools?: string[];
}

export interface ConfigSnapshot {
  config: Record<string, unknown>;
  path: string;
}

export interface HealthSnapshot {
  status: string;
  uptime: number;
  version: string;
}

export interface LogEntry {
  timestamp: string;
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  context?: string;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: string;
  thinking?: string;
  tools?: unknown[];
}

export interface ChatSession {
  key: string;
  label?: string;
  displayName?: string;
  lastMessage?: string;
  updatedAt: string;
}

export interface UsageSummary {
  totalTokens: number;
  totalCost: number;
  sessionCount: number;
  dateRange: {
    start: string;
    end: string;
  };
}

export type ThemeMode = 'light' | 'dark' | 'system';