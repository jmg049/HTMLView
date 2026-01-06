use crate::{AppLocator, ViewerError, ViewerHandle, ViewerOptions, ViewerResult, ViewerWaitMode};
use html_view_shared::{PROTOCOL_VERSION, ViewerExitReason, ViewerExitStatus, ViewerRequest};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

/// RAII guard for temporary directory cleanup.
/// Ensures the directory is removed when this guard is dropped, even on panics.
struct TempDirGuard {
    path: PathBuf,
    /// If true, the directory will be cleaned up when dropped.
    /// Can be set to false to transfer ownership.
    cleanup_on_drop: bool,
}

impl TempDirGuard {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            cleanup_on_drop: true,
        }
    }

    /// Get the path to the temp directory.
    fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Disable cleanup on drop (transfers responsibility elsewhere).
    fn disable_cleanup(&mut self) {
        self.cleanup_on_drop = false;
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        if self.cleanup_on_drop {
            // Best effort cleanup - ignore errors
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

/// Launch a viewer with the given options and app locator.
pub(crate) fn launch_viewer(
    options: ViewerOptions,
    locator: &dyn AppLocator,
) -> Result<ViewerResult, ViewerError> {
    // Generate unique ID
    let id = Uuid::new_v4();

    // Create temporary directory for config and result files
    let temp_dir_path = std::env::temp_dir().join(format!("html_view_{}", id));
    fs::create_dir_all(&temp_dir_path).map_err(|e| {
        ViewerError::ConfigWriteFailed(format!(
            "Failed to create temporary directory at {}: {}\n\
             Suggestion: Check that {} has write permissions and sufficient space",
            temp_dir_path.display(),
            e,
            std::env::temp_dir().display()
        ))
    })?;

    // Set appropriate permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = std::fs::Permissions::from_mode(0o700);
        fs::set_permissions(&temp_dir_path, permissions).map_err(|e| {
            ViewerError::ConfigWriteFailed(format!(
                "Failed to set directory permissions on {}: {}",
                temp_dir_path.display(),
                e
            ))
        })?;
    }

    // Create RAII guard for automatic cleanup
    let mut temp_dir = TempDirGuard::new(temp_dir_path);

    let config_path = temp_dir.path().join("config.json");
    let result_path = temp_dir.path().join("result.json");
    let command_path = temp_dir.path().join("commands.json");
    let response_path = temp_dir.path().join("command_responses.json");

    // Create request with command path
    let request = ViewerRequest {
        id,
        content: options.content,
        window: options.window,
        behaviour: options.behaviour,
        environment: options.environment,
        dialog: options.dialog,
        command_path: Some(command_path.clone()),
    };

    // Write config file
    let config_json = serde_json::to_string_pretty(&request).map_err(|e| {
        ViewerError::SerdeError(format!(
            "Failed to serialize viewer configuration: {}\nThis is likely a bug in html_view. Please report at https://github.com/jmg049/HTMLView/issues",
            e
        ))
    })?;
    fs::write(&config_path, config_json).map_err(|e| {
        ViewerError::ConfigWriteFailed(format!(
            "Failed to write config file to {}: {}",
            config_path.display(),
            e
        ))
    })?;

    // Locate binary
    let app_binary = locator.locate_app_binary()?;

    // Spawn process
    let mut cmd = Command::new(&app_binary);
    cmd.arg("--config-path")
        .arg(&config_path)
        .arg("--result-path")
        .arg(&result_path);

    let mut child = cmd.spawn().map_err(|e| {
        ViewerError::SpawnFailed(format!(
            "Failed to spawn viewer process at {}: {}\n\
             Suggestion: Verify the binary exists and is executable",
            app_binary.display(),
            e
        ))
    })?;

    // Handle based on wait mode
    match options.wait {
        ViewerWaitMode::Blocking => {
            // Wait for process to exit
            let exit_status = child.wait()?;

            // Read result file
            let result = read_result_file(&result_path, id)?;

            // Temp directory will be automatically cleaned up when temp_dir is dropped

            // Check exit code
            if !exit_status.success() {
                if let ViewerExitReason::Error { .. } = result.reason {
                    // Error already captured in result
                } else {
                    return Err(ViewerError::SpawnFailed(format!(
                        "Process exited with code {:?}",
                        exit_status.code()
                    )));
                }
            }

            Ok(ViewerResult::Blocking(result))
        }
        ViewerWaitMode::NonBlocking => {
            // Transfer cleanup responsibility to the handle
            temp_dir.disable_cleanup();
            let handle = ViewerHandle::new(
                id,
                child,
                result_path,
                temp_dir.path().clone(),
                Some(command_path),
                Some(response_path),
            );
            Ok(ViewerResult::NonBlocking(handle))
        }
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

/// Read and parse the result file with exponential backoff.
fn read_result_file(path: &PathBuf, expected_id: Uuid) -> Result<ViewerExitStatus, ViewerError> {
    // Exponential backoff parameters
    const MAX_ATTEMPTS: u32 = 10;
    const INITIAL_DELAY_MS: u64 = 10;
    const MAX_DELAY_MS: u64 = 1000;

    let mut delay_ms = INITIAL_DELAY_MS;
    let mut last_error = None;

    for attempt in 0..MAX_ATTEMPTS {
        match fs::read_to_string(path) {
            Ok(data) => {
                // Successfully read file, try to parse it
                let status: ViewerExitStatus = serde_json::from_str(&data).map_err(|e| {
                    ViewerError::InvalidResponse(format!(
                        "Failed to parse viewer response JSON: {}\nResponse content (first 200 chars): {}",
                        e,
                        data.chars().take(200).collect::<String>()
                    ))
                })?;

                // Verify ID matches
                if status.id != expected_id {
                    return Err(ViewerError::InvalidResponse(format!(
                        "Result ID mismatch: expected {}, got {}",
                        expected_id, status.id
                    )));
                }

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
        path.display(),
        MAX_ATTEMPTS,
        last_error
            .map(|e| e.to_string())
            .unwrap_or_else(|| "unknown error".to_string())
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ViewerError;
    use std::path::PathBuf;

    struct MockAppLocator {
        path: Option<PathBuf>,
    }

    impl AppLocator for MockAppLocator {
        fn locate_app_binary(&self) -> Result<PathBuf, ViewerError> {
            self.path
                .clone()
                .ok_or_else(|| ViewerError::BinaryNotFound("Mock binary not found".to_string()))
        }
    }

    #[test]
    fn test_launcher_binary_not_found() {
        let options = ViewerOptions::inline_html("<h1>Test</h1>");
        let locator = MockAppLocator { path: None };

        let result = launch_viewer(options, &locator);
        assert!(matches!(result, Err(ViewerError::BinaryNotFound(_))));
    }
}
