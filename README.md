<p align="center">
  <img src="crates/pieuvre-cli/logo.svg" width="256" alt="pieuvre logo">
</p>

<h1 align="center">pieuvre</h1>

<p align="center">
  <strong>Outil d'alignement système pour Windows 11</strong>
</p>

<p align="center">
pieuvre est un utilitaire système en Rust pour le contrôle des paramètres Windows. Gestion des politiques de sécurité et optimisation via snapshots.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows%2011-lightgrey" alt="Platform">
  <img src="https://img.shields.io/badge/Rust-1.75+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/License-MIT%20/%20Apache--2.0-blue.svg" alt="License">
</p>

---

<p align="center">
  <img src="crates/pieuvre-cli/screen.png" width="800" alt="pieuvre TUI Dashboard">
</p>

## Architecture

- **API Native** : Interaction Win32/NT.
- **Audit** : Analyse d'état pré-modification.
- **Persistance** : Snapshots compressés (zstd), intégrité SHA256.
- **Adaptation** : Configuration selon matériel détecté.
- **Asynchrone** : Interface TUI via tokio.

---

## Fonctionnalités

### 1. Interface (TUI)
- **Command Pattern** : Exécution modulaire.
- **Métriques** : Monitoring CPU, RAM, Uptime.
- **Navigation** : Organisation par catégories.
- **Logs** : Retour d'exécution en temps réel.

### 2. Sécurité et Confidentialité
- **Confidentialité** : Application des politiques de groupe (GPO).
- **Télémétrie** : Désactivation (Services, Registre, Pare-feu, Hosts).
- **IA** : Désactivation Windows Recall et CoPilot.
- **Sentinel** : Surveillance des clés critiques.

### 3. Analyse
- **Moteur YARA-X** : Scan de signatures.
- **Navigateurs** : Analyse Chrome, Edge, Firefox.
- **Persistance** : Détection des mécanismes dans le registre.

### 4. Optimisation
- **Latence** : Timer kernel (0.5ms), MSI mode, DPC/ISR.
- **Hardware** : Core parking, compression mémoire, GPU scheduling.
- **Réseau** : Algorithme de Nagle, modération des interruptions.

---

## Installation

### Prérequis
- Windows 10/11 (64-bit)
- Rust 1.75+
- Privilèges Administrateur

### Compilation
```powershell
git clone https://github.com/FeelTheFonk/pieuvre.git
cd pieuvre
cargo build --release
```

---

## Utilisation

```powershell
# Analyse d'état
pieuvre audit --full

# Interface interactive
pieuvre interactive

# Gestion granulaire
pieuvre tweak list
pieuvre tweak apply <id>

# Restauration
pieuvre rollback --last
```

---

## Commandes

| Commande | Description |
|:---|:---|
| `audit` | Inspection et rapport. |
| `interactive` | Interface TUI. |
| `tweak` | Gestion des optimisations. |
| `status` | État de l'alignement. |
| `verify` | Vérification d'intégrité. |
| `rollback` | Restauration snapshot. |

---

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Détails Techniques](docs/TECHNICAL.md)
- [Référence CLI](crates/pieuvre-cli/README.md)
- [Contribution](CONTRIBUTING.md)

---

## Licence

MIT / Apache-2.0.
