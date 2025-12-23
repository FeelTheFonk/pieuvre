use thiserror::Error;

#[derive(Error, Debug)]
pub enum PieuvreError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("System error: {0}")]
    System(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Registry error: {0}")]
    Registry(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    #[error("Snapshot not found: {0}")]
    SnapshotNotFound(String),

    #[error("Windows error: {0}")]
    Windows(#[from] windows::core::Error),

    #[error("TUI error: {0}")]
    Tui(String),

    #[error("Event error: {0}")]
    Event(String),

    #[error("Scan error: {0}")]
    Scan(String),
}

pub type Result<T> = std::result::Result<T, PieuvreError>;
