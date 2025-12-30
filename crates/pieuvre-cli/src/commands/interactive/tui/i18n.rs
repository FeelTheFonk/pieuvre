//! i18n module for Pieuvre TUI
//! Centralizes all UI strings for easy localization.

pub const TITLE: &str = " pieuvre ";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Header
pub const CPU: &str = " CPU ";
pub const MEM: &str = " MEM ";
pub const UPTIME: &str = " UP ";
pub const ADMIN: &str = " ADMIN ";
pub const USER: &str = " USER ";

// Sidebar
pub const CATEGORIES: &str = " Catégories ";

// Main View
pub const OPTIMIZATIONS: &str = " Optimisations ";
pub const DETAILS: &str = " Détails ";
pub const ID: &str = " ID : ";
pub const RISK: &str = " Risque : ";
pub const DESCRIPTION: &str = " Description :";

// Catégories (Fusionnées)
pub const CAT_ANALYSIS: &str = " Analyse & Diagnostic ";
pub const CAT_CONFIDENTIALITY: &str = " Confidentialité & Télémétrie ";
pub const CAT_PERFORMANCE: &str = " Performance ";
pub const CAT_SECURITY: &str = " Sécurité ";
pub const CAT_SYSTEM: &str = " Système & Nettoyage ";
pub const CAT_SERVICES: &str = " Services ";
pub const CAT_NETWORK: &str = " Réseau ";
pub const CAT_SYNC: &str = " Synchronisation ";

// Footer
pub const KEY_TABS: &str = " ←→ ";
pub const KEY_NEXT: &str = "Onglets | ";
pub const KEY_NAV: &str = " ↑↓ ";
pub const KEY_NAVIGATE: &str = "Naviguer | ";
pub const KEY_SPACE: &str = " ESPACE ";
pub const KEY_TOGGLE: &str = "Sélection | ";
pub const KEY_ENTER: &str = " ENTRÉE ";
pub const KEY_APPLY: &str = "Appliquer | ";
pub const KEY_Q: &str = " Q ";
pub const KEY_QUIT: &str = "Quitter";

// Confirmation
pub const CONFIRM_APPLY_TITLE: &str = " Confirmer les changements ";
pub const CONFIRM_APPLY_MSG: &str = "Voulez-vous appliquer les optimisations sélectionnées ?";
pub const CONFIRM_SCAN_MSG: &str = "Voulez-vous exécuter les scans sélectionnés ?";
pub const CONFIRM_KEYS: &str = " [ENTRÉE] Confirmer | [ESC] Annuler ";

// Logs
pub const LOGS_TITLE: &str = " Journal d'exécution ";
pub const LOG_RUNNING: &str = " ⚙ ";
pub const LOG_SUCCESS: &str = " ✔ ";
pub const LOG_ERROR: &str = " ✖ ";
