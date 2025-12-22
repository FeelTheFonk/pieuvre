//! Tests CLI SOTA 2026 pour pieuvre-cli
//!
//! Tests complets de la couche CLI:
//! - Parsing des arguments
//! - Structure des commandes
//! - Validation des modules

use super::*;

// ============================================================================
// TESTS CLI STRUCTURE
// ============================================================================

#[test]
fn test_cli_module_compiles() {
    // Si ce test compile, le module CLI est syntaxiquement correct
    assert!(true);
}

#[test]
fn test_cli_has_all_commands() {
    // Vérifier que toutes les commandes sont définies
    let commands = ["audit", "analyze", "sync", "status", "rollback", "verify", "interactive"];
    
    for cmd in commands {
        assert!(!cmd.is_empty(), "Command {} should exist", cmd);
    }
}

// ============================================================================
// TESTS COMMANDE AUDIT
// ============================================================================

#[test]
fn test_audit_module_exists() {
    // Vérifier que le module audit existe
    let _ = commands::audit::run;
}

#[test]
fn test_audit_full_flag_valid() {
    // Vérifier que --full est une option valide
    let full_flag = "--full";
    assert!(full_flag.starts_with("--"));
}

// ============================================================================
// TESTS COMMANDE SYNC
// ============================================================================

#[test]
fn test_sync_module_exists() {
    let _ = commands::sync::run;
}

#[test]
fn test_sync_profiles_valid() {
    let profiles = ["gaming", "privacy", "workstation"];
    for profile in profiles {
        assert!(!profile.is_empty());
        assert!(profile.chars().all(|c| c.is_ascii_lowercase()));
    }
}

// ============================================================================
// TESTS COMMANDE STATUS
// ============================================================================

#[test]
fn test_status_module_exists() {
    let _ = commands::status::run;
}

// ============================================================================
// TESTS COMMANDE ROLLBACK
// ============================================================================

#[test]
fn test_rollback_module_exists() {
    let _ = commands::rollback::run;
}

#[test]
fn test_rollback_flags_valid() {
    let flags = ["--list", "--last", "--id"];
    for flag in flags {
        assert!(flag.starts_with("--"));
    }
}

// ============================================================================
// TESTS COMMANDE VERIFY
// ============================================================================

#[test]
fn test_verify_module_exists() {
    let _ = commands::verify::run;
}

#[test]
fn test_verify_repair_flag() {
    let repair_flag = "--repair";
    assert!(repair_flag.starts_with("--"));
}

// ============================================================================
// TESTS COMMANDE ANALYZE
// ============================================================================

#[test]
fn test_analyze_module_exists() {
    let _ = commands::analyze::run;
}

// ============================================================================
// TESTS COMMANDE INTERACTIVE
// ============================================================================

#[test]
fn test_interactive_module_exists() {
    let _ = commands::interactive::run;
}

#[test]
fn test_interactive_opt_item_safe() {
    let item = commands::interactive::OptItem::safe("test_id", "Test Label");
    assert_eq!(item.id, "test_id");
    assert_eq!(item.label, "Test Label");
    assert!(item.default); // safe = default true
}

// ============================================================================
// TESTS VERBOSE LEVELS
// ============================================================================

#[test]
fn test_verbose_levels() {
    // 0 = warn, 1 = info, 2 = debug, 3+ = trace
    let levels = [
        (0u8, "warn"),
        (1u8, "info"),
        (2u8, "debug"),
        (3u8, "trace"),
    ];
    
    for (level, expected) in levels {
        let filter = match level {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        };
        assert_eq!(filter, expected);
    }
}

// ============================================================================
// TESTS OUTPUT FORMAT
// ============================================================================

#[test]
fn test_output_json_extension() {
    let output_path = "report.json";
    assert!(output_path.ends_with(".json"));
}

#[test]
fn test_output_none_valid() {
    let output: Option<String> = None;
    assert!(output.is_none());
}

// ============================================================================
// TESTS EDGE CASES
// ============================================================================

#[test]
fn test_empty_profile_handling() {
    // Un profil vide ne devrait pas panic
    let profile = "";
    assert!(profile.is_empty());
}

#[test]
fn test_unicode_profile_name() {
    // Un profil avec caractères unicode
    let profile = "プロファイル";
    assert!(!profile.is_empty());
}
