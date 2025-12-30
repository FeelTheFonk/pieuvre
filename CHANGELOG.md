# Changelog - pieuvre

## [0.8.4] - 2025-12-30

### Sentinel (`pieuvre-sync`)
- **Robustesse** : Ajout fonction `key_exists()` pour verifier l'existence des cles avant verrouillage.
- **Nettoyage** : Retrait des cles TrustedInstaller protegees de `CRITICAL_KEYS` (SESSION_MANAGER_KERNEL, IFEO, WINLOGON).
- **Nettoyage** : Retrait des cles potentiellement inexistantes (APPINIT_DLLS, AI_DATA_ANALYSIS, EXPLORER_SHELL_DELAY).
- **Logging** : Migration de ERROR vers WARN pour les echecs de verrouillage (cles systeme protegees).
- **Filtrage** : Pre-verification `key_exists()` avant spawn des threads de monitoring.

### TUI/CLI (`pieuvre-cli`)
- **i18n** : Harmonisation complete de l'interface en francais (Categories, Optimisations, Journal d'execution).
- **UX** : Affichage des erreurs detaillees avec format Debug dans le panel de logs.

---

## [0.8.3] - 2025-12-30

### Scan (`pieuvre-scan`)
- **Structure Threat** : Refonte du modele de donnees avec structure `Threat` (name, description, severity, source, location).
- **YARA-X** : Enrichissement des regles par defaut (8 regles : adware, stealers, obfuscation, droppers, LNK).
- **Navigateurs** : Extension du support (Chrome, Edge, Brave, Firefox) avec detection des extensions forcees.
- **LNK Forensics** : Detection elargie (11 patterns : PowerShell, mshta, bitsadmin, certutil, encoded commands).
- **Registre** : Extension des cles ASEP (10 cles : Run, RunOnce, IFEO, Services, Winlogon, Shell Folders).
- **Performance** : Parallelisation avec Rayon et filtrage Aho-Corasick integre.

### Architecture
- **Exports** : Exposition publique de `ScanEngine`, `Threat`, `ThreatSeverity` via `lib.rs`.
- **Typage** : Harmonisation des retours de fonctions (`Vec<Threat>` au lieu de `Vec<String>`).

---

## [0.8.2] - 2025-12-30

### TUI/CLI (`pieuvre-cli`)
- **Scan** : Desactivation par defaut des options de scan pour eviter les executions accidentelles.
- **Isolation** : Separation logique stricte entre les scans et les optimisations systeme dans l'interface interactive.
- **UX** : Implementation d'un prompt de confirmation dynamique avant toute application de changements.
- **Robustesse** : Correction de warnings de compilation et optimisation de la gestion des evenements clavier (`ESC`, `ENTER`).

## [0.8.1] - 2025-12-30

### Sentinel (`pieuvre-sync`)
- **Resilience** : Correction des erreurs critiques `WIN32_ERROR(5)` (Access Denied) et `WIN32_ERROR(1060)` (Service Not Found).
- **Hardening** : Implementation d'un mecanisme automatique de prise de possession (`take_ownership`) pour les cles de registre verrouillees.
- **Services** : Validation proactive de l'existence des services avant toute operation de verrouillage.
- **Monitoring** : Optimisation de la boucle de surveillance pour ignorer silencieusement les services inexistants sur l'hote.

### Architecture SOTA
- **Consolidation DRY** : Centralisation de toutes les constantes de registre et de services dans `hardening.rs` (Source unique de verite).
- **Unification** : Migration massive de `executor.rs` vers le `SyncOperationCommand` pattern, eliminant les structures de commandes redondantes.
- **Refactorisation** : Epuration de `registry.rs` des fonctions de haut niveau au profit d'operations atomiques et modulaires.
- **Reset** : Refonte de la logique de reinitialisation pour utiliser les modules specialises (`privacy_o_o`, `security`).

### Scan (`pieuvre-scan`)
- **Privileges** : Acquisition de `SeDebugPrivilege` pour l'analyse memoire.
- **Moteur** : Migration vers YARA-X (Rust natif).
- **Navigateurs** :
    - Chrome/Edge : Detection `ExtensionInstallForcelist`.
    - Firefox : Analyse `user.js` et extensions tierces.
- **Signatures** : Ajout patterns Blitz (IFEO, AppInit, PowerShell).

## [0.7.0] - 2025-12-30

### Architecture
- **Command Pattern** : Migration vers `TweakCommand`.
- **Commandes** : Ajout `cleanup_edge`, `dns_doh`, `hardening_unlock`.
- **TUI** : Modularisation par categories.
- **CLI** : Exposition via sous-commande `tweak`.

### Systeme
- **API** : Utilisation de `windows-rs` (suppression `reg.exe`).
- **Registre** : Support Multi-Hive (HKLM + HKU).
- **Hardening** : Configuration SDDL et support PPL.

### Optimisations
- **Modules** : `hvci`, `vbs`, `spectre`, `activity_history`, `cortana`, `uac_level`.
- **Nettoyage** : WinSxS, cache Edge, fichiers temporaires, flush DNS.
- **Reseau** : Timer kernel, MSI mode, algorithme de Nagle.

## [0.6.2] - 2025-12-23

### Refactoring
- **Dependances** : Suppression `rusqlite` et `db.rs`.
- **Code** : Nettoyage `executor.rs`.
- **Erreurs** : Gestion explicite (suppression `unwrap`).

## [0.6.0] - 2025-12-23

### Interface
- **Sidebar** : Navigation par categories.
- **Async** : Utilisation de `tokio` pour les taches de fond.
- **Monitoring** : Metriques CPU/RAM en temps reel.

## [0.5.5] - 2025-12-23

### Detection
- **LOLDrivers** : Scan des drivers vulnerables.
- **WMI** : Scan de persistance.
- **Metriques** : Suivi latence DPC.

## [0.5.0] - 2024-12-20

### Initial
- **Core** : Moteur d'optimisation services/registre.
- **TUI** : Interface Ratatui.
- **Audit** : Rapport materiel et securite.