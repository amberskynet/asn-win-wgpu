#!/bin/bash

# ASN Web Example Quick Start Script

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting ASN Web Example...${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found. Please run this script from the ex_web directory.${NC}"
    exit 1
fi

# Check if web build exists
if [ ! -d "pkg" ]; then
    echo -e "${YELLOW}⚠️  Web build not found. Building first...${NC}"
    ./build.sh
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Build failed. Cannot start server.${NC}"
        exit 1
    fi
fi

# Check if pkg directory has the required files
if [ ! -f "pkg/ex_web.js" ] || [ ! -f "pkg/ex_web_bg.wasm" ]; then
    echo -e "${RED}❌ Incomplete build detected. Rebuilding...${NC}"
    ./build.sh
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Build failed. Cannot start server.${NC}"
        exit 1
    fi
fi

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo -e "${RED}❌ Python3 is not installed or not in PATH${NC}"
    echo -e "${YELLOW}💡 Please install Python3 to run the HTTP server${NC}"
    exit 1
fi

# Find available port
PORT=8091
while lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null 2>&1; do
    echo -e "${YELLOW}⚠️  Port $PORT is in use, trying $((PORT+1))...${NC}"
    PORT=$((PORT+1))
done

echo -e "${GREEN}✅ Build verified successfully!${NC}"
echo ""
echo -e "${BLUE}🌐 Starting HTTP server on http://localhost:$PORT${NC}"
echo -e "${BLUE}📱 Open your browser and navigate to: ${GREEN}http://localhost:$PORT${NC}"
echo -e "${YELLOW}🛑 Press Ctrl+C to stop the server${NC}"
echo ""

# Try to open browser automatically (macOS)
if command -v open &> /dev/null; then
    echo -e "${BLUE}🔗 Opening browser automatically...${NC}"
    sleep 2
    open "http://localhost:$PORT" &
fi

# Start HTTP server
python3 -m http.server $PORT
