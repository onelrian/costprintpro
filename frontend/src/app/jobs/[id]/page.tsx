'use client';

import { useState, useEffect } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { useAuth } from '@/contexts/AuthContext';
import { jobsApi } from '@/lib/api';
import type { Job } from '@/types';
import { ArrowLeft, Calendar, DollarSign, Package, User, Edit, Trash2 } from 'lucide-react';
import { formatCurrency, CURRENCY_INFO } from '@/lib/currency';
import Link from 'next/link';

export default function JobDetailPage() {
  const { user } = useAuth();
  const params = useParams();
  const router = useRouter();
  const jobId = params.id as string;
  
  const [job, setJob] = useState<Job | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    const fetchJob = async () => {
      if (!jobId) return;
      
      try {
        setLoading(true);
        const jobData = await jobsApi.getById(jobId);
        setJob(jobData);
      } catch (error: unknown) {
        console.error('Failed to fetch job:', error);
        const errorMessage = error instanceof Error ? error.message : 'Failed to load job details';
        setError(errorMessage);
      } finally {
        setLoading(false);
      }
    };

    fetchJob();
  }, [jobId]);

  const handleDelete = async () => {
    if (!job || !confirm('Are you sure you want to delete this job?')) return;
    
    try {
      await jobsApi.delete(job.id);
      router.push('/');
    } catch (error: unknown) {
      console.error('Failed to delete job:', error);
      setError('Failed to delete job. Please try again.');
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
          <p className="mt-4 text-gray-600">Loading job details...</p>
        </div>
      </div>
    );
  }

  if (error || !job) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="bg-red-50 border border-red-200 rounded-lg p-6 max-w-md">
            <h2 className="text-lg font-semibold text-red-800 mb-2">Error Loading Job</h2>
            <p className="text-red-600 mb-4">{error || 'Job not found'}</p>
            <Link
              href="/"
              className="inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
            >
              <ArrowLeft className="h-4 w-4 mr-2" />
              Back to Dashboard
            </Link>
          </div>
        </div>
      </div>
    );
  }

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'draft': return 'bg-gray-100 text-gray-800';
      case 'quoted': return 'bg-blue-100 text-blue-800';
      case 'approved': return 'bg-green-100 text-green-800';
      case 'in_production': return 'bg-yellow-100 text-yellow-800';
      case 'completed': return 'bg-purple-100 text-purple-800';
      case 'cancelled': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
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
            <ArrowLeft className="h-4 w-4 mr-2" />
            Back to Dashboard
          </Link>
          
          <div className="flex justify-between items-start">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">{job.title}</h1>
              <div className="mt-2 flex items-center space-x-4">
                <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(job.status)}`}>
                  {job.status}
                </span>
                <span className="text-sm text-gray-500">Job #{job.id.slice(0, 8)}</span>
              </div>
            </div>
            
            <div className="flex space-x-2">
              <button
                onClick={() => router.push(`/jobs/${job.id}/edit`)}
                className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
              >
                <Edit className="h-4 w-4 mr-2" />
                Edit
              </button>
              <button
                onClick={handleDelete}
                className="inline-flex items-center px-3 py-2 border border-red-300 shadow-sm text-sm leading-4 font-medium rounded-md text-red-700 bg-white hover:bg-red-50"
              >
                <Trash2 className="h-4 w-4 mr-2" />
                Delete
              </button>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Job Details */}
          <div className="lg:col-span-2 space-y-6">
            {/* Basic Information */}
            <div className="bg-white shadow rounded-lg p-6">
              <h2 className="text-lg font-medium text-gray-900 mb-4">Job Information</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-500">Job Type</label>
                  <p className="mt-1 text-sm text-gray-900">{job.jobType}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Quantity</label>
                  <p className="mt-1 text-sm text-gray-900">{job.quantity.toLocaleString()}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Paper Type</label>
                  <p className="mt-1 text-sm text-gray-900">{job.specifications.paperType}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Paper Size</label>
                  <p className="mt-1 text-sm text-gray-900">{job.specifications.paperSize}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Pages</label>
                  <p className="mt-1 text-sm text-gray-900">{job.specifications.pages || 1}</p>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-500">Colors</label>
                  <p className="mt-1 text-sm text-gray-900">
                    Front: {job.specifications.colors.frontColors}, Back: {job.specifications.colors.backColors}
                  </p>
                </div>
                {job.specifications.binding && (
                  <div>
                    <label className="block text-sm font-medium text-gray-500">Binding</label>
                    <p className="mt-1 text-sm text-gray-900">{job.specifications.binding}</p>
                  </div>
                )}
                {job.specifications.lamination && (
                  <div>
                    <label className="block text-sm font-medium text-gray-500">Lamination</label>
                    <p className="mt-1 text-sm text-gray-900">{job.specifications.lamination}</p>
                  </div>
                )}
              </div>
              
              {job.specifications.finishing && job.specifications.finishing.length > 0 && (
                <div className="mt-4">
                  <label className="block text-sm font-medium text-gray-500">Finishing</label>
                  <div className="mt-1 flex flex-wrap gap-2">
                    {job.specifications.finishing.map((finish, index) => (
                      <span
                        key={index}
                        className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                      >
                        {finish}
                      </span>
                    ))}
                  </div>
                </div>
              )}
              
              {job.specifications.specialRequirements && (
                <div className="mt-4">
                  <label className="block text-sm font-medium text-gray-500">Special Requirements</label>
                  <p className="mt-1 text-sm text-gray-900">{job.specifications.specialRequirements}</p>
                </div>
              )}
            </div>

            {/* Timeline */}
            <div className="bg-white shadow rounded-lg p-6">
              <h2 className="text-lg font-medium text-gray-900 mb-4">Timeline</h2>
              <div className="space-y-3">
                <div className="flex items-center">
                  <Calendar className="h-5 w-5 text-gray-400 mr-3" />
                  <div>
                    <p className="text-sm font-medium text-gray-900">Created</p>
                    <p className="text-sm text-gray-500">{formatDate(job.createdAt)}</p>
                  </div>
                </div>
                <div className="flex items-center">
                  <Calendar className="h-5 w-5 text-gray-400 mr-3" />
                  <div>
                    <p className="text-sm font-medium text-gray-900">Last Updated</p>
                    <p className="text-sm text-gray-500">{formatDate(job.updatedAt)}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Cost Breakdown */}
          <div className="lg:col-span-1">
            <div className="bg-white shadow rounded-lg p-6 sticky top-8">
              <h2 className="text-lg font-medium text-gray-900 mb-4">Cost Breakdown</h2>
              
              <div className="space-y-3">
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Paper Cost:</span>
                  <span className="font-medium">{formatCurrency(parseFloat(job.costBreakdown.paperCost.toString()), 'USD')}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Plate Cost:</span>
                  <span className="font-medium">{formatCurrency(parseFloat(job.costBreakdown.plateCost.toString()), 'USD')}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Labor Cost:</span>
                  <span className="font-medium">{formatCurrency(parseFloat(job.costBreakdown.laborCost.toString()), 'USD')}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Binding Cost:</span>
                  <span className="font-medium">{formatCurrency(parseFloat(job.costBreakdown.bindingCost.toString()), 'USD')}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Finishing Cost:</span>
                  <span className="font-medium">{formatCurrency(parseFloat(job.costBreakdown.finishingCost.toString()), 'USD')}</span>
                </div>
                <div className="flex justify-between items-center">
                  <span className="text-sm text-gray-600">Overhead:</span>
                  <span className="font-medium">{formatCurrency(parseFloat(job.costBreakdown.overhead.toString()), 'USD')}</span>
                </div>
              </div>
              
              <div className="border-t pt-4 mt-4">
                <div className="flex justify-between items-center text-lg font-semibold">
                  <span>Total Cost:</span>
                  <span className="text-blue-600">{formatCurrency(parseFloat(job.totalCost.toString()), 'USD')}</span>
                </div>
                <div className="flex justify-between items-center mt-2">
                  <span className="text-sm text-gray-500">Per Unit:</span>
                  <span className="font-medium">{formatCurrency(parseFloat(job.unitCost.toString()), 'USD')}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
