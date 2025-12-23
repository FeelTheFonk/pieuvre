//! Unit tests for pieuvre-audit
//!
//! Read-only/dry-run tests to validate all functionalities.

use crate::{
    appx, full_audit, hardware, is_laptop, network, network_audit, registry, security,
    security_audit, services,
};
use pieuvre_common::{RemovalRisk, ServiceCategory, ServiceStartType, ServiceStatus};

// ========================================================================
// TESTS HARDWARE
// ========================================================================

#[test]
fn test_probe_hardware_returns_valid_data() {
    let result = hardware::probe_hardware();
    assert!(result.is_ok(), "probe_hardware should succeed");

    let hw = result.unwrap();

    // CPU must have at least 1 core
    assert!(
        hw.cpu.logical_cores >= 1,
        "Should have at least 1 logical core"
    );
    assert!(!hw.cpu.vendor.is_empty(), "CPU vendor should not be empty");
    assert!(
        !hw.cpu.model_name.is_empty(),
        "CPU model_name should not be empty"
    );

    // RAM must be > 0
    assert!(hw.memory.total_bytes > 0, "Total RAM should be > 0");
    assert!(hw.memory.available_bytes > 0, "Available RAM should be > 0");
    assert!(
        hw.memory.available_bytes <= hw.memory.total_bytes,
        "Available <= Total"
    );
}

#[test]
fn test_is_laptop_returns_bool() {
    // Test that the function does not panic
    let _is_laptop = hardware::is_laptop();
}

#[test]
fn test_gpu_detection() {
    let result = hardware::probe_hardware();
    assert!(result.is_ok());

    let hw = result.unwrap();
    // GPU might be empty on some systems (VM without GPU)
    for gpu in &hw.gpu {
        assert!(!gpu.name.is_empty(), "GPU name should not be empty");
        assert!(!gpu.vendor.is_empty(), "GPU vendor should not be empty");
    }
}

#[test]
fn test_storage_detection() {
    let result = hardware::probe_hardware();
    assert!(result.is_ok());

    let hw = result.unwrap();
    // At least one fixed drive expected
    assert!(
        !hw.storage.is_empty(),
        "Should detect at least one storage device"
    );

    for drive in &hw.storage {
        assert!(!drive.device_id.is_empty(), "Device ID should not be empty");
    }
}

// ========================================================================
// TESTS SERVICES
// ========================================================================

#[test]
fn test_inspect_services_returns_list() {
    let result = services::inspect_services();
    assert!(result.is_ok(), "inspect_services should succeed");

    let svcs = result.unwrap();
    assert!(!svcs.is_empty(), "Should detect services");

    // These services should exist on any Windows system
    let common_services = ["EventLog", "PlugPlay", "RpcSs"];
    for expected in common_services {
        assert!(
            svcs.iter().any(|s| s.name.eq_ignore_ascii_case(expected)),
            "Should find {} service",
            expected
        );
    }
}

#[test]
fn test_service_start_type_not_all_manual() {
    let result = services::inspect_services();
    assert!(result.is_ok());

    let svcs = result.unwrap();

    let manual_count = svcs
        .iter()
        .filter(|s| matches!(s.start_type, ServiceStartType::Manual))
        .count();

    let total = svcs.len();
    let manual_ratio = manual_count as f64 / total as f64;

    // If > 80% are Manual, it's likely a bug
    assert!(
        manual_ratio < 0.8,
        "Too many services are Manual ({}/{})",
        manual_count,
        total
    );
}

#[test]
fn test_service_categories_diverse() {
    let result = services::inspect_services();
    assert!(result.is_ok());

    let svcs = result.unwrap();

    let has_system = svcs
        .iter()
        .any(|s| matches!(s.category, ServiceCategory::System));
    assert!(has_system, "Should categorize some services as System");
}

#[test]
fn test_get_active_telemetry_services() {
    let result = services::inspect_services();
    assert!(result.is_ok());

    let svcs = result.unwrap();
    let telemetry = services::get_active_telemetry_services(&svcs);

    for svc in telemetry {
        assert!(matches!(svc.category, ServiceCategory::Telemetry));
        assert!(matches!(svc.status, ServiceStatus::Running));
    }
}

// ========================================================================
// TESTS REGISTRY
// ========================================================================

#[test]
fn test_get_telemetry_status() {
    let result = registry::get_telemetry_status();
    assert!(result.is_ok(), "get_telemetry_status should succeed");

    let status = result.unwrap();
    assert!(
        status.data_collection_level <= 3,
        "Data collection level should be 0-3, got {}",
        status.data_collection_level
    );
}

#[test]
fn test_get_defender_status() {
    let result = registry::get_defender_status();
    assert!(result.is_ok(), "get_defender_status should succeed");

    let status = result.unwrap();
    assert!(
        status.sample_submission <= 3,
        "Sample submission should be 0-3, got {}",
        status.sample_submission
    );
}

#[test]
fn test_get_uac_status() {
    let result = registry::get_uac_status();
    assert!(result.is_ok(), "get_uac_status should succeed");

    let status = result.unwrap();
    assert!(
        status.consent_prompt_behavior <= 5,
        "Consent prompt should be 0-5, got {}",
        status.consent_prompt_behavior
    );
}

