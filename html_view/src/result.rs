use crate::ViewerError;
use html_view_shared::ViewerExitStatus;
use std::path::PathBuf;
use std::process::Child;
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
}

impl ViewerHandle {
    /// Create a new viewer handle.
    pub(crate) fn new(id: Uuid, child: Child, result_path: PathBuf) -> Self {
        Self {
            id,
            child,
            result_path,
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

    /// Read and parse the result file.
    fn read_result_file(&self) -> Result<ViewerExitStatus, ViewerError> {
        // Wait a brief moment for the file to be written
        std::thread::sleep(std::time::Duration::from_millis(50));

        let data = std::fs::read_to_string(&self.result_path)
            .map_err(|e| ViewerError::ResultReadFailed(e.to_string()))?;

        let status: ViewerExitStatus =
            serde_json::from_str(&data).map_err(|e| ViewerError::InvalidResponse(e.to_string()))?;

        Ok(status)
    }
}
