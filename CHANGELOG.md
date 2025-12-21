# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.6.0] - 2025-12-21 (GUI)

### Added

- **pieuvre-gui**: Native GUI crate using Slint 1.14
  - Dark theme design system (Catppuccin-inspired)
  - Tokens: colors, spacing, typography, animations
  - Globals: AppState, SystemInfo, ProfileConfig
  - Components: Button, Toggle, Checkbox, ProgressBar, StatusBadge, Toast, CategoryGroup
  - Views: Dashboard, Audit, Optimizations, Profiles, Snapshots, Settings
  - Sidebar navigation with 6 sections
  - Header with profile badge and refresh action
  - StatusBar with system info and selection counter
  - Toast notification system with variants
  - Collapsible sections with smooth animations
  - Worker thread for async operations

### Backend Integration

- **init.rs**: Real system info via WMI/Registry (CPU, RAM, GPU, hostname, OS version)
- **callbacks.rs**: Full integration with pieuvre_audit, pieuvre_sync, pieuvre_persist
- **worker.rs**: Async execution with proper error handling
- Handlers: handle_run_audit, handle_apply_optimizations, handle_load_profile, handle_rollback_snapshot

### Changed

- Workspace: added pieuvre-gui member
- Dependencies: slint 1.14, slint-build 1.14, chrono, serde_json

### Technical

- Build system: build.rs with slint_build::compile
- Architecture: separate crate, modular design
- Zero TODO/FIXME, zero clippy warnings
- Release build optimized (1m27s)

### Phase 1 Complete - Slint-Rust Bindings

- **main.rs**: Full callback integration
  - `on_run_audit`, `on_apply_optimizations`, `on_load_profile`
  - `on_restore_snapshot`, `on_create_snapshot`, `on_save_settings`
- **Worker poll**: Timer 100ms, Arc<Mutex> thread-safe
- **Toast system**: Auto-dismiss 3s, success/error variants

### Phase 2.1 Complete - Dynamic Dashboard

- **MainWindow properties**: sys-os-version, sys-cpu-name, sys-ram-gb, etc.
- **DashboardView bindings**: Receives real system data from Rust
- **Data flow**: init.rs -> MainWindow -> DashboardView cards

### Phase 2.2 Complete - Dynamic Audit

- **8 services tracked**: DiagTrack, SysMain, WSearch, BITS, wuauserv, etc.
- **ServiceStates struct**: Extracted from pieuvre_audit::full_audit()
- **Real-time update**: Services status after audit completion

### Phase 2.5 Complete - Dynamic Snapshots

- **setup_snapshots()**: Loads 3 latest snapshots via pieuvre_persist::list_snapshots()
- **SnapshotsView properties**: snap1/2/3-id, timestamp, description, changes
- **Dynamic SnapshotRow**: Display real snapshot data with restore/delete actions

### Phase 3 Complete - Worker Enhancement

- **ProfileLoaded with profile_name**: Tracks loaded profile
- **OptimizationsApplied with profile_name**: Tracks applied profile
- **Tests**: 6/6 unit tests passing

## [0.5.0] - 2025-12-21 (SOTA P4)

### Added

- **Network Optimizations**: Disable Nagle Algorithm (TcpNoDelay, TcpAckFrequency)
- **CPU Power Throttling**: Disable PowerThrottling for max performance
- **Windows Recall Block**: Block 24H2 AI feature via Group Policy
- **Group Policy Telemetry**: Enterprise-level telemetry control

### Changed

- Section PRIVACY: 11 options (was 9)
- Section PERFORMANCE: 14 options (was 12)
- 17 modules pieuvre-sync (was 16)
- Total options interactive: 45 (was 39)

## [0.4.0] - 2025-12-21 (SOTA P3)

### Added

- **Edge Management**: Disable sidebar, shopping, collections, auto-updates (edge.rs)
- **Explorer Tweaks**: Show extensions, This PC, hide recent, no chat/task view (explorer.rs)
- **Game Mode**: Disable Game Bar/DVR, Fullscreen Optimizations, HAGS control (game_mode.rs)

### Changed

- Section PERFORMANCE: 12 options (was 7)
- 16 modules pieuvre-sync (was 13)
- Total options interactive: 39 (was 32)

## [0.3.1] - 2025-12-21 (SOTA P2)

### Added

- **Context Menu**: Classic menu + remove 7 clutter items (Paint3D, Photos, Clipchamp)
- **Widgets**: Disable Win11 widgets board and service
- **Windows Update Control**: Pause 35 days + disable driver auto-updates

### Changed

- Section Privacy: 9 options (was 5)
- 13 modules pieuvre-sync (was 10)

