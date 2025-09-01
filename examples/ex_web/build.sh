#!/bin/bash

# ASN Web Example Build Script
# This script builds the ex_web example for web using wasm-pack

set -e

echo "🚀 Building ASN Web Example..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf target/wasm32-unknown-unknown
rm -rf pkg

# Build for wasm32-unknown-unknown target
echo "🔨 Building for WASM target..."
wasm-pack build --target web

echo "✅ Build completed successfully!"
echo "🌐 To run the web example:"
echo "   ./run.sh"
echo "   Or manually: python3 -m http.server 8080"
echo "   Then open http://localhost:8080 in your browser"
