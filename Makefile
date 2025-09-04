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
	@echo "  clean-web - Clean the web example build artifacts"
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

# Web clean target
.PHONY: clean-web
clean-web:
	@echo "🧹 Cleaning web example..."
	@cd examples/ex_web && ./clean.sh
