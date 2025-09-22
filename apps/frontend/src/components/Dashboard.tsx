'use client';

import { useState, useEffect } from 'react';
import { useAuth } from '@/contexts/AuthContext';
import { jobsApi } from '@/lib/api';
import type { Job, JobListResponse } from '@/types';
import { Calculator, Settings, FileText, Plus } from 'lucide-react';
import Link from 'next/link';

export default function Dashboard() {
  const { user, logout } = useAuth();
  const [jobs, setJobs] = useState<Job[]>([]);
  const [loading, setLoading] = useState(true);
  const [stats, setStats] = useState({
    totalJobs: 0,
    totalValue: 0,
    avgJobValue: 0,
    recentJobs: 0,
  });

  useEffect(() => {
    const fetchDashboardData = async () => {
      try {
        const response: JobListResponse = await jobsApi.list({ limit: 10 });
        const jobsData = response?.jobs || [];
        setJobs(jobsData);
        
        // Calculate stats
        const totalValue = jobsData.reduce((sum, job) => sum + (job.totalCost || 0), 0);
        setStats({
          totalJobs: response?.total || 0,
          totalValue,
          avgJobValue: jobsData.length > 0 ? totalValue / jobsData.length : 0,
          recentJobs: jobsData.length,
        });
      } catch (error) {
        console.error('Failed to fetch dashboard data:', error);
        // Set empty state on error
        setJobs([]);
        setStats({
          totalJobs: 0,
          totalValue: 0,
          avgJobValue: 0,
          recentJobs: 0,
        });
      } finally {
        setLoading(false);
      }
    };

    fetchDashboardData();
  }, []);

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(amount);
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString();
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Draft': return 'bg-gray-100 text-gray-800';
      case 'Quoted': return 'bg-blue-100 text-blue-800';
      case 'Approved': return 'bg-green-100 text-green-800';
      case 'InProduction': return 'bg-yellow-100 text-yellow-800';
      case 'Completed': return 'bg-emerald-100 text-emerald-800';
      case 'Cancelled': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">CostPrint Pro</h1>
              <p className="text-gray-600">Welcome back, {user?.name || user?.email}</p>
            </div>
            <div className="flex items-center space-x-4">
              <Link
                href="/settings"
                className="text-gray-500 hover:text-gray-700 flex items-center space-x-2"
              >
                <Settings className="h-5 w-5" />
                <span>Settings</span>
              </Link>
              <Link
                href="/jobs/new"
                className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md flex items-center space-x-2"
              >
                <Plus className="h-5 w-5" />
                <span>New Job</span>
              </Link>
              <button
                onClick={logout}
                className="text-gray-500 hover:text-gray-700"
              >
                Logout
              </button>
            </div>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <FileText className="h-6 w-6 text-gray-400" />
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Total Jobs</dt>
                    <dd className="text-lg font-medium text-gray-900">{stats.totalJobs}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <Calculator className="h-6 w-6 text-gray-400" />
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Total Value</dt>
                    <dd className="text-lg font-medium text-gray-900">{formatCurrency(stats.totalValue)}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <Calculator className="h-6 w-6 text-gray-400" />
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Avg Job Value</dt>
                    <dd className="text-lg font-medium text-gray-900">{formatCurrency(stats.avgJobValue)}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white overflow-hidden shadow rounded-lg">
            <div className="p-5">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <FileText className="h-6 w-6 text-gray-400" />
                </div>
                <div className="ml-5 w-0 flex-1">
                  <dl>
                    <dt className="text-sm font-medium text-gray-500 truncate">Recent Jobs</dt>
                    <dd className="text-lg font-medium text-gray-900">{stats.recentJobs}</dd>
                  </dl>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
          <Link
            href="/jobs/new"
            className="bg-white p-6 rounded-lg shadow hover:shadow-md transition-shadow"
          >
            <div className="flex items-center">
              <Plus className="h-8 w-8 text-blue-600" />
              <div className="ml-4">
                <h3 className="text-lg font-medium text-gray-900">Create New Job</h3>
                <p className="text-gray-500">Start a new print job calculation</p>
              </div>
            </div>
          </Link>

          <Link
            href="/jobs"
            className="bg-white p-6 rounded-lg shadow hover:shadow-md transition-shadow"
          >
            <div className="flex items-center">
              <FileText className="h-8 w-8 text-green-600" />
              <div className="ml-4">
                <h3 className="text-lg font-medium text-gray-900">View All Jobs</h3>
                <p className="text-gray-500">Browse and manage your jobs</p>
              </div>
            </div>
          </Link>

          <Link
            href="/settings"
            className="bg-white p-6 rounded-lg shadow hover:shadow-md transition-shadow"
          >
            <div className="flex items-center">
              <Settings className="h-8 w-8 text-purple-600" />
              <div className="ml-4">
                <h3 className="text-lg font-medium text-gray-900">Settings</h3>
                <p className="text-gray-500">Manage cost parameters</p>
              </div>
            </div>
          </Link>
        </div>

        {/* Recent Jobs */}
        <div className="bg-white shadow rounded-lg">
          <div className="px-4 py-5 sm:p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg leading-6 font-medium text-gray-900">Recent Jobs</h3>
              <Link
                href="/jobs"
                className="text-blue-600 hover:text-blue-500 text-sm font-medium"
              >
                View all
              </Link>
            </div>
            
            {jobs && jobs.length === 0 ? (
              <div className="text-center py-8">
                <FileText className="mx-auto h-12 w-12 text-gray-400" />
                <h3 className="mt-2 text-sm font-medium text-gray-900">No jobs yet</h3>
                <p className="mt-1 text-sm text-gray-500">Get started by creating your first job.</p>
                <div className="mt-6">
                  <Link
                    href="/jobs/new"
                    className="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                  >
                    <Plus className="-ml-1 mr-2 h-5 w-5" />
                    New Job
                  </Link>
                </div>
              </div>
            ) : (
              <div className="overflow-hidden">
                <table className="min-w-full divide-y divide-gray-200">
                  <thead className="bg-gray-50">
                    <tr>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Job
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Type
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Quantity
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Total Cost
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Status
                      </th>
                      <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Date
                      </th>
                    </tr>
                  </thead>
                  <tbody className="bg-white divide-y divide-gray-200">
                    {jobs.map((job) => (
                      <tr key={job.id} className="hover:bg-gray-50">
                        <td className="px-6 py-4 whitespace-nowrap">
                          <Link
                            href={`/jobs/${job.id}`}
                            className="text-blue-600 hover:text-blue-500 font-medium"
                          >
                            {job.title}
                          </Link>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                          {job.jobType.replace('_', ' ')}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                          {job.quantity.toLocaleString()}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                          {formatCurrency(job.totalCost)}
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap">
                          <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${getStatusColor(job.status)}`}>
                            {job.status.replace('_', ' ')}
                          </span>
                        </td>
                        <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                          {formatDate(job.createdAt)}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </div>
      </main>
    </div>
  );
}
