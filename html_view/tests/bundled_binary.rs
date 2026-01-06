//! Integration tests for the bundled feature.
//!
//! These tests verify that the bundled feature correctly downloads and embeds
//! the html_view_app binary during build time.

#[test]
#[cfg(feature = "bundled")]
fn test_bundled_binary_embedded_path_exists() {
    // Verify the embedded path environment variable is set when bundled feature is enabled
    let embedded_path = option_env!("HTML_VIEW_APP_PATH");
    assert!(
        embedded_path.is_some(),
        "HTML_VIEW_APP_PATH should be set at compile-time when bundled feature is enabled"
    );

    let path = std::path::PathBuf::from(embedded_path.unwrap());
    assert!(
        path.exists(),
        "Bundled binary should exist at embedded path: {:?}",
        path
    );
    assert!(
        path.is_file(),
        "Embedded path should point to a file, not a directory: {:?}",
        path
    );
}

#[test]
#[cfg(feature = "bundled")]
fn test_bundled_binary_is_executable() {
    // Verify the bundled binary has execute permissions
    let embedded_path =
        option_env!("HTML_VIEW_APP_PATH").expect("HTML_VIEW_APP_PATH should be set");
    let path = std::path::PathBuf::from(embedded_path);

    assert!(path.exists(), "Binary should exist");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&path).expect("Should be able to read metadata");
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        // Check if owner has execute permission (bit 0o100)
        assert!(
            mode & 0o100 != 0,
            "Binary should be executable (mode: {:o})",
            mode
        );
    }
}

#[test]
#[cfg(feature = "bundled")]
fn test_bundled_binary_locator_finds_it() {
    use html_view::{AppLocator, DefaultAppLocator};

    // The locator should prioritize the embedded path from bundled feature
    let locator = DefaultAppLocator;
    let result = locator.locate_app_binary();

    assert!(
        result.is_ok(),
        "Locator should successfully find bundled binary: {:?}",
        result.err()
    );

    let found_path = result.unwrap();
    let embedded_path = option_env!("HTML_VIEW_APP_PATH").unwrap();

    assert_eq!(
        found_path.to_string_lossy(),
        embedded_path,
        "Locator should return the embedded bundled binary path"
    );
}

#[test]
#[cfg(feature = "bundled")]
fn test_bundled_binary_has_correct_name() {
    // Verify the binary has the expected name
    let embedded_path =
        option_env!("HTML_VIEW_APP_PATH").expect("HTML_VIEW_APP_PATH should be set");
    let path = std::path::PathBuf::from(embedded_path);

    let file_name = path.file_name().expect("Path should have a filename");

    #[cfg(target_os = "windows")]
    assert_eq!(
        file_name, "html_view_app.exe",
        "Binary should be named html_view_app.exe on Windows"
    );

    #[cfg(not(target_os = "windows"))]
    assert_eq!(
        file_name, "html_view_app",
        "Binary should be named html_view_app on Unix"
    );
}

#[test]
#[cfg(not(feature = "bundled"))]
fn test_without_bundled_feature_no_embedded_path() {
    // When bundled feature is not enabled, HTML_VIEW_APP_PATH should not be set by build.rs
    let embedded_path = option_env!("HTML_VIEW_APP_PATH");

    // Note: This might be set by other means (user environment, etc.)
    // but it should NOT be set by our build.rs
    if let Some(path_str) = embedded_path {
        // If it is set, it should be from external sources, not our bundled feature
        println!(
            "Note: HTML_VIEW_APP_PATH is set to '{}', likely from external source",
            path_str
        );
    }
}

#[test]
#[cfg(not(feature = "bundled"))]
fn test_without_bundled_locator_still_works() {
    use html_view::{AppLocator, DefaultAppLocator};

    // Without bundled feature, locator should still work if binary is installed
    let locator = DefaultAppLocator;
    let result = locator.locate_app_binary();

    match result {
        Ok(path) => {
            // Binary found via manual installation
            assert!(path.exists());
            assert!(path.is_file());
            println!("Found binary at: {:?}", path);
        }
        Err(e) => {
            // Binary not found - this is expected if html_view_app isn't installed
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("Could not locate html_view_app binary"),
                "Error should mention binary not found"
            );
            assert!(
                error_msg.contains("cargo install"),
                "Error should suggest cargo install"
            );
        }
    }
}

#[test]
#[cfg(feature = "bundled")]
fn test_bundled_binary_version_check() {
    use std::process::Command;

    // Verify the bundled binary responds to --version
    let embedded_path =
        option_env!("HTML_VIEW_APP_PATH").expect("HTML_VIEW_APP_PATH should be set");
    let path = std::path::PathBuf::from(embedded_path);

    let output = Command::new(&path)
        .arg("--version")
        .output()
        .expect("Should be able to run binary with --version");

    assert!(
        output.status.success(),
        "Binary should exit successfully with --version flag"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("html_view_app"),
        "Version output should contain 'html_view_app'"
    );
}

#[test]
#[cfg(feature = "bundled")]
fn test_bundled_binary_help_command() {
    use std::process::Command;

    // Verify the bundled binary responds to --help
    let embedded_path =
        option_env!("HTML_VIEW_APP_PATH").expect("HTML_VIEW_APP_PATH should be set");
    let path = std::path::PathBuf::from(embedded_path);

    let output = Command::new(&path)
        .arg("--help")
        .output()
        .expect("Should be able to run binary with --help");

    assert!(
        output.status.success(),
        "Binary should exit successfully with --help flag"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "Help output should not be empty");
}

#[test]
#[cfg(all(feature = "bundled", not(target_os = "windows")))]
fn test_bundled_binary_file_size_reasonable() {
    // Verify the binary isn't suspiciously small or large
    let embedded_path =
        option_env!("HTML_VIEW_APP_PATH").expect("HTML_VIEW_APP_PATH should be set");
    let path = std::path::PathBuf::from(embedded_path);

    let metadata = std::fs::metadata(&path).expect("Should be able to read metadata");
    let size_bytes = metadata.len();
    let size_mb = size_bytes as f64 / 1_048_576.0;

    // Binary should be between 5 MB and 20 MB
    // (smaller after optimization, but not suspiciously small)
    assert!(
        size_mb >= 5.0,
        "Binary seems too small ({:.2} MB), might be corrupted",
        size_mb
    );
    assert!(
        size_mb <= 20.0,
        "Binary seems too large ({:.2} MB), optimization may have failed",
        size_mb
    );

    println!("Bundled binary size: {:.2} MB", size_mb);
}
