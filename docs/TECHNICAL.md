# Architecture Technique : Pieuvre TUI (v0.6.2)

## 1. Moteur de Navigation Sidebar (v0.6.0)
La TUI utilise une navigation par volet latéral (Sidebar) pour une gestion efficace des catégories d'optimisation.

### Navigation & Contrôles
- **Structure** : Layout horizontal divisé en `Sidebar` (Catégories) et `MainView` (Options & Détails).
- **Contrôles** :
    - `Tab` / `BackTab` : Navigation entre les catégories de la Sidebar.
    - `Up` / `Down` : Navigation verticale dans la liste des options de la catégorie active.
    - `Space` : Basculer l'état de sélection d'une option ([X] / [ ]).
    - `Enter` : Exécuter toutes les optimisations sélectionnées.
    - `Q` / `Esc` : Quitter l'application.

### Overlay HUD & Logging
- **Système de Logs** : Panneau de logs asynchrone en bas de l'écran avec retour visuel en temps réel (RUNNING, SUCCESS, ERROR).
- **Avantage** : Centralisation du feedback d'exécution sans interrompre le flux de configuration.

## 2. Durcissement Système (Hardening)
Pieuvre intègre un moteur de durcissement basé sur les ACLs natives de Windows et la manipulation directe du registre.
- **SDDL (Security Descriptor Definition Language)** : Verrouillage des clés de registre et services via descripteurs de sécurité.
- **Privilèges** : Gestion des privilèges `SeTakeOwnershipPrivilege` pour les objets système protégés.
- **Registre** : Détection précise de l'OS et du Build Number via `winreg`.

## 3. Gestion de l'IA & Confidentialité
- **Recall Blocking** : Désactivation via GPO (`DisableAIDataAnalysis`) et registres.
- **CoPilot** : Suppression complète des intégrations barre des tâches et Edge.
- **Télémétrie** : Blocage multi-niveaux (Services, Tâches planifiées, Fichier Hosts, et Firewall).

## 4. Architecture Multi-Crates
- `pieuvre-cli` : Point d'entrée TUI et orchestrateur.
- `pieuvre-sync` : Moteur d'exécution des optimisations (Services, Registre, Cleanup).
- `pieuvre-audit` : Moteur d'analyse et de détection.
- `pieuvre-persist` : Gestion des snapshots et de la persistance.

## 5. Optimisation de la Latence (DPC/ISR)
- **Timer Resolution** : Forçage à 0.5ms pour réduire l'input lag.
- **Interrupt Affinity** : Distribution des interruptions sur les cœurs physiques.
- **MSI (Message Signaled Interrupts)** : Migration PCI vers MSI pour éliminer les conflits d'IRQ.

## 6. Monitoring & Audit
- **Sentinel Engine** : Surveillance des clés critiques via `RegNotifyChangeKeyValue`.
- **Audit Hardware** : Sondage exhaustif CPU/RAM/Software via `pieuvre-audit`.
- **Métriques Temps Réel** : Acquisition via `sysinfo` dans un thread dédié.

