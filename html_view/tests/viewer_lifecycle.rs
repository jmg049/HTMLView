//! Integration tests for the full viewer lifecycle
//!
//! These tests require the html_view_app binary to be built.
//! They will be skipped if the binary is not available.

use html_view::{ViewerOptions, ViewerWaitMode};
use html_view_shared::ViewerContent;

#[test]
#[ignore] // Run with: cargo test --ignored
fn test_viewer_inline_html_blocking() {
    // This test requires the html_view_app binary
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test</title></head>
        <body><h1>Integration Test</h1></body>
        </html>
    "#;

    let mut options = ViewerOptions::inline_html(html);
    options.environment.timeout_seconds = Some(1); // Auto-close after 1 second
    options.window.width = Some(400);
    options.window.height = Some(300);

    // This should complete successfully after ~1 second
    let result = html_view::open(options);

    match result {
        Ok(_) => println!("Viewer lifecycle test passed"),
        Err(e) => {
            if e.to_string().contains("binary not found") {
                println!("Skipping test: html_view_app not available");
                println!("Run 'cargo build' in html_view_app directory first");
            } else {
                panic!("Viewer lifecycle test failed: {}", e);
            }
        }
    }
}

#[test]
#[ignore]
fn test_viewer_non_blocking() {
    let html = "<h1>Non-blocking Test</h1>";

    let mut options = ViewerOptions::inline_html(html);
    options.wait = ViewerWaitMode::NonBlocking;
    options.environment.timeout_seconds = Some(2);
    options.window.width = Some(300);
    options.window.height = Some(200);

    let result = html_view::open(options);

    match result {
        Ok(viewer_result) => {
            if let html_view::ViewerResult::NonBlocking(handle) = viewer_result {
                println!("Got handle, waiting for completion...");
                match handle.wait() {
                    Ok(status) => {
                        println!("Viewer exited: {:?}", status.reason);
                    }
                    Err(e) => {
                        panic!("Wait failed: {}", e);
                    }
                }
            } else {
                panic!("Expected NonBlocking result");
            }
        }
        Err(e) => {
            if e.to_string().contains("binary not found") {
                println!("Skipping test: html_view_app not available");
            } else {
                panic!("Non-blocking test failed: {}", e);
            }
        }
    }
}

#[test]
#[ignore]
fn test_viewer_try_wait() {
    use std::thread;
    use std::time::Duration;

    let html = "<h1>Try Wait Test</h1>";

    let mut options = ViewerOptions::inline_html(html);
    options.wait = ViewerWaitMode::NonBlocking;
    options.environment.timeout_seconds = Some(3);

    let result = html_view::open(options);

    match result {
        Ok(viewer_result) => {
            if let html_view::ViewerResult::NonBlocking(mut handle) = viewer_result {
                // Poll for completion
                let mut attempts = 0;
                loop {
                    attempts += 1;
                    match handle.try_wait() {
                        Ok(Some(status)) => {
                            println!(
                                "Viewer completed after {} attempts: {:?}",
                                attempts, status.reason
                            );
                            break;
                        }
                        Ok(None) => {
                            println!("Attempt {}: Still running...", attempts);
                            thread::sleep(Duration::from_millis(500));
                        }
                        Err(e) => {
                            panic!("try_wait failed: {}", e);
                        }
                    }

                    if attempts > 10 {
                        panic!("try_wait took too long");
                    }
                }
            } else {
                panic!("Expected NonBlocking result");
            }
        }
        Err(e) => {
            if e.to_string().contains("binary not found") {
                println!("Skipping test: html_view_app not available");
            } else {
                panic!("Try wait test failed: {}", e);
            }
        }
    }
}

#[test]
#[ignore]
fn test_viewer_terminate() {
    use std::thread;
    use std::time::Duration;

    let html = "<h1>Terminate Test</h1>";

    let mut options = ViewerOptions::inline_html(html);
    options.wait = ViewerWaitMode::NonBlocking;
    // No timeout - we'll terminate it manually
    options.window.width = Some(300);
    options.window.height = Some(200);

    let result = html_view::open(options);

    match result {
        Ok(viewer_result) => {
            if let html_view::ViewerResult::NonBlocking(mut handle) = viewer_result {
                // Give it time to start
                thread::sleep(Duration::from_millis(500));

                // Terminate it
                match handle.terminate() {
                    Ok(_) => println!("Viewer terminated successfully"),
                    Err(e) => panic!("Terminate failed: {}", e),
                }

                // Verify it's no longer running
                match handle.try_wait() {
                    Ok(Some(_)) => println!("Viewer confirmed terminated"),
                    Ok(None) => panic!("Viewer still running after terminate"),
                    Err(e) => {
                        // On some platforms, trying to wait after kill might error
                        println!("try_wait after terminate: {}", e);
                    }
                }
            } else {
                panic!("Expected NonBlocking result");
            }
        }
        Err(e) => {
            if e.to_string().contains("binary not found") {
                println!("Skipping test: html_view_app not available");
            } else {
                panic!("Terminate test failed: {}", e);
            }
        }
    }
}

