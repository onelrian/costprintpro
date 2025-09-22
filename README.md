# CostPrint Pro

A modern print job costing application for professional print shops.

## Features

- Real-time cost calculations with detailed breakdowns
- Multi-currency support (USD, FCFA, EUR, GBP, CAD)
- Job management with CRUD operations
- User authentication and role-based access
- Configurable cost parameters and branding

## Technology Stack

- **Frontend**: Next.js 15 + TypeScript + Tailwind CSS
- **Backend**: Rust + Axum framework
- **Database**: PostgreSQL 15 + Redis 7
- **Infrastructure**: Docker + Docker Compose

## Quick Start

**Prerequisites:** Docker and Docker Compose

```bash
# Clone the repository
git clone https://github.com/onelrian/costprintpro.git
cd costprintpro

# Start all services (uses pre-built images from GitHub Container Registry)
docker-compose -f docker/docker-compose.yml up -d
```

**Access:**
- Frontend: http://localhost:3000
- Backend API: http://localhost:8080

**Images:**
- Frontend: `ghcr.io/onelrian/costprintpro-frontend:latest`
- Backend: `ghcr.io/onelrian/costprintpro-backend:latest`

**Useful Commands:**
```bash
# Stop all services
docker-compose -f docker/docker-compose.yml down

# View logs
docker-compose -f docker/docker-compose.yml logs -f

# Update to latest images
docker-compose -f docker/docker-compose.yml pull
docker-compose -f docker/docker-compose.yml up -d

# Check service status
docker-compose -f docker/docker-compose.yml ps
```

## Development

The application uses pre-built Docker images that are automatically built and pushed to GitHub Container Registry via GitHub Actions. Images are updated on every push to the main branch.

## License

MIT License - see LICENSE file for details.
