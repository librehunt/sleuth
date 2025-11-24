//! Username validation

/// Validate a username format
pub fn validate_username(username: &str) -> bool {
    !username.is_empty() && username.len() <= 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username_empty() {
        assert!(!validate_username(""));
    }

    #[test]
    fn test_validate_username_valid() {
        assert!(validate_username("testuser"));
    }

    #[test]
    fn test_validate_username_too_long() {
        let long_username = "a".repeat(101);
        assert!(!validate_username(&long_username));
    }
}
