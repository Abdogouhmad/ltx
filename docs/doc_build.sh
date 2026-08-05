#!/bin/bash

# Exit on error
set -e

# Find the directory containing 'src' and navigate to it
dirpath=$(realpath "$(find . -type d -name 'src' -print -quit | xargs dirname)")
cd "$dirpath"

# Build the book
mdbook build

echo "Build complete"
