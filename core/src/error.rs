//! Error types with context and tracing

use thiserror::Error;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type with detailed context
#[derive(Error, Debug)]
pub enum Error {
    /// Route not found
    #[error("Route not found: {route_id}")]
    RouteNotFound {
        /// Route ID that was not found
        route_id: String,
    },
    
    /// Dimension mismatch between embedding and configuration
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch {
        /// Expected dimension from configuration
        expected: usize,
        /// Actual dimension received
        actual: usize,
    },
    
    /// Index not built before routing
    #[error("Index not built. Call build_index() before routing")]
    IndexNotBuilt,
    
    /// Invalid input provided
    #[error("Invalid input: {message}")]
    InvalidInput {
        /// Error message
        message: String,
    },
    
    /// IO error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// No routes available
    #[error("No routes available in router")]
    NoRoutes,
    
    /// Unknown error
    #[error("Unknown error: {message}")]
    Unknown {
        /// Error message
        message: String,
    },
}

impl Error {
    /// Check if error is recoverable
    ///
    /// Recoverable errors indicate user input problems that can be fixed.
    /// Non-recoverable errors indicate system or programming errors.
    #[must_use]
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::InvalidInput { .. }
                | Self::IndexNotBuilt
                | Self::NoRoutes
                | Self::RouteNotFound { .. }
                | Self::DimensionMismatch { .. }
        )
    }
    
    /// Get error severity
    #[must_use]
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Io(_) | Self::Unknown { .. } => ErrorSeverity::Critical,
            Self::Serialization(_) => ErrorSeverity::High,
            Self::IndexNotBuilt | Self::NoRoutes => ErrorSeverity::Medium,
            Self::InvalidInput { .. }
                | Self::RouteNotFound { .. }
                | Self::DimensionMismatch { .. } => ErrorSeverity::Low,
        }
    }
    
    /// Create InvalidInput error
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }
    
    /// Create DimensionMismatch error
    #[must_use]
    pub fn dimension_mismatch(expected: usize, actual: usize) -> Self {
        Self::DimensionMismatch {
            expected,
            actual,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity - user input error
    Low,
    /// Medium severity - operation cannot proceed
    Medium,
    /// High severity - data corruption or inconsistency
    High,
    /// Critical severity - system error
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_recoverable() {
        let err = Error::invalid_input("test");
        assert!(err.is_recoverable());
        
        let err = Error::dimension_mismatch(384, 256);
        assert!(err.is_recoverable());
        
        let err = Error::Unknown { message: "test".into() };
        assert!(!err.is_recoverable());
    }
    
    #[test]
    fn test_error_severity() {
        let err = Error::invalid_input("test");
        assert_eq!(err.severity(), ErrorSeverity::Low);
        
        let err = Error::dimension_mismatch(384, 256);
        assert_eq!(err.severity(), ErrorSeverity::Low);
        
        let err = Error::Unknown { message: "test".into() };
        assert_eq!(err.severity(), ErrorSeverity::Critical);
    }
    
    #[test]
    fn test_error_display() {
        let err = Error::dimension_mismatch(384, 256);
        let display = err.to_string();
        assert!(display.contains("384"));
        assert!(display.contains("256"));
    }
}
