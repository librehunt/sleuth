# Site Implementation Guide

## Overview

Each site checker in sleuth implements the `Site` trait defined in `src/sites/site.rs`.

## Site Trait

```rust
#[async_trait]
pub trait Site: Send + Sync {
    fn name(&self) -> &str;
    async fn check_username(&self, username: &str) -> Result<SearchResult>;
    fn url_pattern(&self) -> &str;
    fn site_type(&self) -> SiteType;
}
```

## Site Types

Sites are categorized into types for filtering:

- **dev**: Development/tech platforms (GitHub, GitLab, etc.)
- **social**: Social media platforms (Twitter, Instagram, etc.)
- **nsfw**: NSFW/adult content platforms
- **professional**: Professional networks (LinkedIn, etc.)
- **gaming**: Gaming platforms (Steam, Xbox, etc.)
- **forum**: Forums and communities
- **other**: Other/uncategorized

Users can filter sites by type using the `--type` / `-t` CLI flag.

## Directory Structure

Sites are organized by type in subdirectories:

```
src/sites/
├── mod.rs              # Main module, re-exports all sites
├── site.rs             # Site trait definition
├── dev/                # Development/tech platforms
│   ├── mod.rs
│   ├── github.rs
│   └── ...
├── social/             # Social media platforms
│   ├── mod.rs
│   └── ...
├── professional/       # Professional networks
│   ├── mod.rs
│   └── ...
├── gaming/             # Gaming platforms
│   ├── mod.rs
│   └── ...
├── forum/              # Forums and communities
│   ├── mod.rs
│   └── ...
├── nsfw/               # NSFW/adult content
│   ├── mod.rs
│   └── ...
└── other/              # Other/uncategorized
    ├── mod.rs
    └── ...
```

## Implementation Steps

1. Determine the site type and create a file in the appropriate subdirectory (e.g., `src/sites/dev/github.rs`)
2. Implement the `Site` trait
3. Add tests in the same file
4. Register in the type-specific `mod.rs` (e.g., `src/sites/dev/mod.rs`)
5. The site will be automatically included via `sites::all_sites()`

## Site Registry

The `SiteRegistry` provides a centralized way to work with all sites:

```rust
use sleuth::SiteRegistry;

// Create a registry with all sites
let registry = SiteRegistry::new();

// Get all sites
let all = registry.all();

// Get sites by type
let dev_sites = registry.by_type(SiteType::Dev);

// Get sites by multiple types
let social_and_gaming = registry.by_types(&[SiteType::Social, SiteType::Gaming]);

// Find a site by name (case-insensitive)
let github = registry.by_name("github");

// Filter by both type and name
let filtered = registry.filter(&[SiteType::Dev], &["github".to_string()]);

// Get statistics
let stats = registry.statistics();
println!("Total sites: {}", stats.total);
```

### Registry Methods

- `new()` - Create registry with all registered sites
- `all()` - Get all sites
- `by_type(type)` - Get sites of a specific type
- `by_types(types)` - Get sites matching any of the given types
- `by_name(name)` - Find a site by name (case-insensitive)
- `by_names(names)` - Get sites matching any of the given names
- `filter(types, names)` - Filter by both type and name
- `count()` - Get total number of sites
- `count_by_type(type)` - Get count of sites by type
- `statistics()` - Get detailed statistics

## Example

See future site implementations for examples.

## Best Practices

- Handle rate limiting appropriately
- Use appropriate user agents
- Handle errors gracefully
- Add comprehensive tests
- Document any site-specific quirks

