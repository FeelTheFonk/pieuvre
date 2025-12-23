# pieuvre-cli

Command-line interface for pieuvre Windows optimization tool.

[![SOTA 2026](https://img.shields.io/badge/SOTA-2026-brightgreen)]()
[![Tests](https://img.shields.io/badge/tests-12%20passed-success)]()

## Architecture SOTA

Le mode interactif utilise une architecture modulaire:

| Module | Description |
|--------|-------------|
| `interactive/mod.rs` | Orchestrateur principal |
| `interactive/sections.rs` | 11 sections avec `OptItem` typé et `RiskLevel` |
| `interactive/executor.rs` | Trait `OptExecutor` + implémentations |
| `interactive/ui.rs` | Interface SOTA (ASCII Art, NO EMOJI) |

---
### `audit`

Collect current system state.

```powershell
pieuvre audit [--full] [--output <PATH>]
```

**Example:**
```powershell
pieuvre audit --full
```

---

### `interactive`

Granular selection interface with 100+ options.
**Note:** Ce mode est lancé par défaut si aucun argument n'est fourni.

```powershell
pieuvre interactive
```

**Categories:**
- Telemetry (13 options)
- Privacy (11 options)
- Performance & GPU (18 options)
- Scheduler (5 options)
- AppX Bloatware (10 categories)
- CPU & Memory (4 options)
- DPC Latency (5 options)
- Security (3 options)
- Network Avancé (5 options)
- Cleanup (SOTA 2026)
- AI & DNS (Recall, CoPilot, DoH)

---

### `status`

Display current optimization state.

```powershell
pieuvre status [--live]
```

---

### `rollback`

Restore previous system state.

```powershell
pieuvre rollback [--list] [--last] [--id <UUID>]
```

---

### `verify`

Check integrity of applied changes.

```powershell
pieuvre verify [--repair]
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Permission denied (run as Administrator) |

---

## Safety

- **Laptop detection** - Adjusts recommendations for battery devices.
- **Automatic snapshots** - Every modification is reversible.
- **Non-destructive analysis** - `audit` is read-only.
