fn main() {
    #[cfg(feature = "bundled")]
    download_binary();
}

#[cfg(feature = "bundled")]
fn download_binary() {
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    // Determine target platform
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let binary_name = match (target_os.as_str(), target_arch.as_str()) {
        ("linux", "x86_64") => "html_view_app-linux-x86_64",
        ("macos", "x86_64") => "html_view_app-macos-x86_64",
        ("macos", "aarch64") => "html_view_app-macos-aarch64",
        ("windows", "x86_64") => "html_view_app-windows-x86_64.exe",
        _ => {
            println!(
                "cargo:warning=Unsupported platform: {}-{}. Falling back to system-installed html_view_app",
                target_os, target_arch
            );
            println!("cargo:warning=Run: cargo install html_view_app");
            return;
        }
    };

    // Get version from environment (set during build)
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    // GitHub release URL
    let url = format!(
        "https://github.com/jmg049/HTMLView/releases/download/v{}/{}",
        version, binary_name
    );

    // Download to OUT_DIR
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dest_path = out_dir.join(if cfg!(windows) {
        "html_view_app.exe"
    } else {
        "html_view_app"
    });

    // Check if already downloaded
    if dest_path.exists() {
        println!(
            "cargo:warning=Binary already exists at {:?}, skipping download",
            dest_path
        );
        embed_binary_path(&dest_path);
        return;
    }

    println!(
        "cargo:warning=Downloading html_view_app binary from {}",
        url
    );
    println!("cargo:warning=This is a one-time download (~6-12 MB depending on platform)");

    // Download binary
    match download_file(&url, &dest_path) {
        Ok(_) => {
            // Make executable on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                match fs::metadata(&dest_path) {
                    Ok(metadata) => {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        if let Err(e) = fs::set_permissions(&dest_path, perms) {
                            println!("cargo:warning=Failed to set executable permissions: {}", e);
                            println!(
                                "cargo:warning=You may need to manually run: chmod +x {:?}",
                                dest_path
                            );
                        }
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to read file metadata: {}", e);
                    }
                }
            }

            println!(
                "cargo:warning=Successfully downloaded binary to {:?}",
                dest_path
            );
            embed_binary_path(&dest_path);
        }
        Err(e) => {
            println!("cargo:warning=Failed to download binary: {}", e);
            println!("cargo:warning=Falling back to system-installed html_view_app");
            println!("cargo:warning=Run: cargo install html_view_app");
            // Don't panic - allow fallback to system installation
        }
    }
}

#[cfg(feature = "bundled")]
fn download_file(url: &str, dest: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    // Use ureq for simple HTTP download
    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(300)) // 5 min timeout for slow connections
        .call()?;

    let mut file = std::fs::File::create(dest)?;
    let mut reader = response.into_reader();
    std::io::copy(&mut reader, &mut file)?;

    Ok(())
}

#[cfg(feature = "bundled")]
fn embed_binary_path(path: &std::path::Path) {
    // Set environment variable for compile-time embedding
    println!("cargo:rustc-env=HTML_VIEW_APP_PATH={}", path.display());

    // Rerun if binary disappears
    println!("cargo:rerun-if-changed={}", path.display());

    // Rerun if this build script changes
    println!("cargo:rerun-if-changed=build.rs");
}
