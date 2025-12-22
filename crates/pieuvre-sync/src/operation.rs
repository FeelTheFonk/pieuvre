//! Trait SyncOperation SOTA 2026
//!
//! Abstraction pour toutes les opérations de synchronisation et d'optimisation.

use async_trait::async_trait;
use pieuvre_common::{ChangeRecord, Result};
use tracing::instrument;

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
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    #[instrument(skip(self))]
    async fn is_applied(&self) -> Result<bool> {
        let name = self.name.clone();
        let target = self.target_start_type;
        tokio::task::spawn_blocking(move || {
            Ok(crate::services::get_service_start_type(&name)? == target)
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
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
                original_data: original.unwrap_or_default().to_le_bytes().to_vec(),
            }])
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    #[instrument(skip(self))]
    async fn is_applied(&self) -> Result<bool> {
        let key = self.key.clone();
        let value = self.value.clone();
        let data = self.target_data;
        tokio::task::spawn_blocking(move || {
            Ok(crate::registry::read_dword_value(&key, &value).unwrap_or(u32::MAX) == data)
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }
}

/// Opération sur les interruptions MSI
pub struct MsiOperation {
    pub devices: Vec<String>,
    pub priority: String,
}

#[async_trait]
impl SyncOperation for MsiOperation {
    fn name(&self) -> &str {
        "MSI Interrupt Optimization"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let devices = self.devices.clone();
        let priority = self.priority.clone();
        tokio::task::spawn_blocking(move || {
            crate::msi::configure_msi_for_devices(&devices, &priority)?;
            Ok(vec![]) // MSI rollback non supporté pour l'instant
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false) // Toujours appliquer pour l'instant
    }
}

/// Opération sur les packages AppX
pub struct AppxOperation {
    pub packages_to_remove: Vec<String>,
}

#[async_trait]
impl SyncOperation for AppxOperation {
    fn name(&self) -> &str {
        "AppX Package Removal"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let packages = self.packages_to_remove.clone();
        tokio::task::spawn_blocking(move || {
            for pkg in packages {
                let _ = crate::appx::remove_package(&pkg);
            }
            Ok(vec![])
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false)
    }
}

/// Opération sur le plan d'alimentation
pub struct PowerPlanOperation {
    pub plan: String,
}

#[async_trait]
impl SyncOperation for PowerPlanOperation {
    fn name(&self) -> &str {
        "Power Plan Configuration"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let plan = self.plan.clone();
        tokio::task::spawn_blocking(move || {
            match plan.as_str() {
                "ultimate_performance" => crate::power::apply_gaming_power_config()?,
                "high_performance" => {
                    crate::power::set_power_plan(crate::power::PowerPlan::HighPerformance)?
                }
                _ => crate::power::set_power_plan(crate::power::PowerPlan::Balanced)?,
            }
            Ok(vec![])
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false)
    }
}

/// Opération d'optimisation CPU
pub struct CpuOptimizationOperation {
    pub disable_core_parking: bool,
    pub disable_memory_compression: bool,
    pub disable_superfetch: bool,
}

#[async_trait]
impl SyncOperation for CpuOptimizationOperation {
    fn name(&self) -> &str {
        "CPU SOTA Optimization"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        tokio::task::spawn_blocking({
            let cp = self.disable_core_parking;
            let mc = self.disable_memory_compression;
            let sf = self.disable_superfetch;
            move || {
                if cp {
                    crate::cpu::disable_core_parking()?;
                }
                if mc {
                    crate::cpu::disable_memory_compression()?;
                }
                if sf {
                    crate::cpu::disable_superfetch_registry()?;
                }
                Ok(vec![])
            }
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    async fn is_applied(&self) -> Result<bool> {
        tokio::task::spawn_blocking({
            let cp = self.disable_core_parking;
            let mc = self.disable_memory_compression;
            move || {
                let cp_ok = if cp {
                    crate::cpu::is_core_parking_disabled()
                } else {
                    true
                };
                let mc_ok = if mc {
                    !crate::cpu::is_memory_compression_enabled()
                } else {
                    true
                };
                Ok(cp_ok && mc_ok)
            }
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }
}

/// Opération d'optimisation Mémoire
pub struct MemoryOptimizationOperation {
    pub enable_large_system_cache: bool,
    pub io_page_lock_limit_mb: Option<u32>,
}

#[async_trait]
impl SyncOperation for MemoryOptimizationOperation {
    fn name(&self) -> &str {
        "Memory SOTA Optimization"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        tokio::task::spawn_blocking({
            let lsc = self.enable_large_system_cache;
            let iopl = self.io_page_lock_limit_mb;
            move || {
                if lsc {
                    crate::memory::enable_large_system_cache()?;
                }
                if let Some(mb) = iopl {
                    crate::memory::set_io_page_lock_limit(mb * 1024 * 1024)?;
                }
                crate::memory::trim_current_working_set()?;
                Ok(vec![])
            }
        })
        .await
        .map_err(|e| pieuvre_common::PieuvreError::Internal(e.to_string()))?
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false) // Toujours appliquer le trim
    }
}
