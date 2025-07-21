#!/bin/bash

# dotchk Uninstallation Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_error() {
    echo -e "${RED}ERROR: $1${NC}"
}

print_success() {
    echo -e "${GREEN}[OK] $1${NC}"
}

print_info() {
    echo -e "${YELLOW}INFO: $1${NC}"
}

# Check if dotchk is installed
if ! command -v dotchk &> /dev/null; then
    print_error "dotchk is not installed"
    exit 1
fi

# Uninstall dotchk
print_info "Uninstalling dotchk..."
cargo uninstall dotchk

if [ $? -eq 0 ]; then
    print_success "dotchk uninstalled successfully"
else
    print_error "Failed to uninstall dotchk"
    exit 1
fi


print_success "Uninstallation complete!"