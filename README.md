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

```bash
sleuth <username>
```

## Acknowledgments

This project is inspired by and based on [sherlock](https://github.com/sherlock-project/sherlock) by [sherlock-project](https://github.com/sherlock-project). Special thanks to the original creators for their excellent work.

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.
