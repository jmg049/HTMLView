//! Example demonstrating comprehensive error handling patterns.
//!
//! Shows:
//! - Handling BinaryNotFound, Timeout, VersionMismatch errors
//! - Non-blocking error handling
//! - Retry logic with exponential backoff
//! - Proper error context handling
//!
//! Run with: cargo run --example error_handling

use html_view::{ViewerError, ViewerOptions, ViewerResult, ViewerWaitMode};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Error Handling Example\n");
    println!("======================\n");

    // Pattern 1: Simple error handling with context
    println!("Pattern 1: Basic error handling");
    match html_view::show("<h1>Test</h1>") {
        Ok(_) => println!("✓ Success!"),
        Err(e) => {
            println!("✗ Error occurred:");
            handle_viewer_error(e);
        }
    }
    println!();

    // Pattern 2: Timeout handling
    println!("Pattern 2: Timeout handling");
    let mut opts = ViewerOptions::inline_html(
        "<h1>Timeout Test</h1><p>This window will auto-close in 5 seconds.</p>",
    );
    opts.environment.timeout_seconds = Some(5);

    match html_view::open(opts) {
        Ok(_) => println!("✓ Completed within timeout"),
        Err(ViewerError::Timeout) => println!("✓ Window closed due to timeout (expected)"),
        Err(e) => {
            println!("✗ Unexpected error:");
            handle_viewer_error(e);
        }
    }
    println!();

    // Pattern 3: Non-blocking with error recovery
    println!("Pattern 3: Non-blocking error handling");
    match demonstrate_non_blocking_errors() {
        Ok(_) => println!("✓ Non-blocking example completed"),
        Err(e) => println!("✗ Non-blocking example failed: {}", e),
    }
    println!();

    // Pattern 4: Retry with exponential backoff
    println!("Pattern 4: Retry with exponential backoff");
    match demonstrate_retry_pattern() {
        Ok(_) => println!("✓ Retry pattern completed"),
        Err(e) => println!("✗ Retry pattern failed: {}", e),
    }

    Ok(())
}

/// Handle different ViewerError variants with appropriate context
fn handle_viewer_error(err: ViewerError) {
    match err {
        ViewerError::BinaryNotFound(msg) => {
            eprintln!("  Error: Binary not found");
            eprintln!("  Details: {}", msg);
            eprintln!("  Solution: cargo install html_view_app");
        }
        ViewerError::VersionMismatch {
            library,
            viewer,
            suggestion,
        } => {
            eprintln!("  Error: Version mismatch!");
            eprintln!("  Library version: {}", library);
            eprintln!("  Viewer version: {}", viewer);
            eprintln!("  \n  {}", suggestion);
        }
        ViewerError::Timeout => {
            eprintln!("  Error: Operation timed out");
            eprintln!("  The viewer window was automatically closed after the timeout period.");
        }
        ViewerError::SpawnFailed(msg) => {
            eprintln!("  Error: Failed to spawn viewer process");
            eprintln!("  Details: {}", msg);
            eprintln!("  Check system resources and permissions.");
        }
        ViewerError::ConfigWriteFailed(msg) => {
            eprintln!("  Error: Failed to write configuration file");
            eprintln!("  Details: {}", msg);
            eprintln!("  Check write permissions for /tmp directory.");
        }
        ViewerError::ResultReadFailed(msg) => {
            eprintln!("  Error: Failed to read result from viewer");
            eprintln!("  Details: {}", msg);
            eprintln!("  The viewer may have crashed or been terminated.");
        }
        ViewerError::InvalidResponse(msg) => {
            eprintln!("  Error: Invalid response from viewer");
            eprintln!("  Details: {}", msg);
            eprintln!("  This may indicate a protocol mismatch.");
        }
        ViewerError::IoError(e) => {
            eprintln!("  Error: I/O error occurred");
            eprintln!("  Details: {}", e);
        }
        ViewerError::SerdeError(msg) => {
            eprintln!("  Error: Serialization error");
            eprintln!("  Details: {}", msg);
            eprintln!("  This is likely a bug. Please report it.");
        }
        ViewerError::CommandTimeout { seq, timeout_secs } => {
            eprintln!(" Error: Command timeout error");
            eprintln!(" Details: seq = {}, timeout_secs = {}",seq, timeout_secs);
        },
        ViewerError::CommandFailed(msg) => {
            eprintln!(" Error: Command failed error");
            eprintln!(" Details: {}", msg)
        },
        ViewerError::RefreshNotSupported(msg) => {
            eprintln!(" Error: Refresh not supported error");
            eprintln!(" Details: {}", msg)
        },}
}

/// Demonstrate error handling in non-blocking mode
fn demonstrate_non_blocking_errors() -> Result<(), ViewerError> {
    let mut opts = ViewerOptions::inline_html(
        "<h1>Non-blocking Test</h1><p>This demonstrates error handling in non-blocking mode.</p>",
    );
    opts.wait = ViewerWaitMode::NonBlocking;
    opts.environment.timeout_seconds = Some(2);

    match html_view::open(opts)? {
        ViewerResult::NonBlocking(mut handle) => {
            println!("  Viewer started with ID: {}", handle.id);

            // Poll for completion
            loop {
                match handle.try_wait() {
                    Ok(Some(status)) => {
                        println!("  Viewer exited: {:?}", status.reason);
                        break;
                    }
                    Ok(None) => {
                        // Still running
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => {
                        eprintln!("  Error while waiting: {}", e);
                        return Err(e);
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

/// Demonstrate retry pattern with exponential backoff
fn demonstrate_retry_pattern() -> Result<(), Box<dyn std::error::Error>> {
    const MAX_RETRIES: u32 = 3;
    let mut attempt = 0;
    let mut backoff = Duration::from_millis(100);

    loop {
        attempt += 1;
        println!("  Attempt {}/{}", attempt, MAX_RETRIES);

        let mut opts = ViewerOptions::inline_html(
            "<h1>Retry Test</h1><p>Testing retry logic with exponential backoff.</p>",
        );
        opts.environment.timeout_seconds = Some(2);

        match html_view::open(opts) {
            Ok(_) => {
                println!("  ✓ Success on attempt {}", attempt);
                break;
            }
            Err(e) if attempt < MAX_RETRIES => {
                eprintln!("  ✗ Attempt {} failed: {}", attempt, e);
                println!("  Retrying in {:?}...", backoff);
                std::thread::sleep(backoff);
                backoff *= 2; // Exponential backoff
            }
            Err(e) => {
                eprintln!("  ✗ Failed after {} attempts: {}", MAX_RETRIES, e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}
