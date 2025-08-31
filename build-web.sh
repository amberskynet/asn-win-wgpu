#!/bin/bash

# ASN Web WGPU Build Script
# This script builds the project for web using wasm-pack

set -e

echo "🚀 Building ASN Web WGPU..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf target/wasm32-unknown-unknown
rm -rf web/pkg

# Build for wasm32-unknown-unknown target
echo "🔨 Building for WASM target..."
wasm-pack build --target web --out-dir web/pkg

# Copy HTML file to web directory if it doesn't exist
if [ ! -f "web/index.html" ]; then
    echo "📄 Creating index.html..."
    cp web/index.html web/index.html.bak 2>/dev/null || true
fi

echo "✅ Build completed successfully!"
echo "🌐 To run the web version:"
echo "   cd web && python3 -m http.server 8080"
echo "   Then open http://localhost:8080 in your browser"
