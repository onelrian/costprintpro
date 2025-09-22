# =============================================================================
# Production-Grade Next.js Frontend Dockerfile
# =============================================================================
# This Dockerfile creates a minimal, secure, and efficient container for the
# CostPrint Pro Next.js frontend using standalone output and best practices.

# =============================================================================
# Stage 1: Dependency Installation
# =============================================================================
# Use Alpine Linux for smaller image size and better security
FROM node:20-alpine AS deps

# Install system dependencies required for node-gyp and native modules
RUN apk add --no-cache libc6-compat

WORKDIR /app

# Copy package files for dependency installation
# This layer will be cached unless package.json or package-lock.json changes
COPY package.json package-lock.json ./

# Install dependencies with exact versions for reproducibility
# Use npm ci for faster, reliable, reproducible builds
RUN npm ci --only=production --ignore-scripts && npm cache clean --force

# =============================================================================
# Stage 2: Build Stage
# =============================================================================
# Separate build stage to install dev dependencies and build the application
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files and install ALL dependencies (including dev)
COPY package.json package-lock.json ./
RUN npm ci --ignore-scripts

# Copy source code (next.config.ts already configured for standalone output)
COPY . ./

# Build the application with production optimizations
ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1

# Build the Next.js application
RUN npm run build

# =============================================================================
# Stage 3: Runtime Image (Minimal & Secure)
# =============================================================================
# Use minimal Alpine image for the runtime
FROM node:20-alpine AS runner

# Install security updates and minimal runtime dependencies
RUN apk add --no-cache \
    dumb-init \
    wget \
    && apk upgrade --no-cache

WORKDIR /app

# Create a non-root user for security
RUN addgroup --system --gid 1001 nodejs && \
    adduser --system --uid 1001 nextjs

# Set production environment
ENV NODE_ENV=production
ENV NEXT_TELEMETRY_DISABLED=1
ENV PORT=3000
ENV HOSTNAME="0.0.0.0"

# Copy the standalone build output from builder stage
# This includes only the necessary files for runtime
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static
COPY --from=builder --chown=nextjs:nodejs /app/public ./public

# Switch to non-root user for security
USER nextjs

# Expose the application port
EXPOSE 3000

# Add health check for container orchestration
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/ || exit 1

# Use dumb-init to handle signals properly and start the application
ENTRYPOINT ["dumb-init", "--"]
CMD ["node", "server.js"]

# =============================================================================
# Build Instructions:
# docker build -f Dockerfile.frontend -t costprint-frontend:latest .
#
# Security Features:
# - Alpine Linux base (minimal attack surface)
# - Non-root user execution (nextjs:nodejs)
# - dumb-init for proper signal handling
# - No unnecessary packages or tools
# - Security headers configured in Next.js
#
# Performance Features:
# - Standalone output (minimal runtime dependencies)
# - Multi-stage build (no dev dependencies in final image)
# - Compressed assets and optimized builds
# - Efficient layer caching for dependencies
# - Health checks for container orchestration
#
# Next.js Specific Optimizations:
# - Standalone output mode for minimal runtime
# - Telemetry disabled for privacy and performance
# - Static asset optimization
# - Proper file ownership and permissions
# =============================================================================
