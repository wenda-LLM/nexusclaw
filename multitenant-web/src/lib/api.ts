import type { User, LoginCredentials, RegisterData } from '@/types';

const API_BASE = '/api';

async function request<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const token = localStorage.getItem('token');
  
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...(token ? { Authorization: `Bearer ${token}` } : {}),
    ...options.headers,
  };

  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: 'Request failed' }));
    throw new Error(error.error || `HTTP ${response.status}`);
  }

  const json = await response.json();
  if (json.data !== undefined) {
    return json.data as T;
  }
  return json as T;
}

export async function login(credentials: LoginCredentials) {
  const data = await request<{ token: string; user: User }>('/auth/login', {
    method: 'POST',
    body: JSON.stringify(credentials),
  });
  
  localStorage.setItem('token', data.token);
  return data;
}

export async function register(data: RegisterData) {
  const result = await request<{ token: string; user: User }>('/auth/register', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  
  localStorage.setItem('token', result.token);
  return result;
}

export async function logout() {
  localStorage.removeItem('token');
}

export async function getCurrentUser() {
  const data = await request<{ user: User; isAdmin: boolean }>('/auth/me');
  return data.user;
}

export async function pairDevice(code: string) {
  const data = await request<{ token: string; user: User }>('/auth/pair', {
    method: 'POST',
    body: JSON.stringify({ code }),
  });
  
  localStorage.setItem('token', data.token);
  return data;
}

export async function updateProfile(data: { name?: string; avatar?: string }) {
  return request<User>('/auth/profile', {
    method: 'PUT',
    body: JSON.stringify(data),
  });
}

export async function changePassword(oldPassword: string, newPassword: string) {
  return request<{ success: boolean }>('/auth/password', {
    method: 'PUT',
    body: JSON.stringify({ oldPassword, newPassword }),
  });
}