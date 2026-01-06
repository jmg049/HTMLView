use crate::ViewerError;
use html_view_shared::{
    PROTOCOL_VERSION, ViewerCommand, ViewerCommandResponse, ViewerContent, ViewerExitStatus,
};
use std::path::PathBuf;
use std::process::Child;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use uuid::Uuid;

/// The result of opening a viewer.
#[derive(Debug)]
pub enum ViewerResult {
    /// The viewer was opened in blocking mode and has completed.
    Blocking(ViewerExitStatus),

    /// The viewer was opened in non-blocking mode.
    NonBlocking(ViewerHandle),
}

/// A handle to a running viewer process in non-blocking mode.
#[derive(Debug)]
pub struct ViewerHandle {
    /// Unique identifier for this viewer instance.
    pub id: Uuid,

    /// The spawned child process.
    child: Child,

    /// Path to the result JSON file.
    result_path: PathBuf,

    /// Path to the temporary directory (will be cleaned up when handle is dropped).
    temp_dir: PathBuf,

    /// Optional path to the command file for sending runtime updates.
    command_path: Option<PathBuf>,

    /// Sequence counter for commands.
    command_seq: Arc<AtomicU64>,

    /// Optional path to the command response file.
    response_path: Option<PathBuf>,
}

impl ViewerHandle {
    /// Create a new viewer handle.
    pub(crate) fn new(
        id: Uuid,
        child: Child,
        result_path: PathBuf,
        temp_dir: PathBuf,
        command_path: Option<PathBuf>,
        response_path: Option<PathBuf>,
    ) -> Self {
        Self {
            id,
            child,
            result_path,
            temp_dir,
            command_path,
            command_seq: Arc::new(AtomicU64::new(0)),
            response_path,
        }
    }

