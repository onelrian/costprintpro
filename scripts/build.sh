#!/bin/bash
# Build script for CostPrint Pro

set -e

echo "Building CostPrint Pro..."

# Build backend
echo "Building backend..."
cd apps/backend
cargo build
cd ../..

# Build frontend
echo "Building frontend..."
cd apps/frontend
npm ci
npm run build
cd ../..

echo "Build completed successfully!"
