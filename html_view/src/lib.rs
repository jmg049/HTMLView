//! html_view: A lightweight, cross-platform HTML viewer for Rust.
//!
//! This library provides a simple API to display HTML content in a native window,
//! similar to `matplotlib.pyplot.show()` in Python.
//!
//! # Quick Start
//!
//! ```no_run
//! use html_view;
//!
//! fn main() -> Result<(), html_view::ViewerError> {
//!     html_view::show("<h1>Hello, World!</h1>")?;
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! - Display inline HTML, local files, directories, or remote URLs
//! - Blocking and non-blocking modes
//! - Window configuration (size, position, title)
//! - Security controls for navigation and remote content
//! - Cross-platform (Windows, macOS, Linux)

mod error;
mod launcher;
mod locator;
mod options;
mod result;

pub use error::ViewerError;
pub use locator::{AppLocator, DefaultAppLocator};
pub use options::{ViewerOptions, ViewerWaitMode};
pub use result::{ViewerHandle, ViewerResult};

// Re-export commonly used types from shared crate
pub use html_view_shared::{
    BehaviourOptions, DialogOptions, EnvironmentOptions, ToolbarOptions, ViewerContent,
    ViewerExitReason, ViewerExitStatus, WindowOptions, WindowTheme,
};

use launcher::launch_viewer;

/// Display inline HTML in a new viewer window and block until the window is closed.
///
/// This is the simplest way to show HTML content. It uses default window and
/// behaviour options with secure defaults.
///
/// # Security
///
/// By default:
/// - Remote content loading is disabled
/// - External navigation is blocked
/// - Developer tools are disabled
/// - Only the provided HTML is displayed
///
/// # Examples
///
/// Basic HTML display:
///
/// ```no_run
/// html_view::show("<h1>Hello!</h1>").unwrap();
/// ```
///
/// Display with inline styles:
///
/// ```no_run
/// html_view::show(r#"
///     <html>
///     <head>
///         <style>
///             body { font-family: Arial; padding: 40px; }
///             h1 { color: #4A90E2; }
///         </style>
///     </head>
///     <body>
///         <h1>Styled Content</h1>
///         <p>This HTML includes CSS styling.</p>
///     </body>
///     </html>
/// "#).unwrap();
/// ```
///
/// Generate HTML dynamically:
///
/// ```no_run
/// let items = vec!["Item 1", "Item 2", "Item 3"];
/// let html = format!(
///     "<h1>My List</h1><ul>{}</ul>",
///     items.iter()
///         .map(|item| format!("<li>{}</li>", item))
///         .collect::<String>()
/// );
/// html_view::show(html).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the viewer binary cannot be found or launched, or if
/// there's an I/O error during the process.
///
/// Common error scenarios:
/// - [`ViewerError::BinaryNotFound`]: The `html_view_app` binary is not installed
/// - [`ViewerError::SpawnFailed`]: Failed to start the viewer process
/// - [`ViewerError::VersionMismatch`]: Library and viewer versions are incompatible
///
/// # See Also
///
/// - [`show_with_options`] for custom window configuration
/// - [`open`] for more advanced usage with full option control
pub fn show<S: Into<String>>(html: S) -> Result<(), ViewerError> {
    let options = ViewerOptions::inline_html(html);
    match open(options)? {
        ViewerResult::Blocking(_status) => Ok(()),
        ViewerResult::NonBlocking(_) => unreachable!("inline_html uses Blocking mode"),
    }
}

/// Display HTML with custom window configuration.
///
/// This is a convenience function that allows you to customize the window
/// while using the simple blocking API.
///
/// # Examples
///
/// Custom window size and title:
///
/// ```no_run
/// use html_view::{show_with_options, WindowOptions};
///
/// let mut window = WindowOptions::default();
/// window.width = Some(800);
/// window.height = Some(600);
/// window.title = Some("My Custom Window".to_string());
///
/// show_with_options("<h1>Custom Window</h1>", window).unwrap();
/// ```
///
/// Frameless window:
///
/// ```no_run
/// use html_view::{show_with_options, WindowOptions};
///
/// let mut window = WindowOptions::default();
/// window.decorations = false;
/// window.always_on_top = true;
///
/// show_with_options("<h1>Always on Top</h1>", window).unwrap();
/// ```
///
/// # Errors
///
/// See [`show`] for error documentation.
pub fn show_with_options<S: Into<String>>(
    html: S,
    window_options: WindowOptions,
) -> Result<(), ViewerError> {
    let mut options = ViewerOptions::inline_html(html);
    options.window = window_options;

    match open(options)? {
        ViewerResult::Blocking(_status) => Ok(()),
        ViewerResult::NonBlocking(_) => unreachable!("inline_html uses Blocking mode"),
    }
}

/// Open a viewer window with the given options.
///
/// This is the most flexible way to use html_view. It provides full control over
/// all window, behavior, and environment options.
///
/// Returns either a blocking result with the exit status or a non-blocking
/// handle, depending on `options.wait`.
///
/// # Examples
///
/// Non-blocking mode with handle:
///
/// ```no_run
/// use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};
///
/// let mut options = ViewerOptions::inline_html("<h1>Non-blocking</h1>");
/// options.wait = ViewerWaitMode::NonBlocking;
///
/// match html_view::open(options).unwrap() {
///     ViewerResult::NonBlocking(mut handle) => {
///         println!("Viewer started with ID: {}", handle.id);
///
///         // Do other work while viewer is open...
///         std::thread::sleep(std::time::Duration::from_secs(2));
///
///         // Wait for it to close
///         let status = handle.wait().unwrap();
///         println!("Viewer exited: {:?}", status.reason);
///     }
///     _ => unreachable!(),
/// }
/// ```
///
/// With timeout:
///
/// ```no_run
/// use html_view::ViewerOptions;
///
/// let mut options = ViewerOptions::inline_html("<h1>Auto-close</h1>");
/// options.environment.timeout_seconds = Some(5);
///
/// html_view::open(options).unwrap();
/// // Window closes automatically after 5 seconds
/// ```
///
/// Loading a file with devtools:
///
/// ```no_run
/// use html_view::ViewerOptions;
/// use std::path::PathBuf;
///
/// let mut options = ViewerOptions::local_file(PathBuf::from("index.html"));
/// options.behaviour.enable_devtools = true;
/// options.window.width = Some(1200);
/// options.window.height = Some(800);
///
/// html_view::open(options).unwrap();
/// ```
///
/// Remote URL with security settings:
///
/// ```no_run
/// use html_view::ViewerOptions;
/// use url::Url;
///
/// let mut options = ViewerOptions::remote_url(
///     Url::parse("https://example.com").unwrap()
/// );
/// options.behaviour.allow_external_navigation = true;
/// options.behaviour.allowed_domains = Some(vec![
///     "example.com".to_string(),
///     "cdn.example.com".to_string(),
/// ]);
///
/// html_view::open(options).unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the viewer binary cannot be found or launched, or if
/// there's an I/O error during the process.
///
/// See [`ViewerError`] for all possible error types.
///
/// # See Also
///
/// - [`show`] for the simplest API
/// - [`ViewerOptions`] for all configuration options
/// - [`ViewerWaitMode`] for blocking vs non-blocking behavior
pub fn open(options: ViewerOptions) -> Result<ViewerResult, ViewerError> {
    launch_viewer(options, &DefaultAppLocator)
}
