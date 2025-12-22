//! Pieuvre Persistence Engine
//!
//! Gestion des snapshots et rollback.

pub mod snapshot;

#[cfg(test)]
mod tests;

use pieuvre_common::Result;

/// Liste tous les snapshots disponibles
pub fn list_snapshots() -> Result<Vec<pieuvre_common::Snapshot>> {
    snapshot::list_all()
}

/// Restaure un snapshot par ID
pub fn restore_snapshot(id: &str) -> Result<()> {
    snapshot::restore(id)
}
