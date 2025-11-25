//! Sleuth CLI entry point

use clap::Parser;
use sleuth::cli::{print_results, Args};
use sleuth::core::Engine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let engine = Engine::new();
    let site_types = args.parsed_site_types();

    println!("Searching for username: {}", args.username);
    if !site_types.is_empty() {
        println!("Filtering by types: {:?}", site_types);
    }
    if !args.sites.is_empty() {
        println!("Filtering by sites: {:?}", args.sites);
    }

    let results = engine
        .search(&args.username, &site_types, &args.sites, None, args.verify)
        .await?;

    print_results(&results, &args.output_format);

    Ok(())
}
