# CostPrint Pro

A comprehensive print job costing application built with modern web technologies for professional print shops and businesses.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Technology Stack](#technology-stack)
- [Features](#features)
- [Project Structure](#project-structure)
- [Installation](#installation)
- [Usage](#usage)
- [API Documentation](#api-documentation)
- [Development](#development)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [License](#license)

## Overview

CostPrint Pro is a full-stack web application designed for print shops and businesses to calculate accurate printing costs, manage jobs, and handle multi-currency pricing. The application provides real-time cost calculations, comprehensive job management, and detailed reporting capabilities with support for multiple currencies and complex printing specifications.

### Key Capabilities

- Real-time cost calculations with detailed breakdowns
- Multi-currency support with live exchange rates
- Comprehensive job lifecycle management
- Advanced printing specifications handling
- User authentication and authorization
- Configurable cost parameters and branding
- RESTful API with comprehensive endpoints
- Responsive web interface optimized for tablets and desktops

## Architecture

The application follows a modern microservices architecture with clear separation of concerns:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │    Backend      │    │   Database      │
│   (Next.js)     │◄──►│    (Rust)       │◄──►│ (PostgreSQL)    │
│   Port: 3000    │    │   Port: 8080    │    │   Port: 5432    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │     Redis       │
                    │   (Caching)     │
                    │   Port: 6379    │
                    └─────────────────┘
```

### Core Components

1. **Presentation Layer** - Next.js frontend with TypeScript and Tailwind CSS
2. **API Layer** - Rust backend with Axum framework providing RESTful services
3. **Business Logic** - Costing engine with complex calculation algorithms
4. **Data Layer** - PostgreSQL database with Redis caching
5. **Authentication** - JWT-based security with role-based access control
6. **Infrastructure** - Docker containerization with Docker Compose orchestration

## Technology Stack

### Frontend Technologies

| Technology | Version | Purpose |
|------------|---------|----------|
| Next.js | 15.5.3 | React framework with App Router |
| TypeScript | 5.x | Type-safe JavaScript development |
| Tailwind CSS | 3.x | Utility-first CSS framework |
| Lucide React | Latest | Modern icon library |
| Axios | 1.x | HTTP client for API communication |

### Backend Technologies

| Technology | Version | Purpose |
|------------|---------|----------|
| Rust | 1.75+ | Systems programming language |
| Axum | 0.7 | Modern async web framework |
| Tokio | 1.0 | Async runtime for Rust |
| SQLx | 0.7 | Async SQL toolkit |
| Serde | 1.0 | Serialization framework |
| JWT | Latest | JSON Web Tokens for authentication |

### Database & Infrastructure

| Technology | Version | Purpose |
|------------|---------|----------|
| PostgreSQL | 15 | Primary relational database |
| Redis | 7 | Caching and session storage |
| Docker | Latest | Application containerization |
| Docker Compose | 3.8 | Multi-container orchestration |

## Features

### Job Management

- **Create Jobs**: Define print jobs with detailed specifications
- **Update Jobs**: Modify existing jobs and recalculate costs
- **Delete Jobs**: Remove jobs with confirmation dialogs
- **Job History**: Track all job modifications and status changes
- **Search & Filter**: Find jobs by title, type, status, or date range
- **Pagination**: Handle large job lists efficiently

### Cost Calculation Engine

- **Real-time Calculations**: Instant cost updates as specifications change
- **Detailed Breakdowns**: Paper, labor, binding, finishing, and overhead costs
- **Volume Pricing**: Automatic tier-based pricing for quantity discounts
- **Quick Estimates**: Fast calculations without saving job data
- **Currency Conversion**: Live exchange rates for international pricing

### Print Job Types Supported

- **Books**: Perfect binding, saddle stitch, spiral binding
- **Flyers**: Single and multi-page promotional materials
- **Business Cards**: Standard and premium card stock options
- **Brochures**: Folded marketing materials with various finishes
- **Posters**: Large format printing with mounting options
- **Banners**: Outdoor and indoor signage materials
- **Stickers**: Die-cut labels and promotional stickers
- **Custom**: Flexible specifications for unique requirements

### Multi-Currency Support

- **Supported Currencies**: USD, FCFA, EUR, GBP, CAD
- **Live Exchange Rates**: Real-time currency conversion
- **Regional Pricing**: Localized cost parameters per currency
- **Currency Selection**: User preference storage and management

### User Management

- **Authentication**: Secure JWT-based login system
- **Authorization**: Role-based access control
- **User Profiles**: Customizable user information and preferences
- **Session Management**: Secure session handling with Redis

### Settings & Configuration

- **Cost Parameters**: Configurable pricing rules and multipliers
- **Branding**: Company logo, colors, and contact information
- **Currency Settings**: Exchange rate management and defaults
- **System Configuration**: Application-wide settings and preferences

## Project Structure

```
costprint1/
├── README.md                 # Main project documentation
├── DOCKER_SETUP.md          # Docker deployment guide
├── LICENSE                   # Project license
├── memory.md                 # Architectural documentation
├── progress.md               # Implementation progress tracking
├── backend/                  # Rust backend application
│   ├── Cargo.toml           # Rust dependencies and metadata
│   ├── Cargo.lock           # Locked dependency versions
│   └── src/                 # Source code directory
│       ├── main.rs          # Application entry point
│       ├── lib.rs           # Library root module
│       ├── handlers/        # HTTP request handlers
│       │   ├── mod.rs       # Handlers module definition
│       │   ├── auth.rs      # Authentication endpoints
│       │   ├── jobs.rs      # Job management endpoints
│       │   ├── costing.rs   # Cost calculation endpoints
│       │   ├── currency.rs  # Currency conversion endpoints
│       │   ├── settings.rs  # Settings management endpoints
│       │   └── export.rs    # PDF/Excel export endpoints
│       ├── models/          # Data models and types
│       │   ├── mod.rs       # Models module definition
│       │   ├── user.rs      # User data structures
│       │   ├── job.rs       # Job data structures
│       │   ├── currency.rs  # Currency data structures
│       │   └── settings.rs  # Settings data structures
│       ├── services/        # Business logic services
│       │   ├── mod.rs       # Services module definition
│       │   ├── auth_service.rs      # Authentication logic
│       │   ├── job_service.rs       # Job management logic
│       │   ├── costing_service.rs   # Cost calculation logic
│       │   ├── currency_service.rs  # Currency conversion logic
│       │   └── settings_service.rs  # Settings management logic
│       ├── utils/           # Utility functions and helpers
│       │   ├── mod.rs       # Utils module definition
│       │   ├── errors.rs    # Error handling utilities
│       │   ├── jwt.rs       # JWT token utilities
│       │   └── validation.rs # Input validation utilities
│       └── db/              # Database related code
│           ├── mod.rs       # Database module definition
│           └── migrations/  # Database migration files
├── frontend/                # Next.js frontend application
│   ├── package.json         # Node.js dependencies and scripts
│   ├── package-lock.json    # Locked dependency versions
│   ├── next.config.js       # Next.js configuration
│   ├── tailwind.config.js   # Tailwind CSS configuration
│   ├── tsconfig.json        # TypeScript configuration
│   ├── public/              # Static assets
│   │   ├── favicon.ico      # Application favicon
│   │   └── images/          # Image assets
│   └── src/                 # Source code directory
│       ├── app/             # Next.js App Router pages
│       │   ├── layout.tsx   # Root layout component
│       │   ├── page.tsx     # Home page component
│       │   ├── login/       # Authentication pages
│       │   ├── jobs/        # Job management pages
│       │   │   ├── page.tsx # Jobs list page
│       │   │   ├── new/     # New job creation
│       │   │   └── [id]/    # Job detail pages
│       │   └── settings/    # Settings pages
│       ├── components/      # Reusable React components
│       │   ├── Dashboard.tsx    # Main dashboard component
│       │   ├── JobForm.tsx      # Job creation/editing form
│       │   ├── CostBreakdown.tsx # Cost display component
│       │   └── LoadingSpinner.tsx # Loading indicator
│       ├── contexts/        # React context providers
│       │   └── AuthContext.tsx  # Authentication context
│       ├── lib/             # Utility libraries
│       │   ├── api.ts       # API client configuration
│       │   ├── currency.ts  # Currency utilities
│       │   └── utils.ts     # General utilities
│       └── types/           # TypeScript type definitions
│           └── index.ts     # Exported type definitions
├── docker/                  # Docker configuration files
│   ├── docker-compose.yml   # Production container orchestration
│   ├── docker-compose.dev.yml # Development container orchestration
│   ├── Dockerfile.backend   # Backend container definition
│   ├── Dockerfile.frontend  # Frontend container definition
│   ├── Dockerfile.backend.dev  # Development backend container
│   ├── Dockerfile.frontend.dev # Development frontend container
│   ├── start.sh            # Container startup script
│   ├── validate.sh         # System validation script
│   ├── .env.example        # Environment variables template
│   ├── init.sql            # Database initialization script
│   └── README.md           # Docker-specific documentation
└── infra/                  # Infrastructure and deployment
    └── ansible/            # Ansible playbooks (future implementation)
```

### Key Directories Explained

#### Backend (`/backend`)
The Rust backend follows a modular architecture with clear separation of concerns:

- **handlers/**: HTTP request handlers that process incoming API requests
- **models/**: Data structures and type definitions for the application
- **services/**: Business logic layer containing core application functionality
- **utils/**: Shared utilities for error handling, JWT processing, and validation
- **db/**: Database connection management and migration files

#### Frontend (`/frontend`)
The Next.js frontend uses the App Router pattern with TypeScript:

- **app/**: Page components using Next.js 13+ App Router
- **components/**: Reusable UI components for the application
- **contexts/**: React context providers for state management
- **lib/**: Client-side utilities and API configuration
- **types/**: TypeScript type definitions shared across the frontend

#### Docker (`/docker`)
Comprehensive containerization setup for both development and production:

- **docker-compose.yml**: Production-ready container orchestration
- **docker-compose.dev.yml**: Development environment with hot reload
- **Dockerfile.***: Container definitions for different environments
- **Scripts**: Automation tools for container management and validation

## Installation

### Prerequisites

Before installing CostPrint Pro, ensure you have the following software installed:

- **Docker** (version 20.0 or higher)
- **Docker Compose** (version 3.8 or higher)
- **Git** (for cloning the repository)

### Quick Start

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd costprint1
   ```

2. **Configure environment variables**:
   ```bash
   cp docker/.env.example docker/.env
   # Edit docker/.env with your configuration
   ```

3. **Start the application**:
   ```bash
   # Production environment
   ./docker/start.sh prod -d
   
   # Development environment (with hot reload)
   ./docker/start.sh dev
   ```

4. **Access the application**:
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080
   - API Documentation: http://localhost:8080/health

### Manual Installation

If you prefer to run the services individually:

#### Backend Setup

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Set up PostgreSQL and Redis**:
   ```bash
   # Using Docker
   docker run -d --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 postgres:15
   docker run -d --name redis -p 6379:6379 redis:7-alpine
   ```

3. **Configure environment**:
   ```bash
   export DATABASE_URL="postgresql://postgres:password@localhost:5432/costprint"
   export REDIS_URL="redis://localhost:6379"
   export JWT_SECRET="your-secret-key"
   export PORT=8080
   ```

4. **Run the backend**:
   ```bash
   cd backend
   cargo run
   ```

#### Frontend Setup

1. **Install Node.js** (version 18 or higher)

2. **Install dependencies**:
   ```bash
   cd frontend
   npm install
   ```

3. **Configure environment**:
   ```bash
   export NEXT_PUBLIC_API_URL="http://localhost:8080"
   ```

4. **Run the frontend**:
   ```bash
   npm run dev
   ```

## Usage

### Basic Workflow

1. **Authentication**: Log in with your credentials or create a new account
2. **Job Creation**: Create a new print job with detailed specifications
3. **Cost Calculation**: Use quick calculation for estimates or detailed calculation for precise pricing
4. **Job Management**: View, edit, and track your print jobs
5. **Settings Configuration**: Adjust cost parameters and branding settings

### Creating a Print Job

1. Navigate to the "New Job" page
2. Fill in the job details:
   - Job title and description
   - Print job type (Book, Flyer, Business Card, etc.)
   - Quantity required
   - Paper specifications (type, size, weight)
   - Color requirements (CMYK, spot colors)
   - Binding options (if applicable)
   - Finishing requirements (lamination, cutting, folding)
   - Special requirements or notes

3. Use the calculation options:
   - **Quick Calc**: Fast estimate for immediate pricing
   - **Detailed Calc**: Comprehensive calculation with full breakdown

4. Review the cost breakdown and save the job

### Multi-Currency Operations

1. Select your preferred currency from the dropdown menu
2. All calculations will automatically convert to the selected currency
3. Exchange rates are updated in real-time
4. Supported currencies: USD, FCFA, EUR, GBP, CAD

### Managing Settings

Access the Settings page to configure:

- **Cost Parameters**: Paper costs, labor rates, overhead percentages
- **Branding**: Company information, logo, contact details
- **Currency Settings**: Default currency and exchange rate preferences
- **User Preferences**: Interface settings and notifications

## API Documentation

### Base URL
- Development: `http://localhost:8080`
- Production: `https://your-domain.com`

### Authentication
All protected endpoints require a JWT token in the Authorization header:
```
Authorization: Bearer <your-jwt-token>
```

### Core Endpoints

#### Health Check
```http
GET /health
```
Returns the API health status and version information.

#### Authentication
```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password"
}
```

#### Job Management
```http
# List all jobs
GET /api/jobs?page=1&limit=10&search=query

# Create a new job
POST /api/jobs
Content-Type: application/json

{
  "title": "Business Cards",
  "jobType": "BusinessCard",
  "quantity": 500,
  "specifications": {
    "paperType": "250gsm_card",
    "paperSize": "A4",
    "paperWeight": "250gsm",
    "colors": {
      "frontColors": 4,
      "backColors": 4,
      "spotColors": [],
      "isFullColor": true
    },
    "pages": 1,
    "binding": "",
    "lamination": "gloss",
    "finishing": ["cutting"],
    "specialRequirements": "Double-sided printing"
  }
}

# Get job by ID
GET /api/jobs/{job_id}

# Update job
PUT /api/jobs/{job_id}

# Delete job
DELETE /api/jobs/{job_id}
```

#### Cost Calculations
```http
# Quick calculation (no job creation)
POST /api/cost/quick
Content-Type: application/json

{
  "jobType": "Flyer",
  "quantity": 1000,
  "specifications": { ... },
  "currency": "USD"
}

# Detailed calculation with breakdown
POST /api/cost/calculate
Content-Type: application/json

{
  "jobType": "Book",
  "quantity": 100,
  "specifications": { ... }
}
```

#### Currency Operations
```http
# Get supported currencies
GET /api/currency/supported

# Get current exchange rates
GET /api/currency/rates

# Convert currency
GET /api/currency/convert?from=USD&to=FCFA&amount=100
```

#### Settings Management
```http
# Get cost parameters
GET /api/settings/cost-parameters

# Update cost parameters
PUT /api/settings/cost-parameters

# Get branding settings
GET /api/settings/branding

# Update branding settings
PUT /api/settings/branding
```

### Response Format

All API responses follow a consistent format:

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

### Status Codes

| Code | Description |
|------|-------------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not Found |
| 422 | Unprocessable Entity |
| 500 | Internal Server Error |

## Development

### Development Environment Setup

1. **Clone and setup**:
   ```bash
   git clone <repository-url>
   cd costprint1
   ./docker/start.sh dev
   ```

2. **Development features**:
   - Hot reload for both frontend and backend
   - Debug logging enabled
   - Live code synchronization
   - Automatic dependency installation

### Code Style and Standards

#### Rust Backend
- Follow Rust standard formatting with `rustfmt`
- Use `clippy` for linting and best practices
- Implement comprehensive error handling
- Write unit and integration tests
- Document public APIs with rustdoc

#### TypeScript Frontend
- Use ESLint and Prettier for code formatting
- Follow React and Next.js best practices
- Implement proper TypeScript typing
- Use semantic HTML and accessibility standards
- Write component tests with Jest and React Testing Library

### Testing

#### Backend Tests
```bash
cd backend
cargo test
cargo clippy
cargo fmt --check
```

#### Frontend Tests
```bash
cd frontend
npm test
npm run lint
npm run type-check
```

#### Integration Tests
```bash
# Start services
./docker/start.sh dev -d

# Run validation tests
./docker/validate.sh
```

### Database Management

#### Migrations
Database migrations are handled automatically on startup. For manual migration management:

```bash
# Run migrations
cd backend
sqlx migrate run

# Create new migration
sqlx migrate add create_new_table
```

#### Database Schema
The application uses the following main tables:
- `users` - User authentication and profiles
- `jobs` - Print job specifications and status
- `cost_parameters` - Configurable pricing rules
- `branding_settings` - Company branding information
- `exchange_rates` - Currency conversion rates

## Deployment

### Production Deployment

#### Docker Deployment (Recommended)
```bash
# Clone repository
git clone <repository-url>
cd costprint1

# Configure production environment
cp docker/.env.example docker/.env
# Edit docker/.env with production values

# Deploy services
./docker/start.sh prod -d

# Verify deployment
./docker/validate.sh
```

#### Manual Deployment

1. **Server Requirements**:
   - Ubuntu 20.04 LTS or similar
   - Docker and Docker Compose
   - Minimum 2GB RAM, 20GB storage
   - SSL certificate for HTTPS

2. **Security Configuration**:
   - Change default passwords and JWT secrets
   - Configure firewall rules
   - Set up SSL/TLS certificates
   - Enable database backups
   - Configure monitoring and logging

3. **Environment Variables**:
   ```bash
   # Production environment variables
   DATABASE_URL=postgresql://user:password@db:5432/costprint
   JWT_SECRET=your-very-secure-production-secret
   CORS_ORIGINS=https://yourdomain.com
   NODE_ENV=production
   ```

#### Scaling Considerations

- **Horizontal Scaling**: Use Docker Swarm or Kubernetes
- **Database**: Consider read replicas for high traffic
- **Caching**: Redis cluster for distributed caching
- **Load Balancing**: Nginx or HAProxy for traffic distribution
- **Monitoring**: Prometheus and Grafana for metrics

### Backup and Recovery

#### Database Backups
```bash
# Create backup
docker-compose exec db pg_dump -U postgres costprint > backup.sql

# Restore backup
docker-compose exec -T db psql -U postgres costprint < backup.sql
```

#### Application Data
- Regular database backups (daily recommended)
- Configuration file backups
- SSL certificate backups
- Application logs retention

## Contributing

### Development Workflow

1. **Fork the repository** and create a feature branch
2. **Make changes** following the coding standards
3. **Write tests** for new functionality
4. **Run the test suite** to ensure everything works
5. **Submit a pull request** with detailed description

### Coding Guidelines

- Follow existing code style and conventions
- Write clear, descriptive commit messages
- Include tests for new features and bug fixes
- Update documentation for API changes
- Use meaningful variable and function names

### Pull Request Process

1. Ensure all tests pass
2. Update documentation if needed
3. Add appropriate labels to the PR
4. Request review from maintainers
5. Address feedback and make necessary changes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### MIT License Summary

- Commercial use allowed
- Modification allowed
- Distribution allowed
- Private use allowed
- No warranty provided
- License and copyright notice required

## Support and Contact

For questions, issues, or contributions:

- **Issues**: Use GitHub Issues for bug reports and feature requests
- **Discussions**: Use GitHub Discussions for general questions
- **Security**: Report security issues privately to the maintainers

## Acknowledgments

- Built with Rust and Next.js
- Uses PostgreSQL and Redis for data storage
- Containerized with Docker for easy deployment
- Designed for professional print shop operations

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For support and questions:
- Check the documentation in `memory.md`
- Review the progress tracking in `progress.md`
- Create an issue in the repository
