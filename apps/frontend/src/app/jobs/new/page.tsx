'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { useAuth } from '@/contexts/AuthContext';
import { jobsApi, costingApi, currencyApi } from '@/lib/api';
import type { JobType, JobSpecifications, CostCalculationResponse, Currency } from '@/types';
import { ArrowLeft, Calculator, Save, DollarSign, Zap } from 'lucide-react';
import { formatCurrency, CURRENCY_INFO, getDefaultCurrency } from '@/lib/currency';
import Link from 'next/link';

export default function NewJobPage() {
  const { user: _user } = useAuth();
  const router = useRouter();
  
  const [formData, setFormData] = useState({
    title: '',
    jobType: 'Flyer' as JobType,
    quantity: 100,
    specifications: {
      paperType: '80gsm_offset',
      paperSize: 'A4',
      paperWeight: '80gsm',
      colors: {
        frontColors: 4,
        backColors: 0,
        spotColors: [],
        isFullColor: true,
      },
      pages: 1,
      binding: '',
      lamination: '',
      finishing: [],
      specialRequirements: '',
    } as JobSpecifications,
  });

  const [costCalculation, setCostCalculation] = useState<CostCalculationResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [calculating, setCalculating] = useState(false);
  const [error, setError] = useState('');
  const [selectedCurrency, setSelectedCurrency] = useState<Currency>(getDefaultCurrency());
  const [supportedCurrencies, setSupportedCurrencies] = useState<Currency[]>(['USD', 'FCFA', 'EUR', 'GBP', 'CAD']);

  const jobTypes: { value: JobType; label: string }[] = [
    { value: 'Flyer', label: 'Flyer' },
    { value: 'Brochure', label: 'Brochure' },
    { value: 'BusinessCard', label: 'Business Card' },
    { value: 'Book', label: 'Book' },
    { value: 'Poster', label: 'Poster' },
    { value: 'Banner', label: 'Banner' },
    { value: 'Sticker', label: 'Sticker' },
    { value: 'Custom', label: 'Custom' },
  ];

  const paperSizes = ['A4', 'A3', 'A2', 'A1', 'Letter', 'Legal', 'Tabloid'];
  const paperTypes = ['80gsm_offset', '120gsm_coated', '160gsm_coated', '250gsm_card'];
  const bindingOptions = ['', 'saddle_stitch', 'perfect_bind', 'spiral_bind', 'wire_bind'];
  const laminationOptions = ['', 'gloss', 'matte', 'soft_touch'];
  const finishingOptions = ['cutting', 'folding', 'scoring', 'perforation', 'embossing'];

  useEffect(() => {
    const loadSupportedCurrencies = async () => {
      try {
        const currencies = await currencyApi.getSupportedCurrencies();
        setSupportedCurrencies(currencies);
      } catch (error) {
        console.error('Failed to load supported currencies:', error);
      }
    };
    
    loadSupportedCurrencies();
  }, []);

  const handleInputChange = (field: string, value: string | number | boolean) => {
    if (field.startsWith('specifications.')) {
      const specField = field.replace('specifications.', '');
      if (specField.startsWith('colors.')) {
        const colorField = specField.replace('colors.', '');
        setFormData(prev => ({
          ...prev,
          specifications: {
            ...prev.specifications,
            colors: {
              ...prev.specifications.colors,
              [colorField]: value,
            },
          },
        }));
      } else {
        setFormData(prev => ({
          ...prev,
          specifications: {
            ...prev.specifications,
            [specField]: value,
          },
        }));
      }
    } else {
      setFormData(prev => ({
        ...prev,
        [field]: value,
      }));
    }
    
    // Clear previous cost calculation when form changes
    setCostCalculation(null);
  };

  const handleFinishingChange = (finishing: string, checked: boolean) => {
    setFormData(prev => ({
      ...prev,
      specifications: {
        ...prev.specifications,
        finishing: checked
          ? [...prev.specifications.finishing, finishing]
          : prev.specifications.finishing.filter(f => f !== finishing),
      },
    }));
  };

  const calculateCost = async () => {
    setCalculating(true);
    setError('');

    try {
      const result = await costingApi.calculate({
        jobType: formData.jobType,
        quantity: formData.quantity,
        specifications: formData.specifications,
        currency: selectedCurrency,
      });
      setCostCalculation(result);
    } catch (error: unknown) {
      console.error('Cost calculation failed:', error);
      let errorMessage = 'Failed to calculate cost. Please try again.';
      
      if (error instanceof Error) {
        if (error.message.includes('network') || error.message.includes('fetch')) {
          errorMessage = 'Network error. Please check your connection and try again.';
        } else if (error.message.includes('validation')) {
          errorMessage = 'Invalid job specifications. Please check your inputs.';
        } else {
          errorMessage = error.message;
        }
      }
      
      setError(errorMessage);
    } finally {
      setCalculating(false);
    }
  };

  const quickCalculate = async () => {
    setCalculating(true);
    setError('');

    try {
      const result = await costingApi.quickCalculate({
        jobType: formData.jobType,
        quantity: formData.quantity,
        specifications: formData.specifications,
        currency: selectedCurrency,
      });
      setCostCalculation(result);
    } catch (error: unknown) {
      console.error('Quick calculation failed:', error);
      let errorMessage = 'Failed to perform quick calculation. Please try again.';
      
      if (error instanceof Error) {
        if (error.message.includes('network') || error.message.includes('fetch')) {
          errorMessage = 'Network error. Please check your connection and try again.';
        } else if (error.message.includes('validation')) {
          errorMessage = 'Invalid job specifications. Please check your inputs.';
        } else {
          errorMessage = error.message;
        }
      }
      
      setError(errorMessage);
    } finally {
      setCalculating(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Validate form data
    if (!formData.title.trim()) {
      setError('Please enter a job title');
      return;
    }
    
    if (formData.quantity <= 0) {
      setError('Please enter a valid quantity greater than 0');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const job = await jobsApi.create({
        title: formData.title,
        jobType: formData.jobType,
        quantity: formData.quantity,
        specifications: formData.specifications,
      });
      
      // Show success message briefly before redirecting
      console.log('Job created successfully:', job);
      router.push(`/jobs/${job.id}`);
    } catch (error: unknown) {
      console.error('Job creation failed:', error);
      
      let errorMessage = 'Failed to create job. Please try again.';
      
      if (error instanceof Error) {
        if (error.message.includes('network') || error.message.includes('fetch')) {
          errorMessage = 'Network error. Please check your connection and try again.';
        } else if (error.message.includes('validation')) {
          errorMessage = 'Invalid job data. Please check your inputs and try again.';
        } else if (error.message.includes('unauthorized')) {
          errorMessage = 'You are not authorized to create jobs. Please log in again.';
        } else {
          errorMessage = error.message;
        }
      }
      
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  };


  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Header */}
        <div className="mb-8">
          <Link
            href="/"
            className="inline-flex items-center text-blue-600 hover:text-blue-500 mb-4"
          >
            <ArrowLeft className="h-5 w-5 mr-2" />
            Back to Dashboard
          </Link>
          <h1 className="text-3xl font-bold text-gray-900">Create New Job</h1>
          <p className="text-gray-600">Enter job details to calculate costs and create a quote</p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Form */}
          <div className="lg:col-span-2">
            <form onSubmit={handleSubmit} className="space-y-6">
              <div className="bg-white shadow rounded-lg p-6">
                <h2 className="text-lg font-medium text-gray-900 mb-4">Job Information</h2>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Job Title
                    </label>
                    <input
                      type="text"
                      required
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.title}
                      onChange={(e) => handleInputChange('title', e.target.value)}
                      placeholder="Enter job title"
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Job Type
                    </label>
                    <select
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.jobType}
                      onChange={(e) => handleInputChange('jobType', e.target.value)}
                    >
                      {jobTypes.map(type => (
                        <option key={type.value} value={type.value}>
                          {type.label}
                        </option>
                      ))}
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Quantity
                    </label>
                    <input
                      type="number"
                      min="1"
                      required
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.quantity || ''}
                      onChange={(e) => handleInputChange('quantity', parseInt(e.target.value) || 0)}
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      <DollarSign className="h-4 w-4 inline mr-1" />
                      Currency
                    </label>
                    <select
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={selectedCurrency}
                      onChange={(e) => setSelectedCurrency(e.target.value as Currency)}
                    >
                      {supportedCurrencies.map(currency => (
                        <option key={currency} value={currency}>
                          {CURRENCY_INFO[currency]?.symbol} {CURRENCY_INFO[currency]?.name}
                        </option>
                      ))}
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Pages
                    </label>
                    <input
                      type="number"
                      min="1"
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.pages || ''}
                      onChange={(e) => handleInputChange('specifications.pages', parseInt(e.target.value) || 1)}
                    />
                  </div>
                </div>
              </div>

              <div className="bg-white shadow rounded-lg p-6">
                <h2 className="text-lg font-medium text-gray-900 mb-4">Paper Specifications</h2>
                
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Paper Size
                    </label>
                    <select
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.paperSize}
                      onChange={(e) => handleInputChange('specifications.paperSize', e.target.value)}
                    >
                      {paperSizes.map(size => (
                        <option key={size} value={size}>{size}</option>
                      ))}
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Paper Type
                    </label>
                    <select
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.paperType}
                      onChange={(e) => handleInputChange('specifications.paperType', e.target.value)}
                    >
                      {paperTypes.map(type => (
                        <option key={type} value={type}>
                          {type.replace('_', ' ').toUpperCase()}
                        </option>
                      ))}
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Paper Weight
                    </label>
                    <input
                      type="text"
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.paperWeight || ''}
                      onChange={(e) => handleInputChange('specifications.paperWeight', e.target.value)}
                      placeholder="e.g., 80gsm"
                    />
                  </div>
                </div>
              </div>

              <div className="bg-white shadow rounded-lg p-6">
                <h2 className="text-lg font-medium text-gray-900 mb-4">Colors</h2>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Front Colors
                    </label>
                    <input
                      type="number"
                      min="0"
                      max="8"
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.colors.frontColors || ''}
                      onChange={(e) => handleInputChange('specifications.colors.frontColors', parseInt(e.target.value) || 0)}
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Back Colors
                    </label>
                    <input
                      type="number"
                      min="0"
                      max="8"
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.colors.backColors || ''}
                      onChange={(e) => handleInputChange('specifications.colors.backColors', parseInt(e.target.value) || 0)}
                    />
                  </div>
                </div>

                <div className="mt-4">
                  <label className="flex items-center">
                    <input
                      type="checkbox"
                      className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      checked={formData.specifications.colors.isFullColor}
                      onChange={(e) => handleInputChange('specifications.colors.isFullColor', e.target.checked)}
                    />
                    <span className="ml-2 text-sm text-gray-700">Full Color (CMYK)</span>
                  </label>
                </div>
              </div>

              <div className="bg-white shadow rounded-lg p-6">
                <h2 className="text-lg font-medium text-gray-900 mb-4">Finishing Options</h2>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Binding
                    </label>
                    <select
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.binding || ''}
                      onChange={(e) => handleInputChange('specifications.binding', e.target.value)}
                    >
                      {bindingOptions.map(option => (
                        <option key={option} value={option}>
                          {option ? option.replace('_', ' ').toUpperCase() : 'None'}
                        </option>
                      ))}
                    </select>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Lamination
                    </label>
                    <select
                      className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={formData.specifications.lamination || ''}
                      onChange={(e) => handleInputChange('specifications.lamination', e.target.value)}
                    >
                      {laminationOptions.map(option => (
                        <option key={option} value={option}>
                          {option ? option.charAt(0).toUpperCase() + option.slice(1) : 'None'}
                        </option>
                      ))}
                    </select>
                  </div>
                </div>

                <div className="mt-4">
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Additional Finishing
                  </label>
                  <div className="grid grid-cols-2 md:grid-cols-3 gap-2">
                    {finishingOptions.map(option => (
                      <label key={option} className="flex items-center">
                        <input
                          type="checkbox"
                          className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                          checked={formData.specifications.finishing.includes(option)}
                          onChange={(e) => handleFinishingChange(option, e.target.checked)}
                        />
                        <span className="ml-2 text-sm text-gray-700 capitalize">
                          {option}
                        </span>
                      </label>
                    ))}
                  </div>
                </div>

                <div className="mt-4">
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Special Requirements
                  </label>
                  <textarea
                    rows={3}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={formData.specifications.specialRequirements || ''}
                    onChange={(e) => handleInputChange('specifications.specialRequirements', e.target.value)}
                    placeholder="Any special requirements or notes..."
                  />
                </div>
              </div>

              {error && (
                <div className="bg-red-50 border border-red-200 rounded-md p-4">
                  <p className="text-red-700 text-sm">{error}</p>
                </div>
              )}

              <div className="flex flex-wrap gap-3">
                <button
                  type="button"
                  onClick={quickCalculate}
                  disabled={calculating}
                  className="flex items-center px-4 py-2 border border-orange-500 text-orange-600 rounded-md hover:bg-orange-50 disabled:opacity-50"
                >
                  <Zap className="h-5 w-5 mr-2" />
                  {calculating ? 'Calculating...' : 'Quick Calc'}
                </button>

                <button
                  type="button"
                  onClick={calculateCost}
                  disabled={calculating}
                  className="flex items-center px-4 py-2 border border-blue-600 text-blue-600 rounded-md hover:bg-blue-50 disabled:opacity-50"
                >
                  <Calculator className="h-5 w-5 mr-2" />
                  {calculating ? 'Calculating...' : 'Detailed Calc'}
                </button>

                <button
                  type="submit"
                  disabled={loading || !costCalculation}
                  className="flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50"
                >
                  <Save className="h-5 w-5 mr-2" />
                  {loading ? 'Creating...' : 'Create Job'}
                </button>
              </div>
            </form>
          </div>

          {/* Cost Calculation */}
          <div className="lg:col-span-1">
            <div className="bg-white shadow rounded-lg p-6 sticky top-8">
              <h2 className="text-lg font-medium text-gray-900 mb-4">Cost Calculation</h2>
              
              {!costCalculation ? (
                <div className="text-center py-8">
                  <Calculator className="mx-auto h-12 w-12 text-gray-400 mb-4" />
                  <p className="text-gray-500">Use &quot;Quick Calc&quot; for fast estimates or &quot;Detailed Calc&quot; for precise pricing</p>
                </div>
              ) : (
                <div className="space-y-4">
                  <div className="border-b pb-4">
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-sm text-gray-600">Paper Cost:</span>
                      <span className="font-medium">{formatCurrency(costCalculation.costBreakdown.paperCost, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-sm text-gray-600">Plate Cost:</span>
                      <span className="font-medium">{formatCurrency(costCalculation.costBreakdown.plateCost, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-sm text-gray-600">Labor Cost:</span>
                      <span className="font-medium">{formatCurrency(costCalculation.costBreakdown.laborCost, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-sm text-gray-600">Binding Cost:</span>
                      <span className="font-medium">{formatCurrency(costCalculation.costBreakdown.bindingCost, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-sm text-gray-600">Finishing Cost:</span>
                      <span className="font-medium">{formatCurrency(costCalculation.costBreakdown.finishingCost, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                    <div className="flex justify-between items-center mb-2">
                      <span className="text-sm text-gray-600">Overhead:</span>
                      <span className="font-medium">{formatCurrency(costCalculation.costBreakdown.overhead, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    <div className="flex justify-between items-center text-lg font-semibold">
                      <span>Total Cost:</span>
                      <span className="text-blue-600">{formatCurrency(costCalculation.totalCost, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-sm text-gray-500">Per Unit:</span>
                      <span className="font-medium">{formatCurrency(costCalculation.unitCost, costCalculation.currency || selectedCurrency)}</span>
                    </div>
                    {costCalculation.exchangeRate && costCalculation.exchangeRate !== 1 && (
                      <div className="flex justify-between items-center">
                        <span className="text-xs text-gray-500">Exchange Rate:</span>
                        <span className="text-xs text-gray-500">1 USD = {costCalculation.exchangeRate} {costCalculation.currency}</span>
                      </div>
                    )}
                    <div className="flex justify-between items-center">
                      <span className="text-sm text-gray-600">Delivery:</span>
                      <span className="font-medium">{costCalculation.estimatedDeliveryDays} days</span>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
