# Costing Application - Complete Architecture & Requirements

## 1. Project Overview

A comprehensive costing application for print shops to calculate costs for various print jobs (books, flyers, business cards, etc.) with a modern, responsive interface optimized for tablets and browsers.

## 2. Architectural Overview

### High-Level Architecture
```
[Frontend (Next.js)] <--> [Backend API (Rust/Axum)]
         |                              |
         |                              +--> [Costing Engine Module]
         |                              +--> [Settings/Admin Module]
         |                              +--> [Authentication Module]
         |                              +--> [Export/Reporting Module]
         |
         +--> [Branding/Theming assets]
         |
         +--> [History/Dashboard UI]

[Backend API] <--> [Database (PostgreSQL)] 
                 [Cache (Redis)] 

[Deployment] = Docker + CI/CD + Environments (Dev/Staging/Prod)
```

### Core Components

1. **Presentation Layer (Frontend/UI)**
   - Display forms, dashboards, user interactions
   - Handle client-side validation
   - Responsive design for tablets/browsers

2. **API Layer/Backend Application Logic**
   - Serve REST endpoints for all operations
   - Apply security and authorization
   - Handle business logic coordination

3. **Costing Engine/Business Logic Module**
   - Implement formulas and rules for costing
   - Handle adjustments for overrides/inflation
   - Calculate breakdowns by component

4. **Data Layer (Database/Storage)**
   - Persist cost parameters, user accounts, jobs
   - Store branding assets and settings
   - Maintain job history and analytics

5. **Authentication/Authorization**
   - Manage login and user roles
   - Protect endpoints with JWT tokens
   - Role-based access control

6. **Admin/Settings Module**
   - UI and backend for cost parameter management
   - Brand appearance customization
   - User management

7. **Branding & Theming**
   - Customizable logos, colors, fonts
   - Theme management system
   - Asset storage and serving

8. **Reports/Export Module**
   - Generate PDF/Excel cost summaries
   - Printable quotes and reports
   - Export job data

## 3. Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| Frontend | Next.js + TypeScript + Tailwind CSS | SSR/SSG, responsive design, type safety |
| Backend | Rust + Axum | Performance, safety, async support |
| Database | PostgreSQL | ACID compliance, relational integrity |
| Cache | Redis | Fast parameter lookup, session storage |
| Authentication | JWT tokens | Secure, stateless, standard |
| Exports | Rust PDF/Excel libraries | Integrated with backend |
| Containerization | Docker | Reproducible deployments |
| Infrastructure | Ansible + CI/CD | Automated provisioning and deployment |

## 4. End-to-End User Flow

### 4.1 Authentication Flow
1. User opens app on tablet/browser
2. Login screen → credentials entry
3. Backend verifies → issues JWT token
4. Frontend stores token securely

### 4.2 Dashboard Flow
1. Frontend queries API for job list, metrics
2. Display recent jobs, average costs
3. Quick access to new job creation

### 4.3 Job Creation Flow
1. User selects job type (book, flyer, etc.)
2. Form collects inputs:
   - Quantity
   - Paper type and size
   - Colors (CMYK, spot colors)
   - Binding options
   - Lamination/finishing
   - Special requirements
3. Real-time cost preview as user types
4. Submit for detailed calculation

### 4.4 Cost Calculation Flow
1. Frontend sends job data to `/api/cost/calculate`
2. Backend costing engine:
   - Fetches current cost parameters
   - Applies formulas for each component
   - Calculates paper, plate, labor, binding costs
   - Applies margins and adjustments
3. Returns detailed breakdown
4. Frontend displays visual breakdown with charts

### 4.5 Job Management Flow
1. Save calculated job with metadata
2. View job history with search/filter
3. Duplicate/modify existing jobs
4. Export quotes and reports

## 5. Data Models

