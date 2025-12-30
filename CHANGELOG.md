# Changelog - pieuvre

## [0.8.2] - 2025-12-30

### TUI/CLI (`pieuvre-cli`)
- **Scan** : Désactivation par défaut des options de scan pour éviter les exécutions accidentelles.
- **Isolation** : Séparation logique stricte entre les scans et les optimisations système dans l'interface interactive.
- **UX** : Implémentation d'un prompt de confirmation dynamique avant toute application de changements.
- **Robustesse** : Correction de warnings de compilation et optimisation de la gestion des événements clavier (`ESC`, `ENTER`).

## [0.8.1] - 2025-12-30

### Sentinel (`pieuvre-sync`)
- **Résilience** : Correction des erreurs critiques `WIN32_ERROR(5)` (Access Denied) et `WIN32_ERROR(1060)` (Service Not Found).
- **Hardening** : Implémentation d'un mécanisme automatique de prise de possession (`take_ownership`) pour les clés de registre verrouillées.
- **Services** : Validation proactive de l'existence des services avant toute opération de verrouillage.
- **Monitoring** : Optimisation de la boucle de surveillance pour ignorer silencieusement les services inexistants sur l'hôte.

### Architecture SOTA
- **Consolidation DRY** : Centralisation de toutes les constantes de registre et de services dans `hardening.rs` (Source unique de vérité).
- **Unification** : Migration massive de `executor.rs` vers le `SyncOperationCommand` pattern, éliminant les structures de commandes redondantes.
- **Refactorisation** : Épuration de `registry.rs` des fonctions de haut niveau au profit d'opérations atomiques et modulaires.
- **Reset** : Refonte de la logique de réinitialisation pour utiliser les modules spécialisés (`privacy_o_o`, `security`).

### Scan (`pieuvre-scan`)
- **Privilèges** : Acquisition de `SeDebugPrivilege` pour l'analyse mémoire.
- **Moteur** : Migration vers YARA-X (Rust natif).
- **Navigateurs** :
    - Chrome/Edge : Détection `ExtensionInstallForcelist`.
    - Firefox : Analyse `user.js` et extensions tierces.
- **Signatures** : Ajout patterns Blitz (IFEO, AppInit, PowerShell).

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