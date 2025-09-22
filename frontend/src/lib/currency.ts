import type { Currency, CurrencyInfo } from '@/types';

export const CURRENCY_INFO: Record<Currency, CurrencyInfo> = {
  USD: {
    code: 'USD',
    symbol: '$',
    name: 'US Dollar',
  },
  FCFA: {
    code: 'FCFA',
    symbol: 'FCFA',
    name: 'Central African CFA Franc',
  },
  EUR: {
    code: 'EUR',
    symbol: '€',
    name: 'Euro',
  },
  GBP: {
    code: 'GBP',
    symbol: '£',
    name: 'British Pound',
  },
  CAD: {
    code: 'CAD',
    symbol: 'C$',
    name: 'Canadian Dollar',
  },
};

export const formatCurrency = (amount: number, currency: Currency = 'USD'): string => {
  const currencyInfo = CURRENCY_INFO[currency];
  
  // Format based on currency type
  if (currency === 'FCFA') {
    // FCFA typically doesn't use decimal places
    return `${Math.round(amount).toLocaleString()} ${currencyInfo.symbol}`;
  }
  
  // For other currencies, use standard formatting
  // Map FCFA to XAF for Intl.NumberFormat (though this branch won't execute for FCFA)
  const intlCurrency = currency === 'FCFA' ? 'XAF' : currency;
  
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: intlCurrency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
};

export const getCurrencySymbol = (currency: Currency): string => {
  return CURRENCY_INFO[currency]?.symbol || '$';
};

export const getCurrencyName = (currency: Currency): string => {
  return CURRENCY_INFO[currency]?.name || 'US Dollar';
};

export const getDefaultCurrency = (): Currency => {
  // Handle localStorage safely - it might not be available in SSR
  if (typeof window === 'undefined') {
    return 'USD';
  }
  
  try {
    const storedCurrency = localStorage.getItem('preferredCurrency');
    // Validate that the stored currency is actually a valid Currency type
    if (storedCurrency && Object.keys(CURRENCY_INFO).includes(storedCurrency)) {
      return storedCurrency as Currency;
    }
  } catch (error) {
    console.warn('Failed to access localStorage:', error);
  }
  
  return 'USD';
};

export const setDefaultCurrency = (currency: Currency): void => {
  if (typeof window === 'undefined') {
    return;
  }
  
  try {
    localStorage.setItem('preferredCurrency', currency);
  } catch (error) {
    console.warn('Failed to save currency preference:', error);
  }
};
