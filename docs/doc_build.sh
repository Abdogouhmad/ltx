#!/bin/bash

# Exit on error
set -e

# Anchor to this script's own directory so it builds the right book no matter
# where it is invoked from (CI runs it from the repo root).
cd "$(dirname "$(realpath "$0")")"

# Build the book
mdbook build

echo "Build complete"
