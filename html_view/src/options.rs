use html_view_shared::{BehaviourOptions, EnvironmentOptions, ViewerContent, WindowOptions};

/// Options for configuring a viewer instance.
///
/// This struct provides full control over how the viewer window behaves, what content
/// it displays, and how it interacts with the user.
///
/// # Examples
///
/// Creating options for different content types:
///
/// ```
/// use html_view::ViewerOptions;
/// use std::path::PathBuf;
///
/// // Inline HTML
/// let opts1 = ViewerOptions::inline_html("<h1>Hello</h1>");
///
/// // Local file
/// let opts2 = ViewerOptions::local_file(PathBuf::from("index.html"));
///
/// // Application directory
/// let opts3 = ViewerOptions::app_dir(PathBuf::from("./dist"));
/// ```
///
/// Customizing options:
///
/// ```
/// use html_view::ViewerOptions;
///
/// let mut opts = ViewerOptions::inline_html("<h1>Custom</h1>");
/// opts.window.width = Some(800);
/// opts.window.height = Some(600);
/// opts.window.title = Some("My App".to_string());
/// opts.behaviour.enable_devtools = true;
/// opts.environment.timeout_seconds = Some(30);
/// ```
#[derive(Debug, Clone)]
pub struct ViewerOptions {
    /// The content to display.
    ///
    /// See [`ViewerContent`] for all supported content types.
    pub content: ViewerContent,

    /// Window configuration.
    ///
    /// Controls size, position, decorations, and visual appearance.
    pub window: WindowOptions,

    /// Behaviour and security options.
    ///
    /// Controls navigation, remote content, devtools, and notifications.
    pub behaviour: BehaviourOptions,

    /// Environment options.
    ///
    /// Controls working directory and timeout behavior.
    pub environment: EnvironmentOptions,

    /// Dialog configuration.
    ///
    /// Controls whether file and message dialogs are allowed.
    pub dialog: html_view_shared::DialogOptions,

    /// Whether to wait for the viewer to close.
    ///
    /// In [`ViewerWaitMode::Blocking`] mode, the call blocks until the window closes.
    /// In [`ViewerWaitMode::NonBlocking`] mode, returns immediately with a handle.
    pub wait: ViewerWaitMode,
}

/// Determines whether the viewer call blocks or returns immediately.
///
/// # Examples
///
/// Blocking mode (default):
///
/// ```no_run
/// use html_view::{ViewerOptions, ViewerWaitMode};
///
/// let opts = ViewerOptions::inline_html("<h1>Hello</h1>");
/// assert_eq!(opts.wait, ViewerWaitMode::Blocking);
///
/// // This call will block until the window is closed
/// html_view::open(opts).unwrap();
/// ```
///
/// Non-blocking mode:
///
/// ```no_run
/// use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};
///
/// let mut opts = ViewerOptions::inline_html("<h1>Hello</h1>");
/// opts.wait = ViewerWaitMode::NonBlocking;
///
/// match html_view::open(opts).unwrap() {
///     ViewerResult::NonBlocking(handle) => {
///         // Returns immediately, window is open in background
///         println!("Window opened with ID: {}", handle.id);
///     }
///     _ => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerWaitMode {
    /// Block until the viewer exits and return the exit status.
    ///
    /// This is the default mode and suitable for most use cases.
    /// The function call will not return until the user closes the window
    /// or a timeout is reached.
    Blocking,

    /// Return immediately with a handle to the running viewer process.
    ///
    /// Use this mode when you need to:
    /// - Continue working while the viewer is open
    /// - Manage multiple viewers simultaneously
    /// - Poll for window status
    /// - Terminate the viewer programmatically
    NonBlocking,
}

impl ViewerOptions {
    /// Create options for displaying inline HTML with default settings.
    ///
    /// This uses secure defaults:
    /// - Window size: 1024x768
    /// - Resizable: true
    /// - External navigation: disabled
    /// - Devtools: disabled
    /// - Remote content: disabled
    /// - Wait mode: Blocking
    ///
    /// # Example
    ///
    /// ```
    /// use html_view::ViewerOptions;
    ///
    /// let options = ViewerOptions::inline_html("<h1>Hello!</h1>");
    /// ```
    pub fn inline_html<S: Into<String>>(html: S) -> Self {
        Self {
            content: ViewerContent::InlineHtml {
                html: html.into(),
                base_dir: None,
            },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions::default(),
            environment: EnvironmentOptions::default(),
            dialog: html_view_shared::DialogOptions::default(),
            wait: ViewerWaitMode::Blocking,
        }
    }

