# Agent Instructions for opencode-watch

## Build/Lint/Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Single test: `cargo test <test_name>`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Check: `cargo check`

## Code Style Guidelines
- Use Rust 2021 edition
- Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `Result<T>` for error handling with `anyhow::Result` for application errors
- Prefer `serde` for serialization with derive macros
- Use `tokio` for async operations
- Organize code into modules with clear separation of concerns
- Add doc comments for public APIs
- Use `clap` for CLI argument parsing with derive macros