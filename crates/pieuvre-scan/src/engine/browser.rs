use crate::engine::{Threat, ThreatSeverity};
use crate::Result;
use serde::Deserialize;
use simd_json::prelude::*;
use std::path::{Path, PathBuf};

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

/// Patterns suspects connus dans les moteurs de recherche hijackés
const SUSPICIOUS_SEARCH_PATTERNS: &[&str] = &[
    "conduit",
    "babylon",
    "mywebsearch",
    "trovi",
    "sweetim",
    "ask.com",
    "delta-search",
    "searchprotect",
];

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

    /// Scan un profil Chromium (Chrome, Edge, Brave)
    pub fn scan_chrome_profile(&self, profile_path: PathBuf) -> Result<Vec<Threat>> {
        let mut findings = Vec::new();
        let browser_name = self.detect_browser_name(&profile_path);

        // 1. Preferences (Extensions & Search)
        let prefs_path = profile_path.join("Preferences");
        if prefs_path.exists() {
            let mut content = std::fs::read(&prefs_path)?;
            if let Ok(json) = simd_json::to_owned_value(&mut content) {
                if let Some(extensions) = json.get("extensions").and_then(|e| e.get("settings")) {
                    if let Some(obj) = extensions.as_object() {
                        for (id, ext_info) in obj {
                            // Vérifier si l'extension a été installée par policy (forcée)
                            let is_policy = ext_info
                                .get("from_webstore")
                                .and_then(|v| v.as_bool())
                                .map(|v| !v)
                                .unwrap_or(false);

                            if is_policy {
                                findings.push(Threat {
                                    name: "Policy-Installed Extension".to_string(),
                                    description: format!("Extension forcée par GPO/Policy: {}", id),
                                    severity: ThreatSeverity::High,
                                    source: browser_name.clone(),
                                    location: prefs_path.to_string_lossy().to_string(),
                                });
                            }
                        }
                    }
                }

                // Analyse du moteur de recherche par défaut
                if let Some(search) = json.get("default_search_provider") {
                    if let Some(search_url) = search.get("search_url").and_then(|v| v.as_str()) {
                        let search_lower = search_url.to_lowercase();
                        for pattern in SUSPICIOUS_SEARCH_PATTERNS {
                            if search_lower.contains(pattern) {
                                findings.push(Threat {
                                    name: "Search Engine Hijack".to_string(),
                                    description: format!(
                                        "Moteur de recherche suspect: {}",
                                        search_url
                                    ),
                                    severity: ThreatSeverity::High,
                                    source: browser_name.clone(),
                                    location: prefs_path.to_string_lossy().to_string(),
                                });
                                break;
                            }
                        }
                    }
                }
            }
        }

        // 2. Web Data (Search Engines Hijack via SQLite)
        let web_data_path = profile_path.join("Web Data");
        if web_data_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(&web_data_path) {
                let query = format!(
                    "SELECT url FROM keywords WHERE {}",
                    SUSPICIOUS_SEARCH_PATTERNS
                        .iter()
                        .map(|p| format!("url LIKE '%{}%'", p))
                        .collect::<Vec<_>>()
                        .join(" OR ")
                );

                if let Ok(mut stmt) = conn.prepare(&query) {
                    if let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(0)) {
                        for url in rows.flatten() {
                            findings.push(Threat {
                                name: "Suspicious Search Engine".to_string(),
                                description: format!("Moteur de recherche suspect: {}", url),
                                severity: ThreatSeverity::High,
                                source: browser_name.clone(),
                                location: web_data_path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }

        // 3. History & Bookmarks (places.sqlite équivalent pour Chromium)
        let history_path = profile_path.join("History");
        if history_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(&history_path) {
                let query = format!(
                    "SELECT url FROM urls WHERE {}",
                    SUSPICIOUS_SEARCH_PATTERNS
                        .iter()
                        .map(|p| format!("url LIKE '%{}%'", p))
                        .collect::<Vec<_>>()
                        .join(" OR ")
                );

                if let Ok(mut stmt) = conn.prepare(&query) {
                    if let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(0)) {
                        for url in rows.flatten() {
                            findings.push(Threat {
                                name: "Suspicious URL in History".to_string(),
                                description: format!("URL suspecte dans l'historique: {}", url),
                                severity: ThreatSeverity::Medium,
                                source: browser_name.clone(),
                                location: history_path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(findings)
    }

    pub fn scan_firefox_profile(&self, profile_path: PathBuf) -> Result<Vec<Threat>> {
        let mut findings = Vec::new();

        // 1. extensions.json (Analyse Forensique)
        let ext_path = profile_path.join("extensions.json");
        if ext_path.exists() {
            let mut content = std::fs::read(&ext_path)?;
            if let Ok(ext_data) = simd_json::from_slice::<FirefoxExtensions>(&mut content) {
                for addon in ext_data.addons {
                    if !addon.user_disabled && addon.foreign_install {
                        findings.push(Threat {
                            name: "Foreign Extension Install".to_string(),
                            description: format!(
                                "Extension installée sans consentement: {}",
                                addon.id
                            ),
                            severity: ThreatSeverity::High,
                            source: "Firefox".to_string(),
                            location: ext_path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }

        // 2. user.js (Persistence Hijack)
        let user_js_path = profile_path.join("user.js");
        if user_js_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&user_js_path) {
                let suspicious_patterns = [
                    (
                        "browser.search.selectedEngine",
                        "Moteur de recherche modifié",
                    ),
                    ("browser.newtab.url", "Page de nouvel onglet modifiée"),
                    ("browser.startup.homepage", "Page d'accueil modifiée"),
                    ("extensions.enabledScopes", "Scopes d'extensions modifiés"),
                ];

                for (pattern, desc) in suspicious_patterns {
                    if content.contains(pattern) {
                        findings.push(Threat {
                            name: "User.js Persistence".to_string(),
                            description: format!("{} via user.js", desc),
                            severity: ThreatSeverity::High,
                            source: "Firefox".to_string(),
                            location: user_js_path.to_string_lossy().to_string(),
                        });
                    }
                }
            }
        }

        // 3. places.sqlite (History/Bookmarks)
        let places_path = profile_path.join("places.sqlite");
        if places_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(&places_path) {
                let query = format!(
                    "SELECT url FROM moz_places WHERE {}",
                    SUSPICIOUS_SEARCH_PATTERNS
                        .iter()
                        .map(|p| format!("url LIKE '%{}%'", p))
                        .collect::<Vec<_>>()
                        .join(" OR ")
                );

                if let Ok(mut stmt) = conn.prepare(&query) {
                    if let Ok(rows) = stmt.query_map([], |row| row.get::<_, String>(0)) {
                        for url in rows.flatten() {
                            findings.push(Threat {
                                name: "Suspicious URL".to_string(),
                                description: format!("URL suspecte: {}", url),
                                severity: ThreatSeverity::Medium,
                                source: "Firefox".to_string(),
                                location: places_path.to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(findings)
    }

    /// Détecte le nom du navigateur basé sur le chemin du profil
    fn detect_browser_name(&self, path: &Path) -> String {
        let path_str = path.to_string_lossy().to_lowercase();
        if path_str.contains("chrome") {
            "Chrome".to_string()
        } else if path_str.contains("edge") {
            "Edge".to_string()
        } else if path_str.contains("brave") {
            "Brave".to_string()
        } else if path_str.contains("opera") {
            "Opera".to_string()
        } else {
            "Chromium".to_string()
        }
    }
}
