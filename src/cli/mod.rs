//! Command-line interface module

pub mod args;
pub mod output;

pub use args::Args;
pub use output::print_results;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_module() {
        // CLI module tests
        assert!(true);
    }
}
