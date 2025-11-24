//! Configuration management

pub mod sites;
pub mod timeout;
pub mod user_agent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_module() {
        // Config module tests
        assert!(true);
    }
}
