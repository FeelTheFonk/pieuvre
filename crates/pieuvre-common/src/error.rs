//! pieuvre error types

use thiserror::Error;

/// Main pieuvre error enum
#[derive(Error, Debug)]
pub enum PieuvreError {
    #[error("Registry access error: {0}")]
    Registry(String),

    #[error("Windows system error: {0}")]
    Windows(#[from] windows::core::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Parsing error: {0}")]
    Parse(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Snapshot not found: {0}")]
    SnapshotNotFound(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, PieuvreError>;
