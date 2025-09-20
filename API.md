# CostPrint Pro API Documentation

This document provides comprehensive documentation for the CostPrint Pro REST API.

## Table of Contents

- [Overview](#overview)
- [Authentication](#authentication)
- [Base URL and Versioning](#base-url-and-versioning)
- [Request/Response Format](#requestresponse-format)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [Endpoints](#endpoints)
- [Data Models](#data-models)
- [Examples](#examples)

## Overview

The CostPrint Pro API is a RESTful web service that provides programmatic access to print job costing, management, and configuration functionality. The API uses JSON for data exchange and follows standard HTTP methods and status codes.

### Key Features

- Print job creation and management
- Real-time cost calculations
- Multi-currency support with live conversion
- Settings and configuration management
- User authentication and authorization
- Comprehensive error handling and validation

## Authentication

The API uses JWT (JSON Web Token) based authentication. All protected endpoints require a valid JWT token in the Authorization header.

### Obtaining a Token

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "name": "John Doe",
      "role": "user"
    }
  },
  "message": "Login successful"
}
```

### Using the Token

Include the token in the Authorization header for all protected requests:

```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### Token Expiration

Tokens expire after 24 hours by default. When a token expires, you'll receive a 401 Unauthorized response and need to obtain a new token.

## Base URL and Versioning

- **Development**: `http://localhost:8080`
- **Production**: `https://api.costprint.com`

All API endpoints are prefixed with `/api` for the current version.

## Request/Response Format

### Content Type

All requests and responses use JSON format:
- **Request**: `Content-Type: application/json`
- **Response**: `Content-Type: application/json`

### Standard Response Format

All API responses follow a consistent structure:

#### Success Response
```json
{
  "success": true,
  "data": { ... },
  "message": "Operation completed successfully"
}
```

#### Error Response
```json
{
  "success": false,
  "error": "Error message",
  "code": "ERROR_CODE",
  "details": { ... }
}
```

### HTTP Status Codes

| Code | Description | Usage |
|------|-------------|-------|
| 200 | OK | Successful GET, PUT, DELETE |
| 201 | Created | Successful POST |
| 400 | Bad Request | Invalid request data |
| 401 | Unauthorized | Missing or invalid authentication |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 422 | Unprocessable Entity | Validation errors |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |

## Error Handling

### Error Response Structure

```json
{
  "success": false,
  "error": "Human-readable error message",
  "code": "MACHINE_READABLE_CODE",
  "details": {
    "field": "Additional error details",
    "validation_errors": [
      {
        "field": "email",
        "message": "Invalid email format"
      }
    ]
  }
}
```

### Common Error Codes

| Code | Description |
|------|-------------|
| `INVALID_CREDENTIALS` | Login credentials are incorrect |
| `TOKEN_EXPIRED` | JWT token has expired |
| `VALIDATION_ERROR` | Request data validation failed |
| `RESOURCE_NOT_FOUND` | Requested resource doesn't exist |
| `INSUFFICIENT_PERMISSIONS` | User lacks required permissions |
| `RATE_LIMIT_EXCEEDED` | Too many requests |

## Rate Limiting

The API implements rate limiting to ensure fair usage:

- **Authenticated users**: 1000 requests per hour
- **Unauthenticated users**: 100 requests per hour

Rate limit headers are included in responses:
```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

## Endpoints

### Authentication

#### Login
```http
POST /api/auth/login
```

Authenticate a user and receive a JWT token.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "token": "jwt_token_here",
    "user": {
      "id": "user_id",
      "email": "user@example.com",
      "name": "John Doe",
      "role": "user"
    }
  }
}
```

#### Logout
```http
POST /api/auth/logout
Authorization: Bearer {token}
```

Invalidate the current JWT token.

#### Get Current User
```http
GET /api/auth/me
Authorization: Bearer {token}
```

Get information about the currently authenticated user.

### Jobs

#### List Jobs
```http
GET /api/jobs?page=1&limit=10&search=query&status=active
Authorization: Bearer {token}
```

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 10, max: 100)
- `search` (optional): Search term for job title
- `status` (optional): Filter by job status
- `job_type` (optional): Filter by job type

**Response:**
```json
{
  "success": true,
  "data": {
    "jobs": [
      {
        "id": "job_id",
        "title": "Business Cards",
        "job_type": "BusinessCard",
        "quantity": 500,
        "status": "active",
        "created_at": "2024-09-20T10:00:00Z",
        "updated_at": "2024-09-20T10:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 10,
      "total": 25,
      "pages": 3
    }
  }
}
```

#### Create Job
```http
POST /api/jobs
Authorization: Bearer {token}
Content-Type: application/json
```

**Request Body:**
```json
{
  "title": "Business Cards for ABC Corp",
  "job_type": "BusinessCard",
  "quantity": 500,
  "specifications": {
    "paper_type": "250gsm_card",
    "paper_size": "A4",
    "paper_weight": "250gsm",
    "colors": {
      "front_colors": 4,
      "back_colors": 4,
      "spot_colors": [],
      "is_full_color": true
    },
    "pages": 1,
    "binding": "",
    "lamination": "gloss",
    "finishing": ["cutting"],
    "special_requirements": "Double-sided printing with bleed"
  }
}
```

#### Get Job
```http
GET /api/jobs/{job_id}
Authorization: Bearer {token}
```

#### Update Job
```http
PUT /api/jobs/{job_id}
Authorization: Bearer {token}
Content-Type: application/json
```

#### Delete Job
```http
DELETE /api/jobs/{job_id}
Authorization: Bearer {token}
```

### Cost Calculations

#### Quick Calculation
```http
POST /api/cost/quick
Authorization: Bearer {token}
Content-Type: application/json
```

Perform a quick cost calculation without creating a job.

**Request Body:**
```json
{
  "job_type": "Flyer",
  "quantity": 1000,
  "specifications": {
    "paper_type": "80gsm_offset",
    "paper_size": "A4",
    "colors": {
      "front_colors": 4,
      "back_colors": 0,
      "is_full_color": true
    }
  },
  "currency": "USD"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "total_cost": 125.50,
    "unit_cost": 0.13,
    "currency": "USD",
    "breakdown": {
      "paper_cost": 45.00,
      "plate_cost": 25.00,
      "labor_cost": 35.50,
      "finishing_cost": 10.00,
      "overhead": 10.00
    },
    "calculation_time": "2024-09-20T10:00:00Z"
  }
}
```

#### Detailed Calculation
```http
POST /api/cost/calculate
Authorization: Bearer {token}
Content-Type: application/json
```

Perform a detailed cost calculation with comprehensive breakdown.

### Currency Operations

#### Get Supported Currencies
```http
GET /api/currency/supported
```

**Response:**
```json
{
  "success": true,
  "data": {
    "currencies": [
      {
        "code": "USD",
        "name": "US Dollar",
        "symbol": "$"
      },
      {
        "code": "FCFA",
        "name": "Central African CFA Franc",
        "symbol": "FCFA"
      },
      {
        "code": "EUR",
        "name": "Euro",
        "symbol": "€"
      }
    ]
  }
}
```

#### Get Exchange Rates
```http
GET /api/currency/rates?base=USD
```

#### Convert Currency
```http
GET /api/currency/convert?from=USD&to=FCFA&amount=100
```

### Settings

#### Get Cost Parameters
```http
GET /api/settings/cost-parameters
Authorization: Bearer {token}
```

#### Update Cost Parameters
```http
PUT /api/settings/cost-parameters
Authorization: Bearer {token}
Content-Type: application/json
```

**Request Body:**
```json
{
  "paper_costs": {
    "80gsm_offset": 0.05,
    "250gsm_card": 0.15
  },
  "labor_rates": {
    "setup_time": 30.00,
    "run_rate": 0.02
  },
  "overhead_percentage": 15.0,
  "profit_margin": 25.0
}
```

#### Get Branding Settings
```http
GET /api/settings/branding
Authorization: Bearer {token}
```

#### Update Branding Settings
```http
PUT /api/settings/branding
Authorization: Bearer {token}
Content-Type: application/json
```

### Health Check

#### API Health
```http
GET /health
```

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "1.0.0",
    "timestamp": "2024-09-20T10:00:00Z",
    "services": {
      "database": "connected",
      "redis": "connected"
    }
  }
}
```

## Data Models

### Job Specification
```json
{
  "id": "string (UUID)",
  "title": "string",
  "job_type": "enum (Book|Flyer|BusinessCard|Brochure|Poster|Banner|Sticker|Custom)",
  "quantity": "integer",
  "status": "enum (draft|active|completed|cancelled)",
  "specifications": {
    "paper_type": "string",
    "paper_size": "string",
    "paper_weight": "string",
    "colors": {
      "front_colors": "integer",
      "back_colors": "integer",
      "spot_colors": "array of strings",
      "is_full_color": "boolean"
    },
    "pages": "integer",
    "binding": "string",
    "lamination": "string",
    "finishing": "array of strings",
    "special_requirements": "string"
  },
  "created_at": "string (ISO 8601)",
  "updated_at": "string (ISO 8601)"
}
```

### Cost Breakdown
```json
{
  "total_cost": "number",
  "unit_cost": "number",
  "currency": "string",
  "breakdown": {
    "paper_cost": "number",
    "plate_cost": "number",
    "labor_cost": "number",
    "binding_cost": "number",
    "finishing_cost": "number",
    "overhead": "number",
    "profit_margin": "number"
  },
  "details": {
    "sheets_needed": "integer",
    "impressions": "integer",
    "setup_time": "number",
    "run_time": "number"
  }
}
```

### User
```json
{
  "id": "string (UUID)",
  "email": "string",
  "name": "string",
  "role": "enum (admin|manager|user)",
  "created_at": "string (ISO 8601)",
  "updated_at": "string (ISO 8601)"
}
```

## Examples

### Complete Job Creation Workflow

1. **Authenticate**:
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'
```

