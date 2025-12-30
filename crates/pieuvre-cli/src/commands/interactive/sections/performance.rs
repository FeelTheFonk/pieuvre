use crate::commands::interactive::types::{OptItem, RiskLevel};

pub fn get_options() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "timer",
            label: "Force 0.5ms Timer Resolution",
            description: "Forces the system global timer resolution to 0.5ms for reduced input lag and better frame pacing.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "power_ultimate",
            label: "Enable Ultimate Performance Plan",
            description: "Unlocks and activates the hidden 'Ultimate Performance' power scheme.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cpu_throttle",
            label: "Disable CPU Power Throttling",
            description: "Prevents Windows from throttling CPU performance for background tasks.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "msi",
            label: "Enable MSI Mode for All Devices",
            description: "Migrates all eligible hardware devices to Message Signaled Interrupts (MSI) for lower latency.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "hags",
            label: "Disable HAGS (Latency Fix)",
            description: "Disables Hardware-Accelerated GPU Scheduling which can cause stuttering in some scenarios.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "nagle",
            label: "Disable Nagle's Algorithm",
            description: "Disables TCP NoDelay to reduce network latency in online games.",
            default: true,
            risk: RiskLevel::Performance,
        },
    ]
}
