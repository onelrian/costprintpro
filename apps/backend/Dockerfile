# =============================================================================
# Production-Grade Rust Backend Dockerfile
# =============================================================================
# This Dockerfile creates a minimal, secure, and efficient container for the
# CostPrint Pro Rust backend using a runtime-only approach for reliability.

# Note: Build the binary locally first with: cargo build --release

# =============================================================================
# Runtime Image (Minimal & Secure)
# =============================================================================
FROM debian:bookworm-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -r -s /bin/false -u 1001 appuser

# Copy the pre-built binary (built locally with: cargo build --release)
COPY target/release/costprint-backend /usr/local/bin/costprint-backend

# Set proper permissions
RUN chmod +x /usr/local/bin/costprint-backend

# Set runtime environment variables
# These can be overridden at container runtime
ENV RUST_LOG=info
ENV PORT=8080
ENV BIND_ADDRESS=0.0.0.0:8080

# Expose the application port
EXPOSE 8080

# Use non-root user for security
USER appuser

# Set the entrypoint to our application
ENTRYPOINT ["/usr/local/bin/costprint-backend"]

# =============================================================================
# Build Instructions:
# 1. Build binary locally: cd apps/backend && cargo build --release
# 2. Build Docker image: docker build -t costprint-backend:latest apps/backend
#
# Security Features:
# - Debian slim base image (minimal attack surface)
# - Non-root user execution (appuser)
# - Optimized release binary
# - No build tools in final image
# - Pinned base image versions for reproducibility
#
# Performance Features:
# - Pre-built binary (fast Docker builds)
# - Release build with optimizations
# - Minimal runtime dependencies
# =============================================================================
