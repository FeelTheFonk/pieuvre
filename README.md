<p align="center">
  <img src="crates/pieuvre-cli/logo.svg" width="256" alt="pieuvre logo">
</p>

<h1 align="center">pieuvre</h1>

Windows system control and optimization tool. Full registry/service/network/power management with snapshot-based rollback.
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)]()
[![Platform](https://img.shields.io/badge/platform-Windows%2011-lightgrey)]()

---

<p align="center">
  <img src="crates/pieuvre-cli/screen.png"alt="screen">
</p>

## Overview

pieuvre is a Windows optimization toolkit built in Rust.

Unlike batch scripts or registry tweaks, pieuvre:

- **Native API Integration** - Direct interaction with Windows APIs.
- **Audit-First Approach** - System state analysis before any modification.
- **Automatic Snapshots** - Change tracking and rollback via `zstd` compressed backups.
- **System Monitoring** - Background monitoring and restoration of critical settings.
- **Hardware-Aware** - Optimization tailoring based on detected hardware.
- **System Tuning** - CPU Quantum, Working Set management, and network stack adjustments.

**Target Users**: Power users, system administrators, and enthusiasts.

---

- **Interactive Dashboard** : Premium TUI interface with real-time system metrics and async execution.
- **Audit Engine** : Full hardware, services, telemetry, and security analysis.
- **Persistence Engine** : Snapshot creation and rollback (zstd + SHA256).

---

## Installation

### Prerequisites

- Windows 10/11 (64-bit)
- Rust 1.75+ (for building)
- Administrator privileges

### Build

```powershell
git clone https://github.com/FeelTheFonk/pieuvre.git
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
pieuvre interactive

# 3. Rollback if needed
pieuvre rollback --last
```

---

## Commands

| Command | Description |
|---------|-------------|
| `audit` | Collect system state |
| `interactive` | Granular selection (Interactive TUI Dashboard) |
| `rollback` | Restore previous state |
| `status` | Display current state |
| `verify` | Check applied changes |

---

## Safety

- **Laptop detection** - Adjusts recommendations for battery devices.
- **Automatic snapshots** - Every modification is reversible.
- **Non-destructive analysis** - `audit` is read-only.

---

## Documentation

| Document | Description |
|----------|-------------|
| [CLI Reference](crates/pieuvre-cli/README.md) | Full command documentation |
| [Architecture](docs/ARCHITECTURE.md) | Project structure |
| [Technical Details](docs/TECHNICAL.md) | Implementation specifics |
| [Contributing](CONTRIBUTING.md) | Development guidelines |

---

## License

MIT License. See [LICENSE](LICENSE) for details.
