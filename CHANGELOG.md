# Changelog - pieuvre

## [0.8.0] - 2025-12-30

### Scan (`pieuvre-scan`)
- **Privilèges** : Acquisition de `SeDebugPrivilege`.
- **Moteur** : Migration vers YARA-X (Rust natif).
- **Navigateurs** :
    - Chrome/Edge : Détection `ExtensionInstallForcelist`.
    - Firefox : Analyse `user.js` et extensions tierces.
    - Performance : Parsing via `simd-json`.
- **Signatures** : Ajout patterns Blitz (IFEO, AppInit, PowerShell).

### Maintenance
- **Audit** : Résolution des warnings Clippy.
- **Tests** : Intégration de 20 tests unitaires.
- **Harmonisation** : Alignement des types entre modules.

## [0.7.0] - 2025-12-30

### Architecture
- **Command Pattern** : Migration vers `TweakCommand`.
- **Commandes** : Ajout `cleanup_edge`, `dns_doh`, `hardening_unlock`.
- **TUI** : Modularisation par catégories.
- **CLI** : Exposition via sous-commande `tweak`.

### Système
- **API** : Utilisation de `windows-rs` (suppression `reg.exe`).
- **Registre** : Support Multi-Hive (HKLM + HKU).
- **Hardening** : Configuration SDDL et support PPL.

### Optimisations
- **Modules** : `hvci`, `vbs`, `spectre`, `activity_history`, `cortana`, `uac_level`.
- **Nettoyage** : WinSxS, cache Edge, fichiers temporaires, flush DNS.
- **Réseau** : Timer kernel, MSI mode, algorithme de Nagle.

## [0.6.2] - 2025-12-23

### Refactoring
- **Dépendances** : Suppression `rusqlite` et `db.rs`.
- **Code** : Nettoyage `executor.rs`.
- **Erreurs** : Gestion explicite (suppression `unwrap`).

## [0.6.0] - 2025-12-23

### Interface
- **Sidebar** : Navigation par catégories.
- **Async** : Utilisation de `tokio` pour les tâches de fond.
- **Monitoring** : Métriques CPU/RAM en temps réel.

## [0.5.5] - 2025-12-23

### Détection
- **LOLDrivers** : Scan des drivers vulnérables.
- **WMI** : Scan de persistance.
- **Métriques** : Suivi latence DPC.

## [0.5.0] - 2024-12-20

### Initial
- **Core** : Moteur d'optimisation services/registre.
- **TUI** : Interface Ratatui.
- **Audit** : Rapport matériel et sécurité.