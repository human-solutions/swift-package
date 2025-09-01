# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Cargo plugin and library for building Apple Swift packages from Rust code using [uniffi](https://mozilla.github.io/uniffi-rs/latest/) for bindings generation and [xcframework](https://crates.io/crates/xcframework) for building binary frameworks. It generates Swift packages with resource support and convenience accessors.

## Development Commands

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Running Examples
The project uses end-to-end testing with Swift execution. To run the full test suite that includes Swift compilation:
```bash
cargo test end_to_end_static
```

### Using the Plugin
As a cargo subcommand:
```bash
cargo swift-package --manifest-path path/to/Cargo.toml
```

## Architecture

### Core Components

- **Main CLI entry**: `src/main.rs` - Simple wrapper around the library
- **Library core**: `src/lib.rs` - Main build orchestration and public API
- **Configuration**: `src/conf/` - CLI parsing and configuration management
  - `args.rs` - Command-line argument parsing using xflags
  - `configuration.rs` - Main configuration loading and validation
  - `swift_package.rs` - Swift package specific configuration parsing
- **Code generation**: 
  - `src/bindings.rs` - uniffi binding generation
  - `src/swift_resources_ext.rs` - Resource accessor extension generation
  - `src/swift_package_file.rs` - Package.swift file generation

### Configuration System

Projects using swift-package require a `[package.metadata.swift-package]` section in their Cargo.toml:

```toml
[package.metadata.swift-package]
# Required
package-name = "SwiftMath"
udl-file = "src/mymath.udl"

# Optional
resource-dirs = ["resources"]
macOS = true
```

The system validates uniffi version compatibility between swift-package build version and project dependencies.

### Build Process Flow

1. **Binding Generation**: Uses uniffi_bindgen to create Swift bindings from UDL files
2. **XCFramework Build**: Leverages xcframework crate for binary framework compilation  
3. **Resource Handling**: Copies resource directories and generates Swift accessor extensions
4. **Package Assembly**: Creates Package.swift file and assembles final Swift package structure

### Examples Structure

- `examples/end-to-end/mymath-lib/` - Complete working example with Rust library
- `examples/end-to-end/swift-exe/` - Swift executable consuming the generated package
- `examples/uniffi/` - Pure uniffi example without swift-package
- `manual/greeter/` - Manual uniffi setup example

### Test Strategy

Tests use the end-to-end example to validate the entire pipeline:
1. Build the Rust library with swift-package
2. Copy and configure the Swift executable  
3. Run `swift run` to verify the generated package works correctly
4. Validate both stdout output and stderr completion messages

The test creates temporary directories under `tests/temp/` and uses the examples as integration test fixtures.