# CostPrint Pro

A professional print job costing application for print shops with real-time cost calculations, multi-currency support, and comprehensive job management.

## MVP Overview

This is a Minimum Viable Product (MVP) version designed for evaluation and testing. The application provides core functionality for print job cost estimation with a demonstration authentication system.

## Prerequisites

- Docker and Docker Compose
- Git

## Quick Start

### Clone and Start
```bash
git clone https://github.com/onelrian/costprintpro.git
cd costprintpro
docker-compose up -d
```

### Access Points
- Frontend Application: http://localhost:3000
- Backend API: http://localhost:8080
- API Health Check: http://localhost:8080/health

### Testing the Application
1. Authentication: Use any email/password combination (demo mode)
2. Cost Calculator: Access the main dashboard for cost calculations
3. Job Management: Create, view, and manage print jobs
4. Multi-Currency: Test currency conversion functionality
5. Export Features: Generate PDF and Excel reports

## System Management

### Service Control
```bash
# Start all services
docker-compose up -d

# Stop all services
docker-compose down

# View service status
docker-compose ps

# View logs
docker-compose logs [service_name]

# Clean restart
docker-compose down -v && docker-compose up -d
```

### Environment Configuration
Default configuration works without modification. Optional customization:
```bash
DATABASE_URL=postgresql://postgres:password@db:5432/costprint
REDIS_URL=redis://redis:6379
JWT_SECRET=your-super-secret-jwt-key-change-in-production-environment
```

## Technology Stack

- **Backend**: Rust, Axum framework, PostgreSQL, Redis
- **Frontend**: Next.js 15, TypeScript, Tailwind CSS
- **Infrastructure**: Docker, Docker Compose
- **Container Registry**: GitHub Container Registry

## Architecture

The application uses a microservices architecture with the following components:
- PostgreSQL database for persistent data storage
- Redis cache for session management and performance optimization
- Rust backend API with Axum web framework
- Next.js frontend with server-side rendering
- Docker containerization for all services

For detailed architecture documentation, see [architecture.md](./architecture.md).

## Features

Current MVP features include:
- Real-time cost calculation engine
- Multi-currency support (USD, FCFA, EUR, GBP, CAD)
- Job management system with CRUD operations
- PDF and Excel export functionality
- Demo authentication system
- Responsive web interface

For complete feature documentation and roadmap, see [FEATURES.md](./FEATURES.md).

## MVP Limitations

- Demo authentication accepts any credentials
- Exchange rates are hardcoded for testing
- Limited error handling and validation
- No user registration or management
- Basic security implementation

This MVP is intended for evaluation purposes and requires additional development for production deployment.
