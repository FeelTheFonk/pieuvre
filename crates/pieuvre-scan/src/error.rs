use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScanError {
    #[error("Windows API error: {0}")]
    WindowsError(u32),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Registry error: {0}")]
    Registry(String),

    #[error("Privilege error: {0}")]
    Privilege(String),

    #[error("YARA error: {0}")]
    Yara(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Other error: {0}")]
    Other(String),
}
