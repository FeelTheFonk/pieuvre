//! Gestion des packages AppX
//!
//! Suppression des bloatware et packages non desires.

use pieuvre_common::Result;
use std::process::Command;

/// Supprime un package AppX par son nom
pub fn remove_package(name: &str) -> Result<()> {
    tracing::info!("Suppression package: {}", name);

    // Utiliser PowerShell pour supprimer
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "Get-AppxPackage -Name '*{}*' | Remove-AppxPackage -ErrorAction SilentlyContinue",
                name
            ),
        ])
        .output()?; // io::Error est From pour PieuvreError

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignorer erreur "not found" car normal si deja supprime
        if !stderr.contains("not found") && !stderr.is_empty() {
            tracing::warn!("AppX {}: {}", name, stderr);
        }
    }

    Ok(())
}

/// Liste des bloatware a supprimer (SOTA - base Win11Debloat)
const BLOATWARE: &[&str] = &[
    // Bing apps
    "Microsoft.BingNews",
    "Microsoft.BingWeather",
    "Microsoft.BingFinance",
    "Microsoft.BingSports",
    "Microsoft.BingSearch",
    // Microsoft apps non essentiels
    "Microsoft.GetHelp",
    "Microsoft.Getstarted",
    "Microsoft.MicrosoftSolitaireCollection",
    "Microsoft.People",
    "Microsoft.PowerAutomateDesktop",
    "Microsoft.Todos",
    "Microsoft.WindowsFeedbackHub",
    "Microsoft.WindowsMaps",
    "Microsoft.YourPhone",
    "Microsoft.ZuneMusic",
    "Microsoft.ZuneVideo",
    "Microsoft.MixedReality.Portal",
    "Microsoft.SkypeApp",
    "Microsoft.549981C3F5F10", // Cortana
    "Microsoft.WindowsAlarms",
    "Microsoft.windowscommunicationsapps", // Mail & Calendar
    "Microsoft.MicrosoftOfficeHub",
    "Microsoft.OutlookForWindows",
    "Microsoft.Paint3D",
    "Microsoft.3DBuilder",
    "Microsoft.OneConnect",
    "Microsoft.Wallet",
    "Microsoft.Messaging",
    "Microsoft.Print3D",
    "Microsoft.NetworkSpeedTest",
    // Microsoft extras
    "Clipchamp.Clipchamp",
    "MicrosoftCorporationII.QuickAssist",
    "MicrosoftTeams",
    "Microsoft.MicrosoftStickyNotes",
    "Microsoft.ScreenSketch", // Snipping Tool moderne
    // Third party pre-installs
    "SpotifyAB.SpotifyMusic",
    "Disney.37853FC22B2CE",
    "king.com.CandyCrushSaga",
    "king.com.CandyCrushSodaSaga",
    "FACEBOOK.FACEBOOK",
    "AdobeSystemsIncorporated.AdobePhotoshopExpress",
    // Copilot
    "Microsoft.Copilot",
    "Microsoft.Windows.Ai.Copilot.Provider",
    // Windows 11 24H2 additions
    "Microsoft.DevHome",
    "Microsoft.CrossDevice",
    "MicrosoftWindows.CrossDevice",
    "Microsoft.Windows.DevHomeAzureExtension",
    "Microsoft.Windows.DevHomeGitHubExtension",
];

/// Supprime tous les bloatware connus
pub fn remove_bloatware() -> Result<Vec<String>> {
    let mut removed = Vec::new();

    for pkg in BLOATWARE {
        match remove_package(pkg) {
            Ok(_) => {
                removed.push(pkg.to_string());
            }
            Err(e) => {
                tracing::warn!("Echec suppression {}: {}", pkg, e);
            }
        }
    }

    tracing::info!("Bloatware: {} packages traites", removed.len());
    Ok(removed)
}

/// Supprime les packages Xbox (optionnel car utile pour Game Pass)
pub fn remove_xbox_packages() -> Result<Vec<String>> {
    let xbox = &[
        "Microsoft.Xbox.TCUI",
        "Microsoft.XboxGameOverlay",
        "Microsoft.XboxGamingOverlay",
        "Microsoft.XboxIdentityProvider",
        "Microsoft.XboxSpeechToTextOverlay",
        "Microsoft.GamingApp",
    ];

    let mut removed = Vec::new();
    for pkg in xbox {
        if remove_package(pkg).is_ok() {
            removed.push(pkg.to_string());
        }
    }

    Ok(removed)
}

