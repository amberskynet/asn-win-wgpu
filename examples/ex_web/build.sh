#!/bin/bash

# ASN Web Example Build Script
# This script builds the ex_web example for web using wasm-pack

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Building ASN Web Example...${NC}"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}❌ Error: Cargo.toml not found. Please run this script from the ex_web directory.${NC}"
    exit 1
fi

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}⚠️  wasm-pack is not installed. Installing...${NC}"
    cargo install wasm-pack
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Failed to install wasm-pack${NC}"
        exit 1
    fi
fi

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}⚠️  wasm32-unknown-unknown target not installed. Installing...${NC}"
    rustup target add wasm32-unknown-unknown
fi

# Clean previous builds
echo -e "${BLUE}🧹 Cleaning previous builds...${NC}"
rm -rf target/wasm32-unknown-unknown
rm -rf pkg

# Build for wasm32-unknown-unknown target
echo -e "${BLUE}🔨 Building for WASM target...${NC}"
echo -e "${YELLOW}   This may take a few minutes on first build...${NC}"

# Build with progress
if wasm-pack build --target web --out-dir pkg; then
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
    
    # Check if pkg directory was created
    if [ -d "pkg" ]; then
        echo -e "${GREEN}📦 WASM package created in ./pkg/${NC}"
        
        # Show package size
        if command -v du &> /dev/null; then
            PKG_SIZE=$(du -sh pkg 2>/dev/null | cut -f1)
            echo -e "${BLUE}📊 Package size: ${PKG_SIZE}${NC}"
        fi
    else
        echo -e "${RED}❌ Warning: pkg directory was not created${NC}"
    fi
    
    echo ""
    echo -e "${GREEN}🌐 To run the web example:${NC}"
    echo -e "   ${BLUE}./run.sh${NC}"
    echo -e "   ${BLUE}Or manually: python3 -m http.server 8091${NC}"
    echo -e "   ${BLUE}Then open http://localhost:8091 in your browser${NC}"
    
else
    echo -e "${RED}❌ Build failed!${NC}"
    echo -e "${YELLOW}💡 Common solutions:${NC}"
    echo -e "   - Make sure all dependencies are installed"
    echo -e "   - Check that you're in the correct directory"
    echo -e "   - Try running: cargo clean && ./build.sh"
    exit 1
fi
