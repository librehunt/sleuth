# Browser Support for JavaScript-Rendered Sites

## Problem

Some websites (like OnlyFans, Reddit, etc.) render their content dynamically using JavaScript. When we fetch the HTML statically with a simple HTTP request, we only get the initial HTML skeleton without the actual content that JavaScript would render.

## Solution

For sites that require JavaScript rendering, we need to use a headless browser to:
1. Load the page
2. Execute JavaScript
3. Wait for content to render
4. Extract the rendered HTML

## Implementation Plan

### 1. Add Browser Request Type

Add a new `BrowserRequest` implementation that uses a headless browser (e.g., `headless_chrome` crate in Rust).

### 2. Site Trait Enhancement

Add a method to the `Site` trait to indicate if a site requires browser rendering:

```rust
fn requires_browser(&self) -> bool {
    false // Default: no browser needed
}
```

### 3. Scanner Logic

Update the scanner to:
- Check if a site requires browser rendering
- Use `BrowserRequest` for sites that need it
- Use regular `HttpRequest` for others

### 4. Performance Considerations

- Browser rendering is slower than HTTP requests
- Should be used only when necessary
- Consider caching rendered content
- May need longer timeouts

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

