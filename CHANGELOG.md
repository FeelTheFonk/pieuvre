# Changelog

Toutes les modifications notables de ce projet sont documentées ici.

Le format suit [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-22

### Added

#### Tests SOTA 2026
- **pieuvre-sync**: 25 tests unitaires (timer, services, power, dpc, cpu, security, game_mode, network)
- **pieuvre-persist**: 14 tests unitaires (snapshot structure, sérialisation JSON, restore/delete validation)
- **pieuvre-cli**: 30 tests unitaires (commandes, parsing, verbose levels, modules interactive)
- **pieuvre-audit**: 28 tests existants validés

#### Architecture
- Workspace Rust 2021 avec 5 crates modulaires
- `pieuvre-common`: Types partagés, erreurs (thiserror), configuration
- `pieuvre-audit`: Audit système read-only (hardware, registry, services, security)
- `pieuvre-sync`: 20 modules de synchronisation (SOTA 24H2/25H2)
- `pieuvre-persist`: Snapshots JSON avec rollback service/registry/firewall
- `pieuvre-cli`: Interface CLI clap 4.5 avec mode interactif

#### Modules Sync (20 modules)
- `services.rs`: Gestion services via APIs Windows natives
- `timer.rs`: NtSetTimerResolution (0.5ms)
- `power.rs`: Plans d'alimentation (Ultimate Performance)
- `registry.rs`: Écritures atomiques HKLM
- `firewall.rs`: Règles Windows Firewall
- `game_mode.rs`: Game Bar, HAGS, pre-rendered frames
- `network.rs`: Nagle, Interrupt Moderation, LSO, RSS
- `cpu.rs`: Core Parking, Memory Compression
- `dpc.rs`: DisablePagingExecutive, Dynamic Tick, TSC Sync
- `security.rs`: VBS, HVCI, Spectre/Meltdown
- `appx.rs`: Suppression bloatware (47 packages)
- `hosts.rs`: Blocage domaines télémétrie
- `scheduled_tasks.rs`: Désactivation tâches planifiées
- `onedrive.rs`: Suppression OneDrive
- `context_menu.rs`: Menu contextuel classique
- `widgets.rs`: Désactivation widgets Win11
- `windows_update.rs`: Contrôle updates
- `edge.rs`: Gestion Edge
- `explorer.rs`: Tweaks Explorer
- `msi.rs`: MSI Mode GPU/NIC

#### Profils
- `gaming`: Timer 0.5ms, Ultimate Performance, services télémétrie/perf
- `privacy`: Services télémétrie complets 24H2, règles firewall
- `workstation`: Timer 1ms, High Performance, télémétrie minimale

#### Mode Interactif
- Interface dialoguer + indicatif
- 5 sections: Télémétrie, Privacy, Performance, Scheduler, AppX
- Progress bars colorées, confirmation avant application
- Snapshot automatique pré-modification

### Fixed
- Suppression import `std::fs` inutilisé dans tests persist
- Annotation `#[allow(dead_code)]` sur `create_spinner` (réservé extensions futures)

### Security
- Services API via `windows` crate (pas shell injection)
- Timer API via `ntdll` native
- Snapshots pour rollback complet

---

## Roadmap

### [0.2.0] - Prévu
- [ ] Checksum SHA256 pour snapshots
- [ ] Compression zstd des snapshots
- [ ] Module `bios.rs` (TPM, Secure Boot via WMI)
- [ ] Module `defender.rs` (exclusions ciblées)
- [ ] Migration APIs natives restantes (bcdedit → BCD APIs)

### [0.3.0] - Prévu
- [ ] Trait `SyncOperation` pour polymorphisme
- [ ] Configuration profiles TOML externes
- [ ] Shell completion (clap_complete)
- [ ] Async runtime (tokio) pour opérations parallèles
