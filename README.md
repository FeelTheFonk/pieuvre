# Pieuvre

Windows system control and optimization tool. Full registry/service/network/power management with snapshot-based rollback.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)]()
[![Platform](https://img.shields.io/badge/platform-Windows%2011-lightgrey)]()

---

## Overview

Pieuvre is a state-of-the-art Windows optimization toolkit built in Rust, now in its **Climax (0.3.0)** phase.

Unlike batch scripts or registry tweaks, Pieuvre:

- **100% Native API Integration** - Zero reliance on external CLI tools (`netsh`, `powercfg`, etc.)
- **Audits before modifying** - Full system state capture with deep ETW analysis
- **Creates automatic snapshots** - Every change is reversible via `zstd` compressed backups
- **Sentinel Engine** - Real-time monitoring and self-healing of critical system settings
- **Hardware-Aware Intelligence** - Advanced detection for optimal DPC/ISR steering

**Target Users**: Power users, gaming enthusiasts, privacy-focused users, system administrators.

---

## Features

- **Audit Engine** - Hardware (DXGI GPU, SSD), services, telemetry (40+ keys), security scoring
- **ETW Monitoring** - Real-time Kernel DPC/ISR latency capture with `DriverResolver` (SOTA 2026)
- **Security Audit** - Defender status, Firewall profiles, UAC, SecureBoot, HVCI/VBS detection
- **Sync Engine** - 30+ optimization modules including **Interrupt Affinity Steering**
- **Persistence Engine** - Snapshot creation & rollback (zstd compression + SHA256)
- **Sentinel Engine** - Event-driven background monitoring & auto-restoration (Self-Healing)
- **Interactive Mode** - 9 sections, 80+ granular options (Default launch mode)

→ See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for details

---

## Installation

### Prerequisites

- Windows 10/11 (64-bit)
- Rust 1.75+ (for building)
- Administrator privileges

### Build

```powershell
git clone https://github.com/username/pieuvre.git
cd pieuvre
cargo build --release
```

Binary: `target/release/pieuvre.exe`

---

## Quick Start

```powershell
# 1. Audit current state
pieuvre audit --full

# 2. Interactive mode (recommended)
pieuvre interactive --profile gaming

# 3. Or apply profile directly
pieuvre sync --profile gaming --dry-run   # Preview
pieuvre sync --profile gaming             # Apply

# 4. Rollback if needed
pieuvre rollback --last
```

---

## Commands

| Command | Description |
|---------|-------------|
| `audit` | Collect system state |
| `analyze` | Generate recommendations |
| `sync` | Apply optimizations |
| `interactive` | Granular selection |
| `rollback` | Restore previous state |
| `status` | Display current state |
| `verify` | Check applied changes |

→ See [CLI Documentation](crates/pieuvre-cli/README.md) for full options

---

## Profiles

| Profile | Focus |
|---------|-------|
| **Gaming** | Minimum latency, maximum performance |
| **Privacy** | Minimize telemetry and data collection |
| **Workstation** | Balance performance with stability |

→ See [Configuration](config/README.md) for details

---

## Safety

- **Laptop detection** - Adjusts recommendations for battery devices
- **Automatic snapshots** - Every modification is reversible
- **Non-destructive analysis** - `audit` and `analyze` are read-only

---

## Documentation

| Document | Description |
|----------|-------------|
| [CLI Reference](crates/pieuvre-cli/README.md) | Full command documentation |
| [Configuration](config/README.md) | Profiles and settings |
| [Architecture](docs/ARCHITECTURE.md) | Project structure |
| [Technical Details](docs/TECHNICAL.md) | Implementation specifics |
| [Contributing](CONTRIBUTING.md) | Development guidelines |

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

### References

- [ChrisTitusTech/winutil](https://github.com/ChrisTitusTech/winutil)
- [Farag2/Sophia-Script](https://github.com/farag2/Sophia-Script-for-Windows)
- [Raphire/Win11Debloat](https://github.com/Raphire/Win11Debloat)
- [privacy.sexy](https://github.com/undergroundwires/privacy.sexy)

### Rust Ecosystem

- [windows-rs](https://github.com/microsoft/windows-rs)
- [clap](https://github.com/clap-rs/clap)
- [dialoguer](https://github.com/mitsuhiko/dialoguer)
