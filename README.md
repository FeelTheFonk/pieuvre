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

## Overview

pieuvre is a Windows optimization toolkit built in Rust, currently in its **0.5.0** phase.

Unlike batch scripts or registry tweaks, pieuvre:

- **Native API Integration** - Direct interaction with Windows APIs (no reliance on `netsh`, `powercfg`, etc.)
- **Audit-First Approach** - System state analysis before any modification
- **Automatic Snapshots** - Change tracking and rollback via `zstd` compressed backups
- **System Monitoring** - Background monitoring and restoration of critical settings
- **Hardware-Aware** - Optimization tailoring based on detected hardware
- **System Tuning** - CPU Quantum, Working Set management, and network stack adjustments

**Target Users**: Power users, system administrators, and enthusiasts.

---

## Features

- **Audit Engine** - Hardware, services, telemetry, and security state analysis
- **Latency Monitoring** - Kernel DPC/ISR latency capture via ETW
- **Security Audit** - Defender, Firewall, UAC, and SecureBoot status detection
- **Optimization Engine** - Modules for CPU, Memory, and Network tuning
- **Persistence Engine** - Snapshot creation and rollback (zstd + SHA256)
- **Monitoring Engine** - Event-driven background monitoring
- **Hardening Engine** - System protection and IFEO management
- **Cleanup Engine** - System maintenance (WinSxS, Temp, Windows Update)
- **Interactive Mode** - Granular control via TUI Dashboard

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

- **Laptop detection** - Adjusts recommendations for battery devices
- **Automatic snapshots** - Every modification is reversible
- **Non-destructive analysis** - `audit` is read-only

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
