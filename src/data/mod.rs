//! Data structures

pub mod search_result;
pub mod site_info;

// Re-export commonly used types
pub use site_info::{SiteInfo, SiteType};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_module() {
        // Data module tests
        assert!(true);
    }
}
