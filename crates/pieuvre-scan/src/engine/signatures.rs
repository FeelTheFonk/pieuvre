use crate::{Result, ScanError};
use yara_x::{Rules, Scanner};

pub struct SignatureEngine {
    rules: Rules,
}

impl SignatureEngine {
    pub fn new(yara_rules: &str) -> Result<Self> {
        let rules = yara_x::compile(yara_rules).map_err(|e| ScanError::Yara(e.to_string()))?;

        Ok(Self { rules })
    }

    pub fn scanner(&self) -> Scanner<'_> {
        Scanner::new(&self.rules)
    }

    pub fn scan_data(&self, data: &[u8]) -> Result<Vec<String>> {
        let mut scanner = self.scanner();
        let scan_results = scanner
            .scan(data)
            .map_err(|e| ScanError::Yara(e.to_string()))?;

        let mut findings = Vec::new();
        for matching_rule in scan_results.matching_rules() {
            findings.push(matching_rule.identifier().to_string());
        }
        Ok(findings)
    }
}

/// Règles YARA SOTA intégrées au binaire (Apogée Climax)
pub const DEFAULT_RULES: &str = r#"
rule Adware_SearchHijack {
    meta:
        description = "Détection de hijack de moteur de recherche"
        severity = "high"
    strings:
        $s1 = "search.conduit.com" nocase
        $s2 = "babylon.com" nocase
        $s3 = "mywebsearch.com" nocase
        $s4 = "trovi.com" nocase
        $s5 = "delta-search.com" nocase
        $s6 = "sweetim.com" nocase
        $s7 = "searchprotect" nocase
    condition:
        any of them
}

rule Persistence_IFEO_Hijack {
    meta:
        description = "Détection de détournement IFEO"
        severity = "critical"
    strings:
        $debugger = "Debugger" nocase
        $ifeo = "Image File Execution Options" nocase
    condition:
        all of them
}

rule Browser_Extension_ForceInstall {
    meta:
        description = "Détection d'extensions forcées"
        severity = "high"
    strings:
        $policy = "ExtensionInstallForcelist" nocase
        $force = "ExtensionInstallBlocklist" nocase
    condition:
        any of them
}

rule Persistence_Registry_Run {
    meta:
        description = "Clé de registre Run suspecte"
        severity = "medium"
    strings:
        $run = "CurrentVersion\\Run" nocase
        $runonce = "CurrentVersion\\RunOnce" nocase
    condition:
        any of them
}

rule Script_Obfuscation {
    meta:
        description = "Détection de scripts obfusqués"
        severity = "critical"
    strings:
        $enc1 = "-EncodedCommand" nocase
        $enc2 = "-enc " nocase
        $bypass = "-ExecutionPolicy Bypass" nocase
        $hidden = "-WindowStyle Hidden" nocase
        $noprofile = "-NoProfile" nocase
        $iex = "IEX(" nocase
        $invoke = "Invoke-Expression" nocase
    condition:
        2 of them
}

rule LNK_Malicious {
    meta:
        description = "Fichier LNK potentiellement malveillant"
        severity = "high"
    strings:
        $lnk_header = { 4C 00 00 00 01 14 02 00 }
        $cmd = "cmd.exe" nocase
        $powershell = "powershell" nocase
        $mshta = "mshta" nocase
        $wscript = "wscript" nocase
    condition:
        $lnk_header at 0 and any of ($cmd, $powershell, $mshta, $wscript)
}

rule Browser_Stealer_Patterns {
    meta:
        description = "Patterns de stealers de navigateurs"
        severity = "critical"
    strings:
        $login_data = "Login Data" nocase
        $cookies = "Cookies" nocase
        $web_data = "Web Data" nocase
        $local_state = "Local State" nocase
        $encrypted_key = "encrypted_key" nocase
    condition:
        3 of them
}

rule Dropper_Suspicious_Download {
    meta:
        description = "Téléchargement suspect via outils système"
        severity = "critical"
    strings:
        $certutil = "certutil" nocase
        $bitsadmin = "bitsadmin" nocase
        $urlmon = "URLDownloadToFile" nocase
        $webclient = "WebClient" nocase
        $downloadfile = "DownloadFile" nocase
        $downloadstring = "DownloadString" nocase
    condition:
        any of them
}
"#;
