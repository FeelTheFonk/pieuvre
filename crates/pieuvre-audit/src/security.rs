//! Security Auditor
//!
//! Audit complet de la posture sécurité: Defender, Firewall, UAC, SecureBoot.

use crate::registry::{
    DefenderStatus, FirewallStatus, UacStatus,
    get_defender_status, get_firewall_status, get_uac_status,
    is_secure_boot_enabled, is_credential_guard_enabled,
};
use pieuvre_common::Result;
use serde::{Deserialize, Serialize};

/// Audit de sécurité complet du système
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAudit {
    /// Status Windows Defender
    pub defender: DefenderStatus,
    /// Status Firewall Windows
    pub firewall: FirewallStatus,
    /// Status UAC
    pub uac: UacStatus,
    /// Secure Boot activé
    pub secure_boot_enabled: bool,
    /// Credential Guard activé (VBS)
    pub credential_guard_enabled: bool,
    /// BitLocker activé sur le disque système
    pub bitlocker_system_encrypted: bool,
    /// Score de sécurité global (0-100)
    pub security_score: u32,
    /// Recommandations
    pub recommendations: Vec<SecurityRecommendation>,
}

/// Recommandation de sécurité
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRecommendation {
    pub severity: Severity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub remediation: String,
}

/// Niveau de sévérité
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Effectue un audit de sécurité complet
pub fn audit_security() -> Result<SecurityAudit> {
    let defender = get_defender_status()?;
    let firewall = get_firewall_status()?;
    let uac = get_uac_status()?;
    let secure_boot = is_secure_boot_enabled();
    let credential_guard = is_credential_guard_enabled();
    let bitlocker = check_bitlocker_status();
    
    let mut recommendations = Vec::new();
    let mut score = 100u32;
    
    // Analyser Defender
    if !defender.antispyware_enabled {
        score = score.saturating_sub(25);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "Defender".into(),
            title: "Windows Defender désactivé".into(),
            description: "La protection antivirus/antispyware est désactivée.".into(),
            remediation: "Activer Windows Defender dans les paramètres de sécurité.".into(),
        });
    }
    
    if !defender.realtime_protection {
        score = score.saturating_sub(20);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "Defender".into(),
            title: "Protection en temps réel désactivée".into(),
            description: "Les menaces ne sont pas détectées en temps réel.".into(),
            remediation: "Activer la protection en temps réel dans Windows Security.".into(),
        });
    }
    
    if !defender.tamper_protection {
        score = score.saturating_sub(10);
        recommendations.push(SecurityRecommendation {
            severity: Severity::High,
            category: "Defender".into(),
            title: "Tamper Protection désactivée".into(),
            description: "Les malwares peuvent désactiver Defender.".into(),
            remediation: "Activer Tamper Protection dans Windows Security.".into(),
        });
    }
    
    if !defender.exclusion_paths.is_empty() {
        let count = defender.exclusion_paths.len();
        if count > 5 {
            score = score.saturating_sub(5);
        }
        recommendations.push(SecurityRecommendation {
            severity: if count > 10 { Severity::Medium } else { Severity::Low },
            category: "Defender".into(),
            title: format!("{} exclusions de chemin configurées", count),
            description: "Vérifier que ces exclusions sont légitimes.".into(),
            remediation: "Auditer les exclusions dans Windows Security > Virus protection.".into(),
        });
    }
    
    // Analyser Firewall
    if !firewall.public_enabled {
        score = score.saturating_sub(15);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "Firewall".into(),
            title: "Firewall désactivé (profil Public)".into(),
            description: "Le système est exposé sur les réseaux publics.".into(),
            remediation: "Activer le firewall pour le profil Public.".into(),
        });
    }
    
    if !firewall.private_enabled {
        score = score.saturating_sub(10);
        recommendations.push(SecurityRecommendation {
            severity: Severity::High,
            category: "Firewall".into(),
            title: "Firewall désactivé (profil Privé)".into(),
            description: "Le système est exposé sur les réseaux privés.".into(),
            remediation: "Activer le firewall pour le profil Privé.".into(),
        });
    }
    
    // Analyser UAC
    if !uac.enabled {
        score = score.saturating_sub(15);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Critical,
            category: "UAC".into(),
            title: "UAC désactivé".into(),
            description: "Les applications peuvent s'exécuter avec privilèges sans confirmation.".into(),
            remediation: "Activer UAC dans les paramètres de contrôle de compte.".into(),
        });
    }
    
    if !uac.secure_desktop {
        score = score.saturating_sub(5);
        recommendations.push(SecurityRecommendation {
            severity: Severity::Medium,
            category: "UAC".into(),
            title: "Secure Desktop désactivé".into(),
            description: "Les prompts UAC peuvent être manipulés par des malwares.".into(),
            remediation: "Activer PromptOnSecureDesktop dans les stratégies locales.".into(),
        });
    }
    
    // Secure Boot
    if !secure_boot {
        score = score.saturating_sub(10);
        recommendations.push(SecurityRecommendation {
            severity: Severity::High,
            category: "Boot".into(),
            title: "Secure Boot désactivé".into(),
            description: "Le système est vulnérable aux bootkits.".into(),
            remediation: "Activer Secure Boot dans le BIOS/UEFI.".into(),
        });
    }
    
    // Credential Guard
    if !credential_guard {
        recommendations.push(SecurityRecommendation {
            severity: Severity::Info,
            category: "VBS".into(),
            title: "Credential Guard non activé".into(),
            description: "Protection avancée des credentials non active.".into(),
            remediation: "Activer Credential Guard via Group Policy (Enterprise).".into(),
        });
    }
    
    // BitLocker
    if !bitlocker {
        recommendations.push(SecurityRecommendation {
            severity: Severity::Medium,
            category: "Encryption".into(),
            title: "BitLocker non activé".into(),
            description: "Le disque système n'est pas chiffré.".into(),
            remediation: "Activer BitLocker dans les paramètres de sécurité.".into(),
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

/// Vérifie le status BitLocker
fn check_bitlocker_status() -> bool {
    // Vérifier via WMI serait plus précis
    // Pour l'instant, check registre indirect
    crate::registry::key_exists(
        r"SYSTEM\CurrentControlSet\Control\BitlockerStatus"
    ) && crate::registry::read_dword_value(
        r"SYSTEM\CurrentControlSet\Control\BitlockerStatus",
        "BootStatus"
    ).unwrap_or(0) != 0
}

/// Retourne un résumé texte du score
pub fn score_to_grade(score: u32) -> &'static str {
    match score {
        90..=100 => "A - Excellent",
        80..=89 => "B - Bon",
        70..=79 => "C - Acceptable",
        60..=69 => "D - Améliorable",
        _ => "F - Critique",
    }
}

/// Compte les recommandations par sévérité
pub fn count_by_severity(recommendations: &[SecurityRecommendation]) -> (usize, usize, usize, usize) {
    let critical = recommendations.iter().filter(|r| r.severity == Severity::Critical).count();
    let high = recommendations.iter().filter(|r| r.severity == Severity::High).count();
    let medium = recommendations.iter().filter(|r| r.severity == Severity::Medium).count();
    let low = recommendations.iter().filter(|r| matches!(r.severity, Severity::Low | Severity::Info)).count();
    (critical, high, medium, low)
}
