# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a comprehensive Rust practice question repository designed for learning core Rust concepts through hands-on exercises. Each practice question is contained in its own file with accompanying tests and solution files in an adjacent file of the same name but with `.md` instead of `.rs`.

## Commands

### Building and Testing

```bash
# Build the project
cargo build

# Check for compilation errors without building
cargo check

# Run all tests
cargo test

# Run tests for a specific practice question
cargo test --bin <filename>
# Example: cargo test --bin parallel_counter

# List all available tests
cargo test --lib -- --list
```

### Running Individual Practice Questions

Each `.rs` file contains a cargo test command in its header comments showing how to run that specific question's tests.

## Architecture

### Module Structure

The codebase is organized into focused learning modules:

- `concurrency/` - Fearless concurrency (threads, channels, Arc<Mutex<T>>, thread pools)
- `tokio/` - Async programming with Tokio (async/await, task spawning, futures)
- `collections/` - Standard library collections APIs (Vec, HashMap, HashSet, BTreeMap, etc.)
- `ownership/` - Ownership and borrowing patterns (move semantics, lifetimes, references)
- `iterators/` - Iterator patterns and transformations
- `lifetimes/` - Lifetime annotations and memory safety
- `smart_pointers/` - Smart pointers (Box, Rc, RefCell, Arc)
- `string_manipulation/` - String processing and text manipulation
- `file_io/` - File system operations and I/O patterns

### File Organization Pattern

Every practice question follows this structure:

- **One question per `.rs` file** with descriptive filename
- **Learning objectives** in header comments
- **Cargo test command** for running individual tests
- **Function stub with `todo!()`** for student implementation
- **Comprehensive test suite** covering edge cases
- **Corresponding `.md` answer file** with complete solution and explanation

### Code Style Conventions

- No `use` statements in question files - use fully qualified paths like `std::collections::HashMap::new()`
- `use` statements allowed in test modules
- Function names should be descriptive and match the filename
- Tests should cover normal cases, edge cases, and error conditions

### Dependencies

- `tokio` with full features for async programming practice
- `either` crate for additional utility types
- Standard library only for core practice questions

### Module Integration

Each module directory contains:

- Individual practice question `.rs` files
- Corresponding `.md` solution files
- `mod.rs` file declaring all public modules
- The root `lib.rs` includes all module directories

## Working with Practice Questions

### Creating New Questions

When adding new practice questions:

1. Create a single `.rs` file with one focused question
2. Follow the established naming convention (descriptive, no numbers)
3. Include learning objectives and test command in header
4. Create comprehensive tests covering multiple scenarios
5. Provide a complete `.md` solution file
6. Update the appropriate `mod.rs` file to include the new module
7. Run `cargo check` to ensure compilation

### Testing Strategy

- Each question has its own isolated test suite
- Tests verify correct behavior, error handling, and edge cases
- Use `cargo test --bin <filename>` to run specific question tests
- All questions must compile with `cargo check` even with `todo!()` implementations

### Answer Files Format

Each `.md` solution file contains:

1. Complete working implementation
2. Detailed explanation of the approach
3. Key learning points and Rust concepts demonstrated
