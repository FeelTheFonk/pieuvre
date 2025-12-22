//! Windows Update Control
//!
//! Control Windows Update behavior and deferral.

use pieuvre_common::Result;

/// Pause Windows Updates for 35 days (maximum)
pub fn pause_updates() -> Result<()> {
    // Calculate date 35 days from now
    let pause_date = chrono::Utc::now() + chrono::Duration::days(35);
    let date_str = pause_date.format("%Y-%m-%d").to_string();

    let ux_settings = r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings";

    // Pause feature updates
    let _ =
        crate::registry::set_string_value(ux_settings, "PauseFeatureUpdatesStartTime", &date_str);

    // Pause quality updates
    let _ =
        crate::registry::set_string_value(ux_settings, "PauseQualityUpdatesStartTime", &date_str);

    // Disable auto-restart
    let au_key = r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU";
    let _ = crate::registry::set_dword_value(au_key, "NoAutoRebootWithLoggedOnUsers", 1);

    tracing::info!("Windows Updates paused for 35 days");
    Ok(())
}

/// Disable automatic driver updates
pub fn disable_driver_updates() -> Result<()> {
    let wu_policy = r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate";
    let _ = crate::registry::set_dword_value(wu_policy, "ExcludeWUDriversInQualityUpdate", 1);

    // Also via Device Installation Settings
    let driver_search = r"SOFTWARE\Microsoft\Windows\CurrentVersion\DriverSearching";
    let _ = crate::registry::set_dword_value(driver_search, "SearchOrderConfig", 0);

    tracing::info!("Automatic driver updates disabled");
    Ok(())
}

/// Enable automatic driver updates
pub fn enable_driver_updates() -> Result<()> {
    let wu_policy = r"SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate";
    let _ = crate::registry::delete_value(wu_policy, "ExcludeWUDriversInQualityUpdate");

    tracing::info!("Automatic driver updates enabled");
    Ok(())
}

/// Check if updates are paused
pub fn is_updates_paused() -> bool {
    let ux_settings = r"SOFTWARE\Microsoft\WindowsUpdate\UX\Settings";
    crate::registry::key_exists(ux_settings)
        && crate::registry::read_string_value(ux_settings, "PauseFeatureUpdatesStartTime").is_ok()
}
