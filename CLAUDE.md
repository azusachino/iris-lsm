# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

iris-lsm is a simplified LSM-tree (Log-Structured Merge-tree) implementation in Rust, following the tutorial at https://skyzh.github.io/mini-lsm/

## Architecture

The codebase follows a layered architecture for LSM-tree storage:

```plain
MemTable -> WAL -> Manifest -> SST
```

### Core Components

- **`src/model/mod.rs`**: Defines the `KVOps<K, V>` trait that provides the key-value storage interface (`put`, `get`, `delete`, `scan`). Also contains stub structs for `MemTable`, `WAL`, `Manifest`, and `SST`.

- **`src/core/mod.rs`**: Contains `IrisLsm`, the main LSM-tree implementation that implements the `KVOps` trait.

- **`src/utils/mod.rs`**: Utility functions and helpers.

### Trait Design

The `KVOps<K, V>` trait is generic over key and value types. Methods take `&self` or `&mut self` as receivers (not static methods).

When implementing `KVOps` for types with borrowed references (like `&str`), be aware of lifetime constraints:

- The trait's `get` method returns `Result<V>`, where `V` must be a type that can be constructed independently
- If using `&str` as the value type, the returned reference lifetime needs careful handling
- Consider using owned types (`String`) or adding explicit lifetime parameters to the trait if needed

## Development Commands

### Build and Check

```bash
cargo build          # Build the project
cargo check          # Fast compile check
cargo clippy         # Run Clippy linter
```

### Testing

```bash
cargo test           # Run all tests
cargo test <name>    # Run specific test
```

### Running

```bash
cargo run                              # Run main binary
cargo run --example tokio_basic        # Run example
```

## Code Style

- The project uses `#![allow(dead_code)]` at the crate level to suppress warnings during early development
- Clippy is configured but `clippy.toml` doesn't exist yet
- Uses `edition = "2024"` (Rust 2024 edition)
