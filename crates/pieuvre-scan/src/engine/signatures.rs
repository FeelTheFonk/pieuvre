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
}

/// Règles YARA par défaut intégrées au binaire
pub const DEFAULT_RULES: &str = r#"
rule Adware_Generic {
    meta:
        description = "Détection générique d'adware"
    strings:
        $s1 = "search.conduit.com" nocase
        $s2 = "babylon.com" nocase
    condition:
        any of them
}

rule Persistence_IFEO_Hijack {
    meta:
        description = "Détection de détournement IFEO"
    strings:
        $debugger = "Debugger" nocase
    condition:
        $debugger
}

rule Browser_Extension_ForceInstall {
    meta:
        description = "Détection d'extensions forcées"
    strings:
        $policy = "ExtensionInstallForcelist" nocase
    condition:
        $policy
}
"#;
