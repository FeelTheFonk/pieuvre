# Technical Architecture: pieuvre TUI (v0.7.0)

This document details the technical implementation of the pieuvre Terminal User Interface and its underlying optimization engines.

---

## 1. Sidebar Navigation Engine (v0.6.0)

The TUI utilizes a sidebar-based navigation model for efficient management of optimization categories.

### Navigation & Controls
- **Structure**: A horizontal layout divided into the `Sidebar` (Categories) and the `MainView` (Options & Details).
- **Controls**:
    - `Tab` / `BackTab`: Switch focus between the Sidebar and the MainView.
    - `Up` / `Down`: Navigate through categories or options within the active view.
    - `Space`: Toggle the selection state of an optimization item ([X] / [ ]).
    - `Enter`: Execute all currently selected optimizations.
    - `Q` / `Esc`: Exit the application.

### Overlay HUD & Logging
- **Logging System**: An asynchronous log panel at the bottom of the screen provides real-time execution feedback (RUNNING, SUCCESS, ERROR).
- **Design Benefit**: Centralizes feedback without interrupting the user's configuration flow.

---

## 1.1 Command Execution Engine (v0.7.0)

The TUI is now decoupled from the execution logic via the **Command Pattern**:
- **Atomic Tweaks**: Each optimization is a self-contained `TweakCommand`.
- **Registry-Driven**: The UI dynamically queries the `CommandRegistry` for execution.
- **Async Safety**: All system operations are wrapped in `tokio::spawn_blocking` to prevent UI hangs.

---

## 2. System Hardening Engine

pieuvre implements a hardening engine based on native Windows Access Control Lists (ACLs) and direct registry manipulation.

- **SDDL (Security Descriptor Definition Language)**: Used to lock critical registry keys and services by applying restrictive security descriptors.
- **Privilege Management**: Utilizes `SeTakeOwnershipPrivilege` to modify system-protected objects.
- **Native API (v0.7.0)**: Migration to `windows-rs` for all registry and service operations, eliminating `reg.exe` overhead.

---

## 3. AI & Privacy Management

- **Recall Blocking**: Disables the Windows Recall feature via Group Policy Objects (`DisableAIDataAnalysis`) and registry keys.
- **CoPilot Suppression**: Removes CoPilot integrations from the taskbar and the Edge browser.
- **Telemetry Blocking**: Multi-layered approach involving Services, Scheduled Tasks, Hosts file redirection, and Firewall rules.

---

## 4. Multi-Crate Architecture

- `pieuvre-cli`: TUI entry point and main orchestrator.
- `pieuvre-sync`: Execution engine for optimizations (Services, Registry, Cleanup).
- `pieuvre-audit`: Inspection engine for system analysis and detection.
- `pieuvre-persist`: Management of snapshots, compression, and state persistence.

---

## 5. Latency Optimization (DPC/ISR)

- **Timer Resolution**: Forces the kernel timer to 0.5ms via `NtSetTimerResolution` to minimize input lag.
- **Interrupt Affinity**: Distributes hardware interrupts across physical CPU cores to avoid saturation and reduce DPC latency.
- **MSI (Message Signaled Interrupts)**: Migrates PCI devices to MSI mode to eliminate IRQ conflicts and reduce interrupt overhead.

---

## 6. Monitoring & Audit

- **Sentinel Engine**: Monitors critical registry keys for unauthorized changes using `RegNotifyChangeKeyValue`.
- **Hardware Audit**: Exhaustive probing of CPU, RAM, and software configuration via `pieuvre-audit`.
- **Real-time Metrics**: High-frequency acquisition of system metrics (CPU/RAM) via `sysinfo` in a dedicated background thread.

