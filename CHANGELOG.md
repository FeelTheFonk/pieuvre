# Changelog - pieuvre

## [0.6.2] - 2025-12-23

### Audit "Deep Clean" Optimization
- **Optimization (Phase 3)**:
    - Removed `rusqlite` dependency and `db.rs` module for reduced binary size and improved portability.
    - Cleaned up redundant state capture code in `executor.rs`.
    - Merged `ai.rs` module into direct registry operations within `pieuvre-sync`.
- **Precision**:
    - Implemented native OS and Build Number detection via Windows Registry in `pieuvre-audit`.
    - Harmonized types and synchronous APIs for maximum reactivity across all crates.
- **Robustness & Security**:
    - Exhaustive audit and removal of unsafe `unwrap()` and `expect()` calls in production code.
    - Reinforced error handling in `dns` and `cleanup` modules.

## [0.6.0] - 2025-12-23

### Interface Overhaul (Phases 1 & 2)
- **Sidebar/MainView Architecture**:
    - **Sidebar Navigation**: Centralized category management with instant preview.
    - **Flux/Redux Pattern**: State management via `AppState` and `Action` messages for total predictability.
    - **Async Execution**: Background scans and optimizations via `tokio::spawn` with real-time feedback in the log panel.
    - **Zero Warning**: Cleaned codebase with no unused variables or dead code.
- **Design System (v2)**:
    - **Vibrant Theme**: High-contrast Cyan/Magenta/Yellow palette.
    - **Real-time Metrics**: Dynamic header displaying CPU, RAM, and Uptime via dedicated background threads.
    - **Logging System**: Asynchronous log console with status icons and timestamps.
- **Navigation & Controls**:
    - **Default Launch**: Interactive mode is now the primary entry point.
    - **Optimized Controls**: `Tab` for categories, `Up/Down` for items, `Space` to toggle, `Enter` to apply.

## [0.5.5] - 2025-12-23

### Advanced Threat Detection & UI Alignment
- **LOLDrivers Integration**: Detection of vulnerable drivers used in BYOVD (Bring Your Own Vulnerable Driver) attacks.
- **WMI Persistence Audit**: Scanning for malicious WMI event consumers and filters.
- **UI Risk Indicators**: Visual cues for high-risk optimizations in the TUI.
- **Latency Monitoring**: Real-time DPC latency tracking in the dashboard.

## [0.5.0] - 2024-12-20

### Initial Release
- **Core Engine**: High-performance registry and service optimization engine.
- **TUI v1**: Tab-based navigation using Ratatui.
- **Audit Module**: Comprehensive hardware configuration and security reporting.