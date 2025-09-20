#!/bin/bash

# CostPrint Pro Docker Startup Script
# This script helps you easily start the application in different modes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}    CostPrint Pro Docker Setup   ${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
    print_status "Docker is running ✓"
}

# Function to check if docker-compose is available
check_docker_compose() {
    if ! command -v docker-compose &> /dev/null; then
        print_error "docker-compose is not installed. Please install docker-compose and try again."
        exit 1
    fi
    print_status "docker-compose is available ✓"
}

# Function to show usage
show_usage() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  prod      Start production environment"
    echo "  dev       Start development environment (with hot reload)"
    echo "  stop      Stop all services"
    echo "  logs      Show logs for all services"
    echo "  clean     Stop services and remove volumes"
    echo "  rebuild   Rebuild and restart services"
    echo "  status    Show status of all services"
    echo "  test      Run health checks on all services"
    echo "  help      Show this help message"
    echo ""
    echo "Options:"
    echo "  -d, --detach    Run in detached mode (background)"
    echo "  -f, --follow    Follow logs (only with logs command)"
    echo ""
    echo "Examples:"
    echo "  $0 prod -d              # Start production in background"
    echo "  $0 dev                  # Start development with logs"
    echo "  $0 logs -f              # Follow logs"
    echo "  $0 rebuild              # Rebuild and restart"
}

# Function to start production environment
start_production() {
    print_status "Starting CostPrint Pro in PRODUCTION mode..."
    
    local detach_flag=""
    if [[ "$1" == "-d" || "$1" == "--detach" ]]; then
        detach_flag="-d"
        print_status "Running in detached mode"
    fi
    
    docker-compose -f docker/docker-compose.yml up --build $detach_flag
    
    if [[ "$detach_flag" == "-d" ]]; then
        print_status "Services started in background!"
        print_status "Frontend: http://localhost:3000"
        print_status "Backend API: http://localhost:8080"
        print_status "Use '$0 logs' to view logs"
        print_status "Use '$0 stop' to stop services"
    fi
}

# Function to start development environment
start_development() {
    print_status "Starting CostPrint Pro in DEVELOPMENT mode..."
    print_warning "This includes hot reload for faster development"
    
    local detach_flag=""
    if [[ "$1" == "-d" || "$1" == "--detach" ]]; then
        detach_flag="-d"
        print_status "Running in detached mode"
    fi
    
    docker-compose -f docker/docker-compose.dev.yml up --build $detach_flag
    
    if [[ "$detach_flag" == "-d" ]]; then
        print_status "Development services started in background!"
        print_status "Frontend: http://localhost:3000 (with hot reload)"
        print_status "Backend API: http://localhost:8080 (with hot reload)"
        print_status "Use '$0 logs' to view logs"
        print_status "Use '$0 stop' to stop services"
    fi
}

# Function to stop services
stop_services() {
    print_status "Stopping all CostPrint Pro services..."
    
    # Stop both production and development services
    docker-compose -f docker/docker-compose.yml down 2>/dev/null || true
    docker-compose -f docker/docker-compose.dev.yml down 2>/dev/null || true
    
    print_status "All services stopped ✓"
}

# Function to show logs
show_logs() {
    local follow_flag=""
    if [[ "$1" == "-f" || "$1" == "--follow" ]]; then
        follow_flag="-f"
        print_status "Following logs (Press Ctrl+C to exit)..."
    fi
    
    # Try production first, then development
    if docker-compose -f docker/docker-compose.yml ps -q | grep -q .; then
        docker-compose -f docker/docker-compose.yml logs $follow_flag
    elif docker-compose -f docker/docker-compose.dev.yml ps -q | grep -q .; then
        docker-compose -f docker/docker-compose.dev.yml logs $follow_flag
    else
        print_warning "No services are currently running"
        print_status "Start services with '$0 prod' or '$0 dev'"
    fi
}