#[test]
fn test_get_firewall_status() {
    let result = registry::get_firewall_status();
    assert!(result.is_ok(), "get_firewall_status should succeed");
}

#[test]
fn test_read_dword_invalid_key() {
    let result = registry::read_dword_value(r"SOFTWARE\NonExistent\Key\Path", "NonExistentValue");
    assert!(result.is_err(), "Should fail for non-existent key");
}

#[test]
fn test_key_exists() {
    let exists = registry::key_exists(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion");
    assert!(exists, "Windows NT CurrentVersion key should exist");

    let not_exists = registry::key_exists(r"SOFTWARE\NonExistent\Key\12345");
    assert!(!not_exists, "Non-existent key should return false");
}

#[test]
fn test_is_secure_boot_enabled() {
    let _result = registry::is_secure_boot_enabled();
}

// ========================================================================
// TESTS SECURITY
// ========================================================================

#[test]
fn test_audit_security() {
    let result = security::audit_security();
    assert!(result.is_ok(), "audit_security should succeed");

    let audit = result.unwrap();
    assert!(
        audit.security_score <= 100,
        "Security score should be <= 100, got {}",
        audit.security_score
    );
}

#[test]
fn test_security_recommendations_valid() {
    let result = security::audit_security();
    assert!(result.is_ok());

    let audit = result.unwrap();

    for rec in &audit.recommendations {
        assert!(
            !rec.title.is_empty(),
            "Recommendation title should not be empty"
        );
        assert!(
            !rec.category.is_empty(),
            "Recommendation category should not be empty"
        );
    }
}

#[test]
fn test_score_to_grade() {
    assert_eq!(security::score_to_grade(100), "A - Excellent");
    assert_eq!(security::score_to_grade(95), "A - Excellent");
    assert_eq!(security::score_to_grade(85), "B - Good");
    assert_eq!(security::score_to_grade(75), "C - Acceptable");
    assert_eq!(security::score_to_grade(65), "D - Needs Improvement");
    assert_eq!(security::score_to_grade(50), "F - Critical");
}

#[test]
fn test_count_by_severity() {
    let recs = vec![
        security::SecurityRecommendation {
            severity: security::Severity::Critical,
            category: "Test".into(),
            title: "Test".into(),
            description: "Test".into(),
            remediation: "Test".into(),
        },
        security::SecurityRecommendation {
            severity: security::Severity::High,
            category: "Test".into(),
            title: "Test".into(),
            description: "Test".into(),
            remediation: "Test".into(),
        },
        security::SecurityRecommendation {
            severity: security::Severity::Low,
            category: "Test".into(),
            title: "Test".into(),
            description: "Test".into(),
            remediation: "Test".into(),
        },
    ];

    let (critical, high, medium, low) = security::count_by_severity(&recs);
    assert_eq!(critical, 1);
    assert_eq!(high, 1);
    assert_eq!(medium, 0);
    assert_eq!(low, 1);
}

// ========================================================================
// TESTS APPX
// ========================================================================

#[test]
fn test_scan_packages() {
    let result = appx::scan_packages();
    assert!(result.is_ok(), "scan_packages should succeed");
}

#[test]
fn test_get_bloatware() {
    let result = appx::scan_packages();
    if let Ok(packages) = result {
        let bloat = appx::get_bloatware(&packages);

        for pkg in bloat {
            assert!(matches!(pkg.removal_risk, RemovalRisk::Safe));
        }
    }
}

// ========================================================================
// TESTS NETWORK
// ========================================================================

#[test]
fn test_get_telemetry_domains() {
    let domains = network::get_telemetry_domains();
    assert!(!domains.is_empty(), "Should have telemetry domains list");

    assert!(domains.iter().any(|d: &&str| d.contains("microsoft.com")));
    assert!(domains.iter().any(|d: &&str| d.contains("telemetry")));
}

#[test]
fn test_get_telemetry_ip_ranges() {
    let ranges = network::get_telemetry_ip_ranges();
    assert!(!ranges.is_empty(), "Should have IP ranges list");

    for range in ranges {
        assert!(
            range.contains('/'),
            "IP range should be in CIDR format: {}",
            range
        );
    }
}

#[test]
fn test_is_telemetry_domain() {
    assert!(network::is_telemetry_domain("telemetry.microsoft.com"));
    assert!(network::is_telemetry_domain("vortex.data.microsoft.com"));
    assert!(!network::is_telemetry_domain("www.google.com"));
    assert!(!network::is_telemetry_domain("github.com"));
}

// ========================================================================
// TESTS LIB (FULL AUDIT)
// ========================================================================

#[test]
fn test_full_audit() {
    let result = full_audit();
    assert!(result.is_ok(), "full_audit should succeed");

    let report = result.unwrap();

    assert!(!report.system.os_version.is_empty());
    assert!(report.system.build_number > 0);
    assert!(report.hardware.cpu.logical_cores >= 1);
    assert!(report.hardware.memory.total_bytes > 0);
}

#[test]
fn test_security_audit_func() {
    let result = security_audit();
    assert!(result.is_ok(), "security_audit should succeed");
}

#[test]
fn test_is_laptop_function() {
    let _result = is_laptop();
}

#[test]
fn test_network_audit() {
    let result = network_audit();
    assert!(result.is_ok(), "network_audit should succeed");
}
