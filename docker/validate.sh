#!/bin/bash

# CostPrint Pro Docker Validation Script
# This script validates the Docker setup and tests all functionality

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TESTS_PASSED=0
TESTS_FAILED=0
TOTAL_TESTS=0

# Function to print colored output
print_success() {
    echo -e "${GREEN}✓${NC} $1"
    ((TESTS_PASSED++))
}

print_failure() {
    echo -e "${RED}✗${NC} $1"
    ((TESTS_FAILED++))
}

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}    CostPrint Pro Docker Validation    ${NC}"
    echo -e "${BLUE}========================================${NC}"
}

# Function to run a test
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    ((TOTAL_TESTS++))
    print_info "Testing: $test_name"
    
    if eval "$test_command" > /dev/null 2>&1; then
        print_success "$test_name"
        return 0
    else
        print_failure "$test_name"
        return 1
    fi
}

# Function to test API endpoint
test_api_endpoint() {
    local endpoint="$1"
    local expected_status="$2"
    local description="$3"
    
    ((TOTAL_TESTS++))
    print_info "Testing API: $description"
    
    local status_code=$(curl -s -o /dev/null -w "%{http_code}" "$endpoint" || echo "000")
    
    if [[ "$status_code" == "$expected_status" ]]; then
        print_success "$description (HTTP $status_code)"
        return 0
    else
        print_failure "$description (Expected HTTP $expected_status, got $status_code)"
        return 1
    fi
}

# Function to test JSON API endpoint
test_json_api() {
    local endpoint="$1"
    local description="$2"
    local expected_field="$3"
    
    ((TOTAL_TESTS++))
    print_info "Testing JSON API: $description"
    
    local response=$(curl -s "$endpoint" 2>/dev/null || echo "{}")
    
    if echo "$response" | jq -e "$expected_field" > /dev/null 2>&1; then
        print_success "$description"
        return 0
    else
        print_failure "$description (Invalid JSON response or missing field)"
        return 1
    fi
}

