pub mod browser;
pub mod registry;
pub mod signatures;
pub mod walker;

use crate::engine::browser::BrowserForensics;
use crate::engine::registry::RegistryWalker;
use crate::engine::signatures::{SignatureEngine, DEFAULT_RULES};
use crate::engine::walker::{FastFilter, BLITZ_PATTERNS};
use crate::remediation::Remediator;
use crate::Result;

use std::sync::Arc;

pub struct ScanEngine {
    registry_walker: Arc<RegistryWalker>,
    fast_filter: Arc<FastFilter>,
    #[allow(dead_code)]
    signature_engine: Arc<SignatureEngine>,
    browser_forensics: Arc<BrowserForensics>,
    #[allow(dead_code)]
    remediator: Arc<Remediator>,
}

impl ScanEngine {
    pub fn new() -> Result<Self> {
        // SOTA: Acquisition des privilèges au démarrage
        if let Err(e) = crate::privileges::enable_debug_privilege() {
            tracing::warn!("Impossible d'acquérir SeDebugPrivilege: {:?}", e);
        }

        Ok(Self {
            registry_walker: Arc::new(RegistryWalker::new()),
            fast_filter: Arc::new(FastFilter::new(BLITZ_PATTERNS)?),
            signature_engine: Arc::new(SignatureEngine::new(DEFAULT_RULES)?),
            browser_forensics: Arc::new(BrowserForensics::new()),
            remediator: Arc::new(Remediator::new(r"C:\Pieuvre\Quarantine")),
        })
    }

    pub async fn run_blitz(&self) -> Result<Vec<String>> {
        tracing::info!("Démarrage de la Phase Blitz...");
        let mut findings = Vec::new();

        // 1. Scan Registre (ASEP/IFEO)
        let registry_keys = self.registry_walker.scan_asep()?;
        for key in registry_keys {
            if self.fast_filter.is_suspicious(&key) {
                findings.push(format!("[Registry] Suspicious key: {}", key));
            }
        }

        tracing::info!(
            "Phase Blitz terminée. {} menaces potentielles trouvées.",
            findings.len()
        );
        Ok(findings)
    }

    pub async fn run_deep_scan(&self) -> Result<Vec<String>> {
        tracing::info!("Démarrage du Deep Scan...");
        let mut findings = self.run_blitz().await?;

        // 2. Forensique Navigateurs (Parallélisé avec Rayon)
        use rayon::prelude::*;

        let appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
        let roaming = std::env::var("APPDATA").unwrap_or_default();

        let mut browser_paths = Vec::new();
        if !appdata.is_empty() {
            browser_paths
                .push(std::path::PathBuf::from(&appdata).join(r"Google\Chrome\User Data\Default"));
            browser_paths
                .push(std::path::PathBuf::from(&appdata).join(r"Microsoft\Edge\User Data\Default"));
        }

        // Firefox profiles (énumération simplifiée pour l'exemple)
        if !roaming.is_empty() {
            let ff_base = std::path::PathBuf::from(&roaming).join(r"Mozilla\Firefox\Profiles");
            if let Ok(entries) = std::fs::read_dir(ff_base) {
                for entry in entries.flatten() {
                    browser_paths.push(entry.path());
                }
            }
        }

        let browser_forensics = Arc::clone(&self.browser_forensics);
        let browser_findings: Vec<String> = browser_paths
            .into_par_iter()
            .flat_map(|path| {
                let mut results = Vec::new();
                if path.to_string_lossy().contains("Chrome")
                    || path.to_string_lossy().contains("Edge")
                {
                    if let Ok(f) = browser_forensics.scan_chrome_profile(path) {
                        results.extend(f);
                    }
                } else if path.to_string_lossy().contains("Firefox") {
                    if let Ok(f) = browser_forensics.scan_firefox_profile(path) {
                        results.extend(f);
                    }
                }
                results
            })
            .collect();

        findings.extend(browser_findings);

        tracing::info!(
            "Deep Scan terminé. {} menaces totales trouvées.",
            findings.len()
        );
        Ok(findings)
    }
}
