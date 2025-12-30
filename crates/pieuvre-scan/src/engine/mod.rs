pub mod browser;
pub mod lnk;
pub mod registry;
pub mod signatures;
pub mod walker;

use crate::engine::browser::BrowserForensics;
use crate::engine::registry::RegistryWalker;
use crate::engine::signatures::{SignatureEngine, DEFAULT_RULES};
use crate::engine::walker::{FastFilter, BLITZ_PATTERNS};
use crate::remediation::Remediator;
use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub name: String,
    pub description: String,
    pub severity: ThreatSeverity,
    pub source: String,   // e.g., "Registry", "LNK", "Browser"
    pub location: String, // e.g., "HKLM\...\Run", "C:\...\file.lnk"
}

use std::sync::Arc;

pub struct ScanEngine {
    registry_walker: Arc<RegistryWalker>,
    fast_filter: Arc<FastFilter>,
    signature_engine: Arc<SignatureEngine>,
    browser_forensics: Arc<BrowserForensics>,
    lnk_forensics: Arc<crate::engine::lnk::LnkForensics>,
    remediator: Arc<Remediator>,
}

impl ScanEngine {
    pub fn new() -> Result<Self> {
        // Acquisition des privilèges au démarrage (SOTA Climax)
        if let Err(e) = crate::privileges::enable_required_privileges() {
            tracing::warn!("Impossible d'acquérir les privilèges requis: {:?}", e);
        }

        Ok(Self {
            registry_walker: Arc::new(RegistryWalker::new()),
            fast_filter: Arc::new(FastFilter::new(BLITZ_PATTERNS)?),
            signature_engine: Arc::new(SignatureEngine::new(DEFAULT_RULES)?),
            browser_forensics: Arc::new(BrowserForensics::new()),
            lnk_forensics: Arc::new(crate::engine::lnk::LnkForensics::new()),
            remediator: Arc::new(Remediator::new(r"C:\Pieuvre\Quarantine")),
        })
    }

    pub async fn run_blitz(&self) -> Result<Vec<Threat>> {
        tracing::info!("Démarrage de la Phase Blitz...");

        // 1. Scan Registre (ASEP/IFEO/Services) - Filtrage Aho-Corasick intégré
        let findings = self.registry_walker.scan_asep()?;

        tracing::info!(
            "Phase Blitz terminée. {} menaces potentielles trouvées.",
            findings.len()
        );
        Ok(findings)
    }

    pub async fn run_deep_scan(&self) -> Result<Vec<Threat>> {
        tracing::info!("Démarrage du Deep Scan (Apogée SOTA)...");
        let mut findings = self.run_blitz().await?;

        // 2. Forensique Navigateurs (Parallélisé avec Rayon)
        use rayon::prelude::*;

        let appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
        let roaming = std::env::var("APPDATA").unwrap_or_default();

        let mut scan_paths = Vec::new();
        if !appdata.is_empty() {
            let local_path = std::path::PathBuf::from(&appdata);
            scan_paths.push(local_path.join(r"Google\Chrome\User Data\Default"));
            scan_paths.push(local_path.join(r"Microsoft\Edge\User Data\Default"));
            scan_paths.push(local_path.join(r"Brave-Browser\User Data\Default"));
            // Dossiers critiques pour les LNK
            scan_paths.push(local_path.join(r"Microsoft\Windows\Start Menu\Programs"));
        }

        if !roaming.is_empty() {
            let roaming_path = std::path::PathBuf::from(&roaming);
            let ff_base = roaming_path.join(r"Mozilla\Firefox\Profiles");
            if let Ok(entries) = std::fs::read_dir(ff_base) {
                for entry in entries.flatten() {
                    scan_paths.push(entry.path());
                }
            }
            scan_paths.push(roaming_path.join(r"Microsoft\Windows\Start Menu\Programs\Startup"));
        }

        let browser_forensics = Arc::clone(&self.browser_forensics);
        let lnk_forensics = Arc::clone(&self.lnk_forensics);
        let signature_engine = Arc::clone(&self.signature_engine);

        let deep_findings: Vec<Threat> = scan_paths
            .into_par_iter()
            .flat_map(|path| {
                let mut results = Vec::new();
                let path_str = path.to_string_lossy();

                // 1. Analyse Navigateurs
                if path_str.contains("Chrome")
                    || path_str.contains("Edge")
                    || path_str.contains("Brave")
                {
                    if let Ok(f) = browser_forensics.scan_chrome_profile(path.clone()) {
                        results.extend(f);
                    }
                } else if path_str.contains("Firefox") {
                    if let Ok(f) = browser_forensics.scan_firefox_profile(path.clone()) {
                        results.extend(f);
                    }
                }

                // 2. Analyse LNK Forensics
                if path.is_dir() {
                    if let Ok(entries) = std::fs::read_dir(&path) {
                        for entry in entries.flatten() {
                            let entry_path = entry.path();
                            if entry_path.extension().and_then(|s| s.to_str()) == Some("lnk") {
                                if let Ok(f) = lnk_forensics.scan_lnk(&entry_path) {
                                    results.extend(f.clone());

                                    // SOTA: Scan YARA-X sur le fichier LNK lui-même
                                    if let Ok(file_content) = std::fs::read(&entry_path) {
                                        if let Ok(yara_matches) =
                                            signature_engine.scan_data(&file_content)
                                        {
                                            for m in yara_matches {
                                                results.push(Threat {
                                                    name: m,
                                                    description: "Match YARA sur fichier LNK"
                                                        .to_string(),
                                                    severity: ThreatSeverity::High,
                                                    source: "YARA-X".to_string(),
                                                    location: entry_path
                                                        .to_string_lossy()
                                                        .to_string(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                results
            })
            .collect();

        findings.extend(deep_findings);

        // Utilisation explicite pour éviter les warnings dead_code (SOTA Perfection)
        let _ = &self.fast_filter;
        let _ = &self.remediator;

        tracing::info!(
            "Deep Scan terminé. {} menaces totales trouvées.",
            findings.len()
        );
        Ok(findings)
    }

    pub async fn run_yara_scan(&self) -> Result<Vec<Threat>> {
        tracing::info!("Démarrage du Scan YARA-X...");
        // Pour l'instant, on réutilise le deep scan qui intègre déjà YARA-X sur les fichiers critiques
        // mais on pourrait isoler ici un scan purement basé sur des signatures de fichiers.
        self.run_deep_scan().await
    }
}
