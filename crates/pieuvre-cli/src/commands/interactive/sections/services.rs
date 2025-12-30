use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "svc_telemetry",
            label: "Disable Telemetry Services",
            description: "Disables DiagTrack, dmwappushservice, and WerSvc.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "svc_sysmain",
            label: "Disable SysMain (Superfetch)",
            description: "Disables SysMain to reduce disk I/O and memory usage on SSDs.",
            default: true,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "svc_search",
            label: "Disable Windows Search",
            description:
                "Disables indexing service. Search will be slower but uses fewer resources.",
            default: false,
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "svc_update",
            label: "Optimize Update Services",
            description: "Sets Windows Update services to manual to prevent background activity.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "svc_print",
            label: "Disable Print Spooler",
            description: "Disables printing services if you don't use a printer.",
            default: false,
            risk: RiskLevel::Safe,
        },
    ]
}
