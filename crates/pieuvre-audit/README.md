# pieuvre-audit

A read-only system inspection engine designed for deep analysis of Windows configurations and hardware topology.

---

## Features

- **Hardware Topology**: Detection of CPU P/E cores, RAM specifications, GPU VRAM (via DXGI), and storage types (NVMe/SSD).
- **ETW Monitoring**: Real-time Kernel DPC/ISR latency capture using native Event Tracing for Windows.
- **Service Audit**: Comprehensive enumeration of service states and start types using native SCM APIs.
- **Telemetry Analysis**: Inspection of over 30 critical registry keys related to data collection and privacy.
- **Security Inspection**: Real-time status of Windows Defender, Firewall profiles, SecureBoot, and Credential Guard.
- **AppX Inventory**: Detection of pre-installed bloatware and removal risk assessment.
- **Network Audit**: Verification of telemetry domain resolution and firewall rule status.

---

## API Usage

### Full System Audit

```rust
use pieuvre_audit::full_audit;

let report = full_audit()?;

println!("CPU: {}", report.hardware.cpu.model_name);
println!("GPU VRAM: {} MB", report.hardware.gpu[0].vram_bytes / 1_000_000);
println!("Is Laptop: {}", pieuvre_audit::is_laptop());

for service in &report.services {
    println!("{}: {:?} ({:?})", service.name, service.status, service.start_type);
}
```

### Hardware Probing

```rust
use pieuvre_audit::hardware;

let hw = hardware::probe_hardware()?;

// CPU Details
println!("Vendor: {}", hw.cpu.vendor);
println!("Cores: {} logical, {} physical", hw.cpu.logical_cores, hw.cpu.physical_cores);
println!("Hybrid Architecture: {}", hw.cpu.is_hybrid);

// GPU Details via DXGI
for gpu in &hw.gpu {
    println!("{} - {} MB VRAM", gpu.name, gpu.vram_bytes / 1_000_000);
}
```

### Security & Compliance

```rust
use pieuvre_audit::security;

let audit = security::security_audit()?;

println!("Security Score: {}/100", audit.security_score);
println!("Defender Active: {}", audit.defender.realtime_protection);
println!("SecureBoot Enabled: {}", audit.secure_boot_enabled);
```

---

## Report Schema (JSON)

The audit engine generates a structured report that can be exported to JSON for external analysis.

```json
{
  "timestamp": "2025-12-23T19:00:00Z",
  "system": {
    "os_version": "Windows 11 Pro",
    "build_number": 22631,
    "edition": "Professional"
  },
  "hardware": {
    "cpu": {
      "vendor": "Intel",
      "model_name": "Intel Core i9-13900K",
      "is_hybrid": true
    },
    "gpu": [
      { "name": "NVIDIA GeForce RTX 4090", "vram_bytes": 25769803776 }
    ]
  },
  "telemetry": {
    "diagtrack_enabled": true,
    "data_collection_level": 1
  }
}
```

---

## Safety & Integrity

- **Read-Only**: This crate is strictly non-destructive and never modifies system state.
- **Native Bindings**: Uses the `windows` crate for direct, high-performance API access.
- **Zero Dependencies (External)**: Minimizes the attack surface by relying on native Windows components.
