use crate::Result;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

pub struct FastFilter {
    ac: AhoCorasick,
}

impl Default for FastFilter {
    fn default() -> Self {
        Self::new(BLITZ_PATTERNS).expect("Failed to build default FastFilter")
    }
}

impl FastFilter {
    pub fn new(patterns: &[&str]) -> Result<Self> {
        let ac = AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .build(patterns)
            .map_err(|e| crate::ScanError::Other(e.to_string()))?;

        Ok(Self { ac })
    }

    pub fn is_suspicious(&self, input: &str) -> bool {
        self.ac.find(input).is_some()
    }
}

/// Patterns de pré-filtrage (Blitz)
pub const BLITZ_PATTERNS: &[&str] = &[
    "Conduit",
    "Babylon",
    "SweetIM",
    "MyWebSearch",
    "Trovi",
    "SearchProtect",
    "powershell",
    "cmd.exe",
    "bitsadmin",
    "mshta",
    "certutil",
    "regsvr32",
    "wscript",
    "cscript",
    "AppInit_DLLs",
    "Image File Execution Options",
    "ExtensionInstallForcelist",
    "user.js",
];
