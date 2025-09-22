export interface User {
  id: string;
  email: string;
  name: string;
  role: 'Admin' | 'Manager' | 'User';
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export type JobType = 'Book' | 'Flyer' | 'BusinessCard' | 'Brochure' | 'Poster' | 'Banner' | 'Sticker' | 'Custom';

export type JobStatus = 'Draft' | 'Quoted' | 'Approved' | 'InProduction' | 'Completed' | 'Cancelled';

export interface ColorSpecification {
  frontColors: number;
  backColors: number;
  spotColors: string[];
  isFullColor: boolean;
}

export interface JobSpecifications {
  paperType: string;
  paperSize: string;
  paperWeight?: string;
  colors: ColorSpecification;
  pages?: number;
  binding?: string;
  lamination?: string;
  finishing: string[];
  specialRequirements?: string;
}

export interface CostBreakdown {
  paperCost: number;
  plateCost: number;
  laborCost: number;
  bindingCost: number;
  finishingCost: number;
  overhead: number;
}

export interface Job {
  id: string;
  userId: string;
  title: string;
  jobType: JobType;
  quantity: number;
  specifications: JobSpecifications;
  costBreakdown: CostBreakdown;
  totalCost: number;
  unitCost: number;
  status: JobStatus;
  createdAt: string;
  updatedAt: string;
}

export interface CreateJobRequest {
  title: string;
  jobType: JobType;
  quantity: number;
  specifications: JobSpecifications;
}

export interface CostCalculationRequest {
  jobType: JobType;
  quantity: number;
  specifications: JobSpecifications;
  currency?: Currency;
}

export interface CostCalculationResponse {
  costBreakdown: CostBreakdown;
  totalCost: number;
  unitCost: number;
  estimatedDeliveryDays: number;
  currency?: Currency;
  exchangeRate?: number;
}

export interface CostParameters {
  id: string;
  paperCostPerSheet: number;
  plateCostPerJob: number;
  laborCostPerHour: number;
  bindingCostPerUnit: number;
  overheadPercentage: number;
  profitMarginPercentage: number;
  createdAt: string;
  updatedAt: string;
}

export interface BrandingSettings {
  id: string;
  companyName: string;
  companyLogoUrl?: string;
  primaryColor: string;
  secondaryColor: string;
  createdAt: string;
  updatedAt: string;
}

export interface JobListQuery {
  page?: number;
  limit?: number;
  jobType?: JobType;
  status?: JobStatus;
  search?: string;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
}

export interface JobListResponse {
  jobs: Job[];
  total: number;
  page: number;
  limit: number;
  totalPages: number;
}

export type Currency = 'USD' | 'FCFA' | 'EUR' | 'GBP' | 'CAD';

export interface CurrencyInfo {
  code: Currency;
  symbol: string;
  name: string;
}

export interface ExchangeRates {
  base: Currency;
  rates: Record<string, number>;
  lastUpdated: string;
}

export interface CurrencyConversionRequest {
  amount: number;
  fromCurrency: Currency;
  toCurrency: Currency;
}

export interface CurrencyConversionResponse {
  originalAmount: number;
  convertedAmount: number;
  fromCurrency: Currency;
  toCurrency: Currency;
  exchangeRate: number;
}

export interface CurrencySettings {
  defaultCurrency: Currency;
  supportedCurrencies: Currency[];
}
