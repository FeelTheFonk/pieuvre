pub mod oo_privacy;
pub mod performance;
pub mod privacy;
pub mod security;
pub mod system;
pub mod telemetry;

use crate::commands::interactive::types::OptItem;

pub fn get_all_sections() -> Vec<(&'static str, Vec<OptItem>)> {
    vec![
        ("Telemetry", telemetry::get_options()),
        ("Privacy", privacy::get_options()),
        ("O&O Privacy", oo_privacy::get_options()),
        ("Performance", performance::get_options()),
        ("Security", security::get_options()),
        ("System", system::get_options()),
    ]
}

// --- FIN DES SECTIONS SOTA v0.7.0 ---
