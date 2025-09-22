# CostPrint Pro - System Architecture

## Overview

CostPrint Pro is a modern print job costing application built with a microservices-oriented architecture using Rust (backend) and Next.js (frontend), containerized with Docker and orchestrated via Docker Compose.

## System Architecture Diagram

```mermaid
graph TB
    subgraph "Client Layer"
        UI[Next.js Frontend<br/>Port 3000]
    end
    
    subgraph "API Gateway Layer"
        API[Rust Backend API<br/>Axum Framework<br/>Port 8080]
    end
    
    subgraph "Data Layer"
        DB[(PostgreSQL 15<br/>Port 5432)]
        CACHE[(Redis 7<br/>Port 6379)]
    end
    
    subgraph "External Services"
        EXT[Exchange Rate APIs<br/>Currency Conversion]
    end
    
    UI -->|HTTP/REST| API
    API -->|SQL Queries| DB
    API -->|Caching| CACHE
    API -->|HTTP Requests| EXT
    
    style UI fill:#e1f5fe
    style API fill:#f3e5f5
    style DB fill:#e8f5e8
    style CACHE fill:#fff3e0
    style EXT fill:#fce4ec
```

## Component Architecture

```mermaid
graph LR
    subgraph "Frontend (Next.js)"
        PAGES[Pages]
        COMP[Components]
        CTX[Contexts]
        API_CLIENT[API Client]
        TYPES[TypeScript Types]
    end
    
    subgraph "Backend (Rust)"
        HANDLERS[HTTP Handlers]
        SERVICES[Business Services]
        MODELS[Data Models]
        DB_LAYER[Database Layer]
        UTILS[Utilities]
    end
    
    PAGES --> COMP
    COMP --> CTX
    CTX --> API_CLIENT
    API_CLIENT -->|HTTP| HANDLERS
    
    HANDLERS --> SERVICES
    SERVICES --> MODELS
    SERVICES --> DB_LAYER
    HANDLERS --> UTILS
    
    style PAGES fill:#bbdefb
    style HANDLERS fill:#c8e6c9
```

## Data Flow Architecture

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant A as Auth Handler
    participant C as Costing Handler
    participant S as Costing Service
    participant CS as Currency Service
    participant DB as Database
    participant R as Redis
    
    U->>F: Login Request
    F->>A: POST /api/auth/login
    A->>A: Generate JWT Token
    A->>F: Return Token + User Info
    F->>F: Store Token in localStorage
    
    U->>F: Request Cost Calculation
    F->>C: POST /api/cost/calculate (with JWT)
    C->>S: Calculate Cost
    S->>DB: Get Cost Parameters
    S->>CS: Convert Currency
    CS->>CS: Apply Exchange Rates
    S->>C: Return Calculation Result
    C->>F: JSON Response
    F->>U: Display Results
```

## Backend Architecture Details

### Handler Layer (`src/handlers/`)
**Responsibility**: HTTP request/response handling, input validation, authentication

```mermaid
graph TD
    subgraph "Handlers"
        AUTH[auth.rs<br/>Authentication & JWT]
        COST[costing.rs<br/>Cost Calculations]
        JOBS[jobs.rs<br/>Job Management]
        CURR[currency.rs<br/>Currency Operations]
        EXPORT[export.rs<br/>PDF/Excel Export]
        SETTINGS[settings.rs<br/>Configuration]
    end
    
    AUTH --> JWT[JWT Token Generation]
    COST --> CALC[Cost Algorithm Execution]
    JOBS --> CRUD[Job CRUD Operations]
    CURR --> CONV[Currency Conversion]
    EXPORT --> FILES[File Generation]
    SETTINGS --> CONFIG[Parameter Management]
```

**Key Components**:
- `auth.rs`: JWT-based authentication, user session management
- `costing.rs`: Cost calculation endpoints (calculate, preview, quick)
- `jobs.rs`: Full CRUD operations for print jobs
- `currency.rs`: Multi-currency support and conversion
- `export.rs`: PDF and Excel report generation
- `settings.rs`: Cost parameters and branding configuration

### Service Layer (`src/services/`)
**Responsibility**: Business logic implementation, data processing

```mermaid
graph TD
    subgraph "Services"
        CS[costing_service.rs<br/>Cost Calculations]
        CUS[currency_service.rs<br/>Exchange Rates]
        JS[job_service.rs<br/>Job Operations]
        US[user_service.rs<br/>User Management]
        CPS[cost_parameters_service.rs<br/>Parameter Config]
        BS[branding_service.rs<br/>Branding Config]
    end
    
    CS --> ALGO[Cost Algorithms]
    CUS --> RATES[Exchange Rate APIs]
    JS --> JOB_LOGIC[Job Business Logic]
    US --> USER_LOGIC[User Operations]
    CPS --> PARAMS[Cost Parameter Management]
    BS --> BRAND[Branding Management]
