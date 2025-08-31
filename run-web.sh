#!/bin/bash

# ASN Web WGPU Quick Start Script

echo "🚀 Starting ASN Web WGPU..."

# Check if web build exists
if [ ! -d "web/pkg" ]; then
    echo "❌ Web build not found. Building first..."
    ./build-web.sh
fi

# Start HTTP server
echo "🌐 Starting HTTP server on http://localhost:8095"
echo "📱 Open your browser and navigate to: http://localhost:8095"
echo "🛑 Press Ctrl+C to stop the server"
echo ""

cd web
python3 -m http.server 8095
