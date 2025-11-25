//! html_view_app: The Tauri application binary for html_view.
//!
//! This binary is spawned by the html_view API crate to display HTML content.

mod app;
mod content_loader;

use clap::Parser;
use html_view_shared::{ViewerExitReason, ViewerExitStatus, ViewerRequest};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the configuration JSON file
    #[arg(long)]
    config_path: PathBuf,

    /// Path to write the result JSON file
    #[arg(long)]
    result_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // Read and parse config
    let config_data = std::fs::read_to_string(&args.config_path)?;
    let request: ViewerRequest = serde_json::from_str(&config_data)?;

    // Run the Tauri app
    let exit_status = match app::run_app(request.clone()) {
        Ok(status) => status,
        Err(e) => {
            // If the app fails, return an error status
            ViewerExitStatus {
                id: request.id,
                reason: ViewerExitReason::Error {
                    message: e.to_string(),
                },
            }
        }
    };

    // Write result
    let result_json = serde_json::to_string_pretty(&exit_status)?;
    std::fs::write(&args.result_path, result_json)?;

    Ok(())
}
