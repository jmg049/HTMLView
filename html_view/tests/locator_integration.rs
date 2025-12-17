use html_view::AppLocator;
use std::env;
use std::fs;

/// Integration test that verifies the DefaultAppLocator will find a viewer
/// binary in a `target/debug` directory when invoked from a project root.
#[test]
fn finds_binary_in_target_debug() {
    // Create a temporary directory to act as a fake project root.
    let tmp = tempfile::tempdir().expect("tempdir");
    let tmp_path = tmp.path().to_path_buf();

    // Create target/debug and a dummy binary file inside it.
    let target_debug = tmp_path.join("target").join("debug");
    fs::create_dir_all(&target_debug).expect("create target/debug");

    let bin_name = if cfg!(target_os = "windows") {
        "html_view_app.exe"
    } else {
        "html_view_app"
    };

    let bin_path = target_debug.join(bin_name);
    fs::write(&bin_path, b"stub").expect("write dummy binary");

    // Change current dir to the temp dir so DefaultAppLocator will search here.
    let orig = env::current_dir().expect("cwd");
    let orig_home = env::var_os("HOME");
    let orig_cargo = env::var_os("CARGO_HOME");

    // Point HOME/CARGO_HOME at the tempdir so existing ~/.cargo doesn't interfere
    unsafe { env::set_var("HOME", &tmp_path) };
    unsafe { env::set_var("CARGO_HOME", &tmp_path) };
    env::set_current_dir(&tmp_path).expect("chdir to tmp");

    // Use the locator from the crate to find the binary.
    let locator = html_view::DefaultAppLocator;
    let found = locator.locate_app_binary().expect("locate binary");

    assert_eq!(found, bin_path);

    // Restore original cwd and env
    env::set_current_dir(orig).expect("restore cwd");
    if let Some(h) = orig_home {
        unsafe { env::set_var("HOME", h) };
    }
    if let Some(c) = orig_cargo {
        unsafe { env::set_var("CARGO_HOME", c) };
    }
}