## [0.3.0] - 2025-12-21 (SOTA)

### Added

- **Scheduled Tasks**: Disable 25 telemetry tasks (Sophia Script reference)
  - Microsoft Compatibility Appraiser, CEIP, Disk Diagnostics
  - Family Safety, Feedback, Maps, Office Telemetry
- **Hosts File Blocking**: 50+ domains blocked via native DNS
  - SmartScreen, Copilot, Edge, Office, Bing telemetry
  - Reversible via `remove_telemetry_blocks()`
- **OneDrive Complete Removal**:
  - Process kill, uninstall, folder cleanup
  - Registry disable for Group Policy
- **status.rs enrichi**: +Services, MMCSS, MSI, Hosts sections

### Changed

- Section Telemetrie: 13 options (was 10)
- Firewall: 47 domaines (was 30)
- README: Sources SOTA completes (6 repos + 4 docs + 3 crates)

### Fixed

- Emojis supprimes (sync.rs, status.rs, analyze.rs, lib.rs)

## [0.2.3] - 2025-12-21

### Added

- **Firewall SOTA**: 30 domaines (was 13), 17 IP ranges (was 6)
  - SmartScreen, Copilot, Spotlight/Ads, Watson blobs
- **Verify MMCSS**: 8 checks (was 6)
  - SystemResponsiveness, NetworkThrottlingIndex

## [0.2.2] - 2025-12-21

### Added

- **Selection granulaire bloatware**: 10 categories au lieu de 1
  - Bing apps, Productivity, Media, Communication
  - Legacy, Tools, Third-party, Copilot, Cortana, Xbox
- 9 nouvelles fonctions appx.rs par categorie

## [0.2.1] - 2025-12-21

### Added

- **Bloatware etendu**: 42 packages (was 20)
  - Teams, Copilot, BingSearch
  - Disney+, Spotify, CandyCrush, Facebook
  - Paint3D, 3DBuilder, Print3D
  - WindowsAlarms, Mail/Calendar

## [0.2.0] - 2025-12-21

### Added

- **MMCSS Gaming**: SystemResponsiveness 10%, NetworkThrottling OFF
- **Games Priority**: GPU Priority 8, Task Priority 6
- **Global Timer Resolution**: Permanent timer via registry
- **Startup Delay**: Disable startup apps delay
- **Shutdown Timeout**: 2000ms fast shutdown
- **ChangeRecord**: 9 services telemetrie capturent etat original

### Changed

- Section SCHEDULER: 6 options (vs 1)
- Rollback fonctionnel avec services restaurables

## [0.1.3] - 2025-12-21

### Fixed

- interactive.rs: Capture ChangeRecord avant modification services (DiagTrack, dmwappush)
- Snapshot cree APRES modifications avec changes rempli (rollback fonctionnel)

## [0.1.2] - 2025-12-21

### Added

- verify command: 6 verifications (timer, power, DiagTrack, MSI, firewall, scheduler)
- get_service_start_type function for snapshot capture
- snapshot restore() implementation (registry, services, firewall)

### Fixed

- services.rs: CloseServiceHandle to prevent handle leaks
- snapshot restore now actually restores values instead of TODO

## [0.1.1] - 2025-12-21

### Added

- Interactive mode: 5 sections, 26 options (Telemetrie, Privacy, Performance, Scheduler, AppX)
- Privacy registry tweaks: telemetry level, advertising ID, location, activity history, Cortana
- AppX removal module: bloatware (20+ packages), Xbox apps
- MSI Mode full_path support for correct registry writes
- Detection etat actuel avant recommandations (analyze)

### Fixed

- MSI Mode: enable_msi now uses full registry path with instance (11/11 devices vs 0/25)
- analyze: CPU hybrid recommandation conditionnelle (Win32PrioritySeparation check)
- analyze: MSI recommandation conditionnelle (is_msi_enabled_on_gpu)
- interactive: utilise full_path pour enable_msi

### Changed

- MsiDevice struct: added full_path field
- read_dword_value (pieuvre-audit): now public

## [0.1.0] - 2025-12-21

### Added

- Initial release
- Core audit engine with hardware, services, telemetry detection
- Sync engine with timer, power, services, firewall, MSI modules
- Persistence engine with snapshot and rollback
- CLI with 7 commands: audit, analyze, sync, status, interactive, rollback, verify
- Interactive mode with category-based selection
- Laptop detection with hardware-aware recommendations
- Automatic snapshot before any modification
- Three profiles: gaming, privacy, workstation
- Telemetry domain blocklist (42 domains)
- JSON report export

### Security

- All modifications reversible via snapshot
- Non-destructive audit and analyze commands
- Confirmation required for sync operations
