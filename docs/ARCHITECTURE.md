# Sleuth Architecture

## Overview

Sleuth is a Rust implementation of sherlock, an OSINT tool for finding usernames across social networks.

## Directory Structure

```
sleuth/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library root, public API
│   ├── cli/             # Command-line interface
│   ├── core/            # Core engine functionality
│   ├── sites/           # Site-specific implementations
│   ├── http/            # HTTP client abstraction
│   ├── config/          # Configuration management
│   ├── data/            # Data structures
│   └── utils/           # Utility functions
├── tests/               # Integration tests
├── examples/            # Example programs
├── benches/             # Benchmarks
└── docs/                # Documentation
```

## Core Components

### CLI Module (`src/cli/`)
- Argument parsing with `clap`
- Output formatting (colored terminal, JSON, CSV)
- Progress indicators

### Core Engine (`src/core/`)
- `engine.rs`: Orchestrates searches across sites
- `scanner.rs`: Concurrent scanning logic
- `result.rs`: Result types and aggregators

### Sites Module (`src/sites/`)
- Trait-based design for site checkers
- Organized by type in subdirectories (dev, social, professional, gaming, forum, nsfw, other)
- Each site implements the `Site` trait
- Easy to add new sites by creating files in the appropriate type directory
- All sites are automatically registered via `sites::all_sites()`

### HTTP Client (`src/http/`)
- Abstraction over `reqwest`
- Rate limiting per site
- Retry logic and error handling

### Configuration (`src/config/`)
- Site registry loaded from data files
- User agent rotation
- Timeout and retry configuration

## Design Patterns

- **Trait-based site checkers**: Each site implements a `Site` trait for consistency
- **Async/await**: Concurrent requests using Tokio
- **Error handling**: Custom error types with `thiserror`
- **Configuration**: Site definitions in TOML/JSON for easy updates
- **Modularity**: Clear separation of concerns

## Testing

- Unit tests are co-located with source files using `#[cfg(test)] mod tests {}`
- Integration tests are in the `tests/` directory
- Benchmarks are in the `benches/` directory

