use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "hvci",
            label: "Disable Memory Integrity (HVCI)",
            description: "Disables Hypervisor-Protected Code Integrity to gain significant gaming performance.",
            default: false,
            risk: RiskLevel::High,
        },
        OptItem {
            id: "vbs",
            label: "Disable Virtualization-Based Security",
            description: "Disables VBS to reduce CPU overhead, especially on older processors.",
            default: false,
            risk: RiskLevel::High,
        },
        OptItem {
            id: "spectre",
            label: "Disable Spectre/Meltdown Mitigations",
            description: "Disables CPU security mitigations to recover lost performance. HIGH RISK.",
            default: false,
            risk: RiskLevel::High,
        },
        OptItem {
            id: "uac_level",
            label: "Disable UAC (User Account Control)",
            description: "Sets UAC to 'Never Notify'. Not recommended for security.",
            default: false,
            risk: RiskLevel::High,
        },
    ]
}
