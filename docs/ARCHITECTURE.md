# Architecture

Pieuvre workspace architecture and data flow.

---

## Workspace Structure

```
pieuvre/
├── crates/
│   ├── pieuvre-common/     Shared types, error handling
│   ├── pieuvre-audit/      Read-only system inspection
│   ├── pieuvre-sync/       System modification functions
│   ├── pieuvre-persist/    Snapshot and rollback
│   └── pieuvre-cli/        Command-line interface
├── config/
│   ├── default.toml        Default configuration
│   ├── telemetry-domains.txt
│   └── profiles/           gaming.toml, privacy.toml, workstation.toml
└── docs/
    ├── ARCHITECTURE.md     This file
    └── TECHNICAL.md        Technical implementation details
```

---

## Crate Dependencies

```
pieuvre-cli
    ├── pieuvre-audit
    │   └── pieuvre-common
    ├── pieuvre-sync
    │   └── pieuvre-common
    └── pieuvre-persist
        └── pieuvre-common
```

---

## Data Flow

```
[User Command] ──> [CLI Parser] ──> [Audit Engine]
                                          │
                                          ▼
                                   [System Report]
                                          │
                                          ▼
                               [Intelligence Layer]
                                          │
                                          ▼
                                [Sync Engine] ──> [Snapshot] ──> [Apply Changes]
```

---

## Crate Responsibilities

### pieuvre-common

- `PieuvreError` enum for all error types
- Shared structures and types
- Configuration parsing

### pieuvre-audit

- Hardware detection (CPU, RAM, GPU)
- Service enumeration
- Telemetry level detection
- AppX package inventory
- Network configuration auditing
- **Read-only**: never modifies system state

### pieuvre-sync

17 modification modules:
- `services.rs` - Service state management
- `timer.rs` - Timer resolution control
- `power.rs` - Power plan configuration
- `firewall.rs` - Firewall rule injection
- `msi.rs` - MSI Mode detection/enabling
- `registry.rs` - Atomic registry writes
- `appx.rs` - AppX package removal
- `hosts.rs` - Hosts file blocking
- `scheduled_tasks.rs` - Task disabling
- `onedrive.rs` - OneDrive removal
- `context_menu.rs` - Context menu cleanup
- `widgets.rs` - Widget disabling
- `windows_update.rs` - Update control
- `edge.rs` - Edge management
- `explorer.rs` - Explorer tweaks
- `game_mode.rs` - Game mode configuration
- `network.rs` - Network optimizations

### pieuvre-persist

- Snapshot creation before modifications
- Rollback to any previous state
- Change record tracking
- JSON export

### pieuvre-cli

- Command parsing (clap)
- Interactive mode (dialoguer)
- User-facing output
- Profile loading
