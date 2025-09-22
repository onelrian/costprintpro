# Docker Setup

## Commands

**Start Production (uses pre-built images):**
```bash
docker-compose -f docker/docker-compose.yml up -d
```

**Start Development (with hot reload):**
```bash
docker-compose -f docker/docker-compose.dev.yml up --build
```

**Stop Services:**
```bash
docker-compose -f docker/docker-compose.yml down
```

**View Logs:**
```bash
docker-compose -f docker/docker-compose.yml logs -f
```

**Update Images:**
```bash
docker-compose -f docker/docker-compose.yml pull
docker-compose -f docker/docker-compose.yml up -d
```

## Access

- Frontend: http://localhost:3000
- Backend API: http://localhost:8080

## Images

Pre-built images are automatically built and pushed to GitHub Container Registry:
- Backend: `ghcr.io/onelrian/costprintpro-backend:latest`
- Frontend: `ghcr.io/onelrian/costprintpro-frontend:latest`
