.PHONY: all build test clean install update-tlds

# Default target
all: build

# Build the project in release mode
build:
	cargo build --release

# Run all tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean

# Install the binary
install: build
	cargo install --path .

# Update TLDs from IANA root zone
update-tlds:
	@echo "Updating TLDs from IANA root zone..."
	@python3 scripts/update_tlds.py

# Development build
dev:
	cargo build

# Run with debug logging
debug:
	RUST_LOG=domain_checker=debug cargo run -- $(ARGS)

# Run benchmarks
bench:
	cargo bench

# Format code
fmt:
	cargo fmt

# Run linter
lint:
	cargo clippy -- -D warnings

# Check code without building
check:
	cargo check

# Generate documentation
doc:
	cargo doc --open

# Run a quick check on some domains
test-domains:
	cargo run --release -- check google.com github.com example.com

# Test pattern generation
test-pattern:
	cargo run --release -- pattern "test-[a-z]{3}\.com" --limit 10

# Full CI pipeline
ci: fmt lint test

# Show help
help:
	@echo "Available targets:"
	@echo "  make build       - Build the project in release mode"
	@echo "  make test        - Run all tests"
	@echo "  make install     - Install the binary"
	@echo "  make update-tlds - Update TLD list from IANA"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make dev         - Development build"
	@echo "  make debug       - Run with debug logging (use ARGS=...)"
	@echo "  make bench       - Run benchmarks"
	@echo "  make fmt         - Format code"
	@echo "  make lint        - Run clippy linter"
	@echo "  make doc         - Generate and open documentation"
	@echo "  make ci          - Run full CI pipeline"