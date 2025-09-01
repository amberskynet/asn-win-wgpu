# ASN Project Makefile

# Default target
.PHONY: all
all: help

# Help target
.PHONY: help
help:
	@echo "ASN Project Makefile"
	@echo "==================="
	@echo "Available targets:"
	@echo "  web      - Build the web example using wasm-pack"
	@echo "  run-web  - Run the web example (builds if necessary)"
	@echo "  help     - Show this help message"

# Web build target
.PHONY: web
web:
	@echo "🚀 Building web example..."
	@cd examples/ex_web && ./build.sh

# Web run target
.PHONY: run-web
run-web:
	@echo "🚀 Running web example..."
	@cd examples/ex_web && ./run.sh
