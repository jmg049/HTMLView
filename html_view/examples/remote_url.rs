//! Example showing how to display remote URLs with security controls.
//!
//! This example demonstrates:
//! - Loading remote web content
//! - Security settings for remote content
//! - Navigation controls and domain allowlisting
//! - Error handling for network issues

use html_view::ViewerOptions;
use url::Url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Remote URL Example");
    println!("==================\n");

    // Example 1: Simple remote URL (requires explicit permission)
    println!("Opening Rust documentation...\n");

    let url = Url::parse("https://doc.rust-lang.org/book/")?;
    let mut options = ViewerOptions::remote_url(url);

    // Configure security settings
    options.behaviour.allow_remote_content = true;
    options.behaviour.allow_external_navigation = true;

    // Optional: Restrict navigation to specific domains
    options.behaviour.allowed_domains = Some(vec![
        "doc.rust-lang.org".to_string(),
        "rust-lang.org".to_string(),
    ]);

    // Window configuration
    options.window.title = Some("Rust Documentation".to_string());
    options.window.width = Some(1200);
    options.window.height = Some(900);

    // Enable devtools for debugging network requests
    options.behaviour.enable_devtools = true;

    println!("Security settings:");
    println!("  - Remote content: ENABLED");
    println!("  - External navigation: ENABLED");
    println!("  - Allowed domains: rust-lang.org, doc.rust-lang.org");
    println!("  - DevTools: ENABLED\n");

    println!("Window will open shortly...");
    println!("You can navigate within allowed domains.");
    println!("Press F12 to open DevTools and inspect network requests.\n");

    // Open the viewer
    html_view::open(options)?;

    println!("Viewer closed!");

    Ok(())
}
