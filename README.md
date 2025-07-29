# ASN Win WGPU

A modern window system implementation using Winit + WGPU for the Amberskynet project.

## Features

- 🖥️ **Cross-platform window management** with Winit
- 🎮 **Hardware-accelerated rendering** with WGPU
- 🔧 **Modular architecture** with separate components
- 📦 **WebAssembly support** for web deployment
- 🎨 **Texture rendering** with PNG/JPEG support
- ⚡ **Async/await support** for modern Rust

## Architecture

```
asn-win-wgpu/
├── src/                    # Main application logic
│   ├── app.rs             # Application event handling
│   ├── asn_win_config.rs  # Configuration management
│   └── lib.rs             # Public API
├── modules/
│   ├── asn-wgpu/          # WGPU rendering engine
│   └── asn-node-quad/     # Node.js integration
└── examples/              # Usage examples
```

## Quick Start

```rust
use asn_win_wgpu::{run, asn_win_config::AppConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run with default configuration
    run()
}

// Or with custom configuration
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig {
        window_title: "My App".to_string(),
        window_width: 1024,
        window_height: 768
    };

    asn_win_wgpu::run_with_config(config)
}
```

## Configuration

The `AppConfig` struct allows you to customize:

- **Window title** and dimensions
- **VSync** settings
- **Rendering** preferences

## Examples

See the `examples/` directory for complete usage examples:

- `ex_1.rs` - Basic window with textured triangle
- Custom configurations and event handling

## Dependencies

- **winit** - Cross-platform window creation
- **wgpu** - Modern graphics API
- **asn-logger** - Structured logging
- **pollster** - Async runtime

## Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run examples
cargo run --example ex_1
```

## WebAssembly Support

The project includes WASM support for web deployment:

```bash
# Build for WASM
cargo build --target wasm32-unknown-unknown

# Use wasm-pack for web deployment
wasm-pack build --target web
```

## License

This project is part of the Amberskynet ecosystem.


### zsh autosuggestion:

 source ~/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh
