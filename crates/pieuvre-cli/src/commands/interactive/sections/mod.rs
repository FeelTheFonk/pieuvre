pub mod audit;
pub mod bloatware;
pub mod maintenance;
pub mod network;
pub mod oo_privacy;
pub mod performance;
pub mod privacy;
pub mod scan;
pub mod security;
pub mod services;
pub mod sync;
pub mod system;
pub mod telemetry;

use crate::commands::interactive::types::OptItem;

pub fn get_all_sections() -> Vec<(&'static str, Vec<OptItem>)> {
    vec![
        ("Scan", scan::get_options()),
        ("Audit", audit::get_options()),
        ("Bloatware", bloatware::get_options()),
        ("Telemetry", telemetry::get_options()),
        ("Privacy", privacy::get_options()),
        ("O&O Privacy", oo_privacy::get_options()),
        ("Services", services::get_options()),
        ("Performance", performance::get_options()),
        ("Network", network::get_options()),
        ("Security", security::get_options()),
        ("System", system::get_options()),
        ("Maintenance", maintenance::get_options()),
        ("Sync", sync::get_options()),
    ]
}

// --- FIN DES SECTIONS SOTA v0.7.0 ---
