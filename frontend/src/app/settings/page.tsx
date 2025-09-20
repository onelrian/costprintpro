'use client';

import { useState, useEffect } from 'react';
import { useAuth } from '@/contexts/AuthContext';
import { settingsApi } from '@/lib/api';
import type { CostParameters, BrandingSettings } from '@/types';
import { Settings, Save, ArrowLeft } from 'lucide-react';
import Link from 'next/link';

export default function SettingsPage() {
  const { user } = useAuth();
  const [costParameters, setCostParameters] = useState<CostParameters | null>(null);
  const [branding, setBranding] = useState<BrandingSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');

  useEffect(() => {
    const fetchSettings = async () => {
      try {
        const [costParams, brandingSettings] = await Promise.all([
          settingsApi.getCostParameters(),
          settingsApi.getBranding(),
        ]);
        setCostParameters(costParams);
        setBranding(brandingSettings);
      } catch (error) {
        console.error('Failed to fetch settings:', error);
        setError('Failed to load settings');
      } finally {
        setLoading(false);
      }
    };

    fetchSettings();
  }, []);

  const handleCostParametersChange = (field: keyof CostParameters, value: number) => {
    if (costParameters) {
      setCostParameters({
        ...costParameters,
        [field]: value,
      });
    }
  };

  const handleBrandingChange = (field: keyof BrandingSettings, value: string) => {
    if (branding) {
      setBranding({
        ...branding,
        [field]: value,
      });
    }
  };

  const saveCostParameters = async () => {
    if (!costParameters) return;

    setSaving(true);
    setError('');
    setSuccess('');

    try {
      const updated = await settingsApi.updateCostParameters(costParameters);
      setCostParameters(updated);
      setSuccess('Cost parameters saved successfully');
    } catch (error) {
      console.error('Failed to save cost parameters:', error);
      setError('Failed to save cost parameters');
    } finally {
      setSaving(false);
    }
  };

  const saveBranding = async () => {
    if (!branding) return;

    setSaving(true);
    setError('');
    setSuccess('');

    try {
      const updated = await settingsApi.updateBranding(branding);
      setBranding(updated);
      setSuccess('Branding settings saved successfully');
    } catch (error) {
      console.error('Failed to save branding:', error);
      setError('Failed to save branding settings');
    } finally {
      setSaving(false);
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
          <p className="mt-4 text-gray-600">Loading settings...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div className="flex items-center">
              <Link
                href="/"
                className="mr-4 inline-flex items-center px-3 py-2 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
              >
                <ArrowLeft className="h-4 w-4 mr-2" />
                Back to Dashboard
              </Link>
              <div className="flex items-center">
                <Settings className="h-8 w-8 text-blue-600 mr-3" />
                <div>
                  <h1 className="text-2xl font-bold text-gray-900">Settings</h1>
                  <p className="text-sm text-gray-500">Manage cost parameters and branding</p>
                </div>
              </div>
            </div>
            <div className="flex items-center space-x-2">
              <span className="text-sm text-gray-500">Welcome, {user?.name}</span>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {error && (
          <div className="mb-4 bg-red-50 border border-red-200 rounded-md p-4">
            <p className="text-red-700 text-sm">{error}</p>
          </div>
        )}

        {success && (
          <div className="mb-4 bg-green-50 border border-green-200 rounded-md p-4">
            <p className="text-green-700 text-sm">{success}</p>
          </div>
        )}

        <div className="space-y-6">
          {/* Cost Parameters */}
          <div className="bg-white shadow rounded-lg p-6">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-lg font-medium text-gray-900">Cost Parameters</h2>
              <button
                onClick={saveCostParameters}
                disabled={saving}
                className="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50"
              >
                <Save className="h-4 w-4 mr-2" />
                {saving ? 'Saving...' : 'Save Parameters'}
              </button>
            </div>

            {costParameters && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Paper Cost Per Sheet ($)
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={costParameters.paperCostPerSheet}
                    onChange={(e) => handleCostParametersChange('paperCostPerSheet', parseFloat(e.target.value) || 0)}
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Plate Cost Per Job ($)
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={costParameters.plateCostPerJob}
                    onChange={(e) => handleCostParametersChange('plateCostPerJob', parseFloat(e.target.value) || 0)}
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Labor Cost Per Hour ($)
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={costParameters.laborCostPerHour}
                    onChange={(e) => handleCostParametersChange('laborCostPerHour', parseFloat(e.target.value) || 0)}
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Binding Cost Per Unit ($)
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={costParameters.bindingCostPerUnit}
                    onChange={(e) => handleCostParametersChange('bindingCostPerUnit', parseFloat(e.target.value) || 0)}
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Overhead Percentage (%)
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    max="100"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={costParameters.overheadPercentage * 100}
                    onChange={(e) => handleCostParametersChange('overheadPercentage', (parseFloat(e.target.value) || 0) / 100)}
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Profit Margin Percentage (%)
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    min="0"
                    max="100"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={costParameters.profitMarginPercentage * 100}
                    onChange={(e) => handleCostParametersChange('profitMarginPercentage', (parseFloat(e.target.value) || 0) / 100)}
                  />
                </div>
              </div>
            )}
          </div>

          {/* Branding Settings */}
          <div className="bg-white shadow rounded-lg p-6">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-lg font-medium text-gray-900">Branding Settings</h2>
              <button
                onClick={saveBranding}
                disabled={saving}
                className="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50"
              >
                <Save className="h-4 w-4 mr-2" />
                {saving ? 'Saving...' : 'Save Branding'}
              </button>
            </div>

            {branding && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Company Name
                  </label>
                  <input
                    type="text"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={branding.companyName}
                    onChange={(e) => handleBrandingChange('companyName', e.target.value)}
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Company Logo URL
                  </label>
                  <input
                    type="url"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                    value={branding.companyLogoUrl || ''}
                    onChange={(e) => handleBrandingChange('companyLogoUrl', e.target.value)}
                    placeholder="https://example.com/logo.png"
                  />
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Primary Color
                  </label>
                  <div className="flex items-center space-x-2">
                    <input
                      type="color"
                      className="h-10 w-16 border border-gray-300 rounded-md"
                      value={branding.primaryColor}
                      onChange={(e) => handleBrandingChange('primaryColor', e.target.value)}
                    />
                    <input
                      type="text"
                      className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={branding.primaryColor}
                      onChange={(e) => handleBrandingChange('primaryColor', e.target.value)}
                      placeholder="#3B82F6"
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Secondary Color
                  </label>
                  <div className="flex items-center space-x-2">
                    <input
                      type="color"
                      className="h-10 w-16 border border-gray-300 rounded-md"
                      value={branding.secondaryColor}
                      onChange={(e) => handleBrandingChange('secondaryColor', e.target.value)}
                    />
                    <input
                      type="text"
                      className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-900 bg-white"
                      value={branding.secondaryColor}
                      onChange={(e) => handleBrandingChange('secondaryColor', e.target.value)}
                      placeholder="#1F2937"
                    />
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
