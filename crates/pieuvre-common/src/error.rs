//! Types d'erreur Pieuvre

use thiserror::Error;

/// Erreur principale Pieuvre
#[derive(Error, Debug)]
pub enum PieuvreError {
    #[error("Erreur d'accès au registre: {0}")]
    Registry(String),

    #[error("Erreur système Windows: {0}")]
    Windows(#[from] windows::core::Error),

    #[error("Erreur I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("Erreur de configuration: {0}")]
    Config(String),

    #[error("Erreur de parsing: {0}")]
    Parse(String),

    #[error("Permission refusée: {0}")]
    Permission(String),

    #[error("Snapshot non trouvé: {0}")]
    SnapshotNotFound(String),

    #[error("Service non trouvé: {0}")]
    ServiceNotFound(String),

    #[error("Opération annulée")]
    Cancelled,

    #[error("Fonctionnalité non supportée: {0}")]
    Unsupported(String),

    #[error("Erreur interne: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, PieuvreError>;
