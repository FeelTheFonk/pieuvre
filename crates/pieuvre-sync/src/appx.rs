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
            &format!("Get-AppxPackage -Name '*{}*' | Remove-AppxPackage -ErrorAction SilentlyContinue", name),
        ])
        .output()?;  // io::Error est From pour PieuvreError
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Ignorer erreur "not found" car normal si deja supprime
        if !stderr.contains("not found") && !stderr.is_empty() {
            tracing::warn!("AppX {}: {}", name, stderr);
        }
    }
    
    Ok(())
}

/// Liste des bloatware a supprimer
const BLOATWARE: &[&str] = &[
    "Microsoft.BingNews",
    "Microsoft.BingWeather",
    "Microsoft.BingFinance",
    "Microsoft.BingSports",
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
    "Clipchamp.Clipchamp",
    "MicrosoftCorporationII.QuickAssist",
    "Microsoft.549981C3F5F10", // Cortana
    "Microsoft.MixedReality.Portal",
    "Microsoft.SkypeApp",
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
