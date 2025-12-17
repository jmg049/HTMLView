//! Shared types and protocol definitions for html_view.
//!
//! This crate defines the wire protocol between the API crate and the Tauri app,
//! including all request and response types that cross the process boundary.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;

/// Complete request structure sent to the Tauri viewer application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerRequest {
    /// Unique identifier for this viewer instance.
    pub id: Uuid,

    /// The content to display in the viewer.
    pub content: ViewerContent,

    /// Window configuration options.
    pub window: WindowOptions,

    /// Behaviour and security options.
    pub behaviour: BehaviourOptions,

    /// Environment and runtime options.
    pub environment: EnvironmentOptions,

    /// Dialog configuration.
    pub dialog: DialogOptions,
}

/// The type of content to display in the viewer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ViewerContent {
    /// Plain inline HTML string.
    InlineHtml {
        /// The HTML content to display.
        html: String,

        /// Optional base directory used to resolve relative paths in the HTML,
        /// for example when the HTML refers to local assets.
        base_dir: Option<PathBuf>,
    },

    /// A single local file, usually an HTML file.
    LocalFile {
        /// Path to the HTML file.
        path: PathBuf,
    },

    /// A directory that contains a static HTML application, such as
    /// index.html, JS bundles, CSS, and assets.
    AppDir {
        /// Root directory of the application.
        root: PathBuf,

        /// The entry HTML file relative to root, defaults to "index.html".
        entry: Option<String>,
    },

    /// A remote URL. Only allowed if enabled in BehaviourOptions.
    RemoteUrl {
        /// The URL to load.
        url: Url,
    },
}

/// Window configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowOptions {
    /// Window title. If None, a default title is used.
    pub title: Option<String>,

    /// Window width in logical pixels.
    pub width: Option<u32>,

    /// Window height in logical pixels.
    pub height: Option<u32>,

    /// Initial X position.
    pub x: Option<i32>,

    /// Initial Y position.
    pub y: Option<i32>,

    /// Whether the window can be resized.
    pub resizable: bool,

    /// Whether the window starts maximised.
    pub maximised: bool,

    /// Whether the window starts in fullscreen mode.
    pub fullscreen: bool,

    /// Whether to show window decorations (title bar, border).
    pub decorations: bool,

    /// Whether the window background is transparent.
    pub transparent: bool,

    /// Whether the window should always be on top of other windows.
    pub always_on_top: bool,

    /// Window theme ("light", "dark", or "system").
    pub theme: Option<String>,

    /// Background color in hex format (e.g., "#FFFFFF").
    pub background_color: Option<String>,

    /// Toolbar configuration.
    pub toolbar: ToolbarOptions,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: Some("HTML Viewer".to_string()),
            width: Some(1024),
            height: Some(768),
            x: None,
            y: None,
            resizable: true,
            maximised: false,
            fullscreen: false,
            decorations: true,
            transparent: false,
            always_on_top: false,
            theme: None,
            background_color: None,
            toolbar: ToolbarOptions::default(),
        }
    }
}

/// Toolbar configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ToolbarOptions {
    /// Whether to show the custom toolbar.
    pub show: bool,

    /// Title text to display in the toolbar.
    pub title_text: Option<String>,

    /// Background color of the toolbar (hex).
    pub background_color: Option<String>,

    /// Text color of the toolbar (hex).
    pub text_color: Option<String>,

    /// List of buttons to show in the toolbar.
    pub buttons: Vec<ToolbarButton>,
}


/// A button in the custom toolbar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolbarButton {
    /// Unique identifier for the button action.
    pub id: String,

    /// Text to display on the button.
    pub label: String,

    /// Optional icon name (e.g. from a standard set).
    pub icon: Option<String>,
}

/// Behaviour and security configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct BehaviourOptions {
    /// Whether navigation away from the initial content is allowed.
    pub allow_external_navigation: bool,

    /// Optional allowlist of hostnames that can be navigated to.
    /// Only applies if allow_external_navigation is true.
    pub allowed_domains: Option<Vec<String>>,

    /// Whether devtools are enabled.
    pub enable_devtools: bool,

    /// Whether remote URL loading is permitted at all.
    pub allow_remote_content: bool,

    /// Whether system notifications are allowed.
    pub allow_notifications: bool,
}


/// Dialog configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct DialogOptions {
    /// Whether file dialogs are allowed.
    pub allow_file_dialogs: bool,

    /// Whether message dialogs are allowed.
    pub allow_message_dialogs: bool,
}


/// Environment and runtime configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct EnvironmentOptions {
    /// Optional working directory for resolving relative paths.
    pub working_dir: Option<PathBuf>,

    /// Optional timeout in seconds after which the viewer will auto-close.
    pub timeout_seconds: Option<u64>,
}


/// Exit status returned by the viewer application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerExitStatus {
    /// The unique identifier matching the original request.
    pub id: Uuid,

    /// The reason the viewer exited.
    pub reason: ViewerExitReason,
}

/// The reason the viewer exited.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "reason", rename_all = "snake_case")]
pub enum ViewerExitReason {
    /// The user closed the window.
    ClosedByUser,

    /// The viewer timed out according to EnvironmentOptions.
    TimedOut,

    /// An error occurred.
    Error {
        /// Error message.
        message: String,
    },
}
