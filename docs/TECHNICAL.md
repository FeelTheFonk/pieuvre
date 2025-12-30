# Détails Techniques

Implémentation de l'interface TUI et des moteurs d'optimisation.

---

## 1. Navigation TUI

Modèle basé sur une barre latérale pour la gestion des catégories.

### Contrôles
- **Structure** : Layout horizontal (`Sidebar` et `MainView`).
- **Touches** :
    - `Tab` / `BackTab` : Changement de focus.
    - `Up` / `Down` : Navigation.
    - `Space` : Sélection ([X] / [ ]).
    - `Enter` : Exécution.
    - `Q` / `Esc` : Quitter.

### HUD & Logs
- **Logging** : Panel asynchrone en bas d'écran (RUNNING, SUCCESS, ERROR).

---

## 1.1 Moteur d'Exécution

Découplage via le **Command Pattern** :
- **Tweaks** : Chaque optimisation est un `TweakCommand`.
- **Registre** : Utilisation de `CommandRegistry`.
- **Async** : `tokio::spawn_blocking` pour les opérations système.

---

## 2. Hardening

Utilisation des ACLs natives et manipulation directe du registre.

- **SDDL** : Verrouillage des clés et services via descripteurs de sécurité.
- **Privilèges** : Acquisition de `SeTakeOwnershipPrivilege`.
- **API Native** : Utilisation de `windows-rs`.

---

## 3. Confidentialité

- **Recall** : Désactivation via GPO (`DisableAIDataAnalysis`).
- **CoPilot** : Suppression des points d'entrée (taskbar, Edge).
- **Télémétrie** : Blocage multi-couches (Services, Tasks, Hosts, Firewall).

---

## 4. Architecture Multi-Crate

- `pieuvre-cli` : Point d'entrée et orchestrateur.
- `pieuvre-sync` : Moteur d'exécution.
- `pieuvre-audit` : Moteur d'inspection.
- `pieuvre-persist` : Gestion des snapshots.

---

## 5. Optimisation de Latence

- **Timer** : `NtSetTimerResolution` (0.5ms).
- **Interrupt Affinity** : Distribution des interruptions sur les cœurs CPU.
- **MSI** : Migration des périphériques vers Message Signaled Interrupts.

---

## 6. Surveillance

- **Sentinel** : Surveillance via `RegNotifyChangeKeyValue`.
- **Audit** : Probing matériel via `pieuvre-audit`.
- **Métriques** : Acquisition haute fréquence via `sysinfo`.

