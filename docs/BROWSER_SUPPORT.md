# Browser Support for JavaScript-Rendered Sites

## Problem

Some websites (like OnlyFans, Reddit, etc.) render their content dynamically using JavaScript. When we fetch the HTML statically with a simple HTTP request, we only get the initial HTML skeleton without the actual content that JavaScript would render.

## Solution: Two-Pass Scanning Strategy

Sleuth uses a two-pass scanning strategy to balance speed and accuracy:

### First Pass (Always)
- Uses fast HTTP HEAD requests for all sites (~0.5s per site)
- Sites that `requires_browser()` and return 200 with HEAD are marked as "found" (to be verified)
- Most sites work correctly with HEAD and provide immediate results

### Second Pass (If `--verify` is enabled)
- Verifies found results using headless browser rendering
- Only checks sites that:
  - Were found in the first pass (`exists == true`)
  - Require browser rendering (`requires_browser() == true`)
- Corrects false positives from HEAD-only detection
- Slower (~4s per site) but more accurate for JavaScript-rendered content

## Implementation

### 1. Browser Request Type

A `BrowserRequest` implementation uses `headless_chrome` to:
1. Load the page
2. Execute JavaScript
3. Wait for content to render (2 seconds)
4. Extract the rendered HTML

### 2. Site Trait Enhancement

Sites indicate if they require browser rendering:

```rust
fn requires_browser(&self) -> bool {
    true // OnlyFans, Reddit, etc. need browser rendering
}
```

### 3. Scanner Logic

The scanner automatically:
- Uses HTTP HEAD for all sites in the first pass
- If `--verify` is enabled, performs a second pass with browser for found sites that require it
- Updates results with verified findings

### 4. Performance Considerations

- Browser rendering is slower than HTTP requests (~4s vs ~0.5s)
- Only used when `--verify` is enabled and for sites that require it
- Two-pass approach minimizes browser usage while maintaining accuracy

## Rust Libraries

Options for headless browser in Rust:
- `headless_chrome` - Direct Chrome headless (requires Chrome/Chromium)
- `fantoccini` - WebDriver client (requires WebDriver server)
- `thirtyfour` - WebDriver client (similar to fantoccini)

## Example Usage

```rust
impl Site for OnlyFansChecker {
    // ... other methods ...
    
    fn requires_browser(&self) -> bool {
        true // OnlyFans needs browser rendering
    }
}
```

