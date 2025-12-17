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
    ViewerExitReason, ViewerExitStatus, WindowOptions,
};

use launcher::launch_viewer;

/// Display inline HTML in a new viewer window and block until the window is closed.
///
/// This is the simplest way to show HTML content. It uses default window and
/// behaviour options with secure defaults.
///
/// # Example
///
/// ```no_run
/// html_view::show("<h1>Hello!</h1>").unwrap();
/// ```
///
/// # Errors
///
/// Returns an error if the viewer binary cannot be found or launched, or if
/// there's an I/O error during the process.
pub fn show<S: Into<String>>(html: S) -> Result<(), ViewerError> {
    let options = ViewerOptions::inline_html(html);
    match open(options)? {
        ViewerResult::Blocking(_status) => Ok(()),
        ViewerResult::NonBlocking(_) => unreachable!("inline_html uses Blocking mode"),
    }
}


pub fn show_with_options<S: Into<String>>(html: S, window_options: WindowOptions) -> Result<(), ViewerError> {
    let mut options = ViewerOptions::inline_html(html);
    options.window = window_options;

    match open(options)? {
        ViewerResult::Blocking(_status) => Ok(()),
        ViewerResult::NonBlocking(_) => unreachable!("inline_html uses Blocking mode"),
    }
}

/// Open a viewer window with the given options.
///
/// Returns either a blocking result with the exit status or a non-blocking
/// handle, depending on `options.wait`.
///
/// # Example
///
/// ```no_run
/// use html_view::{ViewerOptions, ViewerWaitMode};
///
/// let mut options = ViewerOptions::inline_html("<h1>Test</h1>");
/// options.wait = ViewerWaitMode::NonBlocking;
///
/// match html_view::open(options).unwrap() {
///     html_view::ViewerResult::NonBlocking(mut handle) => {
///         // Do other work...
///         let status = handle.wait().unwrap();
///         println!("Viewer exited: {:?}", status);
///     }
///     _ => unreachable!(),
/// }
/// ```
///
/// # Errors
///
/// Returns an error if the viewer binary cannot be found or launched, or if
/// there's an I/O error during the process.
pub fn open(options: ViewerOptions) -> Result<ViewerResult, ViewerError> {
    launch_viewer(options, &DefaultAppLocator)
}
