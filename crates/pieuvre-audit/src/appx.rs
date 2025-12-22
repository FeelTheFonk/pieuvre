//! Appx Scanner
//!
//! Détection et classification des packages UWP/Appx.

use pieuvre_common::{AppxCategory, AppxInfo, RemovalRisk, Result};

/// Liste des packages bloatware connus
const KNOWN_BLOATWARE: &[&str] = &[
    "Microsoft.BingNews",
    "Microsoft.BingWeather",
    "Microsoft.BingFinance",
    "Microsoft.BingSports",
    "Microsoft.GamingApp",
    "Microsoft.GetHelp",
    "Microsoft.Getstarted",
    "Microsoft.MicrosoftOfficeHub",
    "Microsoft.MicrosoftSolitaireCollection",
    "Microsoft.People",
    "Microsoft.PowerAutomateDesktop",
    "Microsoft.Todos",
    "Microsoft.WindowsFeedbackHub",
    "Microsoft.WindowsMaps",
    "Microsoft.Xbox.TCUI",
    "Microsoft.XboxGameOverlay",
    "Microsoft.XboxGamingOverlay",
    "Microsoft.XboxIdentityProvider",
    "Microsoft.XboxSpeechToTextOverlay",
    "Microsoft.YourPhone",
    "Microsoft.ZuneMusic",
    "Microsoft.ZuneVideo",
    "Clipchamp.Clipchamp",
    "MicrosoftCorporationII.QuickAssist",
    "MicrosoftTeams",
    "Microsoft.549981C3F5F10", // Cortana
    "Microsoft.MixedReality.Portal",
    "Microsoft.SkypeApp",
    "Microsoft.WindowsAlarms",
    "Microsoft.windowscommunicationsapps", // Mail/Calendar
];

/// Packages système critiques à ne jamais toucher
const SYSTEM_CRITICAL: &[&str] = &[
    "Microsoft.WindowsStore",
    "Microsoft.Windows.ShellExperienceHost",
    "Microsoft.Windows.StartMenuExperienceHost",
    "Microsoft.Windows.Search",
    "Microsoft.AAD.BrokerPlugin",
    "Microsoft.AccountsControl",
    "Microsoft.UI.Xaml",
    "Microsoft.VCLibs",
    "Microsoft.NET",
    "Microsoft.WindowsAppRuntime",
];

/// Scan les packages Appx installés
pub fn scan_packages() -> Result<Vec<AppxInfo>> {
    let mut packages = Vec::new();
    
    // Utiliser PowerShell pour lister les packages (méthode plus simple que WinRT direct)
    // En production, utiliser Windows::Management::Deployment::PackageManager
    
    // Pour l'instant, récupérer depuis le registre
    use windows::Win32::System::Registry::{RegOpenKeyExW, RegEnumKeyExW, RegCloseKey, HKEY_CURRENT_USER, KEY_READ};
    use windows::core::PCWSTR;
    
    unsafe {
        let subkey: Vec<u16> = r"Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        
        let mut hkey = Default::default();
        if RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), Some(0), KEY_READ, &mut hkey).is_ok() {
            let mut index = 0u32;
            loop {
                let mut name_buffer = vec![0u16; 512];
                let mut name_len = name_buffer.len() as u32;
                
                let result = RegEnumKeyExW(
                    hkey,
                    index,
                    Some(windows::core::PWSTR(name_buffer.as_mut_ptr())),
                    &mut name_len,
                    None,
                    None,
                    None,
                    None,
                );
                
                if result.is_err() {
                    break;
                }
                
                let full_name = String::from_utf16_lossy(&name_buffer[..name_len as usize]);
                
                // Extraire le nom du package (partie avant le _)
                let name = full_name.split('_').next().unwrap_or(&full_name).to_string();
                
                let category = categorize_package(&name);
                let removal_risk = assess_removal_risk(&name);
                
                packages.push(AppxInfo {
                    name: name.clone(),
                    full_name,
                    publisher: String::new(),
                    version: String::new(),
                    is_provisioned: false,
                    category,
                    removal_risk,
                });
                
                index += 1;
                
                // Limiter à 200 packages pour éviter explosion
                if index > 200 {
                    break;
                }
            }
            
            let _ = RegCloseKey(hkey);
        }
    }
    
    Ok(packages)
}

fn categorize_package(name: &str) -> AppxCategory {
    let lower = name.to_lowercase();
    
    if lower.starts_with("microsoft.windows") || lower.starts_with("microsoft.ui") || lower.contains("runtime") {
        AppxCategory::System
    } else if lower.contains("xbox") || lower.contains("gaming") {
        AppxCategory::Gaming
    } else if lower.contains("office") || lower.contains("onenote") || lower.contains("outlook") {
        AppxCategory::Productivity
    } else if lower.contains("zune") || lower.contains("media") || lower.contains("photo") {
        AppxCategory::Media
    } else if lower.contains("calculator") || lower.contains("paint") || lower.contains("notepad") {
        AppxCategory::Utility
    } else if lower.starts_with("microsoft.") {
        AppxCategory::Microsoft
    } else {
        AppxCategory::ThirdParty
    }
}

fn assess_removal_risk(name: &str) -> RemovalRisk {
    // Packages système critiques
    for critical in SYSTEM_CRITICAL {
        if name.to_lowercase().starts_with(&critical.to_lowercase()) {
            return RemovalRisk::Critical;
        }
    }
    
    // Bloatware connu = safe
    for bloat in KNOWN_BLOATWARE {
        if name.to_lowercase().starts_with(&bloat.to_lowercase()) {
            return RemovalRisk::Safe;
        }
    }
    
    // Par défaut, prudence
    if name.starts_with("Microsoft.") {
        RemovalRisk::Caution
    } else {
        RemovalRisk::Safe
    }
}

/// Retourne la liste des bloatwares détectés
pub fn get_bloatware(packages: &[AppxInfo]) -> Vec<&AppxInfo> {
    packages.iter()
        .filter(|p| p.removal_risk == RemovalRisk::Safe && 
                    KNOWN_BLOATWARE.iter().any(|b| p.name.to_lowercase().starts_with(&b.to_lowercase())))
        .collect()
}
