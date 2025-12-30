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
    use crate::commands::interactive::tui::i18n::*;
    vec![
        (CAT_ANALYSIS, [scan::get_options(), audit::get_options()].concat()),
        (CAT_CONFIDENTIALITY, [telemetry::get_options(), privacy::get_options(), oo_privacy::get_options()].concat()),
        (CAT_SYSTEM, [system::get_options(), maintenance::get_options(), bloatware::get_options()].concat()),
        (CAT_SERVICES, services::get_options()),
        (CAT_PERFORMANCE, performance::get_options()),
        (CAT_NETWORK, network::get_options()),
        (CAT_SECURITY, security::get_options()),
        (CAT_SYNC, sync::get_options()),
    ]
}
