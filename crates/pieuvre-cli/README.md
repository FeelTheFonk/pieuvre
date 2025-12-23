<p align="center">
  <img src="logo.svg" width="256" alt="pieuvre logo">
</p>

<h1 align="center">pieuvre-cli</h1>

<p align="center">
  <strong>Command-line interface and TUI dashboard for the pieuvre optimization toolkit.</strong>
</p>

---

## Overview

`pieuvre-cli` is the primary entry point for users. It orchestrates the system audit, optimization execution, and snapshot management through a high-performance terminal interface.

## TUI Architecture

The interactive mode utilizes a modular, component-based architecture:

| Module | Description |
|:---|:---|
| `interactive/mod.rs` | Main orchestrator and event loop. |
| `interactive/sections.rs` | Definition of optimization sections with typed `OptItem` and `RiskLevel`. |
| `interactive/executor.rs` | Implementation of the `OptExecutor` trait for applying changes. |
| `interactive/ui.rs` | Rendering logic using `ratatui` with a premium design system. |

---

## Command Reference

### `audit`
Performs a comprehensive inspection of the system state.

```powershell
pieuvre audit [--full] [--output <PATH>]
```

### `interactive`
Launches the premium TUI dashboard. This is the **default mode** if no arguments are provided.

```powershell
pieuvre interactive
```

**Optimization Categories:**
- **Telemetry**: Core data collection and background service management.
- **Privacy**: AI blocking (Recall/CoPilot), location services, and activity history.
- **Performance**: Timer resolution, power plans, and CPU throttling.
- **Scheduler**: Win32 priority separation and MMCSS gaming profiles.
- **AppX Bloatware**: Granular removal of pre-installed Windows applications.
- **CPU & Memory**: Core parking, memory compression, and page file tuning.
- **DPC Latency**: Kernel paging, dynamic tick, and TSC synchronization.
- **Security**: HVCI/VBS control and Defender real-time protection.
- **Network**: Nagle algorithm, interrupt moderation, and LSO/RSS tuning.

### `status`
Displays the current optimization state and system alignment.

```powershell
pieuvre status [--live]
```

### `rollback`
Restores the system to a previous state using snapshots.

```powershell
pieuvre rollback [--list] [--last] [--id <UUID>]
```

### `verify`
Checks the integrity of applied changes and offers repair options.

```powershell
pieuvre verify [--repair]
```

---

## Exit Codes

| Code | Meaning |
|:---|:---|
| 0 | Success |
| 1 | General execution error |
| 2 | Configuration or parsing error |
| 3 | Permission denied (Administrator privileges required) |

---

## Safety Features

- **Automatic Snapshots**: A system snapshot is created before any modification.
- **Hardware Awareness**: Recommendations are automatically adjusted for laptops and battery-powered devices.
- **Non-Destructive Audit**: The `audit` command is strictly read-only.
