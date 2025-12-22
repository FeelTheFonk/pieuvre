# pieuvre-audit

Read-only system inspection engine.

---

## Features

- Hardware detection (CPU, RAM, GPU, vendor)
- Service enumeration with status analysis
- Telemetry level detection
- AppX package inventory
- Network configuration auditing

---

## API

### Full Audit

```rust
use pieuvre_audit;

let report = pieuvre_audit::full_audit()?;

// Access hardware info
println!("CPU: {}", report.hardware.cpu_name);
println!("RAM: {} GB", report.hardware.ram_gb);
println!("Is Laptop: {}", report.hardware.is_laptop);

// Access services
for service in &report.services {
    println!("{}: {:?}", service.name, service.status);
}
```

### Hardware Detection

```rust
use pieuvre_audit::hardware;

let info = hardware::detect()?;

// CPU info
println!("Cores: {}", info.cpu_cores);
println!("Hybrid: {}", info.is_hybrid_cpu);

// Laptop detection
println!("Has Battery: {}", info.has_battery);
```

### Service Enumeration

```rust
use pieuvre_audit::services;

let services = services::enumerate_all()?;

for svc in services {
    println!("{}: {} ({})", svc.name, svc.display_name, svc.status);
}
```

### Telemetry Detection

```rust
use pieuvre_audit::telemetry;

let level = telemetry::get_collection_level()?;
let diagtrack_running = telemetry::is_diagtrack_running()?;
```

---

## Report Format

```json
{
  "timestamp": "2025-12-22T07:30:00Z",
  "hardware": {
    "cpu_name": "Intel Core i9-13900K",
    "cpu_cores": 24,
    "ram_gb": 32,
    "is_laptop": false,
    "is_hybrid_cpu": true
  },
  "services": [...],
  "telemetry": {
    "diagtrack_running": true,
    "collection_level": 3
  },
  "appx_packages": [...]
}
```

---

## Safety

This crate is **read-only** and never modifies system state.
