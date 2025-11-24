//! Result data structures

// Re-export from core for convenience
pub use crate::core::result::SearchResult;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_re_export() {
        // Test re-export
        assert!(true);
    }
}
