# Contributing to CostPrint Pro

Thank you for your interest in contributing to CostPrint Pro! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Guidelines](#contributing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)

## Code of Conduct

This project adheres to a code of conduct that promotes a welcoming and inclusive environment for all contributors. By participating, you agree to uphold these standards.

### Expected Behavior

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment or discriminatory language
- Personal attacks or trolling
- Public or private harassment
- Publishing others' private information without permission
- Other conduct that could reasonably be considered inappropriate

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- Docker and Docker Compose installed
- Git for version control
- Basic knowledge of Rust and TypeScript
- Understanding of web development concepts
- Familiarity with PostgreSQL and Redis

### Development Environment

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/costprint1.git
   cd costprint1
   ```
3. **Set up the development environment**:
   ```bash
   ./docker/start.sh dev
   ```
4. **Verify the setup** by running tests:
   ```bash
   ./docker/validate.sh
   ```

## Development Setup

### Backend Development

The backend is built with Rust and uses the Axum framework:

```bash
# Navigate to backend directory
cd backend

# Install Rust dependencies
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Frontend Development

The frontend uses Next.js with TypeScript:

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Run development server
npm run dev

# Run tests
npm test

# Type checking
npm run type-check

# Linting
npm run lint
```

### Database Development

Database migrations and schema changes:

```bash
# Create new migration
cd backend
sqlx migrate add migration_name

# Run migrations
sqlx migrate run

# Revert migration
sqlx migrate revert
```

## Contributing Guidelines

### Types of Contributions

We welcome various types of contributions:

- **Bug Reports**: Help us identify and fix issues
- **Feature Requests**: Suggest new functionality
- **Code Contributions**: Implement features or fix bugs
- **Documentation**: Improve or expand documentation
- **Testing**: Add or improve test coverage
- **Performance**: Optimize existing functionality

### Before You Start

1. **Check existing issues** to avoid duplicate work
2. **Create an issue** for significant changes to discuss the approach
3. **Start small** with minor fixes or improvements
4. **Follow the coding standards** outlined below

### Branch Naming Convention

Use descriptive branch names that indicate the type of change:

- `feature/add-job-templates` - New features
- `fix/currency-conversion-bug` - Bug fixes
- `docs/api-documentation` - Documentation updates
- `refactor/costing-engine` - Code refactoring
- `test/job-creation-tests` - Test additions

## Pull Request Process

### Before Submitting

1. **Update your fork** with the latest changes from main
2. **Run all tests** and ensure they pass
3. **Update documentation** if your changes affect it
4. **Follow commit message conventions** (see below)

### Commit Message Format

Use clear, descriptive commit messages:

```
type(scope): brief description

Detailed explanation of the change (if needed)

- List specific changes
- Include any breaking changes
- Reference related issues
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(jobs): add job template functionality

- Implement job template creation and management
- Add template selection in job creation form
- Include template validation and error handling

Closes #123
```

### Pull Request Template

When creating a pull request, include:

1. **Description** of the changes
2. **Type of change** (bug fix, feature, etc.)
3. **Testing performed** and results
4. **Screenshots** (if applicable)
5. **Related issues** or discussions

### Review Process

1. **Automated checks** must pass (CI/CD, tests, linting)
2. **Code review** by at least one maintainer
3. **Testing verification** in development environment
4. **Documentation review** if applicable
5. **Final approval** and merge by maintainer

## Coding Standards

### Rust Backend Standards

- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Use `rustfmt` for consistent formatting
- Address all `clippy` warnings
- Include comprehensive error handling
- Write unit tests for new functionality
- Document public APIs with rustdoc comments

**Example:**
```rust
/// Calculates the total cost for a print job
/// 
/// # Arguments
/// 
/// * `job` - The job specification containing all parameters
/// * `parameters` - The cost parameters for calculations
/// 
/// # Returns
/// 
/// Returns a `Result` containing the cost breakdown or an error
pub async fn calculate_job_cost(
    job: &JobSpecification,
    parameters: &CostParameters,
) -> Result<CostBreakdown, CostingError> {
    // Implementation
}
```

### TypeScript Frontend Standards

- Use TypeScript strict mode
- Follow React and Next.js best practices
- Use functional components with hooks
- Implement proper error boundaries
- Include prop types and interfaces
- Write component tests with Jest and React Testing Library

**Example:**
```typescript
interface JobFormProps {
  initialData?: Job;
  onSubmit: (job: CreateJobRequest) => Promise<void>;
  onCancel: () => void;
}

export const JobForm: React.FC<JobFormProps> = ({
  initialData,
  onSubmit,
  onCancel,
}) => {
  // Component implementation
};
```

### Database Standards

- Use descriptive table and column names
- Include proper indexes for performance
- Implement foreign key constraints
- Write migration scripts with rollback support
- Document schema changes

### API Standards

- Follow RESTful conventions
- Use consistent response formats
- Implement proper HTTP status codes
- Include comprehensive error messages
- Document all endpoints

## Testing Requirements

### Backend Testing

- **Unit Tests**: Test individual functions and modules
- **Integration Tests**: Test API endpoints and database operations
- **Performance Tests**: Ensure acceptable response times

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with coverage
cargo tarpaulin
```

### Frontend Testing

- **Component Tests**: Test React components in isolation
- **Integration Tests**: Test component interactions
- **E2E Tests**: Test complete user workflows

```bash
# Run unit tests
npm test

# Run E2E tests
npm run test:e2e

# Generate coverage report
npm run test:coverage
```

### Test Coverage

Maintain minimum test coverage:
- Backend: 80% code coverage
- Frontend: 70% code coverage
- Critical paths: 95% coverage

## Documentation

### Code Documentation

- **Rust**: Use rustdoc comments for public APIs
- **TypeScript**: Use JSDoc comments for complex functions
- **README**: Keep README files up to date
- **API**: Document all endpoints with examples

### Documentation Standards

- Write clear, concise explanations
- Include code examples where helpful
- Update documentation with code changes
- Use proper markdown formatting
- Include diagrams for complex concepts

### Documentation Types

1. **Code Comments**: Explain complex logic
2. **API Documentation**: Endpoint specifications
3. **User Guides**: How to use features
4. **Developer Guides**: How to contribute
5. **Architecture Docs**: System design explanations

## Getting Help

If you need help or have questions:

1. **Check the documentation** in the repository
2. **Search existing issues** for similar problems
3. **Create a new issue** with detailed information
4. **Join discussions** in GitHub Discussions
5. **Contact maintainers** for urgent matters

## Recognition

Contributors will be recognized in:

- **Contributors section** of the README
- **Release notes** for significant contributions
- **GitHub contributor graphs** and statistics

## License

By contributing to CostPrint Pro, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to CostPrint Pro!
