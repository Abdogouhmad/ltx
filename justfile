# =============================================================================
# Justfile
# =============================================================================

alias t := test
alias b := build
alias d := doc
alias f := fmt
alias c := check

default:
    @just --list --unsorted

# Build the workspace
build:
    cargo build --workspace

# Run all tests
test:
    cargo test --workspace

# Test a specific crate: just test-crate ltx_lexer
test-crate crate:
    cargo test -p {{crate}} -- --nocapture

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmtck:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --workspace --all-targets

# Type-check the workspace
check:
    cargo check --workspace --all-targets

# Full quality check (format + lint + test)
qa: fmtck clippy test

# Generate and open docs
doc:
    cargo doc --workspace --open

# Run all examples
examples:
    cargo run -p ltx_diagnostics --example diagnostic_eg
    cargo run -p ltx_lexer --example tokenize_example
    cargo run -p ltx_parser --example parser_example

# Clean build artifacts
clean:
    cargo clean
