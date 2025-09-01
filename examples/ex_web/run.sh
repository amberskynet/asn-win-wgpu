#!/bin/bash

# ASN Web Example Quick Start Script

echo "🚀 Starting ASN Web Example..."

# Check if web build exists
if [ ! -d "pkg" ]; then
    echo "❌ Web build not found. Building first..."
    ./build.sh
fi

# Start HTTP server
echo "🌐 Starting HTTP server on http://localhost:8091"
echo "📱 Open your browser and navigate to: http://localhost:8091"
echo "🛑 Press Ctrl+C to stop the server"
echo ""

python3 -m http.server 8091
