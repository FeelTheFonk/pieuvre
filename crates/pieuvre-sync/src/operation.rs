//! Trait SyncOperation SOTA 2026
//!
//! Abstraction pour toutes les opérations de synchronisation et d'optimisation.

use async_trait::async_trait;
use tracing::instrument;
use pieuvre_common::{Result, ChangeRecord};

/// Une opération de synchronisation unifiée
#[async_trait]
pub trait SyncOperation: Send + Sync {
    /// Nom de l'opération (pour le logging)
    fn name(&self) -> &str;
    
    /// Applique l'optimisation
    async fn apply(&self) -> Result<Vec<ChangeRecord>>;
    
    /// Vérifie si l'optimisation est déjà appliquée
    async fn is_applied(&self) -> Result<bool>;
}

/// Opération sur un service Windows
pub struct ServiceOperation {
    pub name: String,
    pub target_start_type: u32, // 2=Auto, 3=Manual, 4=Disabled
}

#[async_trait]
impl SyncOperation for ServiceOperation {
    fn name(&self) -> &str {
        &self.name
    }

    #[instrument(skip(self))]
    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let name = self.name.clone();
        let target = self.target_start_type;
        
        tokio::task::spawn_blocking(move || {
            let original = crate::services::get_service_start_type(&name)?;
            if original != target {
                crate::services::disable_service(&name)?; // TODO: support other types
                Ok(vec![ChangeRecord::Service {
                    name,
                    original_start_type: original,
                }])
            } else {
                Ok(vec![])
            }
        }).await.map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    #[instrument(skip(self))]
    async fn is_applied(&self) -> Result<bool> {
        let name = self.name.clone();
        let target = self.target_start_type;
        tokio::task::spawn_blocking(move || {
            Ok(crate::services::get_service_start_type(&name)? == target)
        }).await.map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }
}

/// Opération sur le registre (DWORD)
pub struct RegistryDwordOperation {
    pub key: String,
    pub value: String,
    pub target_data: u32,
}

#[async_trait]
impl SyncOperation for RegistryDwordOperation {
    fn name(&self) -> &str {
        &self.value
    }

    #[instrument(skip(self))]
    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let key = self.key.clone();
        let value = self.value.clone();
        let data = self.target_data;
        
        tokio::task::spawn_blocking(move || {
            let original = crate::registry::read_dword_value(&key, &value).ok();
            crate::registry::set_dword_value(&key, &value, data)?;
            
            Ok(vec![ChangeRecord::Registry {
                key,
                value_name: value,
                value_type: "REG_DWORD".to_string(),
                original_data: original.map(|d| d.to_le_bytes().to_vec()).unwrap_or_default(),
            }])
        }).await.map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    #[instrument(skip(self))]
    async fn is_applied(&self) -> Result<bool> {
        let key = self.key.clone();
        let value = self.value.clone();
        let data = self.target_data;
        tokio::task::spawn_blocking(move || {
            Ok(crate::registry::read_dword_value(&key, &value).unwrap_or(u32::MAX) == data)
        }).await.map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }
}
