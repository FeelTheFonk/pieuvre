//! Security Auditor
//!
//! Full audit of security posture: Defender, Firewall, UAC, SecureBoot.

use crate::registry::{
    get_defender_status, get_firewall_status, get_uac_status, is_credential_guard_enabled,
    is_secure_boot_enabled, DefenderStatus, FirewallStatus, UacStatus,
};
use pieuvre_common::Result;
use serde::{Deserialize, Serialize};

/// Full system security audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAudit {
    /// Windows Defender status
    pub defender: DefenderStatus,
    /// Windows Firewall status
    pub firewall: FirewallStatus,
    /// UAC status
    pub uac: UacStatus,
    /// Secure Boot enabled
    pub secure_boot_enabled: bool,
    /// Credential Guard enabled (VBS)
    pub credential_guard_enabled: bool,
    /// BitLocker enabled on system drive
    pub bitlocker_system_encrypted: bool,
    /// Global security score (0-100)
    pub security_score: u32,
    /// Recommendations
    pub recommendations: Vec<SecurityRecommendation>,
}

/// Security recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    pub severity: Severity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub remediation: String,
}

/// Severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Performs a full security audit
pub fn audit_security() -> Result<SecurityAudit> {
    let defender = get_defender_status()?;
    let firewall = get_firewall_status()?;
    let uac = get_uac_status()?;
    let secure_boot = is_secure_boot_enabled();
    let credential_guard = is_credential_guard_enabled();
    let bitlocker = check_bitlocker_status();

    let mut recommendations = Vec::new();
    let mut score = 100u32;

    // Analyze Defender
    if !defender.antispyware_enabled {
        score = score.saturating_sub(25);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "Defender".into(),
            title: "Windows Defender disabled".into(),
            description: "Antivirus/antispyware protection is disabled.".into(),
            remediation: "Enable Windows Defender in security settings.".into(),
        });
    }

    if !defender.realtime_protection {
        score = score.saturating_sub(20);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "Defender".into(),
            title: "Real-time protection disabled".into(),
            description: "Threats are not detected in real-time.".into(),
            remediation: "Enable real-time protection in Windows Security.".into(),
        });
    }

    if !defender.tamper_protection {
        score = score.saturating_sub(10);
        recommendations.push(SecurityRecommendation {
            severity: Severity::High,
            category: "Defender".into(),
            title: "Tamper Protection disabled".into(),
            description: "Malware can disable Defender.".into(),
            remediation: "Enable Tamper Protection in Windows Security.".into(),
        });
    }

    if !defender.exclusion_paths.is_empty() {
        let count = defender.exclusion_paths.len();
        if count > 5 {
            score = score.saturating_sub(5);
        }
        recommendations.push(SecurityRecommendation {
            severity: if count > 10 {
                Severity::Medium
            } else {
                Severity::Low
            },
            category: "Defender".into(),
            title: format!("{} path exclusions configured", count),
            description: "Verify that these exclusions are legitimate.".into(),
            remediation: "Audit exclusions in Windows Security > Virus protection.".into(),
        });
    }

    // Analyze Firewall
    if !firewall.public_enabled {
        score = score.saturating_sub(15);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "Firewall".into(),
            title: "Firewall disabled (Public profile)".into(),
            description: "The system is exposed on public networks.".into(),
            remediation: "Enable firewall for Public profile.".into(),
        });
    }

    if !firewall.private_enabled {
        score = score.saturating_sub(10);
        recommendations.push(SecurityRecommendation {
            severity: Severity::High,
            category: "Firewall".into(),
            title: "Firewall disabled (Private profile)".into(),
            description: "The system is exposed on private networks.".into(),
            remediation: "Enable firewall for Private profile.".into(),
        });
    }

    // Analyze UAC
    if !uac.enabled {
        score = score.saturating_sub(15);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "UAC".into(),
            title: "UAC disabled".into(),
            description: "Applications can run with privileges without confirmation.".into(),
            remediation: "Enable UAC in User Account Control settings.".into(),
        });
    }

    if !uac.secure_desktop {
        score = score.saturating_sub(5);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Medium,
            category: "UAC".into(),
            title: "Secure Desktop disabled".into(),
            description: "UAC prompts can be manipulated by malware.".into(),
            remediation: "Enable PromptOnSecureDesktop in local policies.".into(),
        });
    }

    // Secure Boot
    if !secure_boot {
        score = score.saturating_sub(10);
        recommendations.push(SecurityRecommendation {
            severity: Severity::High,
            category: "Boot".into(),
            title: "Secure Boot disabled".into(),
            description: "The system is vulnerable to bootkits.".into(),
            remediation: "Enable Secure Boot in BIOS/UEFI.".into(),
        });
    }

    // Credential Guard
    if !credential_guard {
        recommendations.push(SecurityRecommendation {
            severity: Severity::Info,
            category: "VBS".into(),
            title: "Credential Guard not enabled".into(),
            description: "Advanced credential protection is not active.".into(),
            remediation: "Enable Credential Guard via Group Policy (Enterprise).".into(),
        });
    }

    // BitLocker
    if !bitlocker {
        recommendations.push(SecurityRecommendation {
            severity: Severity::Medium,
            category: "Encryption".into(),
            title: "BitLocker not enabled".into(),
            description: "System drive is not encrypted.".into(),
            remediation: "Enable BitLocker in security settings.".into(),
        });
    }

    Ok(SecurityAudit {
        defender,
        firewall,
        uac,
        secure_boot_enabled: secure_boot,
        credential_guard_enabled: credential_guard,
        bitlocker_system_encrypted: bitlocker,
        security_score: score.min(100),
        recommendations,
    })
}

/// Checks BitLocker status
fn check_bitlocker_status() -> bool {
    // Checking via WMI would be more accurate
    // For now, indirect registry check
    crate::registry::key_exists(r"SYSTEM\CurrentControlSet\Control\BitlockerStatus")
        && crate::registry::read_dword_value(
            r"SYSTEM\CurrentControlSet\Control\BitlockerStatus",
            "BootStatus",
        )
        .unwrap_or(0)
            != 0
}

/// Returns a text summary of the score
pub fn score_to_grade(score: u32) -> &'static str {
    match score {
        90..=100 => "A - Excellent",
        80..=89 => "B - Good",
        70..=79 => "C - Acceptable",
        60..=69 => "D - Needs Improvement",
        _ => "F - Critical",
    }
}

/// Counts recommendations by severity
pub fn count_by_severity(
    recommendations: &[SecurityRecommendation],
) -> (usize, usize, usize, usize) {
    let critical = recommendations
        .iter()
        .filter(|r| r.severity == Severity::Critical)
        .count();
    let high = recommendations
        .iter()
        .filter(|r| r.severity == Severity::High)
        .count();
    let medium = recommendations
        .iter()
        .filter(|r| r.severity == Severity::Medium)
        .count();
    let low = recommendations
        .iter()
        .filter(|r| matches!(r.severity, Severity::Low | Severity::Info))
        .count();
    (critical, high, medium, low)
}
