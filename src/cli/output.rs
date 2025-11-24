//! Output formatting for CLI

use crate::core::result::SearchResult;
use colored::*;
use serde_json;

pub fn print_results(results: &[SearchResult], format: &str) {
    match format {
        "json" => print_json(results),
        "csv" => print_csv(results),
        _ => print_text(results),
    }
}

fn print_text(results: &[SearchResult]) {
    let found: Vec<&SearchResult> = results.iter().filter(|r| r.exists).collect();
    let not_found: Vec<&SearchResult> = results.iter().filter(|r| !r.exists).collect();

    if !found.is_empty() {
        println!("\n{}", "Found:".green().bold());
        for result in &found {
            if let Some(url) = &result.url {
                println!("  {}: {}", result.site.green(), url.blue());
            } else {
                println!("  {}", result.site.green());
            }
        }
    }

    if !not_found.is_empty() {
        println!("\n{}", "Not Found:".red().bold());
        for result in &not_found {
            println!("  {}", result.site.red());
        }
    }

    println!(
        "\nTotal: {} found, {} not found",
        found.len(),
        not_found.len()
    );
}

fn print_json(results: &[SearchResult]) {
    match serde_json::to_string_pretty(results) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}

fn print_csv(results: &[SearchResult]) {
    println!("site,username,exists,url");
    for result in results {
        let url = result.url.as_deref().unwrap_or("");
        println!(
            "{},{},{},{}",
            result.site, result.username, result.exists, url
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::result::SearchResult;

    #[test]
    fn test_print_results_text() {
        let results = vec![
            SearchResult::found(
                "GitHub".to_string(),
                "test".to_string(),
                "https://github.com/test".to_string(),
            ),
            SearchResult::not_found("Twitter".to_string(), "test".to_string()),
        ];
        print_results(&results, "text");
    }

    #[test]
    fn test_print_results_text_empty() {
        let results: Vec<SearchResult> = vec![];
        print_results(&results, "text");
    }

    #[test]
    fn test_print_results_text_all_found() {
        let results = vec![
            SearchResult::found(
                "GitHub".to_string(),
                "test".to_string(),
                "https://github.com/test".to_string(),
            ),
            SearchResult::found(
                "GitLab".to_string(),
                "test".to_string(),
                "https://gitlab.com/test".to_string(),
            ),
        ];
        print_results(&results, "text");
    }

    #[test]
    fn test_print_results_text_all_not_found() {
        let results = vec![
            SearchResult::not_found("Twitter".to_string(), "test".to_string()),
            SearchResult::not_found("Facebook".to_string(), "test".to_string()),
        ];
        print_results(&results, "text");
    }

    #[test]
    fn test_print_results_json() {
        let results = vec![SearchResult::found(
            "GitHub".to_string(),
            "test".to_string(),
            "https://github.com/test".to_string(),
        )];
        print_results(&results, "json");
    }

    #[test]
    fn test_print_results_json_empty() {
        let results: Vec<SearchResult> = vec![];
        print_results(&results, "json");
    }

    #[test]
    fn test_print_results_json_multiple() {
        let results = vec![
            SearchResult::found(
                "GitHub".to_string(),
                "test".to_string(),
                "https://github.com/test".to_string(),
            ),
            SearchResult::not_found("Twitter".to_string(), "test".to_string()),
        ];
        print_results(&results, "json");
    }

    #[test]
    fn test_print_results_csv() {
        let results = vec![SearchResult::found(
            "GitHub".to_string(),
            "test".to_string(),
            "https://github.com/test".to_string(),
        )];
        print_results(&results, "csv");
    }

    #[test]
    fn test_print_results_csv_empty() {
        let results: Vec<SearchResult> = vec![];
        print_results(&results, "csv");
    }

    #[test]
    fn test_print_results_csv_no_url() {
        let results = vec![SearchResult::not_found(
            "Twitter".to_string(),
            "test".to_string(),
        )];
        print_results(&results, "csv");
    }

    #[test]
    fn test_print_results_invalid_format() {
        let results = vec![SearchResult::found(
            "GitHub".to_string(),
            "test".to_string(),
            "https://github.com/test".to_string(),
        )];
        // Should default to text format
        print_results(&results, "invalid");
    }
}
