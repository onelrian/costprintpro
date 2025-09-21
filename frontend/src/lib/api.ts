import axios from 'axios';
import type { 
  User, 
  LoginRequest, 
  LoginResponse, 
  Job, 
  CreateJobRequest, 
  CostCalculationRequest, 
  CostCalculationResponse,
  JobListQuery,
  JobListResponse,
  CostParameters,
  BrandingSettings,
  Currency,
  ExchangeRates,
  CurrencyConversionResponse,
  CurrencySettings
} from '@/types';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor to add auth token
api.interceptors.request.use((config) => {
  if (typeof window !== 'undefined') {
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
  }
  return config;
});

// Response interceptor to handle auth errors
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401 && typeof window !== 'undefined') {
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Auth API
export const authApi = {
  login: async (credentials: LoginRequest): Promise<LoginResponse> => {
    console.log('Making login request to:', `${API_BASE_URL}/api/auth/login`);
    console.log('Credentials:', credentials);
    const response = await api.post('/api/auth/login', credentials);
    console.log('Login response:', response.data);
    return response.data;
  },

  logout: async (): Promise<void> => {
    await api.post('/api/auth/logout');
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
  },

  me: async (): Promise<User> => {
    const response = await api.get('/api/auth/me');
    return response.data;
  },
};

// Jobs API
export const jobsApi = {
  list: async (query: JobListQuery = {}): Promise<JobListResponse> => {
    const response = await api.get('/api/jobs', { params: query });
    return response.data;
  },

  create: async (job: CreateJobRequest): Promise<Job> => {
    const response = await api.post('/api/jobs', job);
    return response.data;
  },

  get: async (id: string): Promise<Job> => {
    const response = await api.get(`/api/jobs/${id}`);
    return response.data;
  },

  getById: async (id: string): Promise<Job> => {
    const response = await api.get(`/api/jobs/${id}`);
    return response.data;
  },

  update: async (id: string, updates: Partial<CreateJobRequest>): Promise<Job> => {
    const response = await api.put(`/api/jobs/${id}`, updates);
    return response.data;
  },

  delete: async (id: string): Promise<void> => {
    await api.delete(`/api/jobs/${id}`);
  },
};

// Costing API
export const costingApi = {
  calculate: async (request: CostCalculationRequest): Promise<CostCalculationResponse> => {
    const response = await api.post('/api/cost/calculate', request);
    return response.data;
  },

  preview: async (request: CostCalculationRequest): Promise<CostCalculationResponse> => {
    const response = await api.post('/api/cost/preview', request);
    return response.data;
  },

  quickCalculate: async (request: CostCalculationRequest): Promise<CostCalculationResponse> => {
    const response = await api.post('/api/cost/quick', request);
    return response.data;
  },
};

// Settings API
export const settingsApi = {
  getCostParameters: async (): Promise<CostParameters> => {
    const response = await api.get('/api/settings/cost-parameters');
    return response.data;
  },

  updateCostParameters: async (params: Partial<CostParameters>): Promise<CostParameters> => {
    const response = await api.put('/api/settings/cost-parameters', params);
    return response.data;
  },

  getBranding: async (): Promise<BrandingSettings> => {
    const response = await api.get('/api/settings/branding');
    return response.data;
  },

  updateBranding: async (settings: Partial<BrandingSettings>): Promise<BrandingSettings> => {
    const response = await api.put('/api/settings/branding', settings);
    return response.data;
  },
};

// Currency API
export const currencyApi = {
  getSupportedCurrencies: async (): Promise<Currency[]> => {
    const response = await api.get('/api/currency/supported');
    return response.data;
  },

  getExchangeRates: async (): Promise<ExchangeRates> => {
    const response = await api.get('/api/currency/rates');
    return response.data;
  },

  convertCurrency: async (amount: number, from: Currency, to: Currency): Promise<CurrencyConversionResponse> => {
    const response = await api.get(`/api/currency/convert?amount=${amount}&from=${from}&to=${to}`);
    return response.data;
  },

  getCurrencySettings: async (): Promise<CurrencySettings> => {
    const response = await api.get('/api/currency/settings');
    return response.data;
  },
};

// Export API
export const exportApi = {
  exportPdf: async (jobId: string): Promise<Blob> => {
    const response = await api.post(`/api/export/pdf/${jobId}`, {}, {
      responseType: 'blob',
    });
    return response.data;
  },

  exportExcel: async (jobId: string): Promise<Blob> => {
    const response = await api.post(`/api/export/excel/${jobId}`, {}, {
      responseType: 'blob',
    });
    return response.data;
  },
};

export default api;