2. **Create Job**:
```bash
curl -X POST http://localhost:8080/api/jobs \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "title": "Marketing Flyers",
    "job_type": "Flyer",
    "quantity": 1000,
    "specifications": {
      "paper_type": "80gsm_offset",
      "paper_size": "A4",
      "colors": {"front_colors": 4, "back_colors": 0, "is_full_color": true}
    }
  }'
```

3. **Get Cost Calculation**:
```bash
curl -X POST http://localhost:8080/api/cost/calculate \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "job_type": "Flyer",
    "quantity": 1000,
    "specifications": {
      "paper_type": "80gsm_offset",
      "paper_size": "A4",
      "colors": {"front_colors": 4, "back_colors": 0, "is_full_color": true}
    }
  }'
```

### Multi-Currency Calculation

```bash
curl -X POST http://localhost:8080/api/cost/quick \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "job_type": "BusinessCard",
    "quantity": 500,
    "specifications": {
      "paper_type": "250gsm_card",
      "colors": {"front_colors": 4, "back_colors": 4, "is_full_color": true}
    },
    "currency": "FCFA"
  }'
```

## SDK and Libraries

While we don't currently provide official SDKs, the API is designed to be easily consumed by any HTTP client. Community SDKs may be available for popular programming languages.

## Webhooks

Webhook support is planned for future releases to enable real-time notifications for job status changes and other events.

## Changelog

API changes will be documented in the main CHANGELOG.md file with specific attention to breaking changes and new endpoints.

---

**API Version**: 1.0.0  
**Last Updated**: September 20, 2024  
**Support**: For API support, please create an issue at https://github.com/onelrian/costprintpro/issues
