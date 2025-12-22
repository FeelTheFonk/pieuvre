# pieuvre-audit

Read-only system inspection engine SOTA.

---

## Features

- **Hardware Detection**: CPU (hybrid P/E cores), RAM, GPU (DXGI VRAM), Storage (SSD/NVMe)
- **ETW Monitoring**: Real-time Kernel DPC/ISR latency capture via native Windows APIs
- **Service Enumeration**: Real start_type via QueryServiceConfigW, 10 categories, PID tracking
- **Telemetry Audit**: 30+ registry keys, data collection level, advertising, location
- **Security Audit**: Defender status, Firewall profiles, UAC, SecureBoot, Credential Guard
- **AppX Inventory**: Bloatware detection, removal risk assessment
- **Network Audit**: 50+ telemetry domains, IP ranges, DNS resolution status

---

## API

### Full Audit

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

### Security Audit

```rust
use pieuvre_audit::security_audit;

let audit = security_audit()?;

println!("Security Score: {}/100", audit.security_score);
println!("Defender Realtime: {}", audit.defender.realtime_protection);
println!("SecureBoot: {}", audit.secure_boot_enabled);

for rec in &audit.recommendations {
    println!("[{:?}] {}: {}", rec.severity, rec.category, rec.title);
}
```

### Hardware Detection

```rust
use pieuvre_audit::hardware;

let hw = hardware::probe_hardware()?;

// CPU
println!("Vendor: {}", hw.cpu.vendor);
println!("Cores: {} logical, {} physical", hw.cpu.logical_cores, hw.cpu.physical_cores);
println!("Hybrid: {} (P:{}, E:{})", hw.cpu.is_hybrid, hw.cpu.p_cores.len(), hw.cpu.e_cores.len());

// GPU via DXGI
for gpu in &hw.gpu {
    println!("{} ({}) - {} MB VRAM", gpu.name, gpu.vendor, gpu.vram_bytes / 1_000_000);
}

// Storage with SSD detection
for drive in &hw.storage {
    println!("{}: SSD={}, NVMe={}, {} GB", drive.device_id, drive.is_ssd, drive.is_nvme, drive.size_bytes / 1_000_000_000);
}
```

### Services with Real Start Type

```rust
use pieuvre_audit::services;

let services = services::inspect_services()?;

for svc in &services {
    println!("{}: {:?} (start={:?}, cat={:?})", 
        svc.name, svc.status, svc.start_type, svc.category);
    if let Some(pid) = svc.pid {
        println!("  PID: {}", pid);
    }
}

// Get active telemetry services
let telemetry = services::get_active_telemetry_services(&services);
println!("Active telemetry services: {}", telemetry.len());
```

### Registry & Defender Status

```rust
use pieuvre_audit::registry;

// Telemetry status
let telem = registry::get_telemetry_status()?;
println!("DiagTrack: {}", telem.diagtrack_enabled);
println!("Collection Level: {}", telem.data_collection_level);

// Defender audit
let defender = registry::get_defender_status()?;
println!("Realtime: {}", defender.realtime_protection);
println!("Tamper Protection: {}", defender.tamper_protection);
println!("Exclusions: {} paths", defender.exclusion_paths.len());

// UAC status
let uac = registry::get_uac_status()?;
println!("UAC Enabled: {}", uac.enabled);
println!("Secure Desktop: {}", uac.secure_desktop);
```

---

## Report Format

```json
{
  "timestamp": "2025-12-22T08:00:00Z",
  "system": {
    "os_version": "Windows 11 Pro",
    "build_number": 22631,
    "edition": "Professional"
  },
  "hardware": {
    "cpu": {
      "vendor": "Intel",
      "model_name": "Intel Core i9-13900K",
      "logical_cores": 32,
      "physical_cores": 24,
      "is_hybrid": true
    },
    "gpu": [
      { "name": "NVIDIA GeForce RTX 4090", "vendor": "NVIDIA", "vram_bytes": 25769803776 }
    ],
    "storage": [
      { "device_id": "C:", "is_ssd": true, "is_nvme": true, "size_bytes": 1000000000000 }
    ]
  },
  "services": [...],
  "telemetry": {
    "diagtrack_enabled": true,
    "data_collection_level": 1,
    "advertising_id_enabled": false
  },
  "appx": [...]
}
```

---

## Tests

```bash
cargo test -p pieuvre-audit
# 28 passed, 0 failed
```

---

## Safety

This crate is **read-only** and never modifies system state.
