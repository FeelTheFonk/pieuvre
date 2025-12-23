# Changelog - Pieuvre

## [0.6.2] - 2025-12-23
### Audit "Deep Clean" Optimization
- **Optimisation (Phase 3)**:
    - Removed `rusqlite` dependency and `db.rs` module (weight and portability gain).
    - Removed redundant state capture dead code in `executor.rs`.
    - Removed `ai.rs` module (merged into direct registry operations in `pieuvre-sync`).
- **Précision** :
    - Implemented real OS and Build Number detection via Windows Registry in `pieuvre-audit`.
    - Harmonized types and synchronous APIs for maximum reactivity across all crates.
- **Robustesse & Sécurité**:
    - Exhaustive audit of `unwrap()` and `expect()` in production code.
    - Reinforced error handling in `dns` and `cleanup` modules.

## [0.6.0] - 2025-12-23
### Refonte de l'interface (Phases 1 & 2)
- **Architecture Sidebar/MainView & Taffy** :
    - **Sidebar Navigation**: Centralized category management with instant preview.
    - **Flux/Redux Pattern**: State management via `AppState` and `Action` messages for total predictability.
    - **Async Execution**: Scans and optimizations launched in background via `tokio::spawn` with real-time feedback in the log panel.
    - **Zero Warning**: Cleaned codebase, no unused variables, no dead code.
- **Design System (v2)**:
    - **Vibrant Theme**: Cyan/Magenta/Yellow palette optimized for maximum contrast.
    - **Real-time Metrics**: Dynamic header displaying CPU, RAM, and Uptime via dedicated threads.
    - **Système de Logs** : Console de logs asynchrone avec icônes et horodatage.
- **Navigation & Controls**:
    - **Default Launch**: Interactive mode is now the main entry point.
    - **Optimized Controls**: `Tab` for categories, `Up/Down` for items, `Space` to toggle, `Enter` to apply.

## [0.5.5] - 2025-12-23
### Advanced Threat Detection & UI Alignment
- **LOLDrivers Integration** : Detection of vulnerable drivers used in BYOVD attacks.
- **WMI Persistence Audit** : Scanning for malicious WMI event consumers and filters.
- **UI Risk Indicators** : Visual cues for high-risk optimizations in the TUI.
- **Latency Monitoring** : Real-time DPC latency tracking in the dashboard.

## [0.5.0] - 2024-12-20
### Version Initiale
- **Moteur Core** : Optimisation haute performance du registre et des services.
- **TUI v1** : Interface Ratatui avec navigation par onglets.
- **Module Audit** : Rapports de configuration matérielle et de sécurité.