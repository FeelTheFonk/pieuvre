# pieuvre-sync

System modification engine with 17 specialized modules.

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
| AppX | `appx.rs` | AppX package removal (42 packages, 10 categories) |
| Hosts | `hosts.rs` | Hosts file blocking (50+ domains) |
| Scheduled Tasks | `scheduled_tasks.rs` | Telemetry task disabling (25 tasks) |
| OneDrive | `onedrive.rs` | Complete OneDrive removal |
| Context Menu | `context_menu.rs` | Classic menu + clutter removal |
| Widgets | `widgets.rs` | Win11 widget board disabling |
| Windows Update | `windows_update.rs` | Update pause + driver control |
| Edge | `edge.rs` | Edge browser management |
| Explorer | `explorer.rs` | Explorer UI tweaks |
| Game Mode | `game_mode.rs` | Game Bar/DVR/HAGS control |
| Network | `network.rs` | Nagle algorithm disable |

---

## API

### Services

```rust
use pieuvre_sync::services;

// Disable a service
services::set_service_state("DiagTrack", ServiceState::Disabled)?;

// Get current state
let state = services::get_service_start_type("DiagTrack")?;
```

### Timer Resolution

```rust
use pieuvre_sync::timer;

// Set to 0.5ms
timer::set_timer_resolution(5000)?;

// Get current resolution
let (current, max, min) = timer::get_timer_resolution()?;
```

### Power Plans

```rust
use pieuvre_sync::power;

// Activate Ultimate Performance
power::set_power_plan(PowerPlan::UltimatePerformance)?;

// Get current plan
let current = power::get_current_power_plan()?;
```

### Firewall

```rust
use pieuvre_sync::firewall;

// Add telemetry blocking rules
firewall::add_telemetry_rules()?;

// Remove rules
firewall::remove_all_pieuvre_rules()?;
```

### Registry

```rust
use pieuvre_sync::registry;

// Write DWORD value
registry::write_dword_value(
    HKEY_LOCAL_MACHINE,
    r"SYSTEM\CurrentControlSet\Services\DiagTrack",
    "Start",
    4  // Disabled
)?;

// Read DWORD value
let value = registry::read_dword_value(hive, path, name)?;
```

---

## Safety

All modifications:
1. Capture original state first
2. Record in ChangeRecord struct
3. Rollback available via pieuvre-persist
