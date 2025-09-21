# Docker Setup

Docker configuration for CostPrint Pro.

## Quick Start

**Production:**
```bash
docker-compose -f docker/docker-compose.yml up -d --build
```

**Development:**
```bash
docker-compose -f docker/docker-compose.dev.yml up --build
```

## Access

- Frontend: http://localhost:3000
- Backend API: http://localhost:8080

## Services

- **Frontend**: Next.js (Port 3000)
- **Backend**: Rust + Axum (Port 8080) 
- **Database**: PostgreSQL 15 (Port 5432)
- **Cache**: Redis 7 (Port 6379)