# Main validation function
main() {
    print_header
    echo
    
    # Wait for services to be ready
    print_info "Waiting for services to be ready..."
    sleep 10
    
    echo
    print_info "=== INFRASTRUCTURE TESTS ==="
    
    # Test Docker services
    run_test "Docker daemon running" "docker info"
    run_test "Docker Compose available" "command -v docker-compose"
    
    # Test container status
    run_test "Frontend container running" "docker ps | grep -q costprint-frontend"
    run_test "Backend container running" "docker ps | grep -q costprint-backend"
    run_test "Database container running" "docker ps | grep -q costprint-postgres"
    run_test "Redis container running" "docker ps | grep -q costprint-redis"
    
    echo
    print_info "=== NETWORK CONNECTIVITY TESTS ==="
    
    # Test basic connectivity
    test_api_endpoint "http://localhost:3000" "200" "Frontend accessibility"
    test_api_endpoint "http://localhost:8080/health" "200" "Backend health endpoint"
    
    # Test database connectivity
    run_test "Database connectivity" "docker-compose -f docker/docker-compose.yml exec -T db pg_isready -U postgres"
    run_test "Redis connectivity" "docker-compose -f docker/docker-compose.yml exec -T redis redis-cli ping | grep -q PONG"
    
    echo
    print_info "=== API FUNCTIONALITY TESTS ==="
    
    # Test authentication endpoints
    test_api_endpoint "http://localhost:8080/api/auth/login" "400" "Auth login endpoint (expects POST with body)"
    
    # Test job management endpoints
    test_json_api "http://localhost:8080/api/jobs" "Jobs list endpoint" ".jobs"
    
    # Test cost calculation endpoints
    test_api_endpoint "http://localhost:8080/api/cost/calculate" "400" "Cost calculation endpoint (expects POST with body)"
    test_api_endpoint "http://localhost:8080/api/cost/quick" "400" "Quick calculation endpoint (expects POST with body)"
    
    # Test currency endpoints
    test_json_api "http://localhost:8080/api/currency/supported" "Supported currencies endpoint" ". | length"
    test_json_api "http://localhost:8080/api/currency/rates" "Exchange rates endpoint" ".rates"
    
    # Test settings endpoints
    test_json_api "http://localhost:8080/api/settings/cost-parameters" "Cost parameters endpoint" ".paperCostPerSheet"
    test_json_api "http://localhost:8080/api/settings/branding" "Branding settings endpoint" ".companyName"
    
    echo
    print_info "=== FUNCTIONAL TESTS ==="
    
    # Test job creation with real data
    print_info "Testing job creation functionality..."
    ((TOTAL_TESTS++))
    
    local job_response=$(curl -s -X POST -H "Content-Type: application/json" \
        -d '{"title":"Docker Validation Test","jobType":"Flyer","quantity":100,"specifications":{"paperType":"80gsm_offset","paperSize":"A4","paperWeight":"80gsm","colors":{"frontColors":4,"backColors":0,"spotColors":[],"isFullColor":true},"pages":1,"binding":"","lamination":"","finishing":[],"specialRequirements":"Docker validation test"}}' \
        http://localhost:8080/api/jobs 2>/dev/null || echo "{}")
    
    if echo "$job_response" | jq -e '.id' > /dev/null 2>&1; then
        print_success "Job creation functionality"
        
        # Test job retrieval
        local job_id=$(echo "$job_response" | jq -r '.id')
        test_json_api "http://localhost:8080/api/jobs/$job_id" "Job retrieval by ID" ".title"
    else
        print_failure "Job creation functionality"
    fi
    
    # Test quick calculation with currency
    print_info "Testing quick calculation with currency..."
    ((TOTAL_TESTS++))
    
    local calc_response=$(curl -s -X POST -H "Content-Type: application/json" \
        -d '{"jobType":"BusinessCard","quantity":500,"specifications":{"paperType":"250gsm_card","paperSize":"A4","paperWeight":"250gsm","colors":{"frontColors":4,"backColors":4,"spotColors":[],"isFullColor":true},"pages":1,"binding":"","lamination":"gloss","finishing":["cutting"],"specialRequirements":"Docker validation"},"currency":"FCFA"}' \
        http://localhost:8080/api/cost/quick 2>/dev/null || echo "{}")
    
    if echo "$calc_response" | jq -e '.totalCost' > /dev/null 2>&1 && echo "$calc_response" | jq -e '.currency' > /dev/null 2>&1; then
        print_success "Quick calculation with multi-currency"
    else
        print_failure "Quick calculation with multi-currency"
    fi
    
    echo
    print_info "=== CORS AND FRONTEND INTEGRATION TESTS ==="
    
    # Test CORS headers
    print_info "Testing CORS configuration..."
    ((TOTAL_TESTS++))
    
    local cors_response=$(curl -s -H "Origin: http://localhost:3000" \
        -H "Access-Control-Request-Method: POST" \
        -H "Access-Control-Request-Headers: Content-Type" \
        -X OPTIONS http://localhost:8080/api/jobs 2>/dev/null || echo "")
    
    if echo "$cors_response" | grep -q "access-control-allow-origin" || \
       curl -s -I -H "Origin: http://localhost:3000" http://localhost:8080/api/jobs | grep -q "access-control-allow-origin"; then
        print_success "CORS configuration"
    else
        print_failure "CORS configuration"
    fi
    
    echo
    print_info "=== PERFORMANCE AND RELIABILITY TESTS ==="
    
    # Test response times
    print_info "Testing API response times..."
    ((TOTAL_TESTS++))
    
    local response_time=$(curl -s -o /dev/null -w "%{time_total}" http://localhost:8080/health 2>/dev/null || echo "999")
    local response_time_ms=$(echo "$response_time * 1000" | bc 2>/dev/null || echo "999")
    
    if (( $(echo "$response_time < 2.0" | bc -l 2>/dev/null || echo "0") )); then
        print_success "API response time (${response_time_ms%.*}ms)"
    else
        print_failure "API response time too slow (${response_time_ms%.*}ms)"
    fi
    
    # Test concurrent requests
    print_info "Testing concurrent request handling..."
    ((TOTAL_TESTS++))
    
    local concurrent_success=0
    for i in {1..5}; do
        if curl -s -f http://localhost:8080/health > /dev/null 2>&1; then
            ((concurrent_success++))
        fi &
    done
    wait
    
    if [[ $concurrent_success -ge 4 ]]; then
        print_success "Concurrent request handling ($concurrent_success/5 successful)"
    else
        print_failure "Concurrent request handling ($concurrent_success/5 successful)"
    fi
    
    echo
    print_info "=== SUMMARY ==="
    echo
    
    if [[ $TESTS_FAILED -eq 0 ]]; then
        print_success "All tests passed! ($TESTS_PASSED/$TOTAL_TESTS)"
        echo
        print_info "🎉 CostPrint Pro Docker setup is fully functional!"
        print_info "   Frontend: http://localhost:3000"
        print_info "   Backend API: http://localhost:8080"
        print_info "   Database: localhost:5432"
        print_info "   Redis: localhost:6379"
        echo
        print_info "✨ Key features validated:"
        print_info "   ✓ Job creation and management"
        print_info "   ✓ Multi-currency cost calculations"
        print_info "   ✓ Quick calculation functionality"
        print_info "   ✓ Settings management"
        print_info "   ✓ CORS configuration"
        print_info "   ✓ Database connectivity"
        print_info "   ✓ API performance"
        echo
        exit 0
    else
        print_failure "Some tests failed ($TESTS_FAILED/$TOTAL_TESTS failed)"
        echo
        print_warning "Issues detected in the Docker setup."
        print_info "Check the logs with: docker-compose -f docker/docker-compose.yml logs"
        print_info "Or use the startup script: ./docker/start.sh logs -f"
        echo
        exit 1
    fi
}

# Check if bc is available for calculations
if ! command -v bc &> /dev/null; then
    print_warning "bc not found, installing for calculations..."
    if command -v apt-get &> /dev/null; then
        sudo apt-get update && sudo apt-get install -y bc
    elif command -v yum &> /dev/null; then
        sudo yum install -y bc
    elif command -v brew &> /dev/null; then
        brew install bc
    fi
fi

# Run main function
main "$@"
