# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-12-22 (The Ghost)

### Added

#### UI "The Ghost": SOTA 2026
- **Zero-Frame Interface**: Complete removal of ASCII borders for a pure, data-centric experience.
- **Minimalist Header**: New logo `⣠⟬ ⊚ ⟭⣄` in `dimmed cyan` with single-line system metadata.
- **Expert Aesthetics**: Systematic use of `dim` colors for secondary data and `bold white` for active selection.
- **Fluid Feedback**: Borderless progress bars `█` and single-character spinners.

#### Sentinel Engine: Advanced Hardening
- **Service Monitoring**: Real-time tracking of critical service states via `NotifyServiceStatusChange`.
- **Persistence Protection**: Enhanced hardening of RunKeys and Winlogon shell vectors.

### Technical
- **Documentation SOTA**: Full audit and validation of all `.md` files to 2026 standards.
- **Zero Redundancy**: Codebase-wide refactoring for maximum architectural purity.

---

## [0.3.0] - 2025-12-22 (Climax)

#### Sentinel Engine: Reactive Inviolability
- **Event-Driven Mode**: Migration from polling to event-driven monitoring via `RegNotifyChangeKeyValue`.
- **Instant Self-Healing**: Automatic and immediate restoration of critical registry keys (IFEO, Winlogon).
- **Supreme Hardening**: New `hardening.rs` module for persistence vector protection.

#### ETW Engine & Reactive Intelligence
- **Driver Resolution**: Implementation of `DriverResolver` to map kernel addresses to real driver names.
- **Precise Monitoring**: Per-driver DPC/ISR latency capture via native `EventTrace` APIs.
- **Automated Interrupt Steering**: New `interrupts.rs` module dynamically adjusting CPU affinity.

#### Pinnacle Cockpit & UX
- **Live Dashboard**: `pieuvre status --live` command with 500ms refresh rate.
- **Advanced Dashboard**: Complete visual overhaul covering all 9 optimization sections.

#### Profile Alignment & Architecture
- **Data-Driven Architecture**: Total overhaul of the sync engine driven by TOML files.
- **Total Alignment**: Perfect synchronization between `@config/profiles` and crate capabilities.
- **Atomic Operations**: New executors for `MsiOperation`, `AppxOperation`, and `PowerPlanOperation`.
- **Exhaustive Registry Config**: Full support for all privacy and telemetry keys defined in TOML.

### Technical
- **Native API Migration (100%)**: Total elimination of dependencies on CLI tools (`netsh`, `powercfg`, `schtasks`).
- **Async Runtime**: Full integration of `tokio` for parallel monitoring.
- **Zero Clippy Warnings**: Maintained high quality standards.
- **CI/CD Hardening**: Improved GitHub Actions workflows, added `cargo-deny` for security auditing, migrated configuration to v0.18+ standards, resolved license conflicts (GPL-3.0 literal, Unicode-3.0), and fixed missing Nextest CI profile.
- **Documentation Overhaul**: All 12 `.md` files (READMEs, Architecture, Technical) updated and validated to SOTA 2026 standards for clarity, technical excellence, and zero redundancy.

---

## [0.2.1] - 2025-12-22

### Added

#### Interactive Mode - Complete Harmonization
- **9 sections** (previously 5) for exhaustive coverage of sync modules.
- Section 6: **CPU/Memory** (core parking, memory compression, superfetch, static page file).
- Section 7: **DPC Latency** (paging executive, dynamic tick, TSC sync, HPET, interrupt affinity).
- Section 8: **Security** (HVCI, VBS, Spectre/Meltdown) with critical warnings.
- Section 9: **Advanced Network** (interrupt moderation, LSO, EEE, RSS, RSC).

#### UX & Interface - Guided Flow
- **Default Mode**: Automatic launch of interactive mode if `pieuvre.exe` is executed without arguments.
- **ASCII Welcome Screen**: New professional banner with version and architecture detection.
- **Initial Diagnostic**: Quick system state display (Timer, Power Plan, DiagTrack) on launch.
- **Main Menu**: Centralized navigation between customization, quick apply, status, and snapshots.
- **Privilege Check**: Automatic detection of administrator status with contextual warnings.

#### Performance - Enhanced GPU Options
- `enable_game_mode` - Windows hardware Game Mode.
- `prerendered_frames` - Pre-Rendered Frames = 1 (minimal input lag).
- `vrr_opt` - VRR Scheduling optimizations.
- `shader_cache` - DirectX Shader Cache 256MB.

#### User Interface
- **"NO EMOJI" Style**: Complete interface refactoring for a professional, expert, and sober rendering.
- `RiskLevel::Critical` - Risk level for critical security options.
- `print_security_warning()` - Visual warning for Security section.
- `print_selection_summary_full()` - 9-section summary with reboot indicators.
- `print_final_result_with_reboot()` - Reboot notification if DPC/Security selected.

#### Executors
- `CPUExecutor`, `DPCExecutor`, `SecurityExecutor`, `NetworkAdvancedExecutor`.

