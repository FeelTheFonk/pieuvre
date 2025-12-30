# Changelog - pieuvre

## [0.8.0] - 2025-12-30

### Moteur de Scan SOTA (`pieuvre-scan`)
- **Acquisition de Privilèges** : Implémentation de l'acquisition automatique de `SeDebugPrivilege` pour une visibilité système totale (processus protégés, ruches restreintes).
- **Migration YARA-X** : Passage au moteur Rust natif de VirusTotal pour une détection de malwares ultra-performante et sécurisée.
- **Forensique Navigateur Avancée** :
    - **Chrome/Edge** : Détection des politiques d'extensions forcées (`ExtensionInstallForcelist`).
    - **Firefox** : Analyse des fichiers `user.js` et détection des extensions tierces suspectes.
    - **Performance** : Intégration de `simd-json` pour un parsing haute performance des profils.
- **Optimisation Aho-Corasick** : Enrichissement des patterns de scan Blitz avec des vecteurs d'attaque modernes (IFEO, AppInit, PowerShell, etc.).
- **Documentation & Vision** : Création d'un README technique et d'une ROADMAP stratégique pour le module de scan.

### Qualité & Maintenance
- **Zéro Warning** : Audit Clippy exhaustif sur tout le workspace (warnings as errors).
- **Tests Unitaires** : Couverture étendue à 20 tests validés sur l'ensemble des crates.
- **Harmonisation** : Alignement strict des types et des structures de données entre `pieuvre-scan` et le CLI.

## [0.7.0] - 2025-12-30

### Refonte Architecturale & CLI
- **Intégration du Command Pattern** : Migration complète vers un système de commandes atomiques (`TweakCommand`). Chaque optimisation est désormais isolée, testable et extensible.
- **Harmonisation Totale des IDs** : Alignement strict entre le moteur d'exécution (`executor.rs`) et l'interface TUI. Ajout des commandes manquantes : `cleanup_edge`, `dns_doh`, `hardening_unlock`.
- **Modularisation de la TUI** : Fragmentation de l'interface en modules spécialisés (`telemetry`, `privacy`, `oo_privacy`, `performance`, `security`, `system`) pour une maintenance facilitée.
- **Nouveaux Menus SOTA** : Intégration des sections `Scan` (YARA-X, Browser), `Audit` (Hardware, Security) et `Sync` (Persistence) dans le TUI.
- **Sous-commande `tweak`** : Exposition granulaire du moteur via CLI (`list`, `apply`, `apply-all`).

### Moteur de Registre & Sécurité (SOTA)
- **Migration Native API** : Suppression des dépendances aux commandes externes (`reg.exe`) dans `game_mode.rs` au profit de l'API native `windows-rs`.
- **Multi-Hive Registry Support** : Application systématique des tweaks sur `HKLM` et toutes les ruches `HKU` (S-1-5-21) pour une couverture utilisateur totale.
- **Hardening Avancé** : 
    - Verrouillage SDDL des clés et services critiques pour prévenir les réinitialisations système.
    - Support du mode *Protected Process Light* (PPL) pour l'auto-protection du binaire.

### Optimisations & Performance
- **Nouvelles Unités de Tweak** : Implémentation de `hvci`, `vbs`, `spectre`, `activity_history`, `cortana` et `uac_level` avec niveaux de risque SOTA.
- **Nettoyage Système** : Ajout de modules pour WinSxS, cache Edge, fichiers temporaires et flush DNS.
- **Latence & Réseau** : Optimisation du timer système (0.5ms), MSI mode global et désactivation de l'algorithme de Nagle.

### Qualité & Build
- **Audit Zéro Défaut** : Suppression du code mort et des deltas de configuration.
- **Build Release Optimisé** : Activation de LTO (Link Time Optimization) et stripping pour un binaire ultra-performant.
- **Validation Workspace** : 100% de succès sur `cargo test` et `cargo clippy` (zéro warning).

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