    /// Create options for displaying a local HTML file.
    ///
    /// # Example
    ///
    /// ```
    /// use html_view::ViewerOptions;
    /// use std::path::PathBuf;
    ///
    /// let options = ViewerOptions::local_file(PathBuf::from("index.html"));
    /// ```
    pub fn local_file(path: std::path::PathBuf) -> Self {
        Self {
            content: ViewerContent::LocalFile { path },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions::default(),
            environment: EnvironmentOptions::default(),
            dialog: html_view_shared::DialogOptions::default(),
            wait: ViewerWaitMode::Blocking,
        }
    }

    /// Create options for displaying an HTML application directory.
    ///
    /// # Example
    ///
    /// ```
    /// use html_view::ViewerOptions;
    /// use std::path::PathBuf;
    ///
    /// let options = ViewerOptions::app_dir(PathBuf::from("./dist"));
    /// ```
    pub fn app_dir(root: std::path::PathBuf) -> Self {
        Self {
            content: ViewerContent::AppDir { root, entry: None },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions::default(),
            environment: EnvironmentOptions::default(),
            dialog: html_view_shared::DialogOptions::default(),
            wait: ViewerWaitMode::Blocking,
        }
    }

    /// Create options for displaying a remote URL.
    ///
    /// Note: This automatically enables `allow_remote_content` in the behaviour options.
    ///
    /// # Example
    ///
    /// ```
    /// use html_view::ViewerOptions;
    /// use url::Url;
    ///
    /// let url = Url::parse("https://example.com").unwrap();
    /// let options = ViewerOptions::remote_url(url);
    /// ```
    pub fn remote_url(url: url::Url) -> Self {
        Self {
            content: ViewerContent::RemoteUrl { url },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions {
                allow_remote_content: true,
                ..Default::default()
            },
            environment: EnvironmentOptions::default(),
            dialog: html_view_shared::DialogOptions::default(),
            wait: ViewerWaitMode::Blocking,
        }
    }
    /// Create a new builder for ViewerOptions.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ViewerOptionsBuilder {
        ViewerOptionsBuilder::default()
    }
}

/// Builder for constructing ViewerOptions.
#[derive(Default)]
pub struct ViewerOptionsBuilder {
    options: ViewerOptions,
}

impl ViewerOptionsBuilder {
    /// Set the content to display (Inline HTML).
    pub fn content(mut self, content: ViewerContent) -> Self {
        self.options.content = content;
        self
    }

