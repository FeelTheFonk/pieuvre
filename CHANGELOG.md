# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

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

### Changed

- Section Telemetrie: 13 options (was 10)
- Firewall: 47 domaines (was 30)

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
