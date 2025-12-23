//! Connectivité DNS
//!
//! Configuration DNS-over-HTTPS (DoH) et sélecteurs DNS optimisés.

use crate::operation::{RegistryDwordOperation, SyncOperation};
use async_trait::async_trait;
use pieuvre_common::{ChangeRecord, Result};

/// Opération pour configurer DNS-over-HTTPS (DoH)
pub struct ConfigureDohOperation {
    pub provider: DNSProvider,
}

#[derive(Debug, Clone, Copy)]
pub enum DNSProvider {
    Cloudflare,
    Google,
    Quad9,
}

#[async_trait]
impl SyncOperation for ConfigureDohOperation {
    fn name(&self) -> &str {
        "Configure DNS-over-HTTPS"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let mut changes = Vec::new();

        // 1. Enable DoH in Registry
        let op = RegistryDwordOperation {
            key: r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters".to_string(),
            value: "EnableAutoDoh".to_string(),
            target_data: 2, // 2 = Required
        };
        changes.extend(op.apply().await?);

        // Note: La configuration des IPs DNS nécessite normalement des appels netsh ou WMI.
        // Pour rester natif, nous nous concentrons sur les paramètres de registre globaux.

        Ok(changes)
    }

    async fn is_applied(&self) -> Result<bool> {
        let val = crate::registry::read_dword_value(
            r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters",
            "EnableAutoDoh",
        )
        .unwrap_or(0);
        Ok(val == 2)
    }
}

/// Opération pour Flush DNS
pub struct FlushDnsOperation;

#[async_trait]
impl SyncOperation for FlushDnsOperation {
    fn name(&self) -> &str {
        "Flush DNS Cache"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        // Appel natif via DnsFlushResolverCache (non exposé directement dans windows crate facilement)
        // On utilise l'approche système standard
        let _ = std::process::Command::new("ipconfig")
            .arg("/flushdns")
            .output();
        Ok(vec![])
    }

    async fn is_applied(&self) -> Result<bool> {
        Ok(false) // Toujours applicable
    }
}

/// Helper pour configurer DoH
pub fn set_doh_provider(_provider: DNSProvider) -> Result<()> {
    // Les opérations de registre sont synchrones, mais SyncOperation::apply est async.
    // Pour le SOTA, on appelle directement la logique de registre ici.
    let op = RegistryDwordOperation {
        key: r"SYSTEM\CurrentControlSet\Services\Dnscache\Parameters".to_string(),
        value: "EnableAutoDoh".to_string(),
        target_data: 2,
    };
    // On utilise une approche synchrone directe pour éviter le overhead async
    crate::registry::set_dword_value(&op.key, &op.value, op.target_data)?;
    Ok(())
}

/// Helper pour Flush DNS
pub fn flush_dns_cache() -> Result<()> {
    let _ = std::process::Command::new("ipconfig")
        .arg("/flushdns")
        .output();
    Ok(())
}
