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

    // Test/CI shortcut: when `HTML_VIEW_CI_FAKE=1` is set, skip launching
    // the real Tauri UI (which requires frontend assets and a display) and
    // instead simulate a viewer run. This allows CI to execute the binary
    // in headless environments for smoke tests.
    if std::env::var("HTML_VIEW_CI_FAKE").is_ok() {
        use std::thread::sleep;
        use std::time::Duration;

        let timeout = request.environment.timeout_seconds.unwrap_or(0);
        if timeout > 0 {
            sleep(Duration::from_secs(timeout));
            let exit_status = ViewerExitStatus {
                id: request.id,
                reason: html_view_shared::ViewerExitReason::TimedOut,
            };

            let result_json = serde_json::to_string_pretty(&exit_status)?;
            std::fs::write(&args.result_path, result_json)?;
            return Ok(());
        } else {
            // No timeout configured — immediately return ClosedByUser
            let exit_status = ViewerExitStatus {
                id: request.id,
                reason: html_view_shared::ViewerExitReason::ClosedByUser,
            };
            let result_json = serde_json::to_string_pretty(&exit_status)?;
            std::fs::write(&args.result_path, result_json)?;
            return Ok(());
        }
    }

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