```

**Key Responsibilities**:
- **Costing Service**: Implements complex cost calculation algorithms with configurable parameters
- **Currency Service**: Manages exchange rates and currency conversions (currently hardcoded rates)
- **Job Service**: Handles job lifecycle management and business rules
- **User Service**: User authentication and profile management
- **Cost Parameters Service**: Manages configurable cost calculation parameters
- **Branding Service**: Handles company branding and customization

### Data Model Layer (`src/models/` & `src/entities/`)
**Responsibility**: Data structure definitions, serialization/deserialization

```mermaid
erDiagram
    USERS {
        uuid id PK
        string email UK
        string password_hash
        string role
        string first_name
        string last_name
        boolean is_active
        timestamp created_at
        timestamp updated_at
    }
    
    JOBS {
        uuid id PK
        uuid user_id FK
        string title
        string job_type
        integer quantity
        decimal total_cost
        decimal unit_cost
        string status
        timestamp created_at
        timestamp updated_at
    }
    
    COST_PARAMETERS {
        uuid id PK
        decimal paper_cost_per_sheet
        decimal plate_cost_per_job
        decimal labor_cost_per_hour
        decimal binding_cost_per_unit
        decimal overhead_percentage
        decimal profit_margin_percentage
        timestamp created_at
        timestamp updated_at
    }
    
    BRANDING_SETTINGS {
        uuid id PK
        string company_name
        string primary_color
        string secondary_color
        timestamp created_at
        timestamp updated_at
    }
    
    USERS ||--o{ JOBS : creates
```

### Database Layer (`src/db/`)
**Responsibility**: Database connection management, migrations

- **Sea-ORM Integration**: Modern async ORM for Rust
- **Migration System**: Versioned database schema management
- **Connection Pooling**: Efficient database connection management

## Frontend Architecture Details

### Page Structure (`src/app/`)
```mermaid
graph TD
    ROOT[layout.tsx<br/>Root Layout + AuthProvider]
    HOME[page.tsx<br/>Dashboard Route]
    LOGIN[login/page.tsx<br/>Authentication]
    JOBS[jobs/[id]/page.tsx<br/>Job Details]
    SETTINGS[settings/page.tsx<br/>Configuration]
    
    ROOT --> HOME
    ROOT --> LOGIN
    ROOT --> JOBS
    ROOT --> SETTINGS
```

### Component Architecture (`src/components/`)
- **Dashboard.tsx**: Main application interface with cost calculation forms
- **LoadingSpinner.tsx**: Reusable loading indicator
- **Modular Design**: Components designed for reusability and maintainability

### State Management (`src/contexts/`)
- **AuthContext**: Global authentication state management
- **JWT Token Handling**: Automatic token injection and refresh
- **Local Storage Integration**: Persistent authentication state

### API Client (`src/lib/api.ts`)
```mermaid
graph LR
    subgraph "API Client Structure"
        BASE[Base Axios Instance]
        AUTH_API[Authentication API]
        JOBS_API[Jobs API]
        COST_API[Costing API]
        CURR_API[Currency API]
        SET_API[Settings API]
        EXP_API[Export API]
    end
    
    BASE --> AUTH_API
    BASE --> JOBS_API
    BASE --> COST_API
    BASE --> CURR_API
    BASE --> SET_API
    BASE --> EXP_API
```

**Features**:
- **Request Interceptors**: Automatic JWT token injection
- **Response Interceptors**: Global error handling and authentication redirects
- **Type Safety**: Full TypeScript integration with backend models
- **Error Handling**: Centralized error processing and user feedback

## Infrastructure Architecture

### Docker Architecture
```mermaid
graph TB
    subgraph "Docker Compose Stack"
        subgraph "Application Services"
            FE[Frontend Container<br/>Node.js 18 Alpine<br/>Port 3000]
            BE[Backend Container<br/>Rust + Debian Slim<br/>Port 8080]
        end
        
        subgraph "Data Services"
            DB[PostgreSQL 15<br/>Alpine<br/>Port 5432]
            REDIS[Redis 7<br/>Alpine<br/>Port 6379]
        end
        
        subgraph "Volumes"
            PG_DATA[postgres_data]
            REDIS_DATA[redis_data]
        end
    end
    
    FE -->|API Calls| BE
    BE -->|SQL| DB
    BE -->|Cache| REDIS
    DB --> PG_DATA
    REDIS --> REDIS_DATA
```

### Build Process
```mermaid
graph LR
    subgraph "Backend Build"
        RUST_SRC[Rust Source] --> CARGO[Cargo Build]
        CARGO --> RUST_BIN[Binary]
        RUST_BIN --> BE_IMG[Backend Image]
    end
    
    subgraph "Frontend Build"
        NEXT_SRC[Next.js Source] --> NPM[npm build]
        NPM --> NEXT_BUILD[Static Build]
        NEXT_BUILD --> FE_IMG[Frontend Image]
    end
    
    subgraph "Deployment"
        BE_IMG --> GHCR[GitHub Container Registry]
        FE_IMG --> GHCR
        GHCR --> DEPLOY[Docker Compose Deployment]
    end
```

## Security Architecture

### Authentication Flow
```mermaid
sequenceDiagram
    participant C as Client
    participant F as Frontend
    participant A as Auth Handler
    participant DB as Database
    
    C->>F: Login Credentials
    F->>A: POST /api/auth/login
    Note over A: Currently in DEMO mode<br/>Accepts any credentials
    A->>A: Generate JWT Token
    A->>F: Return JWT + User Info
    F->>F: Store in localStorage
    
    loop Subsequent Requests
        F->>A: API Request + JWT Header
        A->>A: Validate JWT
        A->>F: Protected Resource
    end
```

**Current Security Measures**:
- JWT token-based authentication
- bcrypt password hashing (when implemented)
- CORS configuration
- Non-root Docker containers
- Input validation with Rust's type system

**Security Limitations** (MVP Status):
- Demo authentication mode (accepts any credentials)
- Hardcoded JWT secrets
- Missing rate limiting
- No CSRF protection

## Performance Considerations

### Database Performance
- **Connection Pooling**: Sea-ORM manages database connections
- **Async Operations**: Full async/await support throughout the stack
- **Indexing Strategy**: Basic indexes on foreign keys and status fields

### Caching Strategy
- **Redis Integration**: Prepared for caching exchange rates and cost calculations
- **Frontend Caching**: Browser-based caching for static assets
- **API Response Caching**: Potential for implementing response caching

### Scalability Design
- **Stateless Architecture**: JWT tokens enable horizontal scaling
- **Microservice Ready**: Clean separation allows for service extraction
- **Container Orchestration**: Docker Compose foundation for Kubernetes migration

## Development Workflow

### Local Development
```mermaid
graph LR
    DEV[Developer] --> GIT[Git Repository]
    GIT --> ACTIONS[GitHub Actions]
    ACTIONS --> BUILD[Build Images]
    BUILD --> GHCR[Push to GHCR]
    GHCR --> DEPLOY[Deploy via docker-compose]
```

### CI/CD Pipeline
- **Automated Builds**: GitHub Actions on push to main
- **Multi-stage Builds**: Optimized Docker images
- **Container Registry**: GitHub Container Registry for image storage
- **Health Checks**: Built-in health monitoring for all services

## Configuration Management

### Environment Variables
- **Backend**: Database URL, Redis URL, JWT secrets, CORS origins
- **Frontend**: API URLs, environment mode
- **Docker**: Service configuration and networking

### Configuration Files
- **Cargo.toml**: Rust dependencies and build configuration
- **package.json**: Node.js dependencies and scripts
- **docker-compose.yml**: Service orchestration and networking
- **Dockerfile**: Multi-stage build configurations

This architecture provides a solid foundation for a print costing application with clear separation of concerns, type safety, and modern deployment practices. The MVP status is reflected in simplified authentication and hardcoded exchange rates, which are clearly marked for future enhancement.
