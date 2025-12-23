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

pieuvre is an advanced Windows optimization toolkit built in Rust, currently in its **0.5.0** phase.

Unlike batch scripts or registry tweaks, pieuvre:

- **100% Native API Integration** - Zero reliance on external CLI tools (`netsh`, `powercfg`, etc.)
- **Audits before modifying** - Full system state capture with deep ETW analysis
- **Creates automatic snapshots** - Every change is reversible via `zstd` compressed backups
- **System Monitoring** - Real-time monitoring and auto-restoration of critical system settings
- **Hardware Detection** - Advanced detection for optimal DPC/ISR steering
- **Advanced System Optimizations** - Advanced CPU Quantum, Working Set trimming, and TCP stack hardening

**Target Users**: Power users, gaming enthusiasts, privacy-focused users, system administrators.

---

## Features

- **Audit Engine** - Hardware (DXGI GPU, SSD), services, telemetry (40+ keys), security scoring
- **ETW Monitoring** - Real-time Kernel DPC/ISR latency capture with `DriverResolver`
- **Security Audit** - Defender status, Firewall profiles, UAC, SecureBoot, HVCI/VBS detection
- **Sync Engine** - 40+ optimization modules including **CPU Quantum**, **Memory Trimming**, and **TCP Hardening**
- **Persistence Engine** - Snapshot creation & rollback (zstd compression + SHA256)
- **Monitoring Engine** - Event-driven background monitoring & auto-restoration
- **Hardening Engine** - Persistence vector protection and IFEO hardening
- **Cleanup Engine** - Deep system cleaning (WinSxS via DISM, Temp, Edge, Windows Update)
- **Interactive Mode** - 11 sections, 100+ granular options (Interactive TUI Dashboard)

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
