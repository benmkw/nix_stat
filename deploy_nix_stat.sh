#!/usr/bin/env bash
set -euo pipefail

# Configuration
REMOTE_USER=${REMOTE_USER:-asahi}
REMOTE_HOST=${REMOTE_HOST:-192.168.2.230}
REMOTE_PATH=${REMOTE_PATH:-/home/asahi/nix_stat}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Vendor Rust dependencies and run tests before deploying
log_info "Vendoring Rust dependencies..."
if ! cargo vendor; then
    log_error "Failed to vendor dependencies"
    exit 1
fi

log_info "Running cargo test..."
if ! cargo test --release; then
    log_error "Tests failed"
    exit 1
fi

log_info "Checking cross-compilation target..."
if ! cargo check --target aarch64-unknown-linux-gnu; then
    log_warn "Cross-compilation check failed, but continuing..."
fi

# Test connection to remote host
log_info "Testing connection to ${REMOTE_USER}@${REMOTE_HOST}..."
if ! ssh -o ConnectTimeout=10 "${REMOTE_USER}@${REMOTE_HOST}" echo "Connection test successful"; then
    log_error "Cannot connect to remote host"
    exit 1
fi

# Push local repo to remote (using rsync for speed and safety)
log_info "Syncing repo to ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}..."
if rsync -avz --delete --exclude 'target' --exclude '.git' --exclude '*.log' . "${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}"; then
    log_info "Repo synced successfully to ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}"
else
    log_error "Failed to sync repo"
    exit 1
fi

log_info "Deployment completed successfully!"
