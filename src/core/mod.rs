//! Core engine functionality

pub mod engine;
pub mod result;
pub mod scanner;

pub use engine::Engine;
pub use result::SearchResult;
pub use scanner::scan_username;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_module() {
        // Core module tests
        assert!(true);
    }
}
