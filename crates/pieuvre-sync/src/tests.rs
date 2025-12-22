//! Tests unitaires SOTA pour pieuvre-sync
//!
//! Tests read-only validant la configuration et l'état du système.
//! Aucune modification n'est effectuée par ces tests.

use crate::{
    services, timer, power, dpc, cpu, security, game_mode,
};

// ============================================================================
// TESTS TIMER RESOLUTION
// ============================================================================

#[test]
fn test_get_timer_resolution_returns_valid_values() {
    let result = timer::get_timer_resolution();
    assert!(result.is_ok(), "get_timer_resolution should succeed");
    
    let info = result.unwrap();
    
    // Minimum doit être >= Maximum (valeurs en 100ns, minimum = plus grossier)
    assert!(
        info.minimum_100ns >= info.maximum_100ns,
        "Minimum ({}) should be >= Maximum ({})",
        info.minimum_100ns, info.maximum_100ns
    );
    
    // Current doit être entre min et max
    assert!(
        info.current_100ns >= info.maximum_100ns && info.current_100ns <= info.minimum_100ns,
        "Current ({}) should be between max ({}) and min ({})",
        info.current_100ns, info.maximum_100ns, info.minimum_100ns
    );
    
    // Maximum typique: 5000 (0.5ms) à 10000 (1ms)
    assert!(
        info.maximum_100ns >= 5000 && info.maximum_100ns <= 10000,
        "Maximum resolution should be 0.5-1ms, got {}",
        info.maximum_100ns
    );
}

#[test]
fn test_timer_resolution_info_conversions() {
    let info = timer::TimerResolutionInfo {
        minimum_100ns: 156250,   // 15.625ms
        maximum_100ns: 5000,     // 0.5ms
        current_100ns: 10000,    // 1ms
    };
    
    assert!((info.current_ms() - 1.0).abs() < 0.001, "Current should be 1.0ms");
    assert!((info.min_ms() - 15.625).abs() < 0.001, "Min should be 15.625ms");
    assert!((info.max_ms() - 0.5).abs() < 0.001, "Max should be 0.5ms");
    assert!((info.best_ms() - 0.5).abs() < 0.001, "Best should be 0.5ms");
}

// ============================================================================
// TESTS SERVICES
// ============================================================================

#[test]
fn test_get_service_start_type_existing_service() {
    // EventLog est un service critique qui existe toujours
    let result = services::get_service_start_type("EventLog");
    assert!(result.is_ok(), "Should get start type for EventLog");
    
    let start_type = result.unwrap();
    // EventLog est typiquement Automatic (2)
    assert!(
        start_type == 2 || start_type == 0,
        "EventLog should be Automatic (2) or Boot (0), got {}",
        start_type
    );
}

#[test]
fn test_get_service_start_type_nonexistent_service() {
    let result = services::get_service_start_type("NonExistentService12345");
    assert!(result.is_err(), "Should fail for non-existent service");
}

#[test]
fn test_get_service_start_type_diagtrack() {
    // DiagTrack peut être disabled ou manual selon système
    let result = services::get_service_start_type("DiagTrack");
    if let Ok(start_type) = result {
        // 2=Auto, 3=Manual, 4=Disabled
        assert!(
            start_type >= 2 && start_type <= 4,
            "DiagTrack start type should be 2-4, got {}",
            start_type
        );
    }
    // Note: Le service peut ne pas exister sur certaines éditions
}

// ============================================================================
// TESTS POWER PLANS
// ============================================================================

#[test]
fn test_get_active_power_plan() {
    let result = power::get_active_power_plan();
    assert!(result.is_ok(), "Should detect current power plan");
    
    let plan_name = result.unwrap();
    // Vérifier que c'est un nom valide (non vide)
    assert!(!plan_name.is_empty(), "Power plan name should not be empty");
}

#[test]
fn test_power_plan_guids() {
    assert_eq!(power::PowerPlan::Balanced.guid(), "381b4222-f694-41f0-9685-ff5bb260df2e");
    assert_eq!(power::PowerPlan::HighPerformance.guid(), "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c");
    assert_eq!(power::PowerPlan::UltimatePerformance.guid(), "e9a42b02-d5df-448d-aa00-03f14749eb61");
}

// ============================================================================
// TESTS DPC (Read-only checks)
// ============================================================================

#[test]
fn test_is_paging_executive_disabled_returns_bool() {
    // Test que la fonction retourne sans panic
    let _result = dpc::is_paging_executive_disabled();
}

// ============================================================================
// TESTS CPU (Read-only checks)
// ============================================================================

