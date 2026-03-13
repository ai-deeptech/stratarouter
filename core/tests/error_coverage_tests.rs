//! Tests to improve error.rs coverage

use stratarouter_core::Error;

#[test]
fn test_invalid_input_error() {
    let error = Error::invalid_input("test error message");
    let display = format!("{}", error);
    assert!(display.contains("test error message"));
}

#[test]
fn test_dimension_mismatch_error() {
    let error = Error::dimension_mismatch(384, 512);
    let display = format!("{}", error);
    assert!(display.contains("384"));
    assert!(display.contains("512"));
}

#[test]
fn test_index_not_built_error() {
    let error = Error::IndexNotBuilt;
    let display = format!("{}", error);
    assert!(!display.is_empty());
    assert!(display.to_lowercase().contains("index"));
}

#[test]
fn test_no_routes_error() {
    let error = Error::NoRoutes;
    let display = format!("{}", error);
    assert!(!display.is_empty());
    assert!(display.to_lowercase().contains("route"));
}

#[test]
fn test_error_debug() {
    let errors = vec![
        Error::invalid_input("test"),
        Error::dimension_mismatch(384, 512),
        Error::IndexNotBuilt,
        Error::NoRoutes,
    ];

    for error in errors {
        let debug = format!("{:?}", error);
        assert!(!debug.is_empty());
    }
}

#[test]
fn test_error_display_all_variants() {
    let errors = vec![
        Error::invalid_input("input error"),
        Error::dimension_mismatch(100, 200),
        Error::IndexNotBuilt,
        Error::NoRoutes,
    ];

    for error in errors {
        let display = format!("{}", error);
        assert!(!display.is_empty());
        assert!(display.len() > 5); // Meaningful message
    }
}

#[test]
fn test_dimension_mismatch_message_format() {
    let error = Error::dimension_mismatch(384, 768);
    let message = error.to_string();
    assert!(message.contains("384"));
    assert!(message.contains("768"));
    assert!(
        message.to_lowercase().contains("dimension") || message.to_lowercase().contains("mismatch")
    );
}

#[test]
fn test_invalid_input_preserves_message() {
    let original_msg = "This is a custom error message";
    let error = Error::invalid_input(original_msg);
    let display = error.to_string();
    assert!(display.contains(original_msg));
}

#[test]
fn test_error_is_recoverable() {
    // Recoverable errors
    assert!(Error::invalid_input("test").is_recoverable());
    assert!(Error::dimension_mismatch(10, 20).is_recoverable());
    assert!(Error::IndexNotBuilt.is_recoverable());
    assert!(Error::NoRoutes.is_recoverable());

    // Non-recoverable errors
    assert!(!Error::Unknown {
        message: "test".into()
    }
    .is_recoverable());
}

#[test]
fn test_error_severity() {
    use stratarouter_core::error::ErrorSeverity;

    // Low severity
    assert_eq!(Error::invalid_input("test").severity(), ErrorSeverity::Low);
    assert_eq!(
        Error::dimension_mismatch(10, 20).severity(),
        ErrorSeverity::Low
    );

    // Medium severity
    assert_eq!(Error::IndexNotBuilt.severity(), ErrorSeverity::Medium);
    assert_eq!(Error::NoRoutes.severity(), ErrorSeverity::Medium);

    // Critical severity
    assert_eq!(
        Error::Unknown {
            message: "test".into()
        }
        .severity(),
        ErrorSeverity::Critical
    );
}

#[test]
fn test_error_from_io_error() {
    // Test conversion
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let error: Error = io_error.into();

    // Should convert to IO variant
    match error {
        Error::Io(_) => {} // Success
        _ => panic!("Expected Error::Io variant"),
    }
}

#[test]
fn test_result_type_usage() {
    use stratarouter_core::Result;

    fn returns_result() -> Result<i32> {
        Ok(42)
    }

    fn returns_error() -> Result<i32> {
        Err(Error::NoRoutes)
    }

    assert_eq!(returns_result().unwrap(), 42);
    assert!(returns_error().is_err());
}

#[test]
fn test_error_propagation() {
    use stratarouter_core::Result;

    fn inner() -> Result<()> {
        Err(Error::IndexNotBuilt)
    }

    fn outer() -> Result<()> {
        inner()?;
        Ok(())
    }

    let result = outer();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::IndexNotBuilt));
}

#[test]
fn test_error_route_not_found() {
    let error = Error::RouteNotFound {
        route_id: "missing_route".into(),
    };

    let display = error.to_string();
    assert!(display.contains("missing_route"));
}

#[test]
fn test_error_unknown() {
    let error = Error::Unknown {
        message: "unexpected error".into(),
    };

    let display = error.to_string();
    assert!(display.contains("unexpected error"));
}

#[test]
fn test_error_severity_ordering() {
    use stratarouter_core::error::ErrorSeverity;

    assert!(ErrorSeverity::Low < ErrorSeverity::Medium);
    assert!(ErrorSeverity::Medium < ErrorSeverity::High);
    assert!(ErrorSeverity::High < ErrorSeverity::Critical);
}
