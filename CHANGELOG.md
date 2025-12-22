# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.8.0] - 2025-12-22 (Audit SOTA)

### Added

- **pieuvre-audit**: Refonte SOTA complète du module d'audit
  - **security.rs** (NEW): Audit sécurité complet avec scoring 0-100
    - `audit_security()` - Analyse Defender, Firewall, UAC, SecureBoot
    - Recommandations par sévérité (Critical/High/Medium/Low)
    - `score_to_grade()` - Conversion score en note A-F
  - **GPU via DXGI**: Détection multi-GPU avec VRAM précis
    - `CreateDXGIFactory1` + `EnumAdapters1`
    - Fallback registre si DXGI échoue
  - **Storage SSD/NVMe**: Détection via IOCTL
    - `StorageDeviceSeekPenaltyProperty` pour SSD
    - `GetDiskFreeSpaceExW` pour taille réelle
  - **Registry étendu**: 30+ clés privacy/telemetry
    - `get_defender_status()` - Exclusions, Tamper, Cloud, PUA
    - `get_uac_status()` - Enabled, SecureDesktop, ConsentPrompt
    - `get_firewall_status()` - 3 profils Domain/Private/Public
    - `is_secure_boot_enabled()`, `is_credential_guard_enabled()`
  - **Services QueryServiceConfigW**: Vrai start_type (was hardcodé Manual)
    - 10 catégories (was 6): +Network, Gaming, Media, Peripheral
    - PID tracking pour services running
  - **tests.rs** (NEW): 28 tests unitaires couvrant tous les modules

### Changed

- **TelemetryStatus**: +4 champs (activity_history, cortana, web_search, error_reporting)
- **ServiceInfo**: +pid optionnel, +4 catégories
- **ServiceStatus**: +4 états transitoires (StartPending, StopPending, etc.)
- **ServiceStartType**: +Unknown pour cas d'erreur
- **Cargo.toml**: Features Windows +3 (Dxgi, Ioctl, IO)

### Technical

- 28 tests passent (0 failed)
- 7 fichiers sources dans pieuvre-audit (was 6)
- ~800 lignes nouvelles de code SOTA

## [0.7.0] - 2025-12-22 (SOTA 2025)

### Added

- **security.rs** (NEW): VBS/HVCI/Memory Integrity control
  - `disable_memory_integrity()` - 5-10% FPS gain
  - `disable_vbs()` - Complete VBS disable
  - `disable_spectre_meltdown()` - CPU mitigations (advanced)

- **dpc.rs** (NEW): DPC Latency optimizations
  - `disable_paging_executive()` - Keep kernel in RAM
  - `disable_dynamic_tick()` - Consistent timer behavior
  - `set_tsc_sync_enhanced()` - Better timer precision
  - `disable_hpet()` - HPET control
  - `set_interrupt_affinity_spread()` - Multi-core interrupt distribution

- **cpu.rs** (NEW): CPU/Memory optimizations
  - `disable_core_parking()` - All cores active
  - `disable_memory_compression()` - Reduce CPU overhead (16GB+)
  - `set_static_page_file()` - Static page file size

- **network.rs** extensions:
  - `disable_interrupt_moderation()` - Network latency reduction
  - `disable_lso()` - Large Send Offload disable
  - `disable_eee()` - Energy Efficient Ethernet disable
  - `enable_rss()` - Receive Side Scaling
  - `disable_rsc()` - Receive Segment Coalescing disable
  - `apply_all_network_optimizations()` - One-call all tweaks

- **game_mode.rs** extensions:
  - `set_prerendered_frames()` - GPU input lag control
  - `set_shader_cache_size()` - DirectX cache config
  - `disable_vrr_optimizations()` - VRR control
  - `is_hags_enabled()` - HAGS status check
  - `apply_all_gpu_optimizations()` - One-call all GPU tweaks

- **timer.rs** extension:
  - `reset_timer_resolution()` - Restore 15.625ms default

### Changed

- **windows_update.rs**: Date dynamique (chrono) au lieu de hardcodée
- **appx.rs**: +5 bloatware Windows 11 24H2 (DevHome, CrossDevice)
- **scheduled_tasks.rs**: +5 tâches AI/Recall/Copilot 24H2
- **lib.rs**: Services télémétrie +4 (CDPUserSvc, PushToInstall, etc.)

### Technical

- **20 modules** dans pieuvre-sync (was 17)
- **~1000 lignes** nouvelles de code SOTA
- Dépendance chrono ajoutée pour dates dynamiques
- Build optimisé: 1.15s release



## [0.6.0] - 2025-12-22

### Removed

- **pieuvre-gui**: Suppression complète du crate GUI Slint
  - Retrait des 28 fichiers sources (7 .rs, 19 .slint, Cargo.toml, build.rs)
  - Suppression des dépendances slint 1.14 et slint-build 1.14
  - Nettoyage de ~560 dépendances transitives du Cargo.lock
  - Focus sur l'outil CLI uniquement

### Changed

- **Documentation restructurée**: README principal allégé (454 → 96 lignes)
  - `docs/TECHNICAL.md`: Timer, Power, SCM, MSI documentation
  - `docs/ARCHITECTURE.md`: Structure projet et data flow
  - `config/README.md`: Profils et configuration détaillés
  - `crates/pieuvre-cli/README.md`: Référence CLI complète
  - `crates/pieuvre-sync/README.md`: 17 modules documentation
  - `crates/pieuvre-audit/README.md`: API audit
  - `crates/pieuvre-persist/README.md`: Snapshots et rollback
  - `crates/pieuvre-common/README.md`: Types partagés

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
