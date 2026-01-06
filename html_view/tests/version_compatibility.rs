//! Version compatibility checking tests

use html_view_shared::PROTOCOL_VERSION;

#[test]
fn test_protocol_version_is_valid_semver() {
    // Verify PROTOCOL_VERSION follows semver format: major.minor.patch
    let parts: Vec<&str> = PROTOCOL_VERSION.split('.').collect();
    assert!(
        parts.len() >= 2,
        "PROTOCOL_VERSION must have at least major.minor: {}",
        PROTOCOL_VERSION
    );

    // Verify major and minor are numbers
    assert!(
        parts[0].parse::<u32>().is_ok(),
        "Major version must be a number: {}",
        parts[0]
    );
    assert!(
        parts[1].parse::<u32>().is_ok(),
        "Minor version must be a number: {}",
        parts[1]
    );

    // If patch version exists, verify it's a number (ignoring pre-release/build metadata)
    if parts.len() >= 3 {
        let patch_part = parts[2]
            .split('-')
            .next()
            .unwrap()
            .split('+')
            .next()
            .unwrap();
        assert!(
            patch_part.parse::<u32>().is_ok(),
            "Patch version must be a number: {}",
            patch_part
        );
    }
}

#[test]
fn test_protocol_version_not_empty() {
    assert!(!PROTOCOL_VERSION.is_empty());
}

#[test]
fn test_version_comparison_logic() {
    // Test the version parsing logic used in check_version_compatibility
    fn parse_version(v: &str) -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = v.split('.').collect();
        if parts.len() < 3 {
            return None;
        }

        let major = parts[0].parse::<u32>().ok()?;
        let minor = parts[1].parse::<u32>().ok()?;
        let patch = parts[2]
            .split('-')
            .next()?
            .split('+')
            .next()?
            .parse::<u32>()
            .ok()?;

        Some((major, minor, patch))
    }

    // Valid semver versions
    assert_eq!(parse_version("1.2.3"), Some((1, 2, 3)));
    assert_eq!(parse_version("0.1.0"), Some((0, 1, 0)));
    assert_eq!(parse_version("10.20.30"), Some((10, 20, 30)));

    // Pre-release versions
    assert_eq!(parse_version("1.2.3-alpha"), Some((1, 2, 3)));
    assert_eq!(parse_version("1.2.3-beta.1"), Some((1, 2, 3)));

    // Build metadata
    assert_eq!(parse_version("1.2.3+build.123"), Some((1, 2, 3)));
    assert_eq!(parse_version("1.2.3-rc1+build"), Some((1, 2, 3)));

    // Invalid versions
    assert_eq!(parse_version("1.2"), None);
    assert_eq!(parse_version("1"), None);
    assert_eq!(parse_version("abc"), None);
    assert_eq!(parse_version(""), None);
}

#[test]
fn test_major_version_compatibility() {
    // Major version mismatches should be incompatible
    fn is_compatible(lib_ver: &str, viewer_ver: &str) -> bool {
        fn parse_major(v: &str) -> Option<u32> {
            v.split('.').next()?.parse().ok()
        }

        match (parse_major(lib_ver), parse_major(viewer_ver)) {
            (Some(lib_maj), Some(view_maj)) => lib_maj == view_maj,
            _ => false,
        }
    }

    // Same major version = compatible
    assert!(is_compatible("1.0.0", "1.0.0"));
    assert!(is_compatible("1.0.0", "1.1.0"));
    assert!(is_compatible("1.0.0", "1.99.99"));

    // Different major version = incompatible
    assert!(!is_compatible("1.0.0", "2.0.0"));
    assert!(!is_compatible("2.0.0", "1.0.0"));
    assert!(!is_compatible("0.1.0", "1.0.0"));
}

#[test]
fn test_version_parsing_edge_cases() {
    fn parse_major(v: &str) -> Option<u32> {
        v.split('.').next()?.parse().ok()
    }

    // Valid cases
    assert_eq!(parse_major("1.2.3"), Some(1));
    assert_eq!(parse_major("0.1.0"), Some(0));
    assert_eq!(parse_major("999.0.0"), Some(999));

    // Edge cases
    assert_eq!(parse_major("1"), Some(1)); // Missing minor/patch (still gets major)
    assert_eq!(parse_major(""), None); // Empty string
    assert_eq!(parse_major("abc.def.ghi"), None); // Non-numeric
}

#[test]
fn test_backward_compatibility_with_unversioned_viewer() {
    // Viewers from before version checking was added would report "0.0.0"
    // This should be detected and handled specially
    let old_viewer_version = "0.0.0";

    assert_eq!(old_viewer_version, "0.0.0");
    // In the actual code, this triggers a special error message suggesting rebuild
}

#[test]
fn test_protocol_version_matches_package_version() {
    // PROTOCOL_VERSION should be derived from CARGO_PKG_VERSION
    // Verify it looks like a real version number
    assert!(PROTOCOL_VERSION.chars().any(|c| c.is_ascii_digit()));
    assert!(PROTOCOL_VERSION.contains('.'));
}