#[test]
#[ignore]
fn test_viewer_with_devtools() {
    let html = "<h1>DevTools Test</h1><script>console.log('DevTools enabled');</script>";

    let mut options = ViewerOptions::inline_html(html);
    options.behaviour.enable_devtools = true;
    options.environment.timeout_seconds = Some(2);

    let result = html_view::open(options);

    match result {
        Ok(_) => println!("DevTools test completed"),
        Err(e) => {
            if e.to_string().contains("binary not found") {
                println!("Skipping test: html_view_app not available");
            } else {
                panic!("DevTools test failed: {}", e);
            }
        }
    }
}

#[test]
#[ignore]
fn test_viewer_window_options() {
    let html = "<h1>Window Options Test</h1>";

    let mut options = ViewerOptions::inline_html(html);
    options.window.width = Some(800);
    options.window.height = Some(600);
    options.window.resizable = true;
    options.window.title = Some("Integration Test Window".to_string());
    options.environment.timeout_seconds = Some(1);

    let result = html_view::open(options);

    match result {
        Ok(_) => println!("Window options test completed"),
        Err(e) => {
            if e.to_string().contains("binary not found") {
                println!("Skipping test: html_view_app not available");
            } else {
                panic!("Window options test failed: {}", e);
            }
        }
    }
}

#[test]
#[ignore]
fn test_viewer_version_check() {
    // This test verifies that version checking works correctly
    let html = "<h1>Version Check Test</h1>";

    let mut options = ViewerOptions::inline_html(html);
    options.environment.timeout_seconds = Some(1);

    let result = html_view::open(options);

    match result {
        Ok(_) => {
            println!("Version check passed - versions are compatible");
        }
        Err(e) => {
            if e.to_string().contains("version mismatch") {
                println!(
                    "Version mismatch detected (expected if library and viewer versions differ)"
                );
                println!("Error: {}", e);
            } else if e.to_string().contains("binary not found") {
                println!("Skipping test: html_view_app not available");
            } else {
                panic!("Version check test failed: {}", e);
            }
        }
    }
}

#[test]
fn test_viewer_content_variants() {
    // Test that all ViewerContent variants can be constructed
    use std::path::PathBuf;
    use url::Url;

    let inline = ViewerContent::InlineHtml {
        html: "<h1>Test</h1>".to_string(),
        base_dir: None,
    };
    assert!(matches!(inline, ViewerContent::InlineHtml { .. }));

    let file = ViewerContent::LocalFile {
        path: PathBuf::from("/tmp/test.html"),
    };
    assert!(matches!(file, ViewerContent::LocalFile { .. }));

    let app_dir = ViewerContent::AppDir {
        root: PathBuf::from("/tmp/app"),
        entry: Some("index.html".to_string()),
    };
    assert!(matches!(app_dir, ViewerContent::AppDir { .. }));

    let url = ViewerContent::RemoteUrl {
        url: Url::parse("https://example.com").unwrap(),
    };
    assert!(matches!(url, ViewerContent::RemoteUrl { .. }));
}

#[test]
fn test_viewer_options_builder_pattern() {
    // Test that ViewerOptions can be constructed and modified
    let mut options = ViewerOptions::inline_html("<h1>Test</h1>");

    // Modify various options
    options.window.width = Some(1024);
    options.window.height = Some(768);
    options.window.title = Some("Test Window".to_string());
    options.behaviour.enable_devtools = true;
    options.environment.timeout_seconds = Some(30);
    options.wait = ViewerWaitMode::NonBlocking;

    // Verify the modifications
    assert_eq!(options.window.width, Some(1024));
    assert_eq!(options.window.height, Some(768));
    assert_eq!(options.window.title, Some("Test Window".to_string()));
    assert!(options.behaviour.enable_devtools);
    assert_eq!(options.environment.timeout_seconds, Some(30));
    assert!(matches!(options.wait, ViewerWaitMode::NonBlocking));
}
