# Contributing to Sleuth

## Development Setup

1. Clone the repository
2. Install Rust (latest stable)
3. Run tests: `cargo test`
4. Run linter: `cargo clippy`
5. Format code: `cargo fmt`

## Adding a New Site

1. Create a new file in `src/sites/` (e.g., `src/sites/github.rs`)
2. Implement the `Site` trait
3. Add tests in the same file
4. Register the site in `src/sites/mod.rs`
5. Update documentation

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Write tests for all new functionality
- Keep tests in the same file as the code

## Testing

- Unit tests: In source files with `#[cfg(test)] mod tests {}`
- Integration tests: In `tests/` directory
- Run all tests: `cargo test`
- Run with output: `cargo test -- --nocapture`