/// Verifie si un package est installe
pub fn is_package_installed(name: &str) -> bool {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!("(Get-AppxPackage -Name '*{}*') -ne $null", name),
        ])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            stdout.trim().eq_ignore_ascii_case("true")
        }
        Err(_) => false,
    }
}

// ============================================
// FONCTIONS PAR CATEGORIE (selection granulaire)
// ============================================

/// Supprime les apps Bing
pub fn remove_bing_apps() -> Result<Vec<String>> {
    let pkgs = &[
        "Microsoft.BingNews",
        "Microsoft.BingWeather",
        "Microsoft.BingFinance",
        "Microsoft.BingSports",
        "Microsoft.BingSearch",
    ];
    remove_packages_list(pkgs)
}

/// Supprime les apps productivite
pub fn remove_ms_productivity() -> Result<Vec<String>> {
    let pkgs = &[
        "Microsoft.Todos",
        "Microsoft.People",
        "Microsoft.MicrosoftOfficeHub",
        "Microsoft.YourPhone",
        "Microsoft.MicrosoftStickyNotes",
    ];
    remove_packages_list(pkgs)
}

/// Supprime les apps media
pub fn remove_ms_media() -> Result<Vec<String>> {
    let pkgs = &[
        "Microsoft.ZuneMusic",
        "Microsoft.ZuneVideo",
        "Clipchamp.Clipchamp",
    ];
    remove_packages_list(pkgs)
}

/// Supprime les apps communication
pub fn remove_ms_communication() -> Result<Vec<String>> {
    let pkgs = &[
        "Microsoft.windowscommunicationsapps",
        "Microsoft.SkypeApp",
        "MicrosoftTeams",
        "Microsoft.OutlookForWindows",
    ];
    remove_packages_list(pkgs)
}

/// Supprime les apps legacy
pub fn remove_ms_legacy() -> Result<Vec<String>> {
    let pkgs = &[
        "Microsoft.Paint3D",
        "Microsoft.3DBuilder",
        "Microsoft.Print3D",
        "Microsoft.MixedReality.Portal",
        "Microsoft.OneConnect",
        "Microsoft.Wallet",
    ];
    remove_packages_list(pkgs)
}

/// Supprime les outils Microsoft
pub fn remove_ms_tools() -> Result<Vec<String>> {
    let pkgs = &[
        "Microsoft.WindowsFeedbackHub",
        "Microsoft.GetHelp",
        "Microsoft.Getstarted",
        "MicrosoftCorporationII.QuickAssist",
        "Microsoft.WindowsMaps",
    ];
    remove_packages_list(pkgs)
}

/// Supprime les apps third-party
pub fn remove_third_party() -> Result<Vec<String>> {
    let pkgs = &[
        "SpotifyAB.SpotifyMusic",
        "Disney.37853FC22B2CE",
        "king.com.CandyCrushSaga",
        "king.com.CandyCrushSodaSaga",
        "FACEBOOK.FACEBOOK",
        "AdobeSystemsIncorporated.AdobePhotoshopExpress",
    ];
    remove_packages_list(pkgs)
}

/// Supprime Copilot
pub fn remove_copilot() -> Result<Vec<String>> {
    let pkgs = &["Microsoft.Copilot", "Microsoft.Windows.Ai.Copilot.Provider"];
    remove_packages_list(pkgs)
}

/// Supprime Cortana
pub fn remove_cortana() -> Result<Vec<String>> {
    let pkgs = &["Microsoft.549981C3F5F10"];
    remove_packages_list(pkgs)
}

/// Helper: supprime une liste de packages
fn remove_packages_list(pkgs: &[&str]) -> Result<Vec<String>> {
    let mut removed = Vec::new();
    for pkg in pkgs {
        if remove_package(pkg).is_ok() {
            removed.push(pkg.to_string());
        }
    }
    Ok(removed)
}
