# pieuvre-sync

System modification engine with **22 specialized modules** (SOTA 2026).

---

## Modules

| Module | File | Description |
|--------|------|-------------|
| Services | `services.rs` | Service state management (disable/manual/auto) |
| Timer | `timer.rs` | Timer resolution control (0.5ms minimum) |
| Power | `power.rs` | Power plan configuration |
| Firewall | `firewall.rs` | Windows Firewall rule injection |
| MSI | `msi.rs` | MSI Mode detection and enabling |
| Registry | `registry.rs` | Atomic registry writes |
| AppX | `appx.rs` | AppX package removal (47 packages, 10 categories) |
| Hosts | `hosts.rs` | Hosts file blocking (50+ domains) |
| Scheduled Tasks | `scheduled_tasks.rs` | Telemetry task disabling (30 tasks) |
| OneDrive | `onedrive.rs` | Complete OneDrive removal |
| Context Menu | `context_menu.rs` | Classic menu + clutter removal |
| Widgets | `widgets.rs` | Win11 widget board disabling |
| Windows Update | `windows_update.rs` | Update pause + driver control |
| Edge | `edge.rs` | Edge browser management |
| Explorer | `explorer.rs` | Explorer UI tweaks |
| Game Mode | `game_mode.rs` | Game Bar/DVR/HAGS/Pre-rendered frames |
| Network | `network.rs` | Nagle/Interrupt Moderation/LSO/RSS |
| Security | `security.rs` | VBS/HVCI/Memory Integrity |
| DPC | `dpc.rs` | DPC latency optimizations |
| CPU | `cpu.rs` | Core Parking/Memory Compression |
| BIOS | `bios.rs` | TPM/SecureBoot status (WMI) |
| Defender | `defender.rs` | Real-time protection & exclusions |

---

## SOTA 2026 Features

### Security Module (5-10% FPS Gain)

```rust
use pieuvre_sync::security;

// Disable Memory Integrity (HVCI) - major performance gain
security::disable_memory_integrity()?;

// Disable VBS completely
security::disable_vbs()?;

// Disable Spectre/Meltdown mitigations (ADVANCED - security risk)
security::disable_spectre_meltdown()?;
```

### DPC Latency Module (Micro-stutter fix)

```rust
use pieuvre_sync::dpc;

// Keep kernel in RAM
dpc::disable_paging_executive()?;

// Disable dynamic tick for consistent timer
dpc::disable_dynamic_tick()?;

// Enhanced TSC sync
dpc::set_tsc_sync_enhanced()?;

// Spread interrupts across cores
dpc::set_interrupt_affinity_spread()?;

// Apply all at once
dpc::apply_all_dpc_optimizations()?;
```

### CPU Module (Core Parking / Memory)

```rust
use pieuvre_sync::cpu;

// Keep all CPU cores active
cpu::disable_core_parking()?;

// Disable memory compression (16GB+ systems)
cpu::disable_memory_compression()?;

// Set static page file (reduces fragmentation)
cpu::set_static_page_file(8192)?; // 8GB

// Apply all CPU optimizations
cpu::apply_gaming_cpu_optimizations()?;
```

### Network Module (Extended)

```rust
use pieuvre_sync::network;

// All network latency optimizations
network::apply_all_network_optimizations()?;

// Or individually:
network::disable_nagle_algorithm()?;
network::disable_interrupt_moderation()?;
network::disable_lso()?;
network::disable_eee()?;
network::enable_rss()?;
network::disable_rsc()?;
```

### Game Mode Module (Extended)

```rust
use pieuvre_sync::game_mode;

// All GPU optimizations
game_mode::apply_all_gpu_optimizations()?;

// Or individually:
game_mode::set_prerendered_frames(1)?; // Minimum input lag
game_mode::disable_fullscreen_optimizations()?;
game_mode::disable_game_bar()?;
game_mode::enable_game_mode()?;
```

---

## Services API

```rust
use pieuvre_sync::services;

// Disable a service
services::disable_service("DiagTrack")?;

// Get current state
let state = services::get_service_start_type("DiagTrack")?;
```

## Timer Resolution API

```rust
use pieuvre_sync::timer;

// Set to 0.5ms
timer::set_timer_resolution(5000)?;

// Get current resolution
let info = timer::get_timer_resolution()?;

// Reset to default
timer::reset_timer_resolution()?;
```

## Power Plans API

```rust
use pieuvre_sync::power;

// Activate Ultimate Performance
power::set_power_plan(power::PowerPlan::UltimatePerformance)?;

// Full gaming config
power::apply_gaming_power_config()?;
```

---

## Safety

All modifications:
1. Capture original state first
2. Record in ChangeRecord struct
3. Rollback available via pieuvre-persist

### Security Warnings

| Function | Risk | Impact |
|----------|------|--------|
| `disable_memory_integrity()` | Medium | Reduced kernel protection |
| `disable_vbs()` | High | Credential Guard disabled |
| `disable_spectre_meltdown()` | Critical | CPU vulnerability exposure |

---

## Build

```bash
cargo build -p pieuvre-sync --release
```
