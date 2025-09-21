# Docker Setup

## Commands

**Start Production:**
```bash
docker-compose -f docker/docker-compose.yml up -d --build
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

## Access

- Frontend: http://localhost:3000
- Backend API: http://localhost:8080
