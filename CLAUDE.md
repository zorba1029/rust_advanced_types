# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust educational project demonstrating advanced type system features, including Higher-Kinded Types, Generic Associated Types (GATs), Higher-Ranked Types, Type-Level State Machines, Const Generics, and Type-Safe Builder Patterns. The project is structured as a learning resource with comprehensive examples and test binaries.

## Common Commands

### Build and Test
```bash
# Build the entire project
cargo build

# Run main program (shows usage guide)
cargo run

# Run all tests
cargo test
```

### Individual Test Binaries
Each advanced type system feature has its own test binary:
```bash
# Higher-Kinded Types (Associated Type Constructors)
cargo run --bin hkt_test

# Higher-Ranked Types (lifetime polymorphism)
cargo run --bin hrt_test

# Generic Associated Types 
cargo run --bin gat_test

# Type-Level State Machines
cargo run --bin state_machine_test

# Const Generics (compile-time array operations)
cargo run --bin const_generic_test

# Type-Safe Builder Pattern
cargo run --bin typesafe_builder_test

# Functor/Monad implementations
cargo run --bin monad_test
```

### Development
```bash
# Check for compilation errors
cargo check

# Run with release optimizations
cargo build --release
```

## Code Architecture

### Module Structure
- `src/lib.rs` - Library root exposing all custom types modules
- `src/main.rs` - Main entry point with usage instructions
- `src/custom_types/` - Core implementations of advanced type system features:
  - `container.rs` - Higher-Kinded Types using Associated Type Constructors
  - `gat.rs` - Generic Associated Types with lifetime dependencies
  - `with_lifetime.rs` - Higher-Ranked Types with `for<'a>` syntax
  - `state_machine.rs` - Type-Level State Machines using PhantomData
  - `const_generic.rs` - Const Generics for compile-time array operations
  - `typesafe_builder.rs` - Type-Safe Builder Pattern combining multiple features
  - `functor_monad.rs` - Functor and Monad trait implementations
- `src/bin/` - Individual test binaries for each feature

### Key Concepts Implemented

1. **Higher-Kinded Types**: Using Associated Type Constructors to abstract over type constructors like `Option<T>` and `Result<T, E>`

2. **Generic Associated Types (GATs)**: Associated types that can be generic over parameters, enabling complex lifetime-dependent type relationships

3. **Higher-Ranked Types**: Using `for<'a>` syntax for lifetime polymorphism and complex lifetime bounds

4. **Type-Level State Machines**: PhantomData-based state tracking at compile time with zero runtime cost

5. **Const Generics**: Compile-time array size calculations and matrix operations

6. **Type-Safe Builder Pattern**: Combining multiple advanced features for compile-time validation of object construction

### Architecture Patterns

- **Zero-Cost Abstractions**: All type-level computations happen at compile time
- **Trait-Based Design**: Extensive use of traits for abstraction and polymorphism
- **PhantomData Usage**: Type-level state tracking without runtime overhead
- **Compile-Time Safety**: Invalid states and operations are caught at compile time

## Requirements

- Rust 1.65.0 or later (for GATs stabilization)
- Cargo build system

## Korean Documentation

The project includes Korean documentation files:
- `readme-hkt.md` - Higher-Kinded Types guide
- `readme-gat.md` - Generic Associated Types guide
- `readme-hrt.md` - Higher-Ranked Types guide
- `rust-hkt-gat-details.md` - Detailed implementation guide