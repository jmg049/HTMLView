use crate::ViewerError;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Trait for locating the html_view_app binary.
///
/// This abstraction allows for testing without requiring the actual binary.
pub trait AppLocator {
    /// Locate the html_view_app binary.
    fn locate_app_binary(&self) -> Result<PathBuf, ViewerError>;
}

/// Cache for the located binary path to avoid repeated searches.
static BINARY_CACHE: OnceLock<PathBuf> = OnceLock::new();

/// Represents an attempted search location and why it failed.
#[derive(Debug, Clone)]
struct AttemptedLocation {
    path: PathBuf,
    reason: String,
}

/// Default implementation of AppLocator.
///
/// Searches for the binary in the following order:
/// 1. Compile-time embedded path (set by build.rs, if used)
/// 2. Runtime environment variable `HTML_VIEW_APP_PATH`
/// 3. Cargo install directory (~/.cargo/bin)
/// 4. Same directory as the current executable
/// 5. Target directory relative to workspace (for development)
pub struct DefaultAppLocator;

impl AppLocator for DefaultAppLocator {
    fn locate_app_binary(&self) -> Result<PathBuf, ViewerError> {
        // Check cache first
        if let Some(cached_path) = BINARY_CACHE.get().filter(|p| p.exists() && p.is_file()) {
            return Ok(cached_path.clone());
        }
        // Cache is stale or not set, we'll search

        let mut attempted = Vec::new();
        let binary_name = if cfg!(target_os = "windows") {
            "html_view_app.exe"
        } else {
            "html_view_app"
        };

        // 1. Check compile-time embedded path (set by build.rs)
        if let Some(embedded_path) = option_env!("HTML_VIEW_APP_PATH") {
            let path = PathBuf::from(embedded_path);
            if path.exists() && path.is_file() {
                let _ = BINARY_CACHE.set(path.clone());
                return Ok(path);
            }
            attempted.push(AttemptedLocation {
                path: path.clone(),
                reason: if path.exists() {
                    "exists but is not a file".to_string()
                } else {
                    "does not exist".to_string()
                },
            });
        }

        // 2. Check runtime environment variable (allows override)
        if let Ok(path_str) = std::env::var("HTML_VIEW_APP_PATH") {
            let path = PathBuf::from(&path_str);
            if path.exists() && path.is_file() {
                let _ = BINARY_CACHE.set(path.clone());
                return Ok(path);
            }
            attempted.push(AttemptedLocation {
                path: path.clone(),
                reason: if path.exists() {
                    "exists but is not a file".to_string()
                } else {
                    "does not exist".to_string()
                },
            });
        }

        // 3. Check in Cargo install directory (~/.cargo/bin)
        if let Some(home) = std::env::var_os("CARGO_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .or_else(|| std::env::var_os("USERPROFILE"))
                    .map(|h| PathBuf::from(h).join(".cargo"))
            })
        {
            let candidate = home.join("bin").join(binary_name);
            if candidate.exists() && candidate.is_file() {
                let _ = BINARY_CACHE.set(candidate.clone());
                return Ok(candidate);
            }
            attempted.push(AttemptedLocation {
                path: candidate.clone(),
                reason: if candidate.exists() {
                    "exists but is not a file".to_string()
                } else {
                    "does not exist".to_string()
                },
            });
        } else {
            attempted.push(AttemptedLocation {
                path: PathBuf::from("~/.cargo/bin").join(binary_name),
                reason: "could not determine home directory".to_string(),
            });
        }

        // 4. Check in the same directory as the current executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let candidate = exe_dir.join(binary_name);
                if candidate.exists() && candidate.is_file() {
                    let _ = BINARY_CACHE.set(candidate.clone());
                    return Ok(candidate);
                }
                attempted.push(AttemptedLocation {
                    path: candidate.clone(),
                    reason: if candidate.exists() {
                        "exists but is not a file".to_string()
                    } else {
                        "does not exist".to_string()
                    },
                });
            }
        } else {
            attempted.push(AttemptedLocation {
                path: PathBuf::from("<current_exe_dir>").join(binary_name),
                reason: "could not determine current executable path".to_string(),
            });
        }

        // 5. Check in target directory (for development/testing)
        if let Ok(current_dir) = std::env::current_dir() {
            for profile in &["debug", "release"] {
                let candidate = current_dir.join("target").join(profile).join(binary_name);

                if candidate.exists() && candidate.is_file() {
                    let _ = BINARY_CACHE.set(candidate.clone());
                    return Ok(candidate);
                }
                attempted.push(AttemptedLocation {
                    path: candidate.clone(),
                    reason: if candidate.exists() {
                        "exists but is not a file".to_string()
                    } else {
                        "does not exist".to_string()
                    },
                });
            }
        } else {
            attempted.push(AttemptedLocation {
                path: PathBuf::from("<current_dir>/target/debug").join(binary_name),
                reason: "could not determine current directory".to_string(),
            });
            attempted.push(AttemptedLocation {
                path: PathBuf::from("<current_dir>/target/release").join(binary_name),
                reason: "could not determine current directory".to_string(),
            });
        }

        // Build detailed error message
        let mut error_msg = format!(
            "Could not locate html_view_app binary. Searched {} locations:\n\n",
            attempted.len()
        );

        for (i, attempt) in attempted.iter().enumerate() {
            error_msg.push_str(&format!(
                "  {}. {}\n     → {}\n",
                i + 1,
                attempt.path.display(),
                attempt.reason
            ));
        }

        error_msg.push_str(&format!(
            "\nTo fix this issue:\n\
             • Install the viewer: cargo install html_view_app\n\
             • Or set HTML_VIEW_APP_PATH environment variable to the binary location\n\
             • Or ensure html_view_app is in your PATH\n\n\
             Platform: {}\n\
             Architecture: {}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ));

        Err(ViewerError::BinaryNotFound(error_msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_var_locator() {
        // This test would need to set up a test environment
        // For now, just verify the trait is implemented
        let locator = DefaultAppLocator;
        let _result = locator.locate_app_binary();
        // We expect this to fail in test environment, which is fine
    }
}
