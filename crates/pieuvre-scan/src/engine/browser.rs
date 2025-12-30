use crate::Result;
use serde::Deserialize;
use simd_json::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct FirefoxExtension {
    id: String,
    #[serde(rename = "userDisabled")]
    user_disabled: bool,
    #[serde(rename = "foreignInstall")]
    foreign_install: bool,
}

#[derive(Debug, Deserialize)]
struct FirefoxExtensions {
    addons: Vec<FirefoxExtension>,
}

pub struct BrowserForensics {}

impl Default for BrowserForensics {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserForensics {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan_chrome_profile(&self, profile_path: PathBuf) -> Result<Vec<String>> {
        let mut findings = Vec::new();

        // 1. Preferences (Extensions & Search)
        let prefs_path = profile_path.join("Preferences");
        if prefs_path.exists() {
            let mut content = std::fs::read(prefs_path)?;
            // SOTA: simd-json pour le parsing haute performance
            // Utilisation de to_owned_value car on ne peut pas emprunter content qui est local
            if let Ok(json) = simd_json::to_owned_value(&mut content) {
                if let Some(extensions) = json.get("extensions").and_then(|e| e.get("settings")) {
                    if let Some(obj) = extensions.as_object() {
                        for (id, _) in obj {
                            findings.push(format!("[Chrome] Extension found: {}", id));
                        }
                    }
                }

                if let Some(search) = json.get("default_search_provider") {
                    findings.push(format!("[Chrome] Search provider: {:?}", search));
                }
            }
        }

        // 2. SOTA: Check Enterprise Policies (Registry)
        // Note: Normalement fait par le RegistryWalker, mais ici on cible spécifiquement Chrome
        // pour une analyse contextuelle.

        Ok(findings)
    }

    pub fn scan_firefox_profile(&self, profile_path: PathBuf) -> Result<Vec<String>> {
        let mut findings = Vec::new();

        // 1. extensions.json (SOTA Forensics)
        let ext_path = profile_path.join("extensions.json");
        if ext_path.exists() {
            let mut content = std::fs::read(ext_path)?;
            if let Ok(ext_data) = simd_json::from_slice::<FirefoxExtensions>(&mut content) {
                for addon in ext_data.addons {
                    if !addon.user_disabled && addon.foreign_install {
                        findings.push(format!(
                            "[Firefox] Suspicious foreign extension: {}",
                            addon.id
                        ));
                    }
                }
            }
        }

        // 2. user.js (SOTA: Persistence Hijack)
        let user_js_path = profile_path.join("user.js");
        if user_js_path.exists() {
            findings.push("[Firefox] user.js found (Potential persistence hijack)".to_string());
            // On pourrait parser le JS ici pour extraire les URLs
        }

        // 3. places.sqlite (History/Bookmarks)
        let places_path = profile_path.join("places.sqlite");
        if places_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(places_path) {
                let mut stmt = conn.prepare(
                    "SELECT url FROM moz_places WHERE url LIKE '%conduit%' OR url LIKE '%babylon%'",
                )?;
                let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
                for url in rows {
                    findings.push(format!("[Firefox] Suspicious URL: {}", url?));
                }
            }
        }

        Ok(findings)
    }
}