    /// Set the window title.
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.options.window.title = Some(title.into());
        self
    }

    /// Set the window size.
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.options.window.width = Some(width);
        self.options.window.height = Some(height);
        self
    }

    /// Set window position.
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.options.window.x = Some(x);
        self.options.window.y = Some(y);
        self
    }

    /// Set window width only.
    pub fn width(mut self, width: u32) -> Self {
        self.options.window.width = Some(width);
        self
    }

    /// Set window height only.
    pub fn height(mut self, height: u32) -> Self {
        self.options.window.height = Some(height);
        self
    }

    /// Set X position only.
    pub fn x(mut self, x: i32) -> Self {
        self.options.window.x = Some(x);
        self
    }

    /// Set Y position only.
    pub fn y(mut self, y: i32) -> Self {
        self.options.window.y = Some(y);
        self
    }

    /// Disable window decorations (frameless window).
    pub fn no_decorations(mut self) -> Self {
        self.options.window.decorations = false;
        self
    }

    /// Make the window transparent.
    pub fn transparent(mut self) -> Self {
        self.options.window.transparent = true;
        self
    }

    /// Keep window always on top.
    pub fn always_on_top(mut self) -> Self {
        self.options.window.always_on_top = true;
        self
    }

    /// Enable devtools.
    pub fn devtools(mut self) -> Self {
        self.options.behaviour.enable_devtools = true;
        self
    }

    /// Allow external navigation.
    pub fn allow_navigation(mut self) -> Self {
        self.options.behaviour.allow_external_navigation = true;
        self
    }

    /// Set timeout in seconds.
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.options.environment.timeout_seconds = Some(seconds);
        self
    }

    /// Set working directory for resolving relative paths.
    pub fn working_dir(mut self, dir: std::path::PathBuf) -> Self {
        self.options.environment.working_dir = Some(dir);
        self
    }

    /// Allow remote content loading.
    pub fn allow_remote_content(mut self) -> Self {
        self.options.behaviour.allow_remote_content = true;
        self
    }

    /// Set allowed navigation domains (automatically enables allow_external_navigation).
    pub fn allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.options.behaviour.allowed_domains = Some(domains);
        self.options.behaviour.allow_external_navigation = true;
        self
    }

    /// Make window non-resizable.
    pub fn fixed_size(mut self) -> Self {
        self.options.window.resizable = false;
        self
    }

    /// Set non-blocking mode (returns immediately with a handle).
    pub fn non_blocking(mut self) -> Self {
        self.options.wait = ViewerWaitMode::NonBlocking;
        self
    }

    /// Set window theme.
    pub fn theme(mut self, theme: html_view_shared::WindowTheme) -> Self {
        self.options.window.theme_enum = Some(theme);
        self
    }

    /// Enable system notifications.
    pub fn enable_notifications(mut self) -> Self {
        self.options.behaviour.allow_notifications = true;
        self
    }

    /// Enable file and message dialogs.
    pub fn enable_dialogs(mut self) -> Self {
        self.options.dialog.allow_file_dialogs = true;
        self.options.dialog.allow_message_dialogs = true;
        self
    }

    /// Configure the custom toolbar.
    pub fn toolbar(mut self, toolbar: html_view_shared::ToolbarOptions) -> Self {
        self.options.window.toolbar = toolbar;
        self
    }

    /// Open the viewer with the configured options.
    ///
    /// This requires content to be set. If content is not set, it defaults to empty HTML.
    pub fn show(self) -> Result<crate::ViewerResult, crate::ViewerError> {
        crate::open(self.options)
    }

    /// Open the viewer with inline HTML content.
    pub fn show_html<S: Into<String>>(
        mut self,
        html: S,
    ) -> Result<crate::ViewerResult, crate::ViewerError> {
        self.options.content = ViewerContent::InlineHtml {
            html: html.into(),
            base_dir: None,
        };
        crate::open(self.options)
    }

    /// Open the viewer with a local file.
    pub fn show_file(
        mut self,
        path: std::path::PathBuf,
    ) -> Result<crate::ViewerResult, crate::ViewerError> {
        self.options.content = ViewerContent::LocalFile { path };
        crate::open(self.options)
    }

    /// Open the viewer with an app directory.
    pub fn show_app_dir(
        mut self,
        root: std::path::PathBuf,
        entry: Option<String>,
    ) -> Result<crate::ViewerResult, crate::ViewerError> {
        self.options.content = ViewerContent::AppDir { root, entry };
        crate::open(self.options)
    }

    /// Open the viewer with a remote URL (automatically enables allow_remote_content).
    pub fn show_url(mut self, url: url::Url) -> Result<crate::ViewerResult, crate::ViewerError> {
        self.options.content = ViewerContent::RemoteUrl { url };
        self.options.behaviour.allow_remote_content = true;
        crate::open(self.options)
    }

    /// Set base directory for inline HTML (for resolving relative asset paths).
    pub fn base_dir(mut self, dir: std::path::PathBuf) -> Self {
        if let ViewerContent::InlineHtml { html, .. } = &self.options.content {
            self.options.content = ViewerContent::InlineHtml {
                html: html.clone(),
                base_dir: Some(dir),
            };
        }
        self
    }
}

impl Default for ViewerOptions {
    fn default() -> Self {
        Self {
            content: ViewerContent::InlineHtml {
                html: String::new(),
                base_dir: None,
            },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions::default(),
            environment: EnvironmentOptions::default(),
            dialog: html_view_shared::DialogOptions::default(),
            wait: ViewerWaitMode::Blocking,
        }
    }
}
