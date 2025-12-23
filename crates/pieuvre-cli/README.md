<p align="center">
  <img src="logo.svg" width="256" alt="pieuvre logo">
</p>

<h1 align="center">pieuvre-cli</h1>

Command-line interface for pieuvre Windows optimization tool.

[![Tests](https://img.shields.io/badge/tests-12%20passed-success)]()

## Architecture

Le mode interactif utilise une architecture modulaire:

| Module | Description |
|--------|-------------|
| `interactive/mod.rs` | Main orchestrator |
| `interactive/sections.rs` | 12 sections with typed `OptItem` and `RiskLevel` |
| `interactive/executor.rs` | `OptExecutor` trait + implementations |
| `interactive/ui.rs` | User interface (Premium TUI, severity-based colors) |

---
### `audit`

Collect current system state.

```powershell
pieuvre audit [--full] [--output <PATH>]
```

**Example:**
```powershell
pieuvre audit --full
```

---

### `interactive`

Granular selection interface with 110+ options.
**Note:** This mode is launched by default if no arguments are provided.

```powershell
pieuvre interactive
```

**Categories:**
- Telemetry (13 options)
- Privacy (11 options)
- Performance & GPU (18 options)
- Scheduler (5 options)
- AppX Bloatware (10 categories)
- CPU & Memory (4 options)
- DPC Latency (5 options)
- Security (3 options)
- Network (5 options)
- Cleanup
- AI & DNS (Recall, CoPilot, DoH)
- **System Scan** (Winapp2, LOLDrivers, LotL, YARA) - Key `S`

---

### `status`

Display current optimization state.

```powershell
pieuvre status [--live]
```

---

### `rollback`

Restore previous system state.

```powershell
pieuvre rollback [--list] [--last] [--id <UUID>]
```

---

### `verify`

Check integrity of applied changes.

```powershell
pieuvre verify [--repair]
```

---

### `scan`

YARA-based signature scan.

```powershell
pieuvre scan [--rules <PATH>]
```

---

### `kernel`

Kernel driver status.

```powershell
pieuvre kernel
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Permission denied (run as Administrator) |

---

## Safety

- **Laptop detection** - Adjusts recommendations for battery devices.
- **Automatic snapshots** - Every modification is reversible.
- **Non-destructive analysis** - `audit` is read-only.
