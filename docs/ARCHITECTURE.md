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
│   ├── request/         # Request abstraction (HTTP, Tor, etc.)
│   ├── config/          # Configuration management
│   ├── data/            # Data structures
│   └── utils/           # Utility functions
├── tests/               # Integration tests
├── examples/            # Example programs
├── benches/             # Benchmarks
└── docs/                # Documentation
```

## Architecture Flow

The following flowchart shows the complete execution flow from user input to results output:

```mermaid
flowchart TD
    A[main.rs: CLI Entry Point] --> B[Args::parse]
    B --> C[Engine::new]
    C --> D[SiteRegistry::new]
    D --> E[Load all sites]
    E --> F[Engine::search]
    F --> G[SiteRegistry::filter]
    G --> H{Filter by type/name}
    H -->|Filtered sites| I[scan_username]
    H -->|No sites| J[Return empty results]
    I --> K[Create Request<br/>HttpRequest or TorRequest]
    K --> L[For each site]
    L --> M[Site::build_url]
    M --> N[Spawn async task]
    N --> O[Request::request]
    O --> P{Request Type}
    P -->|HTTP| Q[HttpRequest::get/head]
    P -->|Tor| R[TorRequest::get/head<br/>Not implemented yet]
    Q --> S[reqwest::Client]
    S --> T[HTTP Response]
    T --> U[Parse response status]
    U --> V{Status Code}
    V -->|200-299| W[SearchResult::found]
    V -->|404| X[SearchResult::not_found]
    V -->|Other| Y[SearchResult::not_found]
    W --> Z[Collect results]
    X --> Z
    Y --> Z
    Z --> AA[Return Vec<SearchResult>]
    AA --> AB[print_results]
    AB --> AC{Output Format}
    AC -->|text| AD[Colored terminal output]
    AC -->|json| AE[JSON output]
    AC -->|csv| AF[CSV output]
```

## Component Architecture

This diagram shows the relationships between major components:

```mermaid
graph TB
    subgraph "CLI Layer"
        MAIN[main.rs]
        ARGS[Args]
        OUTPUT[Output Formatter]
    end
    
    subgraph "Core Engine"
        ENGINE[Engine]
        SCANNER[Scanner]
        RESULT[SearchResult]
    end
    
    subgraph "Site Management"
        REGISTRY[SiteRegistry]
        SITE_TRAIT[Site Trait]
        SITES[Site Implementations<br/>GitHub, Twitter, etc.]
    end
    
    subgraph "Request Layer"
        REQUEST_TRAIT[Request Trait]
        HTTP_REQ[HttpRequest]
        TOR_REQ[TorRequest]
        REQ_FACTORY[create_request]
    end
    
    subgraph "Data Layer"
        SITE_TYPE[SiteType]
        SITE_INFO[SiteInfo]
    end
    
    MAIN --> ARGS
    MAIN --> ENGINE
    MAIN --> OUTPUT
    ENGINE --> REGISTRY
    ENGINE --> SCANNER
    SCANNER --> SITE_TRAIT
    SCANNER --> REQUEST_TRAIT
    REGISTRY --> SITES
    SITES --> SITE_TRAIT
    REQUEST_TRAIT --> HTTP_REQ
    REQUEST_TRAIT --> TOR_REQ
    REQ_FACTORY --> HTTP_REQ
    REQ_FACTORY --> TOR_REQ
    SCANNER --> RESULT
    REGISTRY --> SITE_TYPE
    SITES --> SITE_INFO
```

## Concurrent Execution Model

This sequence diagram illustrates how concurrent requests are executed using Tokio:

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Engine
    participant Scanner
    participant Site1 as Site (GitHub)
    participant Site2 as Site (Twitter)
    participant Request as HttpRequest
    participant Network as HTTP Network
    
    User->>CLI: sleuth username --type dev
    CLI->>Engine: search(username, types, names)
    Engine->>Engine: filter sites by type
    Engine->>Scanner: scan_username(sites, request)
    
    par Concurrent Requests
        Scanner->>Site1: build_url(username)
        Site1-->>Scanner: "https://github.com/username"
        Scanner->>Request: request("HEAD", url1)
        Request->>Network: HTTP HEAD request
        Network-->>Request: Response (200/404)
        Request-->>Scanner: RequestResponse
        Scanner->>Scanner: Parse status code
        Scanner-->>Scanner: SearchResult
    and
        Scanner->>Site2: build_url(username)
        Site2-->>Scanner: "https://twitter.com/username"
        Scanner->>Request: request("HEAD", url2)
        Request->>Network: HTTP HEAD request
        Network-->>Request: Response (200/404)
        Request-->>Scanner: RequestResponse
        Scanner->>Scanner: Parse status code
        Scanner-->>Scanner: SearchResult
    end
    
    Scanner-->>Engine: Vec<SearchResult>
    Engine-->>CLI: Vec<SearchResult>
    CLI->>CLI: format output
    CLI-->>User: Display results
```

## Core Components

### CLI Module (`src/cli/`)
- Argument parsing with `clap`
- Output formatting (colored terminal, JSON, CSV)
- Progress indicators

### Core Engine (`src/core/`)
- `engine.rs`: Orchestrates searches across sites
- `scanner.rs`: Concurrent scanning logic using Tokio tasks
- `result.rs`: Result types and aggregators

### Sites Module (`src/sites/`)
- Trait-based design for site checkers
- Organized by type in subdirectories (dev, social, professional, gaming, forum, nsfw, other)
- Each site implements the `Site` trait with:
  - `build_url()`: Constructs the URL to check
  - `parse_response()`: Interprets HTTP response (default: 200=exists, 404=not found)
  - `http_method()`: Returns HTTP method to use (default: HEAD)
  - `headers()`: Returns custom headers if needed
- Easy to add new sites by creating files in the appropriate type directory
- All sites are automatically registered via `sites::all_sites()`

### Request Module (`src/request/`)
- Abstraction over HTTP clients via `Request` trait
- `HttpRequest`: Implementation using `reqwest`
- `TorRequest`: Placeholder for future Tor support
- Factory function `create_request()` for creating request implementations
- Allows swapping implementations (HTTP, Tor, etc.) without changing site code

### Configuration (`src/config/`)
- Site registry loaded from data files
- User agent rotation
- Timeout and retry configuration

## Design Patterns

- **Trait-based site checkers**: Each site implements a `Site` trait for consistency
- **Request abstraction**: `Request` trait allows swapping HTTP implementations
- **Async/await**: Concurrent requests using Tokio tasks
- **Error handling**: Custom error types with `thiserror`
- **Separation of concerns**: Sites build URLs and parse responses, Scanner executes requests
- **Modularity**: Clear separation of concerns

## Data Flow

1. **User Input**: CLI parses arguments (username, filters, output format)
2. **Site Filtering**: Engine filters sites by type/name using SiteRegistry
3. **URL Construction**: Each site builds its URL using `Site::build_url()`
4. **Concurrent Requests**: Scanner spawns async tasks for each site
5. **HTTP Execution**: Request trait implementation (HttpRequest) makes actual HTTP calls
6. **Response Parsing**: Scanner parses HTTP responses (currently hardcoded, TODO: use `Site::parse_response()`)
7. **Result Aggregation**: All results collected and returned
8. **Output Formatting**: CLI formats results based on user preference (text/JSON/CSV)

## Testing

- Unit tests are co-located with source files using `#[cfg(test)] mod tests {}`
- Integration tests are in the `tests/` directory
- Benchmarks are in the `benches/` directory
