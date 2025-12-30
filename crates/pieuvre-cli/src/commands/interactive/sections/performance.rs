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
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "cpu_throttle",
            label: "Disable CPU Power Throttling",
            description: "Prevents Windows from throttling CPU performance for background tasks.",
            default: true,
            risk: RiskLevel::Low,
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
            risk: RiskLevel::Low,
        },
        OptItem {
            id: "nagle",
            label: "Disable Nagle's Algorithm",
            description: "Disables TCP NoDelay to reduce network latency in online games.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "interrupts",
            label: "Optimize Interrupt Steering",
            description: "Steers high-latency drivers to specific CPU cores to reduce DPC latency.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "memory",
            label: "Enable Large System Cache",
            description: "Increases the size of the system working set for better file I/O performance.",
            default: true,
            risk: RiskLevel::Performance,
        },
    ]
}
