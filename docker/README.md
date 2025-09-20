# CostPrint Pro Docker Setup

This directory contains Docker configuration files for running the CostPrint Pro application in both production and development environments.

## Quick Start

### Production Setup

1. **Build and run all services:**
   ```bash
   cd /home/onel/Projects/costprint1
   docker-compose -f docker/docker-compose.yml up --build
   ```

2. **Access the application:**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080
   - PostgreSQL: localhost:5432
   - Redis: localhost:6379

### Development Setup (with hot reload)

1. **Run development environment:**
   ```bash
   cd /home/onel/Projects/costprint1
   docker-compose -f docker/docker-compose.dev.yml up --build
   ```

2. **Features:**
   - Hot reload for both frontend and backend
   - Debug logging enabled
   - Volume mounts for live code changes
   - Faster rebuild times with cached dependencies

## Services Overview

### Frontend (Next.js)
- **Port:** 3000
- **Technology:** Next.js 13+ with TypeScript
- **Features:** 
  - Multi-currency support
  - Job creation and management
  - Real-time cost calculations
  - Professional UI with Tailwind CSS

### Backend (Rust + Axum)
- **Port:** 8080
- **Technology:** Rust with Axum framework
- **Features:**
  - RESTful API
  - JWT authentication
  - Multi-currency conversion
  - Job management CRUD operations
  - Cost calculation engine

### Database (PostgreSQL)
- **Port:** 5432
- **Version:** PostgreSQL 15
- **Features:**
  - Persistent data storage
  - Health checks
  - Automatic initialization

### Cache (Redis) - Optional
- **Port:** 6379
- **Version:** Redis 7
- **Features:**
  - Session storage
  - Caching layer
  - Optional service

## Configuration

### Environment Variables

#### Backend
- `DATABASE_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string
- `JWT_SECRET`: Secret key for JWT tokens
- `PORT`: Server port (8080)
- `CORS_ORIGINS`: Allowed CORS origins
- `RUST_LOG`: Logging level

#### Frontend
- `NEXT_PUBLIC_API_URL`: Backend API URL
- `NODE_ENV`: Environment (production/development)
- `PORT`: Server port (3000)

### Port Mapping
- **Frontend:** 3000:3000
- **Backend:** 8080:8080
- **PostgreSQL:** 5432:5432
- **Redis:** 6379:6379

## Development Commands

### Start services
```bash
# Production
docker-compose -f docker/docker-compose.yml up -d

# Development
docker-compose -f docker/docker-compose.dev.yml up -d
```

### View logs
```bash
# All services
docker-compose -f docker/docker-compose.yml logs -f

# Specific service
docker-compose -f docker/docker-compose.yml logs -f backend
```

### Stop services
```bash
docker-compose -f docker/docker-compose.yml down
```

### Rebuild services
```bash
docker-compose -f docker/docker-compose.yml up --build
```

### Clean up (remove volumes)
```bash
docker-compose -f docker/docker-compose.yml down -v
```

## Health Checks

All services include health checks:
- **Backend:** `curl -f http://localhost:8080/health`
- **Frontend:** `curl -f http://localhost:3000`
- **PostgreSQL:** `pg_isready -U postgres`
- **Redis:** `redis-cli ping`

## API Endpoints

### Core Endpoints
- `GET /health` - Health check
- `POST /api/auth/login` - User authentication
- `GET /api/jobs` - List jobs
- `POST /api/jobs` - Create job
- `GET /api/jobs/:id` - Get job details
- `PUT /api/jobs/:id` - Update job
- `DELETE /api/jobs/:id` - Delete job

### Cost Calculation
- `POST /api/cost/calculate` - Detailed cost calculation
- `POST /api/cost/quick` - Quick cost calculation

### Currency Support
- `GET /api/currency/supported` - List supported currencies
- `GET /api/currency/rates` - Get exchange rates
- `GET /api/currency/convert` - Convert currency

### Settings
- `GET /api/settings/cost-parameters` - Get cost parameters
- `PUT /api/settings/cost-parameters` - Update cost parameters
- `GET /api/settings/branding` - Get branding settings
- `PUT /api/settings/branding` - Update branding settings

## Troubleshooting

### Common Issues

1. **Port already in use:**
   ```bash
   # Check what's using the port
   lsof -i :3000
   lsof -i :8080
   
   # Kill the process
   kill -9 <PID>
   ```

2. **Database connection issues:**
   ```bash
   # Check if PostgreSQL is running
   docker-compose -f docker/docker-compose.yml ps db
   
   # View database logs
   docker-compose -f docker/docker-compose.yml logs db
   ```

3. **CORS issues:**
   - Ensure `CORS_ORIGINS` includes your frontend URL
   - Check browser network tab for CORS errors
   - Verify API_URL in frontend environment

4. **Build failures:**
   ```bash
   # Clean Docker cache
   docker system prune -a
   
   # Rebuild without cache
   docker-compose -f docker/docker-compose.yml build --no-cache
   ```

### Logs and Debugging

```bash
# View all logs
docker-compose -f docker/docker-compose.yml logs -f

# View specific service logs
docker-compose -f docker/docker-compose.yml logs -f backend
docker-compose -f docker/docker-compose.yml logs -f frontend
docker-compose -f docker/docker-compose.yml logs -f db

# Execute commands in running containers
docker-compose -f docker/docker-compose.yml exec backend /bin/bash
docker-compose -f docker/docker-compose.yml exec frontend /bin/sh
```

## Security Notes

### Production Deployment
1. Change default passwords and secrets
2. Use environment files for sensitive data
3. Enable SSL/TLS certificates
4. Configure proper firewall rules
5. Regular security updates

### Environment Files
Create `.env` files for sensitive configuration:
```bash
# .env.production
JWT_SECRET=your-super-secure-jwt-secret
POSTGRES_PASSWORD=your-secure-database-password
```

## Performance Optimization

### Production Optimizations
- Multi-stage Docker builds for smaller images
- Non-root users for security
- Health checks for reliability
- Restart policies for availability
- Volume caching for faster builds

### Development Optimizations
- Hot reload for faster development
- Volume mounts for live code changes
- Cached dependencies for faster rebuilds
- Debug logging for troubleshooting

## Deployment

### Production Deployment Steps
1. Update environment variables
2. Build production images
3. Deploy with orchestration (Docker Swarm, Kubernetes)
4. Configure load balancers
5. Set up monitoring and logging
6. Configure backups

### Example Production Command
```bash
# Build and deploy
docker-compose -f docker/docker-compose.yml up -d --build

# Scale services if needed
docker-compose -f docker/docker-compose.yml up -d --scale backend=3
```

## Notes

- All services are connected via the `app-network` bridge network
- Data is persisted using Docker volumes
- Services include proper health checks and restart policies
- CORS is properly configured for frontend-backend communication
- Both production and development configurations are provided
- The setup supports multi-currency functionality and job management
- All 404 errors have been resolved and tested
