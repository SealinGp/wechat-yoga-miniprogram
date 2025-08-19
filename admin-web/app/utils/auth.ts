import axios from 'axios';
import { mockLogin, mockVerifyToken, mockNotices, mockTeachers, mockPosters, mockActionButtons } from './mock-auth';

const API_BASE = '/api'; // Use proxy in development
const USE_MOCK = false; // Set to false when backend is ready

export interface LoginData {
  username: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user_id: number;
  username: string;
}

export interface VerifyResponse {
  valid: boolean;
  user_id: number;
  username: string;
}

// Create axios instance with base URL
export const api = axios.create({
  baseURL: API_BASE,
  timeout: 10000,
});

// Add token to requests if available
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('admin_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Handle auth errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('admin_token');
      localStorage.removeItem('admin_user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export const login = async (data: LoginData): Promise<LoginResponse> => {
  if (USE_MOCK) {
    return await mockLogin(data);
  }
  const response = await api.post('/admin/login', data);
  return response.data;
};

export const verifyToken = async (token: string): Promise<VerifyResponse> => {
  if (USE_MOCK) {
    return await mockVerifyToken(token);
  }
  const response = await api.get(`/admin/verify?token=${token}`);
  return response.data;
};

// Add mock API functions for demo
export const mockApi = {
  async get(endpoint: string) {
    await new Promise(resolve => setTimeout(resolve, 500));
    
    if (endpoint === '/admin/notices') {
      return { data: mockNotices };
    } else if (endpoint === '/admin/teachers') {
      return { data: mockTeachers };
    } else if (endpoint === '/admin/posters') {
      return { data: mockPosters };
    } else if (endpoint === '/admin/actions') {
      return { data: mockActionButtons };
    }
    
    throw new Error('Endpoint not found');
  },
  
  async post(endpoint: string, data: any) {
    await new Promise(resolve => setTimeout(resolve, 800));
    
    if (endpoint === '/admin/notices') {
      const newItem = { id: Date.now(), ...data, created_at: new Date().toISOString() };
      mockNotices.push(newItem);
      return { data: newItem };
    } else if (endpoint === '/admin/teachers') {
      const newItem = { id: Date.now(), ...data, created_at: new Date().toISOString() };
      mockTeachers.push(newItem);
      return { data: newItem };
    }
    
    return { data: { id: Date.now(), ...data } };
  },
  
  async put(endpoint: string, data: any) {
    await new Promise(resolve => setTimeout(resolve, 600));
    return { data: { id: parseInt(endpoint.split('/').pop() || '1'), ...data } };
  },
  
  async delete(endpoint: string) {
    await new Promise(resolve => setTimeout(resolve, 400));
    return { data: { success: true } };
  }
};

export const getStoredAuth = () => {
  const token = localStorage.getItem('admin_token');
  const userStr = localStorage.getItem('admin_user');
  const user = userStr ? JSON.parse(userStr) : null;
  return { token, user };
};

export const setStoredAuth = (token: string, user: any) => {
  localStorage.setItem('admin_token', token);
  localStorage.setItem('admin_user', JSON.stringify(user));
};

export const clearStoredAuth = () => {
  localStorage.removeItem('admin_token');
  localStorage.removeItem('admin_user');
};

export const isAuthenticated = (): boolean => {
  const { token } = getStoredAuth();
  return !!token;
};