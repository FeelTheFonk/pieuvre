//! Shared data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Full audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub system: SystemInfo,
    pub hardware: HardwareInfo,
    pub services: Vec<ServiceInfo>,
    pub telemetry: TelemetryStatus,
    pub latency: Option<LatencyReport>,
    pub appx: Vec<AppxInfo>,
}

/// System information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_version: String,
    pub build_number: u32,
    pub edition: String,
    pub hostname: String,
}

/// Hardware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub storage: Vec<StorageInfo>,
    pub gpu: Vec<GpuInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub vendor: String,
    pub model_name: String,
    pub logical_cores: u32,
    pub physical_cores: u32,
    pub is_hybrid: bool,
    pub p_cores: Vec<u32>,
    pub e_cores: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_bytes: u64,
    pub available_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub device_id: String,
    pub model: String,
    pub size_bytes: u64,
    pub is_ssd: bool,
    pub is_nvme: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub vram_bytes: u64,
}

/// Windows service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: ServiceStatus,
    pub start_type: ServiceStartType,
    pub category: ServiceCategory,
    /// Service process PID if running
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Paused,
    StartPending,
    StopPending,
    ContinuePending,
    PausePending,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStartType {
    Boot,
    System,
    Automatic,
    Manual,
    Disabled,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceCategory {
    Telemetry,
    Performance,
    Security,
    System,
    Network,
    Gaming,
    Media,
    Peripheral,
    User,
    Unknown,
}

/// Full telemetry status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryStatus {
    pub diagtrack_enabled: bool,
    pub data_collection_level: u32,
    pub advertising_id_enabled: bool,
    pub location_enabled: bool,
    pub activity_history_enabled: bool,
    pub cortana_enabled: bool,
    pub web_search_enabled: bool,
    pub error_reporting_enabled: bool,
}

/// DPC/ISR latency report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyReport {
    pub duration_seconds: u64,
    pub dpc_max_us: u64,
    pub dpc_avg_us: f64,
    pub isr_max_us: u64,
    pub isr_avg_us: f64,
    pub top_offenders: Vec<LatencyOffender>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyOffender {
    pub driver_name: String,
    pub max_us: u64,
    pub count: u64,
}

/// Appx package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppxInfo {
    pub name: String,
    pub full_name: String,
    pub publisher: String,
    pub version: String,
    pub is_provisioned: bool,
    pub category: AppxCategory,
    pub removal_risk: RemovalRisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppxCategory {
    System,
    Microsoft,
    Gaming,
    Productivity,
    Media,
    Utility,
    ThirdParty,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemovalRisk {
    Safe,
    Caution,
    Critical,
}

/// Rollback snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub changes: Vec<ChangeRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeRecord {
    Registry {
        key: String,
        value_name: String,
        value_type: String,
        original_data: Vec<u8>,
    },
    Service {
        name: String,
        original_start_type: u32,
    },
    FirewallRule {
        name: String,
    },
    AppX {
        package_full_name: String,
    },
}
