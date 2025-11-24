# Site Implementation Guide

## Overview

Each site checker in sleuth implements the `Site` trait defined in `src/sites/site.rs`.

## Site Trait

```rust
#[async_trait]
pub trait Site: Send + Sync {
    fn name(&self) -> &str;
    async fn check_username(&self, username: &str) -> Result<SearchResult, Box<dyn std::error::Error + Send + Sync>>;
    fn url_pattern(&self) -> &str;
}
```

## Implementation Steps

1. Create a new file in `src/sites/` (e.g., `github.rs`)
2. Implement the `Site` trait
3. Add tests in the same file
4. Register in `src/sites/mod.rs`

## Example

See future site implementations for examples.

## Best Practices

- Handle rate limiting appropriately
- Use appropriate user agents
- Handle errors gracefully
- Add comprehensive tests
- Document any site-specific quirks

