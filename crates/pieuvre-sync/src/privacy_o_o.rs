//! Module Privacy
//! Centralisation des paramètres de confidentialité.

use crate::registry;
use pieuvre_common::Result;

/// Applique tous les paramètres de confidentialité "Recommandés" (Verts) de O&O ShutUp10++
pub fn apply_all_recommended_privacy() -> Result<()> {
    apply_telemetry_settings()?;
    apply_ai_settings()?;
    apply_shell_settings()?;
    apply_network_settings()?;
    apply_app_permissions()?;
    apply_legacy_settings()?;
    Ok(())
}

/// 1. Télémétrie et Collecte de Données
fn apply_telemetry_settings() -> Result<()> {
    // Télémétrie (Security level)
    registry::set_value_multi_hive_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        "AllowTelemetry",
        0,
    )?;

    // Advertising ID
    registry::set_value_multi_hive_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\AdvertisingInfo",
        "DisabledByGroupPolicy",
        1,
    )?;
    registry::set_value_multi_hive_dword(
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
        "Enabled",
        0,
    )?;

    // CEIP / SQM
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\SQMClient\Windows",
        "CEIPEnable",
        0,
    )?;
    registry::set_dword_value(r"SOFTWARE\Microsoft\SQMClient\Windows", "CEIPEnable", 0)?;

    tracing::info!("O&O: Télémétrie et Collecte de données configurées");
    Ok(())
}

/// 2. Services Cognitifs et IA (Windows AI)
fn apply_ai_settings() -> Result<()> {
    // Windows Copilot
    registry::set_value_multi_hive_dword(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
        "TurnOffWindowsCopilot",
        1,
    )?;

    // Windows Recall
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
        "DisableAIDataAnalysis",
        1,
    )?;
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
        "AllowRecallEnablement",
        0,
    )?;

    tracing::info!("O&O: Services IA et Recall désactivés");
    Ok(())
}

/// 3. Interface Utilisateur et Shell Experience
fn apply_shell_settings() -> Result<()> {
    // Widgets
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Dsh",
        "AllowNewsAndInterests",
        0,
    )?;
    registry::set_value_multi_hive_dword(
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
        "TaskbarDa",
        0,
    )?;

    // Start Menu Recommendations
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\Explorer",
        "HideRecommendedSection",
        1,
    )?;

    // Search Highlights & Web Search
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "AllowSearchHighlights",
        0,
    )?;
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\Windows Search",
        "DisableWebSearch",
        1,
    )?;

    tracing::info!("O&O: Interface Shell et Widgets épurés");
    Ok(())
}

/// 4. Sécurité Réseau et Mises à jour
fn apply_network_settings() -> Result<()> {
    // WUDO (Delivery Optimization) - Mode 0 (HTTP Only)
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization",
        "DODownloadMode",
        0,
    )?;

    // Wi-Fi Sense
    registry::set_dword_value(
        r"SOFTWARE\Microsoft\WcmSvc\wifinetworkmanager\config",
        "AutoConnectAllowedOEM",
        0,
    )?;

    tracing::info!("O&O: Réseau et Delivery Optimization sécurisés");
    Ok(())
}

/// 5. Permissions Applicatives (Capability Access Manager)
fn apply_app_permissions() -> Result<()> {
    let base_path =
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore";

    registry::set_string_value(&format!("{}\\{}", base_path, "location"), "Value", "Deny")?;
    registry::set_string_value(&format!("{}\\{}", base_path, "webcam"), "Value", "Deny")?;
    registry::set_string_value(&format!("{}\\{}", base_path, "microphone"), "Value", "Deny")?;
    registry::set_string_value(
        &format!("{}\\{}", base_path, "userNotification"),
        "Value",
        "Deny",
    )?;

    // Background Apps
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\AppPrivacy",
        "LetAppsRunInBackground",
        2,
    )?;

    tracing::info!("O&O: Permissions applicatives (Caméra/Micro/Loc) verrouillées");
    Ok(())
}

/// 6. Fonctionnalités Diverses
fn apply_legacy_settings() -> Result<()> {
    // Password Reveal
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\CredUI",
        "DisablePasswordReveal",
        1,
    )?;

    // Steps Recorder (UAR)
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "DisableUAR",
        1,
    )?;

    // Inventory Collector
    registry::set_dword_value(
        r"SOFTWARE\Policies\Microsoft\Windows\AppCompat",
        "DisableInventory",
        1,
    )?;

    tracing::info!("O&O: Paramètres système divers optimisés");
    Ok(())
}