### 5.1 User Model
```rust
struct User {
    id: Uuid,
    email: String,
    password_hash: String,
    role: UserRole, // Admin, Manager, User
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### 5.2 Job Model
```rust
struct Job {
    id: Uuid,
    user_id: Uuid,
    job_type: JobType, // Book, Flyer, BusinessCard, etc.
    title: String,
    quantity: i32,
    specifications: JobSpecs,
    cost_breakdown: CostBreakdown,
    total_cost: Decimal,
    unit_cost: Decimal,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### 5.3 Cost Parameters Model
```rust
struct CostParameters {
    id: Uuid,
    paper_costs: HashMap<String, Decimal>, // paper_type -> cost_per_sheet
    plate_costs: HashMap<String, Decimal>, // size -> cost
    labor_rates: HashMap<String, Decimal>, // operation -> hourly_rate
    binding_costs: HashMap<String, Decimal>, // binding_type -> cost
    overhead_percentage: Decimal,
    profit_margin: Decimal,
    updated_at: DateTime<Utc>,
}
```

## 6. API Endpoints

### 6.1 Authentication
- `POST /api/auth/login` - User login
- `POST /api/auth/logout` - User logout
- `GET /api/auth/me` - Get current user info

### 6.2 Jobs
- `GET /api/jobs` - List jobs with pagination/filtering
- `POST /api/jobs` - Create new job
- `GET /api/jobs/{id}` - Get job details
- `PUT /api/jobs/{id}` - Update job
- `DELETE /api/jobs/{id}` - Delete job

### 6.3 Costing
- `POST /api/cost/calculate` - Calculate job cost
- `POST /api/cost/preview` - Real-time cost preview

### 6.4 Settings
- `GET /api/settings/cost-parameters` - Get cost parameters
- `PUT /api/settings/cost-parameters` - Update cost parameters
- `GET /api/settings/branding` - Get branding settings
- `PUT /api/settings/branding` - Update branding

### 6.5 Export
- `POST /api/export/pdf/{job_id}` - Generate PDF quote
- `POST /api/export/excel/{job_id}` - Generate Excel report

## 7. Frontend Components

### 7.1 Page Structure
```
/pages
  /login - Authentication
  /dashboard - Main dashboard
  /jobs
    /new - New job creation
    /[id] - Job details/edit
    /history - Job history
  /settings
    /cost-parameters - Cost management
    /branding - Theme customization
    /users - User management
```

### 7.2 Key Components
- `JobForm` - Dynamic form for job specifications
- `CostBreakdown` - Visual cost breakdown display
- `JobHistory` - Searchable job list
- `CostParameterEditor` - Admin cost management
- `ThemeCustomizer` - Branding customization
- `ExportButtons` - PDF/Excel export controls

## 8. Costing Engine Formulas

### 8.1 Paper Cost Calculation
```
paper_cost = (sheets_needed * paper_cost_per_sheet)
sheets_needed = ceil(quantity / impressions_per_sheet)
```

### 8.2 Plate Cost Calculation
```
plate_cost = number_of_colors * plate_cost_per_color
```

### 8.3 Labor Cost Calculation
```
labor_cost = (setup_time + run_time) * labor_rate
run_time = quantity / impressions_per_hour
```

### 8.4 Total Cost Calculation
```
subtotal = paper_cost + plate_cost + labor_cost + binding_cost
overhead = subtotal * overhead_percentage
total_before_margin = subtotal + overhead
final_cost = total_before_margin * (1 + profit_margin)
```

## 9. File Structure

```
/
├── memory.md (this file)
├── progress.md
├── frontend/
│   ├── pages/
│   ├── components/
│   ├── styles/
│   ├── utils/
│   ├── types/
│   └── package.json
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── models/
│   │   ├── handlers/
│   │   ├── services/
│   │   ├── db/
│   │   └── utils/
│   ├── migrations/
│   └── Cargo.toml
├── docker/
│   ├── Dockerfile.frontend
│   ├── Dockerfile.backend
│   └── docker-compose.yml
└── infra/
    └── ansible/
```

## 10. Non-Functional Requirements

### 10.1 Performance
- Cost calculations < 500ms response time
- Page load times < 2 seconds
- Support for 100+ concurrent users

### 10.2 Security
- JWT token authentication
- Input validation and sanitization
- SQL injection prevention
- XSS protection
- HTTPS enforcement

### 10.3 Scalability
- Horizontal scaling capability
- Database connection pooling
- Redis caching for frequently accessed data
- CDN for static assets

### 10.4 User Experience
- Responsive design for tablets (primary) and desktop
- Offline form filling capability
- Real-time cost updates
- Clear error messages
- Intuitive navigation

### 10.5 Reliability
- 99.9% uptime target
- Automated backups
- Error logging and monitoring
- Graceful error handling

## 11. Testing Strategy

### 11.1 Backend Testing
- Unit tests for costing engine
- Integration tests for API endpoints
- Database migration tests
- Performance benchmarks

### 11.2 Frontend Testing
- Component unit tests
- Integration tests for user flows
- E2E tests for critical paths
- Responsive design tests

## 12. Deployment Strategy

### 12.1 Environments
- Development (local)
- Staging (testing)
- Production (live)

### 12.2 CI/CD Pipeline
- Automated testing on commit
- Docker image building
- Automated deployment to staging
- Manual promotion to production

## 13. Monitoring & Maintenance

### 13.1 Logging
- Application logs (structured JSON)
- Access logs
- Error tracking
- Performance metrics

### 13.2 Monitoring
- Health checks
- Database performance
- API response times
- User activity tracking

## 14. Future Enhancements

### 14.1 Phase 2 Features
- Multi-tenant support
- Advanced reporting and analytics
- Mobile app (React Native)
- Integration with accounting systems

### 14.2 Phase 3 Features
- Machine learning for cost optimization
- Supplier integration
- Inventory management
- Customer portal

## 15. Success Criteria

The application will be considered complete when:

1. ✅ All core components are implemented and tested
2. ✅ Authentication system is secure and functional
3. ✅ Costing engine produces accurate calculations
4. ✅ Job management (CRUD) works seamlessly
5. ✅ PDF/Excel export functionality works
6. ✅ Admin settings panel is fully functional
7. ✅ Responsive design works on tablets and desktop
8. ✅ All API endpoints are documented and tested
9. ✅ Database migrations are stable
10. ✅ Docker containerization is complete
11. ✅ Basic monitoring and logging are in place
12. ✅ End-to-end testing passes all scenarios

This document serves as the single source of truth for the application architecture and requirements.
