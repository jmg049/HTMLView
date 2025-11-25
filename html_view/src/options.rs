use html_view_shared::{BehaviourOptions, EnvironmentOptions, ViewerContent, WindowOptions};

/// Options for configuring a viewer instance.
#[derive(Debug, Clone)]
pub struct ViewerOptions {
    /// The content to display.
    pub content: ViewerContent,

    /// Window configuration.
    pub window: WindowOptions,

    /// Behaviour and security options.
    pub behaviour: BehaviourOptions,

    /// Environment options.
    pub environment: EnvironmentOptions,

    /// Dialog configuration.
    pub dialog: html_view_shared::DialogOptions,

    /// Whether to wait for the viewer to close.
    pub wait: ViewerWaitMode,
}

/// Determines whether the viewer call blocks or returns immediately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerWaitMode {
    /// Block until the viewer exits and return the exit status.
    Blocking,

    /// Return immediately with a handle to the running viewer process.
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
    pub fn show_html<S: Into<String>>(mut self, html: S) -> Result<crate::ViewerResult, crate::ViewerError> {
        self.options.content = ViewerContent::InlineHtml {
            html: html.into(),
            base_dir: None,
        };
        crate::open(self.options)
    }
}

impl Default for ViewerOptions {
    fn default() -> Self {
        Self {
            content: ViewerContent::InlineHtml { 
                html: String::new(), 
                base_dir: None 
            },
            window: WindowOptions::default(),
            behaviour: BehaviourOptions::default(),
            environment: EnvironmentOptions::default(),
            dialog: html_view_shared::DialogOptions::default(),
            wait: ViewerWaitMode::Blocking,
        }
    }
}
