# =============================================================================
# Justfile - Project Automation Commands
# =============================================================================
# Usage: just <command> [arguments]
# See: https://github.com/casey/just
# =============================================================================

# -----------------------------------------------------------------------------
# Aliases
# -----------------------------------------------------------------------------

alias t := test
alias b := build
alias d := doc
alias f := fmt
alias fc := fmtck
alias c := check
alias q := cw
alias dia := diagnostics
alias m := mdbook
alias e := examples

# -----------------------------------------------------------------------------
# Default Target
# -----------------------------------------------------------------------------

default:
    @just --list --unsorted

# -----------------------------------------------------------------------------
# Build Commands
# -----------------------------------------------------------------------------

# Build the project in release mode
build:
    @echo "🔨 Building in release mode..."
    cargo build --release

# Build with optimizations (alias for build)
rel: build

# -----------------------------------------------------------------------------
# Test Commands
# -----------------------------------------------------------------------------

# Run all tests
test:
    @echo "🧪 Running all tests..."
    cargo test --workspace

# Test diagnostics crate
test-diagnostics:
    @echo "🧪 Testing diagnostics module..."
    cargo test -p ltx_diagnostics --test severity_test

# test lexer crates
test-lexer:
    @echo "🧪 Testing lexer module..."
    cargo test -p ltx_lexer --test catcode_test

# Run tests with output capture disabled
test-verbose:
    @echo "🧪 Running tests with verbose output..."
    cargo test -- --nocapture

# Run specific integration tests
test-integration:
    @echo "🧪 Running integration tests..."
    cargo test --test '*' -- --nocapture

# -----------------------------------------------------------------------------
# Code Quality Commands
# -----------------------------------------------------------------------------

# Format the entire workspace
fmt:
    @echo "✨ Formatting code..."
    cargo fmt --all

# Check formatting without applying changes
fmtck:
    @echo "🔍 Checking code formatting..."
    cargo fmt --all -- --check

# Run Clippy linter
clippy:
    @echo "🔍 Running Clippy..."
    cargo clippy --workspace -- -D warnings

# Run Clippy with automatic fixes
clippy-fix:
    @echo "🔧 Running Clippy with auto-fix..."
    cargo clippy --workspace --fix --allow-staged --allow-dirty

# Check the workspace for errors
check:
    @echo "✅ Checking workspace..."
    cargo check --workspace

# Run full quality assurance suite
qa: fmtck check clippy test
    @echo "✅ Quality assurance complete!"

# Alias for qa
cw: qa

# -----------------------------------------------------------------------------
# Documentation Commands
# -----------------------------------------------------------------------------

# Generate and open documentation
doc:
    @echo "📚 Generating documentation..."
    cargo doc --workspace --open

# Build documentation without opening
doc-build:
    @echo "📚 Building documentation..."
    cargo doc --workspace --no-deps

# Serve mdbook documentation locally
mdbook:
    @echo "📖 Serving mdbook at http://localhost:3000..."
    mdbook serve docs --open

# Build mdbook static site
mdbook-build:
    @echo "📖 Building mdbook site..."
    mdbook build docs

# -----------------------------------------------------------------------------
# Diagnostics Commands
# -----------------------------------------------------------------------------

# Run diagnostics tests with output
diagnostics:
    @echo "🔬 Running diagnostics tests..."
    cargo test -p ltx_diagnostics -- --nocapture

# Run diagnostics examples
examples:
    @echo "📋 Running lexer errors example..."
    @cargo run -p ltx_diagnostics --example lexer_errors || true
    @echo ""
    @echo "📋 Running parser errors example..."
    @cargo run -p ltx_diagnostics --example parser_errors || true
    @echo ""
    @echo "📋 Running JSON renderer example..."
    @cargo run -p ltx_diagnostics --example json_render | jq '.' || true

# Run a specific example
example example_name:
    @echo "📋 Running example: {{example_name}}"
    @cargo run -p ltx_diagnostics --example {{example_name}}

# -----------------------------------------------------------------------------
# Development Commands
# -----------------------------------------------------------------------------

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean

# Clean and rebuild
rebuild: clean build
    @echo "✅ Rebuild complete!"

# Update dependencies
update:
    @echo "📦 Updating dependencies..."
    cargo update

# Run in watch mode (requires cargo-watch)
watch:
    @echo "👀 Watching for changes..."
    cargo watch -x check -x test

# -----------------------------------------------------------------------------
# Release Commands
# -----------------------------------------------------------------------------

# Prepare a new release
release:
    @echo "🚀 Preparing release..."
    @cargo build --release
    @cargo test --workspace
    @cargo doc --workspace --no-deps
    @echo "✅ Release artifacts built in target/release/"

# Publish to crates.io (dry run)
publish-dry:
    @echo "📦 Publishing dry run..."
    cargo publish --dry-run

# Publish to crates.io
publish:
    @echo "📦 Publishing to crates.io..."
    @echo "⚠️  Are you sure? (y/N) " && read ans && [ $${ans:-N} = y ]
    cargo publish

# -----------------------------------------------------------------------------
# Utility Commands
# -----------------------------------------------------------------------------

# Show project information
info:
    @echo "📊 Project Information"
    @echo "======================"
    @echo "Package: ltx_diagnostics"
    @cargo metadata --no-deps --format-version 1 | jq -r '.packages[0] | "Version: \(.version)\nLicense: \(.license)\nRepository: \(.repository)"'

# Show dependency graph
deps:
    @echo "📊 Generating dependency graph..."
    cargo tree --all-features

# Check for outdated dependencies
outdated:
    @echo "📦 Checking outdated dependencies..."
    cargo outdated

# -----------------------------------------------------------------------------
# Pre-commit Hooks
# -----------------------------------------------------------------------------

# Run pre-commit checks
pre-commit:
    @echo "🔍 Running pre-commit checks..."
    @just fmtck
    @just check
    @just test
    @just clippy
    @echo "✅ Pre-commit checks passed!"

# -----------------------------------------------------------------------------
# CI/CD Commands
# -----------------------------------------------------------------------------

# Run CI pipeline locally
ci:
    @echo "🔄 Running CI pipeline..."
    @just fmtck
    @just check
    @just clippy
    @just test
    @just doc-build
    @echo "✅ CI pipeline passed!"

# -----------------------------------------------------------------------------
# Help
# -----------------------------------------------------------------------------

# Show detailed help
help:
    @echo "📚 Available Commands:"
    @echo ""
    @echo "Aliases:"
    @echo "  t    → test"
    @echo "  b    → build"
    @echo "  d    → doc"
    @echo "  f    → fmt"
    @echo "  fc   → fmtck"
    @echo "  c    → check"
    @echo "  q    → cw"
    @echo "  dia  → diagnostics"
    @echo "  m    → mdbook"
    @echo "  e    → examples"
    @echo ""
    @just --list --unsorted
