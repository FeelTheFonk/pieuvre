use crate::engine::{Threat, ThreatSeverity};
use crate::{Result, ScanError};
use parselnk::Lnk;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct LnkForensics {}

impl Default for LnkForensics {
    fn default() -> Self {
        Self::new()
    }
}

impl LnkForensics {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan_lnk(&self, path: &Path) -> Result<Vec<Threat>> {
        let mut findings = Vec::new();

        let file = File::open(path).map_err(ScanError::Io)?;
        let mut reader = BufReader::new(file);
        let lnk = Lnk::new(&mut reader)
            .map_err(|e| ScanError::Forensic(format!("LNK Parse error: {}", e)))?;

        // 1. Analyse de la ligne de commande (Arguments)
        if let Some(args) = lnk.string_data.command_line_arguments.as_ref() {
            let args_str: &String = args;
            let args_lower = args_str.to_lowercase();

            // Détection élargie des patterns suspects
            let suspicious_patterns = [
                (
                    "http",
                    ThreatSeverity::High,
                    "URL externe dans les arguments",
                ),
                (
                    "powershell",
                    ThreatSeverity::Critical,
                    "Exécution PowerShell",
                ),
                ("cmd.exe", ThreatSeverity::High, "Exécution CMD"),
                ("wscript", ThreatSeverity::High, "Exécution WScript"),
                ("cscript", ThreatSeverity::High, "Exécution CScript"),
                (
                    "mshta",
                    ThreatSeverity::Critical,
                    "Exécution MSHTA (HTA Attack)",
                ),
                (
                    "bitsadmin",
                    ThreatSeverity::Critical,
                    "BITS Transfer (Exfiltration)",
                ),
                (
                    "certutil",
                    ThreatSeverity::High,
                    "CertUtil (Download/Encode)",
                ),
                (
                    "-enc",
                    ThreatSeverity::Critical,
                    "PowerShell Encoded Command",
                ),
                ("bypass", ThreatSeverity::High, "Execution Policy Bypass"),
                ("hidden", ThreatSeverity::Medium, "Fenêtre cachée"),
            ];

            for (pattern, severity, desc) in suspicious_patterns {
                if args_lower.contains(pattern) {
                    findings.push(Threat {
                        name: "LNK Suspicious Arguments".to_string(),
                        description: format!("{}: {}", desc, args_str),
                        severity,
                        source: "LNK".to_string(),
                        location: path.to_string_lossy().to_string(),
                    });
                }
            }
        }

        // 2. Analyse de la cible (Local Base Path)
        if let Some(target) = lnk.link_info.local_base_path.as_ref() {
            let target_str: &String = target;
            let target_lower = target_str.to_lowercase();

            let suspicious_paths = [
                ("temp", ThreatSeverity::High, "Cible dans dossier TEMP"),
                ("appdata", ThreatSeverity::Medium, "Cible dans AppData"),
                (
                    "public",
                    ThreatSeverity::Medium,
                    "Cible dans dossier Public",
                ),
                ("downloads", ThreatSeverity::Low, "Cible dans Downloads"),
            ];

            for (pattern, severity, desc) in suspicious_paths {
                if target_lower.contains(pattern) {
                    findings.push(Threat {
                        name: "LNK Suspicious Target".to_string(),
                        description: format!("{}: {}", desc, target_str),
                        severity,
                        source: "LNK".to_string(),
                        location: path.to_string_lossy().to_string(),
                    });
                }
            }
        }

        Ok(findings)
    }
}
