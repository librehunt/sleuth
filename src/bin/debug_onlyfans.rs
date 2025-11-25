//! Debug script to save OnlyFans HTML for analysis

use sleuth::request::{browser::BrowserRequest, Request};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = BrowserRequest::new(30)?;
    
    // Test with valid profile (bella)
    println!("Fetching bella (valid profile)...");
    let bella_response = request.get("https://onlyfans.com/bella").await?;
    if let Some(body) = &bella_response.body {
        fs::write("/tmp/onlyfans_bella_rendered.html", body)?;
        println!("Saved bella HTML to /tmp/onlyfans_bella_rendered.html ({} bytes)", body.len());
    }
    
    // Test with invalid profile (triviere)
    println!("Fetching triviere (invalid profile)...");
    let triviere_response = request.get("https://onlyfans.com/triviere").await?;
    if let Some(body) = &triviere_response.body {
        fs::write("/tmp/onlyfans_triviere_rendered.html", body)?;
        println!("Saved triviere HTML to /tmp/onlyfans_triviere_rendered.html ({} bytes)", body.len());
    }
    
    Ok(())
}

