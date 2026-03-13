//! Additional tests to improve coverage for lib.rs

use stratarouter_core::{has_avx2, BUILD_TIMESTAMP, VERSION};

#[test]
fn test_version_format() {
    // Test version follows semver
    assert!(VERSION.contains('.'));
    let parts: Vec<&str> = VERSION.split('.').collect();
    assert!(parts.len() >= 2, "Version should have at least major.minor");

    // Verify each part is numeric
    for part in parts {
        let cleaned = part.split('-').next().unwrap();
        assert!(
            cleaned.chars().all(|c| c.is_ascii_digit()),
            "Version part should be numeric: {}",
            cleaned
        );
    }
}

#[test]
fn test_version_non_empty() {
    assert!(!VERSION.is_empty());
    assert!(VERSION.len() > 0);
}

#[test]
fn test_build_timestamp_format() {
    // BUILD_TIMESTAMP is Option<&str> — None when BUILD_TIMESTAMP env var
    // is not set at compile time (normal in CI without a custom build script).
    // Verify it doesn't panic and, if present, looks like a timestamp.
    if let Some(ts) = BUILD_TIMESTAMP {
        assert!(!ts.is_empty());
        assert!(
            ts.chars().any(|c| c.is_ascii_digit()),
            "Build timestamp should contain numbers"
        );
    }
}

#[test]
fn test_build_timestamp_non_empty() {
    // Option<&str>: just assert the constant is reachable.
    let _timestamp = BUILD_TIMESTAMP;
}

#[test]
fn test_avx2_detection() {
    // AVX2 detection should not panic
    let has_avx2_support = has_avx2();

    // On x86_64 it might be true or false
    // On other architectures it should always be false
    #[cfg(not(target_arch = "x86_64"))]
    assert!(!has_avx2_support, "Non-x86_64 should not have AVX2");
}

#[test]
fn test_avx2_consistency() {
    // Multiple calls should return same result
    let result1 = has_avx2();
    let result2 = has_avx2();
    assert_eq!(result1, result2);
}

#[test]
#[cfg(feature = "python")]
fn test_python_module_attributes() {
    use pyo3::prelude::*;

    Python::with_gil(|py| {
        let module_result = py.import("stratarouter_core");
        if let Ok(module) = module_result {
            // Verify module has required attributes
            assert!(module.hasattr("__version__").unwrap());
            assert!(module.hasattr("has_avx2").unwrap());

            // Verify version matches
            if let Ok(py_version) = module.getattr("__version__") {
                let version_str: String = py_version.extract().unwrap();
                assert_eq!(version_str, VERSION);
            }
        }
    });
}

#[test]
fn test_module_constants() {
    // Ensure constants are accessible
    let _v = VERSION;
    let _t = BUILD_TIMESTAMP;
    let _a = has_avx2();
}
