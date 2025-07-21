#!/bin/bash

# dotchk Installation Script
# Supports macOS and Linux

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Print colored output
print_error() {
    echo -e "${RED}ERROR: $1${NC}"
}

print_success() {
    echo -e "${GREEN}[OK] $1${NC}"
}

print_info() {
    echo -e "${YELLOW}INFO: $1${NC}"
}

# Detect OS
OS="Unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macOS"
else
    print_error "Unsupported operating system: $OSTYPE"
    exit 1
fi

print_info "Detected OS: $OS"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_info "Rust is not installed. Installing Rust..."
    
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    
    # Source cargo env
    source "$HOME/.cargo/env"
    
    print_success "Rust installed successfully"
else
    print_success "Rust is already installed"
fi

# Show Rust version
rust_version=$(rustc --version)
print_info "Using $rust_version"

# Build and install dotchk
print_info "Building and installing dotchk..."
cargo install --path . --force

if [ $? -eq 0 ]; then
    print_success "dotchk installed successfully!"
else
    print_error "Installation failed"
    exit 1
fi

# Check if dotchk is in PATH
if ! command -v dotchk &> /dev/null; then
    print_error "dotchk was installed but is not in PATH"
    print_info "Add $HOME/.cargo/bin to your PATH"
    exit 1
fi

# Show version and usage
echo ""
dotchk_version=$(dotchk --version 2>/dev/null || echo "unknown")
print_success "Installation complete"
echo ""
echo "Version: $dotchk_version"
echo ""
echo "Quick start:"
echo "  dotchk example.com"
echo "  dotchk pattern \"[a-z]{3}.com\" --limit 100"
echo "  dotchk tld mybrand --popular"
echo ""
echo "For more information, run: dotchk --help"