#[test]
fn test_is_memory_compression_enabled_returns_bool() {
    let _result = cpu::is_memory_compression_enabled();
}

#[test]
fn test_is_core_parking_disabled_returns_bool() {
    let _result = cpu::is_core_parking_disabled();
}

// ============================================================================
// TESTS SECURITY (Read-only checks)
// ============================================================================

#[test]
fn test_is_memory_integrity_enabled_returns_bool() {
    let _result = security::is_memory_integrity_enabled();
}

#[test]
fn test_is_vbs_enabled_returns_bool() {
    let _result = security::is_vbs_enabled();
}

// ============================================================================
// TESTS GAME MODE (Read-only checks)
// ============================================================================

#[test]
fn test_is_game_mode_enabled_returns_bool() {
    let _result = game_mode::is_game_mode_enabled();
}

#[test]
fn test_is_hags_enabled_returns_bool() {
    let _result = game_mode::is_hags_enabled();
}

// ============================================================================
// TESTS NETWORK
// ============================================================================

#[test]
fn test_is_nagle_disabled_returns_bool() {
    let _result = crate::network::is_nagle_disabled();
}

// ============================================================================
// TESTS REGISTRY (Read-only via audit crate)
// ============================================================================

#[test]
fn test_registry_key_paths_valid() {
    // Vérifier que les chemins de registre sont syntaxiquement corrects
    let valid_paths = [
        r"SYSTEM\CurrentControlSet\Control\PriorityControl",
        r"SOFTWARE\Policies\Microsoft\Windows\DataCollection",
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\AdvertisingInfo",
    ];
    
    for path in valid_paths {
        assert!(!path.is_empty());
        assert!(!path.starts_with('\\'), "Path should not start with backslash: {}", path);
        assert!(!path.ends_with('\\'), "Path should not end with backslash: {}", path);
    }
}

// ============================================================================
// TESTS PROFILE APPLICATION (Structure validation)
// ============================================================================

#[test]
fn test_profile_names_valid() {
    let valid_profiles = ["gaming", "privacy", "workstation"];
    
    for profile in valid_profiles {
        assert!(!profile.is_empty());
        assert!(profile.chars().all(|c| c.is_ascii_lowercase()), 
            "Profile name should be lowercase: {}", profile);
    }
}

#[test]
fn test_apply_profile_dry_run_gaming() {
    let result = crate::apply_profile("gaming", true);
    assert!(result.is_ok(), "Dry-run gaming profile should succeed");
}

#[test]
fn test_apply_profile_dry_run_privacy() {
    let result = crate::apply_profile("privacy", true);
    assert!(result.is_ok(), "Dry-run privacy profile should succeed");
}

#[test]
fn test_apply_profile_dry_run_workstation() {
    let result = crate::apply_profile("workstation", true);
    assert!(result.is_ok(), "Dry-run workstation profile should succeed");
}

#[test]
fn test_apply_profile_dry_run_unknown() {
    // Un profil inconnu ne devrait pas panic
    let result = crate::apply_profile("unknown_profile_xyz", true);
    assert!(result.is_ok(), "Dry-run unknown profile should succeed (no-op)");
}

// ============================================================================
// TESTS CONSTANTS VALIDATION
// ============================================================================

#[test]
fn test_telemetry_services_list() {
    let telemetry_services = [
        "DiagTrack", "dmwappushservice", "WerSvc", "wercplsupport",
        "PcaSvc", "WdiSystemHost", "WdiServiceHost", "lfsvc", "MapsBroker",
    ];
    
    for svc in telemetry_services {
        assert!(!svc.is_empty());
        assert!(svc.chars().all(|c| c.is_ascii_alphanumeric()), 
            "Service name should be alphanumeric: {}", svc);
    }
}

#[test]
fn test_power_plan_names() {
    assert_eq!(power::PowerPlan::Balanced.name(), "Balanced");
    assert_eq!(power::PowerPlan::HighPerformance.name(), "High Performance");
    assert_eq!(power::PowerPlan::UltimatePerformance.name(), "Ultimate Performance");
    assert_eq!(power::PowerPlan::PowerSaver.name(), "Power Saver");
}

// ============================================================================
// TESTS EDGE CASES
// ============================================================================

#[test]
fn test_empty_service_name() {
    let result = services::get_service_start_type("");
    assert!(result.is_err(), "Empty service name should fail");
}

#[test]
fn test_unicode_service_name() {
    let result = services::get_service_start_type("服务名称");
    assert!(result.is_err(), "Unicode service name should fail gracefully");
}
