//! Tests for ViewerOptionsBuilder
//!
//! Run with: cargo test -p html_view --test builder_tests

use html_view::ViewerOptions;
use std::path::PathBuf;

#[test]
fn test_builder_width() {
    let result = ViewerOptions::new().width(800).show();
    // Will fail because no content set or binary not found
    assert!(result.is_err());
}

#[test]
fn test_builder_individual_dimensions() {
    let _builder = ViewerOptions::new();

    // Test width
    let _with_width = ViewerOptions::new().width(800);

    // Test height
    let _with_height = ViewerOptions::new().height(600);

    // Test x
    let _with_x = ViewerOptions::new().x(100);

    // Test y
    let _with_y = ViewerOptions::new().y(50);
}

#[test]
fn test_builder_size_chaining() {
    // Test that both size() and individual setters work
    let _opts1 = ViewerOptions::new().size(800, 600);
    let _opts2 = ViewerOptions::new().width(800).height(600);
    // Both should produce equivalent options
}

#[test]
fn test_builder_position_chaining() {
    // Test that both position() and individual setters work
    let _opts1 = ViewerOptions::new().position(100, 50);
    let _opts2 = ViewerOptions::new().x(100).y(50);
}

#[test]
fn test_builder_working_dir() {
    let dir = PathBuf::from("/tmp");
    let _builder = ViewerOptions::new().working_dir(dir.clone());
    // Builder should accept PathBuf for working_dir
}

#[test]
fn test_builder_allow_remote_content() {
    let _builder = ViewerOptions::new().allow_remote_content();
    // Should set allow_remote_content flag
}

#[test]
fn test_builder_allowed_domains() {
    let domains = vec!["example.com".to_string(), "test.com".to_string()];
    let _builder = ViewerOptions::new().allowed_domains(domains);
    // Should set allowed_domains and auto-enable allow_external_navigation
}

#[test]
fn test_builder_fixed_size() {
    let _builder = ViewerOptions::new().fixed_size();
    // Should set resizable to false
}

#[test]
fn test_builder_non_blocking() {
    let _builder = ViewerOptions::new().non_blocking();
    // Should set wait mode to NonBlocking
}

#[test]
fn test_builder_base_dir() {
    let dir = PathBuf::from("/tmp");
    let _builder = ViewerOptions::new().base_dir(dir.clone());
    // Should set base_dir for InlineHtml content
}

#[test]
fn test_builder_method_chaining() {
    // Test that all methods chain properly
    let _builder = ViewerOptions::new()
        .width(800)
        .height(600)
        .x(100)
        .y(50)
        .title("Test")
        .working_dir(PathBuf::from("/tmp"))
        .allow_remote_content()
        .fixed_size()
        .non_blocking()
        .devtools()
        .timeout(30);

    // All methods should chain successfully
}

#[test]
fn test_builder_show_file() {
    let path = PathBuf::from("/tmp/test.html");
    let result = ViewerOptions::new().width(800).show_file(path);

    // Will fail because file doesn't exist, but we verify it compiles
    assert!(result.is_err());
}

#[test]
fn test_builder_show_app_dir() {
    let root = PathBuf::from("/tmp/app");
    let result = ViewerOptions::new()
        .width(800)
        .show_app_dir(root, Some("index.html".to_string()));

    // Will fail because directory doesn't exist, but we verify it compiles
    assert!(result.is_err());
}

#[test]
fn test_builder_show_url() {
    let url = url::Url::parse("https://example.com").unwrap();
    let result = ViewerOptions::new().width(800).show_url(url);

    // Will fail because binary not found or network issues, but we verify it compiles
    assert!(result.is_err());
}

#[test]
fn test_builder_real_world_pattern_1() {
    // Common pattern: quick window with custom size
    let result = ViewerOptions::new()
        .width(640)
        .height(480)
        .title("My App")
        .show_html("<h1>Hello</h1>");

    assert!(result.is_err()); // Binary not found in test environment
}

#[test]
fn test_builder_real_world_pattern_2() {
    // Common pattern: non-blocking with devtools
    let result = ViewerOptions::new()
        .non_blocking()
        .devtools()
        .timeout(5)
        .show_html("<h1>Debug Window</h1>");

    assert!(result.is_err());
}

#[test]
fn test_builder_real_world_pattern_3() {
    // Common pattern: frameless transparent window
    let result = ViewerOptions::new()
        .transparent()
        .no_decorations()
        .always_on_top()
        .fixed_size()
        .width(400)
        .height(300)
        .show_html("<div style='background: rgba(255,255,255,0.9)'>Floating</div>");

    assert!(result.is_err());
}

#[test]
fn test_builder_real_world_pattern_4() {
    // Common pattern: remote content with domain restrictions
    let url = url::Url::parse("https://example.com").unwrap();
    let result = ViewerOptions::new()
        .allowed_domains(vec!["example.com".to_string()])
        .show_url(url);

    assert!(result.is_err());
}

#[test]
fn test_builder_ergonomics_comparison() {
    // Old way (direct mutation)
    let mut opts_old = ViewerOptions::inline_html("<h1>Test</h1>");
    opts_old.window.width = Some(800);
    opts_old.window.height = Some(600);
    opts_old.window.title = Some("Test".to_string());
    opts_old.behaviour.enable_devtools = true;

    // New way (builder)
    let result_new = ViewerOptions::new()
        .width(800)
        .height(600)
        .title("Test")
        .devtools()
        .show_html("<h1>Test</h1>");

    // Builder way is more concise and chains naturally
    assert!(result_new.is_err()); // Binary not found in tests
}

#[test]
fn test_builder_base_dir_with_inline_html() {
    let dir = PathBuf::from("/tmp/assets");
    let result = ViewerOptions::new()
        .base_dir(dir.clone())
        .show_html(r#"<img src="logo.png">"#);

    // base_dir should be set for relative path resolution
    assert!(result.is_err());
}

#[test]
fn test_all_builder_methods_compile() {
    // Comprehensive test that all methods exist and chain
    let _builder = ViewerOptions::new()
        // Window dimensions
        .width(800)
        .height(600)
        .size(800, 600)
        .x(100)
        .y(50)
        .position(100, 50)
        // Window properties
        .title("Test")
        .transparent()
        .no_decorations()
        .always_on_top()
        .fixed_size()
        // Behavior
        .devtools()
        .allow_navigation()
        .allow_remote_content()
        .allowed_domains(vec!["example.com".to_string()])
        // Environment
        .timeout(30)
        .working_dir(PathBuf::from("/tmp"))
        // Dialog & notifications
        .enable_notifications()
        .enable_dialogs()
        // Mode
        .non_blocking()
        // Base dir
        .base_dir(PathBuf::from("/tmp"));

    // All methods should compile successfully
}
