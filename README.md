# sleuth

**sleuth** is a Rust implementation of [sherlock](https://github.com/sherlock-project/sherlock), a powerful OSINT (Open Source Intelligence) tool for finding usernames across social networks.

[![GitHub last commit](https://img.shields.io/github/last-commit/librehunt/sleuth)](https://github.com/librehunt/sleuth/commits/main)
[![CI](https://github.com/librehunt/sleuth/workflows/CI/badge.svg)](https://github.com/librehunt/sleuth/actions)
[![Codecov](https://codecov.io/gh/librehunt/sleuth/branch/main/graph/badge.svg)](https://codecov.io/gh/librehunt/sleuth)
[![Docs](https://docs.rs/sleuth/badge.svg)](https://docs.rs/sleuth)
[![Crates.io](https://img.shields.io/crates/v/sleuth.svg)](https://crates.io/crates/sleuth)
[![crates.io](https://img.shields.io/crates/d/sleuth)](https://crates.io/crates/sleuth)

## Overview

**sleuth** aims to provide a fast, efficient, and safe way to search for usernames across multiple social media platforms, leveraging Rust's performance, memory safety, and concurrency features.

## Features

- Search for usernames across numerous social media platforms
- Filter by site type/category (dev, social, professional, gaming, forum, nsfw, other)
- Filter by specific site names
- Multiple output formats (text, JSON, CSV)
- High performance and concurrency through Rust's async capabilities
- Memory-safe implementation
- Cross-platform support

## Installation

```bash
cargo install sleuth
```

Or build from source:

```bash
git clone https://github.com/librehunt/sleuth.git
cd sleuth
cargo build --release
```

## Usage

### Basic Usage

```bash
sleuth <username>
```

### Filter by Site Type

Filter sites by category using the `--type` or `-t` flag:

```bash
# Search only development/tech sites
sleuth username --type dev

# Search social media sites
sleuth username --type social

# Search multiple types
sleuth username --type dev --type social

# Search gaming and professional sites
sleuth username -t gaming -t professional
```

### Available Site Types

- `dev` - Development/tech platforms (GitHub, GitLab, etc.)
- `social` - Social media platforms (Twitter, Instagram, etc.)
- `professional` - Professional networks (LinkedIn, etc.)
- `gaming` - Gaming platforms (Steam, Xbox, etc.)
- `forum` - Forums and communities
- `nsfw` - NSFW/adult content platforms
- `other` - Other/uncategorized

### Filter by Specific Sites

Filter by specific site names using the `--site` or `-s` flag:

```bash
# Search only GitHub
sleuth username --site github

# Search multiple specific sites
sleuth username --site github --site twitter

# Combine type and site filters
sleuth username --type dev --site github
```

### Output Formats

```bash
# Text output (default)
sleuth username

# JSON output
sleuth username --format json

# CSV output
sleuth username --format csv
```

### Browser Verification

By default, sleuth uses fast HTTP HEAD requests for all sites. For sites that require JavaScript rendering (like OnlyFans, Reddit), you can use the `--verify` flag to perform a second verification pass using a headless browser:

```bash
# Fast scan (default) - uses HTTP HEAD for all sites
sleuth username

# Verified scan - uses browser for sites that require it
sleuth username --verify
```

**How it works:**
1. **First pass**: HTTP HEAD requests for all sites (fast, ~0.5s per site)
2. **Second pass** (if `--verify`): Headless browser verification for found sites that require JavaScript rendering (slower, ~4s per site, but more accurate)

This two-pass approach provides the best balance between speed and accuracy. Use `--verify` when you need to eliminate false positives from JavaScript-rendered sites.

## Acknowledgments

This project is inspired by and based on [sherlock](https://github.com/sherlock-project/sherlock) by [sherlock-project](https://github.com/sherlock-project). Special thanks to the original creators for their excellent work.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.
