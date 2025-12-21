# Pieuvre

Windows system control and optimization tool. Full registry/service/network/power management with snapshot-based rollback.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)]()
[![Platform](https://img.shields.io/badge/platform-Windows%2011-lightgrey)]()

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
- [Profiles](#profiles)
- [Configuration](#configuration)
- [Safety](#safety)
- [Technical Details](#technical-details)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

Pieuvre is a state-of-the-art Windows optimization toolkit built in Rust. It provides fine-grained control over system telemetry, services, power management, timer resolution, and registry settings.

Unlike batch scripts or registry tweaks, Pieuvre:

- **Audits before modifying** - Full system state capture
- **Creates automatic snapshots** - Every change is reversible
- **Validates sources** - Optimizations based on documented Windows internals
- **Detects hardware** - Laptop/desktop-aware recommendations

### Target Users

- Power users seeking granular system control
- Gaming enthusiasts optimizing for latency
- Privacy-focused users disabling telemetry
- System administrators managing Windows configurations

---

## Features

### Audit Engine
- Hardware detection (CPU hybrid architecture, RAM, vendor)
- Service enumeration with status analysis
- Telemetry level detection (DiagTrack, data collection)
- AppX package inventory
- Network configuration auditing

### Sync Engine
- Service state management (disable/manual/automatic)
- Timer resolution control (0.5ms minimum)
- Power plan configuration (Ultimate Performance)
- Windows Firewall rule injection
- MSI Mode detection for GPU/NVMe
- Registry atomic writes

### Persistence Engine
- Snapshot creation before any modification
- Rollback to any previous state
- Change record tracking with timestamps
- JSON export for external analysis

### Interactive Mode
- Terminal-based selection interface
- Category-based optimization grouping
- Pre-selection based on hardware type
- Real-time application feedback

---

## Architecture

```
pieuvre/
  crates/
    pieuvre-common/     Error types, shared structures
    pieuvre-audit/      System state collection
    pieuvre-sync/       Modification application
    pieuvre-persist/    Snapshot and rollback management
    pieuvre-cli/        Command-line interface
  config/
    default.toml        Default configuration
    telemetry-domains.txt
    profiles/           gaming.toml, privacy.toml, workstation.toml
```

### Data Flow

```
[User Command] --> [CLI Parser] --> [Audit Engine]
                                         |
                                         v
                                  [System Report]
                                         |
                                         v
                               [Intelligence Layer]
                                         |
                                         v
                                [Sync Engine] --> [Snapshot] --> [Apply Changes]
```

---

## Installation

### Prerequisites

- Windows 10/11 (64-bit)
- Rust 1.75+ (for building from source)
- Administrator privileges (for system modifications)

### Build from Source

```powershell
git clone https://github.com/username/pieuvre.git
cd pieuvre
cargo build --release
```

Binary available at `target/release/pieuvre.exe`.

### Pre-built Binary

Download from [Releases](https://github.com/username/pieuvre/releases).

---

## Quick Start

### 1. Audit Current State

```powershell
pieuvre audit --full
```

Generates a complete system report saved to `C:\ProgramData\Pieuvre\reports\`.

### 2. Analyze with Profile

```powershell
pieuvre analyze --profile gaming
```

Displays hardware-aware recommendations for the selected profile.

### 3. Interactive Mode (Recommended)

```powershell
pieuvre interactive --profile gaming
```

Step-by-step selection of optimizations with confirmation.

### 4. Apply Profile

```powershell
pieuvre sync --profile gaming --dry-run   # Preview
pieuvre sync --profile gaming             # Execute
```

### 5. Verify State

```powershell
pieuvre status
```

### 6. Rollback if Needed

```powershell
pieuvre rollback --last
```

---

## Commands

| Command | Description |
|---------|-------------|
| `audit` | Collect system state (services, hardware, telemetry) |
| `analyze` | Generate profile-based recommendations |
| `sync` | Apply profile optimizations |
| `status` | Display current optimization state |
| `interactive` | Granular selection interface |
| `rollback` | Restore previous system state |
| `verify` | Check integrity of applied changes |

### Command Options

```
pieuvre audit [OPTIONS]
  --full              Complete audit including AppX packages
  --output <PATH>     Custom output path

pieuvre analyze --profile <PROFILE>
  --profile           gaming | privacy | workstation

pieuvre sync --profile <PROFILE> [OPTIONS]
  --dry-run           Preview without applying
  --force             Skip confirmation

pieuvre interactive --profile <PROFILE>
  --profile           Base profile for pre-selection

pieuvre rollback [OPTIONS]
  --last              Restore most recent snapshot
  --id <UUID>         Restore specific snapshot
  --list              List available snapshots

pieuvre status
  (no options)

pieuvre verify
  --strict            Fail on any mismatch
```

---

## Profiles

### Gaming

Optimizes for minimum latency and maximum performance.

| Optimization | Value | Risk |
|-------------|-------|------|
| Timer Resolution | 0.5ms | Low (power) |
| Power Plan | Ultimate Performance | Low |
| DiagTrack | Disabled | None |
| SysMain (Superfetch) | Disabled | None (SSD) |
| Win32PrioritySeparation | 0x26 | None |

### Privacy

Minimizes telemetry and data collection.

| Optimization | Value | Risk |
|-------------|-------|------|
| DiagTrack | Disabled | None |
| dmwappushservice | Disabled | None |
| Data Collection Level | 0 (Security) | None |
| Advertising ID | Disabled | None |
| Firewall Rules | Block 42 domains | Low |

### Workstation

Balances performance with stability for professional use.

| Optimization | Value | Risk |
|-------------|-------|------|
| Power Plan | High Performance | None |
| WSearch | Manual | Low |
| Background Apps | Limited | Low |

---

## Configuration

### Default Configuration

Located at `config/default.toml`:

```toml
[general]
log_level = "info"
snapshot_dir = "C:\\ProgramData\\Pieuvre\\snapshots"
default_profile = "gaming"

[audit]
services = true
hardware = true
appx = true
network = true
```

### Telemetry Domains

Firewall blocking references `config/telemetry-domains.txt`:

```
vortex.data.microsoft.com
watson.telemetry.microsoft.com
settings-win.data.microsoft.com
...
```

---

## Safety

### Laptop Detection

Pieuvre automatically detects battery presence and adjusts recommendations:

- Timer 0.5ms: Disabled by default (battery impact)
- Ultimate Performance: Replaced with High Performance
- Visual warning in interactive mode

### Automatic Snapshots

Every modification creates a snapshot:

```powershell
Snapshot: 7be4b13b-051a-4cb2-afb2-257c7a3aff2c
Location: C:\ProgramData\Pieuvre\snapshots\
```

### Rollback

```powershell
pieuvre rollback --last      # Restore previous state
pieuvre rollback --list      # View all snapshots
```

### Non-Destructive Analysis

`audit` and `analyze` commands are read-only. Only `sync` and `interactive` modify system state.

---

## Technical Details

### Timer Resolution

Uses `NtSetTimerResolution` from ntdll.dll:

```rust
NtSetTimerResolution(5000, TRUE, &actual);  // 0.5ms
```

References:
- [Microsoft Timer Resolution](https://docs.microsoft.com/en-us/windows/win32/api/timeapi/)
- [Bruce Dawson Analysis](https://randomascii.wordpress.com/2020/10/04/windows-timer-resolution-the-great-rule-change/)

### Power Plans

GUID-based activation via `powercfg`:

| Plan | GUID |
|------|------|
| Balanced | `381b4222-f694-41f0-9685-ff5bb260df2e` |
| High Performance | `8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c` |
| Ultimate Performance | `e9a42b02-d5df-448d-aa00-03f14749eb61` |

### Service Control

Uses Service Control Manager API:

```rust
OpenSCManagerW(...)
OpenServiceW(...)
ChangeServiceConfigW(SERVICE_DISABLED)
```

### Firewall

Rules injected via `netsh`:

```powershell
netsh advfirewall firewall add rule name="Pieuvre-Block-Telemetry" ...
```

### MSI Mode

Detection via registry enumeration:

```
HKLM\SYSTEM\CurrentControlSet\Enum\PCI\*\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties
```

---

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/improvement`)
3. Commit changes (`git commit -am 'Add improvement'`)
4. Push to branch (`git push origin feature/improvement`)
5. Open a Pull Request

### Development

```powershell
cargo build              # Debug build
cargo test               # Run tests
cargo clippy             # Lint check
cargo fmt                # Format code
```

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

- [Windows Internals](https://docs.microsoft.com/en-us/sysinternals/) for system documentation
- [Hagezi DNS Blocklist](https://github.com/hagezi/dns-blocklists) for telemetry domains
- Rust community for excellent Windows crate ecosystem