    /// Try to check whether the viewer has finished and return its exit status.
    ///
    /// This is non-blocking. Returns `Ok(None)` if the process is still running.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};
    ///
    /// let mut options = ViewerOptions::inline_html("<h1>Test</h1>");
    /// options.wait = ViewerWaitMode::NonBlocking;
    ///
    /// if let ViewerResult::NonBlocking(mut handle) = html_view::open(options).unwrap() {
    ///     loop {
    ///         if let Some(status) = handle.try_wait().unwrap() {
    ///             println!("Viewer exited: {:?}", status);
    ///             break;
    ///         }
    ///         // Do other work...
    ///         std::thread::sleep(std::time::Duration::from_millis(100));
    ///     }
    /// }
    /// ```
    pub fn try_wait(&mut self) -> Result<Option<ViewerExitStatus>, ViewerError> {
        match self.child.try_wait()? {
            Some(_exit_status) => {
                // Process has exited, read the result file
                let result = self.read_result_file()?;
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    /// Block until the viewer finishes and return its exit status.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};
    ///
    /// let mut options = ViewerOptions::inline_html("<h1>Test</h1>");
    /// options.wait = ViewerWaitMode::NonBlocking;
    ///
    /// if let ViewerResult::NonBlocking(handle) = html_view::open(options).unwrap() {
    ///     let status = handle.wait().unwrap();
    ///     println!("Viewer exited: {:?}", status);
    /// }
    /// ```
    pub fn wait(mut self) -> Result<ViewerExitStatus, ViewerError> {
        self.child.wait()?;
        self.read_result_file()
    }

    /// Attempt to terminate the viewer process early.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};
    ///
    /// let mut options = ViewerOptions::inline_html("<h1>Test</h1>");
    /// options.wait = ViewerWaitMode::NonBlocking;
    ///
    /// if let ViewerResult::NonBlocking(mut handle) = html_view::open(options).unwrap() {
    ///     std::thread::sleep(std::time::Duration::from_secs(2));
    ///     handle.terminate().unwrap();
    /// }
    /// ```
    pub fn terminate(&mut self) -> Result<(), ViewerError> {
        self.child.kill()?;
        Ok(())
    }

    /// Refresh the viewer with new content.
    ///
    /// This updates the displayed content without closing the window.
    /// The window and behavior options remain unchanged.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult, ViewerContent};
    ///
    /// let mut options = ViewerOptions::inline_html("<h1>Initial</h1>");
    /// options.wait = ViewerWaitMode::NonBlocking;
    ///
    /// if let ViewerResult::NonBlocking(mut handle) = html_view::open(options).unwrap() {
    ///     std::thread::sleep(std::time::Duration::from_secs(2));
    ///
    ///     // Update content
    ///     handle.refresh(ViewerContent::InlineHtml {
    ///         html: "<h1>Updated!</h1>".to_string(),
    ///         base_dir: None,
    ///     }).unwrap();
    /// }
    /// ```
    pub fn refresh(&mut self, content: ViewerContent) -> Result<(), ViewerError> {
        // Check process is still alive first
        if let Some(_) = self.try_wait()? {
            return Err(ViewerError::CommandFailed("Process has exited".to_string()));
        }

        // Verify viewer supports refresh
        let command_path = self.command_path.clone().ok_or_else(|| {
            ViewerError::RefreshNotSupported(
                "Viewer was launched without refresh support".to_string(),
            )
        })?;

        // Create command with sequence number
        let seq = self.command_seq.fetch_add(1, Ordering::SeqCst);
        let command = ViewerCommand::Refresh { seq, content };

        // Write command atomically (temp file + rename)
        let temp_path = self.temp_dir.join(format!("command_{}.tmp", seq));
        let command_json = serde_json::to_string(&command)
            .map_err(|e| ViewerError::SerdeError(format!("Failed to serialize command: {}", e)))?;
        std::fs::write(&temp_path, &command_json)?;
        std::fs::rename(&temp_path, &command_path)?;

        // Wait for response
        self.wait_for_response(seq, Duration::from_secs(5))
    }

    /// Refresh the viewer with inline HTML (convenience method).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use html_view::{ViewerOptions, ViewerWaitMode, ViewerResult};
    ///
    /// let mut options = ViewerOptions::inline_html("<h1>Initial</h1>");
    /// options.wait = ViewerWaitMode::NonBlocking;
    ///
    /// if let ViewerResult::NonBlocking(mut handle) = html_view::open(options).unwrap() {
    ///     std::thread::sleep(std::time::Duration::from_secs(2));
    ///     handle.refresh_html("<h1>Updated!</h1>").unwrap();
    /// }
    /// ```
    pub fn refresh_html<S: Into<String>>(&mut self, html: S) -> Result<(), ViewerError> {
        self.refresh(ViewerContent::InlineHtml {
            html: html.into(),
            base_dir: None,
        })
    }

    /// Wait for a command response with timeout and exponential backoff.
    fn wait_for_response(&self, seq: u64, timeout: Duration) -> Result<(), ViewerError> {
        let response_path = self.response_path.as_ref().ok_or_else(|| {
            ViewerError::RefreshNotSupported("No response path configured".to_string())
        })?;

        // Exponential backoff parameters
        const MAX_ATTEMPTS: u32 = 50;
        const INITIAL_DELAY_MS: u64 = 10;
        const MAX_DELAY_MS: u64 = 100;

        let start_time = std::time::Instant::now();
        let mut delay_ms = INITIAL_DELAY_MS;

        for _attempt in 0..MAX_ATTEMPTS {
            // Check if we've exceeded the timeout
            if start_time.elapsed() >= timeout {
                return Err(ViewerError::CommandTimeout {
                    seq,
                    timeout_secs: timeout.as_secs(),
                });
            }

            // Try to read response file
            if let Ok(data) = std::fs::read_to_string(response_path) {
                match serde_json::from_str::<ViewerCommandResponse>(&data) {
                    Ok(response) if response.seq == seq => {
                        if response.success {
                            return Ok(());
                        } else {
                            return Err(ViewerError::CommandFailed(
                                response
                                    .error
                                    .unwrap_or_else(|| "Unknown error".to_string()),
                            ));
                        }
                    }
                    _ => {
                        // Wrong sequence number or parse error, keep waiting
                    }
                }
            }

            // Wait before retrying
            std::thread::sleep(Duration::from_millis(delay_ms));
            // Exponential backoff with cap
            delay_ms = (delay_ms * 2).min(MAX_DELAY_MS);
        }

        Err(ViewerError::CommandTimeout {
            seq,
            timeout_secs: timeout.as_secs(),
        })
    }

    /// Read and parse the result file with exponential backoff.
    fn read_result_file(&self) -> Result<ViewerExitStatus, ViewerError> {
        // Exponential backoff parameters
        const MAX_ATTEMPTS: u32 = 10;
        const INITIAL_DELAY_MS: u64 = 10;
        const MAX_DELAY_MS: u64 = 1000;

        let mut delay_ms = INITIAL_DELAY_MS;
        let mut last_error = None;

        for attempt in 0..MAX_ATTEMPTS {
            match std::fs::read_to_string(&self.result_path) {
                Ok(data) => {
                    // Successfully read file, try to parse it
                    let status: ViewerExitStatus = serde_json::from_str(&data)
                        .map_err(|e| ViewerError::InvalidResponse(e.to_string()))?;

                    // Check version compatibility
                    check_version_compatibility(&status.viewer_version)?;

                    return Ok(status);
                }
                Err(e) => {
                    last_error = Some(e);

                    // If this isn't the last attempt, wait before retrying
                    if attempt < MAX_ATTEMPTS - 1 {
                        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
                        // Exponential backoff with cap
                        delay_ms = (delay_ms * 2).min(MAX_DELAY_MS);
                    }
                }
            }
        }

        // All attempts failed
        Err(ViewerError::ResultReadFailed(format!(
            "Failed to read result file at {} after {} attempts: {}\n\
             Suggestion: The viewer process may have crashed. Check system logs or run with devtools enabled.",
            self.result_path.display(),
            MAX_ATTEMPTS,
            last_error
                .map(|e| e.to_string())
                .unwrap_or_else(|| "unknown error".to_string())
        )))
    }
}

/// Check if viewer version is compatible with library version.
fn check_version_compatibility(viewer_version: &str) -> Result<(), ViewerError> {
    let library_version = PROTOCOL_VERSION;

    // Parse versions (simple major.minor.patch parsing)
    let parse_version = |v: &str| -> Result<(u32, u32, u32), ViewerError> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() != 3 {
            return Err(ViewerError::InvalidResponse(format!(
                "Invalid version format: {}",
                v
            )));
        }
        let major = parts[0].parse::<u32>().map_err(|_| {
            ViewerError::InvalidResponse(format!("Invalid major version: {}", parts[0]))
        })?;
        let minor = parts[1].parse::<u32>().map_err(|_| {
            ViewerError::InvalidResponse(format!("Invalid minor version: {}", parts[1]))
        })?;
        let patch = parts[2].parse::<u32>().map_err(|_| {
            ViewerError::InvalidResponse(format!("Invalid patch version: {}", parts[2]))
        })?;
        Ok((major, minor, patch))
    };

    let (lib_major, lib_minor, _lib_patch) = parse_version(library_version)?;
    let (viewer_major, viewer_minor, _viewer_patch) = parse_version(viewer_version)?;

    // Check for version 0.0.0 (old viewer that doesn't report version)
    if viewer_major == 0 && viewer_minor == 0 {
        return Err(ViewerError::VersionMismatch {
            library: library_version.to_string(),
            viewer: viewer_version.to_string(),
            suggestion: "Your html_view_app binary is outdated and doesn't report its version.\n\
                         Please update it with: cargo install html_view_app --force"
                .to_string(),
        });
    }

    // Major version must match (breaking changes)
    if lib_major != viewer_major {
        let suggestion = if lib_major > viewer_major {
            format!(
                "Your html_view_app binary is too old.\n\
                 Please update it with: cargo install html_view_app --version {}.{}.0 --force",
                lib_major, lib_minor
            )
        } else {
            format!(
                "Your html_view_app binary is too new.\n\
                 Either downgrade the viewer or update the html_view library to version {}.{}.x",
                viewer_major, viewer_minor
            )
        };

        return Err(ViewerError::VersionMismatch {
            library: library_version.to_string(),
            viewer: viewer_version.to_string(),
            suggestion,
        });
    }

    // For major version 0, minor version must also match (unstable API)
    if lib_major == 0 && lib_minor != viewer_minor {
        let suggestion = if lib_minor > viewer_minor {
            format!(
                "Your html_view_app binary is too old for this pre-1.0 library.\n\
                 Please update it with: cargo install html_view_app --version 0.{}.0 --force",
                lib_minor
            )
        } else {
            format!(
                "Your html_view_app binary is too new for this pre-1.0 library.\n\
                 Either downgrade the viewer or update the html_view library to version 0.{}.x",
                viewer_minor
            )
        };

        return Err(ViewerError::VersionMismatch {
            library: library_version.to_string(),
            viewer: viewer_version.to_string(),
            suggestion,
        });
    }

    Ok(())
}

impl Drop for ViewerHandle {
    fn drop(&mut self) {
        // Best effort cleanup - ignore errors
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}
