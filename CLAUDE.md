# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands
- Build: `cargo build` (within a specific project directory)
- Run: `cargo run` (within a specific project directory)
- Test: `cargo test` (for running all tests)
- Test single: `cargo test test_name` (to run a specific test)
- Format: `rustfmt src/*.rs` (format Rust code)
- Clippy (lint): `cargo clippy` (check for common mistakes and improvements)

## Code Style Guidelines
- **Naming**: Use snake_case for variables, functions, modules; CamelCase for types/structs/enums
- **Imports**: Group by standard library, external crates, then local modules
- **Error Handling**: Use Result<T, E> for functions that can fail; avoid unwrap() in production code
- **Documentation**: Use /// for documenting public APIs
- **Visibility**: Prefer private by default (no pub keyword), expose only what's necessary
- **Modules**: Follow the module structure in restaurant/ as example with separate files
- **Attributes**: Use #[derive(Debug)] for data structures to enable debugging
- **Formatting**: Use 4 spaces for indentation, max 100 chars per line