# Function to clean up
clean_services() {
    print_warning "This will stop all services and remove volumes (data will be lost!)"
    read -p "Are you sure? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        print_status "Cleaning up all services and volumes..."
        docker-compose -f docker/docker-compose.yml down -v 2>/dev/null || true
        docker-compose -f docker/docker-compose.dev.yml down -v 2>/dev/null || true
        print_status "Cleanup completed ✓"
    else
        print_status "Cleanup cancelled"
    fi
}

# Function to rebuild services
rebuild_services() {
    print_status "Rebuilding all services..."
    
    # Determine which environment is running
    if docker-compose -f docker/docker-compose.yml ps -q | grep -q .; then
        print_status "Rebuilding production environment..."
        docker-compose -f docker/docker-compose.yml up --build -d
    elif docker-compose -f docker/docker-compose.dev.yml ps -q | grep -q .; then
        print_status "Rebuilding development environment..."
        docker-compose -f docker/docker-compose.dev.yml up --build -d
    else
        print_warning "No services are currently running"
        print_status "Use '$0 prod' or '$0 dev' to start services"
        return
    fi
    
    print_status "Rebuild completed ✓"
}

# Function to show service status
show_status() {
    print_status "Checking service status..."
    echo
    
    echo "Production Services:"
    docker-compose -f docker/docker-compose.yml ps 2>/dev/null || echo "  No production services running"
    echo
    
    echo "Development Services:"
    docker-compose -f docker/docker-compose.dev.yml ps 2>/dev/null || echo "  No development services running"
    echo
    
    print_status "Service status check completed ✓"
}

# Function to test services
test_services() {
    print_status "Running health checks..."
    
    local failed=0
    
    # Test frontend
    if curl -f http://localhost:3000 > /dev/null 2>&1; then
        print_status "Frontend (port 3000): ✓ Healthy"
    else
        print_error "Frontend (port 3000): ✗ Not responding"
        failed=1
    fi
    
    # Test backend
    if curl -f http://localhost:8080/health > /dev/null 2>&1; then
        print_status "Backend (port 8080): ✓ Healthy"
    else
        print_error "Backend (port 8080): ✗ Not responding"
        failed=1
    fi
    
    # Test database
    if docker-compose -f docker/docker-compose.yml exec -T db pg_isready -U postgres > /dev/null 2>&1 || \
       docker-compose -f docker/docker-compose.dev.yml exec -T db pg_isready -U postgres > /dev/null 2>&1; then
        print_status "Database (port 5432): ✓ Healthy"
    else
        print_error "Database (port 5432): ✗ Not responding"
        failed=1
    fi
    
    # Test Redis
    if docker-compose -f docker/docker-compose.yml exec -T redis redis-cli ping > /dev/null 2>&1 || \
       docker-compose -f docker/docker-compose.dev.yml exec -T redis redis-cli ping > /dev/null 2>&1; then
        print_status "Redis (port 6379): ✓ Healthy"
    else
        print_warning "Redis (port 6379): ✗ Not responding (optional service)"
    fi
    
    echo
    if [[ $failed -eq 0 ]]; then
        print_status "All critical services are healthy! ✓"
        print_status "Application is ready to use:"
        print_status "  Frontend: http://localhost:3000"
        print_status "  Backend API: http://localhost:8080"
    else
        print_error "Some services are not healthy. Check the logs with '$0 logs'"
        exit 1
    fi
}

# Main script logic
main() {
    print_header
    
    # Check prerequisites
    check_docker
    check_docker_compose
    
    # Change to project root directory
    cd "$(dirname "$0")/.."
    
    case "${1:-help}" in
        "prod"|"production")
            start_production "$2"
            ;;
        "dev"|"development")
            start_development "$2"
            ;;
        "stop")
            stop_services
            ;;
        "logs")
            show_logs "$2"
            ;;
        "clean")
            clean_services
            ;;
        "rebuild")
            rebuild_services
            ;;
        "status")
            show_status
            ;;
        "test"|"health")
            test_services
            ;;
        "help"|"-h"|"--help")
            show_usage
            ;;
        *)
            print_error "Unknown command: $1"
            echo
            show_usage
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
