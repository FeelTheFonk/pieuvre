# pieuvre-cli

Command-line interface for pieuvre Windows optimization tool.

[![SOTA 2026](https://img.shields.io/badge/SOTA-2026-brightgreen)]()
[![Tests](https://img.shields.io/badge/tests-12%20passed-success)]()

## Architecture SOTA

Le mode interactif utilise une architecture modulaire:

| Module | Description |
|--------|-------------|
| `interactive/mod.rs` | Orchestrateur principal (Flow guide par defaut) |
| `interactive/sections.rs` | 9 sections avec `OptItem` typé et `RiskLevel` |
| `interactive/executor.rs` | Trait `OptExecutor` + 9 implémentations |
| `interactive/ui.rs` | Interface SOTA (ASCII Art, NO EMOJI) |

---

## Commands

| Command | Description |
|---------|-------------|
| `audit` | Collect system state (services, hardware, telemetry) |
| `analyze` | Generate profile-based recommendations |
| `sync` | Apply profile optimizations |
| `status` | Display current optimization state |
| `interactive` | Granular selection interface (Default launch) |
| `rollback` | Restore previous system state |
| `verify` | Check integrity of applied changes |

---

## Command Reference

### audit

Collect current system state.

```powershell
pieuvre audit [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--full` | Complete audit including AppX packages |
| `--output <PATH>` | Custom output path for report |

**Example:**
```powershell
pieuvre audit --full
# Output: C:\ProgramData\pieuvre\reports\audit_2025-12-22.json
```

---

### analyze

Generate optimization recommendations.

```powershell
pieuvre analyze --profile <PROFILE>
```

| Option | Description |
|--------|-------------|
| `--profile` | Profile to analyze: `gaming`, `privacy`, `workstation` |

**Example:**
```powershell
pieuvre analyze --profile gaming
```

---

### sync

Apply optimizations from a profile.

```powershell
pieuvre sync --profile <PROFILE> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--profile` | Profile to apply |
| `--dry-run` | Preview changes without applying |
| `--force` | Skip confirmation prompt |

**Example:**
```powershell
pieuvre sync --profile gaming --dry-run   # Preview
pieuvre sync --profile gaming             # Apply
```

---

### interactive

Granular selection interface with 65+ options.
**Note:** Ce mode est lance par defaut si aucun argument n'est fourni a `pieuvre.exe`.

```powershell
pieuvre interactive --profile <PROFILE>
```

| Option | Description |
|--------|-------------|
| `--profile` | Base profile for pre-selection |

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

---

### status

Display current optimization state.

```powershell
pieuvre status
```

Shows:
- Applied optimizations
- Service states
- Power plan
- Timer resolution
- Last snapshot info

---

### rollback

Restore previous system state.

```powershell
pieuvre rollback [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--last` | Restore most recent snapshot |
| `--id <UUID>` | Restore specific snapshot |
| `--list` | List available snapshots |

**Example:**
```powershell
pieuvre rollback --list                              # View snapshots
pieuvre rollback --last                              # Restore latest
pieuvre rollback --id 7be4b13b-051a-4cb2-afb2-257c7a3aff2c
```

---

### verify

Check integrity of applied changes.

```powershell
pieuvre verify [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `--strict` | Fail on any mismatch |

**Checks:**
- Timer resolution
- Power plan
- Service states
- MSI mode
- Firewall rules
- Registry values

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Configuration error |
| 3 | Permission denied (run as Administrator) |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `PIEUVRE_CONFIG` | Custom config file path |
| `PIEUVRE_LOG` | Log level override (trace, debug, info, warn, error) |