### Technical
- **103 unit tests** - 100% passing (+4 new section tests).
- **Zero clippy warnings** (fixed `redundant_guards`).
- Automatic laptop detection for risky options.
- Automatic reboot indicator based on selected options.

---

## [0.2.0] - 2025-12-22

### Added

#### Dependencies Update
- `windows` crate 0.58 → **0.62.2** (modern native APIs).
- `windows-sys` 0.59 → **0.61.0**.
- `clap` pinned to **4.5.23**.
- `zstd` **0.13.3** - Snapshot compression.
- `sha2` **0.10.9** - Integrity checksums.
- `clap_complete` **4.5.62** - Shell completions.

#### Shell Completions
- `pieuvre completions <shell>` command.
- Support: Bash, Zsh, Fish, PowerShell, Elvish.
- Stdout generation, redirection to shell config file.

#### Snapshots
- Automatic **zstd** compression (3-10x ratio).
- **SHA256** checksums with validation on restore.
- Automatic rotation (max 10 snapshots).
- Backward compatibility with legacy .json files.

#### Configuration
- Externalized `telemetry-domains.txt` file.
- Harmonized `default` → `default_profile`.
- Compression enabled by default.

### Changed

#### Native Windows APIs
- `power.rs`: Migration to `PowerGetActiveScheme` / `PowerSetActiveScheme`.
- `security.rs`: Migration to native `set_dword_value` (HVCI, VBS, Spectre).
- Significant reduction in `Command::new` calls.

#### Code Quality
- **Zero clippy warnings** (11 fixes).
- Types `&PathBuf` → `&Path` (clippy::ptr_arg).
- Removed redundant closures.
- Fixed late initialization.

### Technical
- **97 unit tests** - 100% passing.
- Fixed `windows` 0.62 breaking changes (6 files).
- Improved inline documentation.

---

## [0.1.0] - 2025-12-22

### Added

#### Unit Testing
- **pieuvre-sync**: 25 unit tests (timer, services, power, dpc, cpu, security, game_mode, network).
- **pieuvre-persist**: 14 unit tests (snapshot structure, JSON serialization, restore/delete validation).
- **pieuvre-cli**: 30 unit tests (commands, parsing, verbose levels, interactive modules).
- **pieuvre-audit**: 28 existing tests validated.

#### Architecture
- Rust 2021 Workspace with 5 modular crates.
- `pieuvre-common`: Shared types, errors (thiserror), configuration.
- `pieuvre-audit`: Read-only system audit (hardware, registry, services, security).
- `pieuvre-sync`: 20 synchronization modules.
- `pieuvre-persist`: JSON snapshots with service/registry/firewall rollback.
- `pieuvre-cli`: clap 4.5 CLI interface with interactive mode.

#### Sync Modules (20 modules)
- `services.rs`: Service management via native Windows APIs.
- `timer.rs`: NtSetTimerResolution (0.5ms).
- `power.rs`: Power plans (Ultimate Performance).
- `registry.rs`: Atomic HKLM writes.
- `firewall.rs`: Windows Firewall rules.
- `game_mode.rs`: Game Bar, HAGS, pre-rendered frames.
- `network.rs`: Nagle, Interrupt Moderation, LSO, RSS.
- `cpu.rs`: Core Parking, Memory Compression.
- `dpc.rs`: DisablePagingExecutive, Dynamic Tick, TSC Sync.
- `security.rs`: VBS, HVCI, Spectre/Meltdown.
- `appx.rs`: Bloatware removal (47 packages).
- `hosts.rs`: Telemetry domain blocking.
- `scheduled_tasks.rs`: Scheduled task deactivation.
- `onedrive.rs`: OneDrive removal.
- `context_menu.rs`: Classic context menu.
- `widgets.rs`: Win11 widgets deactivation.
- `windows_update.rs`: Update control.
- `edge.rs`: Edge management.
- `explorer.rs`: Explorer tweaks.
- `msi.rs`: GPU/NIC MSI Mode.

#### Profiles
- `gaming`: 0.5ms Timer, Ultimate Performance, telemetry/perf services.
- `privacy`: Full telemetry services, firewall rules.
- `workstation`: 1ms Timer, High Performance, minimal telemetry.

#### Interactive Mode
- dialoguer + indicatif interface.
- 5 sections: Telemetry, Privacy, Performance, Scheduler, AppX.
- Colored progress bars, confirmation before application.
- Automatic pre-modification snapshot.

### Fixed
- Removed unused `std::fs` import in persist tests.
- `#[allow(dead_code)]` annotation on `create_spinner` (reserved for future extensions).

### Security
- Service API via `windows` crate (no shell injection).
- Timer API via native `ntdll`.
- Snapshots for full rollback.

---

## Roadmap

### [0.4.0] - Planned
- [ ] `bios.rs` module (TPM, Secure Boot via WMI).
- [ ] `defender.rs` module (targeted exclusions and extended hardening).
- [ ] Structured logging with `#[instrument]` for kernel debug.
- [ ] Ultra-lightweight Rust GUI (egui/slint).
