# Changelog

Toutes les modifications notables de ce projet sont documentées ici.

Le format suit [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-12-22 (Climax)

### Added

#### Sentinel Engine : Inviolabilité Réactive (SOTA 2026)
- **Mode Event-Driven** : Migration du polling vers une surveillance événementielle via `RegNotifyChangeKeyValue`.
- **Self-Healing Instantané** : Restauration automatique et immédiate des clés de registre critiques (IFEO, Winlogon).
- **Hardening Suprême** : Nouveau module `hardening.rs` pour la protection des vecteurs de persistance.

#### Moteur ETW & Intelligence Réactive (SOTA 2026)
- **Driver Resolution** : Implémentation du `DriverResolver` pour mapper les adresses noyau aux noms réels des drivers.
- **Monitoring Précis** : Capture de la latence DPC/ISR par driver via les APIs `EventTrace` natives.
- **Interrupt Steering Automatisé** : Nouveau module `interrupts.rs` ajustant dynamiquement l'affinité CPU.

#### Cockpit Pinacle & UX (SOTA 2026)
- **Live Dashboard** : Commande `pieuvre status --live` avec rafraîchissement 500ms.
- **Dashboard SOTA** : Refonte visuelle complète couvrant les 9 sections d'optimisation.

#### Alignement Profils & Architecture (SOTA 2026)
- **Architecture Data-Driven** : Refonte totale du moteur de synchronisation piloté par les fichiers TOML.
- **Alignement Total** : Synchronisation parfaite entre `@config/profiles` et les capacités des crates.
- **Opérations Atomiques** : Nouveaux exécuteurs pour `MsiOperation`, `AppxOperation` et `PowerPlanOperation`.
- **Registry Config Exhaustif** : Support complet de toutes les clés de confidentialité et télémétrie définies en TOML.

### Technical
- **Migration Native API (100%)** : Élimination totale des dépendances aux outils CLI (`netsh`, `powercfg`, `schtasks`).
- **Async Runtime** : Intégration complète de `tokio` pour le monitoring parallèle.
- **0 avertissement Clippy** : Standard de qualité SOTA maintenu.

---

## [0.2.1] - 2025-12-22

### Added

#### Mode Interactif SOTA - Harmonisation Complète
- **9 sections** (anciennement 5) pour couverture exhaustive des modules sync
- Section 6: **CPU/Memory** (core parking, memory compression, superfetch, page file statique)
- Section 7: **DPC Latency** (paging executive, dynamic tick, TSC sync, HPET, interrupt affinity)
- Section 8: **Security** (HVCI, VBS, Spectre/Meltdown) avec avertissements critiques
- Section 9: **Network Avancé** (interrupt moderation, LSO, EEE, RSS, RSC)

#### UX & Interface - Flow Guidé SOTA 2026
- **Mode par défaut**: Lancement automatique du mode interactif si `pieuvre.exe` est exécuté sans arguments.
- **Écran d'accueil ASCII**: Nouvelle bannière professionnelle avec détection de version et d'architecture.
- **Diagnostic Initial**: Affichage rapide de l'état système (Timer, Power Plan, DiagTrack) dès le lancement.
- **Menu Principal**: Navigation centralisée entre personnalisation, application rapide, statut et snapshots.
- **Vérification Privilèges**: Détection automatique du statut administrateur avec avertissements contextuels.

#### Performance - Options GPU Enrichies
- `enable_game_mode` - Windows Game Mode hardware
- `prerendered_frames` - Pre-Rendered Frames = 1 (input lag minimal)
- `vrr_opt` - VRR Scheduling optimizations
- `shader_cache` - DirectX Shader Cache 256MB

#### Interface Utilisateur
- **Style "NO EMOJI"**: Refactorisation complète de l'interface pour un rendu pro, expert et sobre.
- `RiskLevel::Critical` - Niveau de risque pour options de sécurité critiques.
- `print_security_warning()` - Avertissement visuel section Security.
- `print_selection_summary_full()` - Résumé 9 sections avec indicateurs reboot.
- `print_final_result_with_reboot()` - Notification reboot si DPC/Security sélectionnés.

#### Executors
- `CPUExecutor`, `DPCExecutor`, `SecurityExecutor`, `NetworkAdvancedExecutor`.

### Technical
- **103 tests unitaires** - 100% passent (+4 nouveaux tests sections).
- **0 warnings clippy** (correction `redundant_guards`).
- Détection automatique laptop pour options risquées.
- Indicateur reboot automatique selon options sélectionnées.

---

## [0.2.0] - 2025-12-22

### Added

#### SOTA 2026 - Dépendances
- `windows` crate 0.58 → **0.62.2** (APIs natives modernes)
- `windows-sys` 0.59 → **0.61.0**
- `clap` précisé **4.5.23**
- `zstd` **0.13.3** - Compression snapshots
- `sha2` **0.10.9** - Checksums intégrité
- `clap_complete` **4.5.62** - Shell completions

#### Shell Completions
- Commande `pieuvre completions <shell>`
- Support: Bash, Zsh, Fish, PowerShell, Elvish
- Génération stdout, redirection vers fichier config shell

#### Snapshots SOTA
- Compression **zstd** automatique (ratio 3-10x)
- Checksums **SHA256** avec validation au restore
- Rotation automatique (max 10 snapshots)
- Rétro-compatibilité fichiers .json ancients

#### Configuration
- Fichier `telemetry-domains.txt` externalisé
- Harmonisation `default` → `default_profile`
- Compression activée par défaut

### Changed

#### APIs Windows Natives
- `power.rs`: Migration vers `PowerGetActiveScheme` / `PowerSetActiveScheme`
- `security.rs`: Migration vers `set_dword_value` natif (HVCI, VBS, Spectre)
- Réduction significative appels `Command::new`

#### Code Quality
- **0 warnings clippy** (11 corrections)
- Types `&PathBuf` → `&Path` (clippy::ptr_arg)
- Closures redondantes supprimées
- Initialisation tardive corrigée

### Technical
- **97 tests unitaires** - 100% passent
- Breaking changes `windows` 0.62 corrigés (6 fichiers)
- Documentation inline améliorée

---

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

### [0.4.0] - Prévu
- [ ] Module `bios.rs` (TPM, Secure Boot via WMI)
- [ ] Module `defender.rs` (exclusions ciblées et hardening étendu)
- [ ] Logging structuré avec `#[instrument]` pour debug noyau
- [ ] Interface graphique (GUI) ultra-légère en Rust (egui/slint)
