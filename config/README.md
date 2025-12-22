# Configuration

pieuvre configuration files and profiles documentation.

---

## Directory Structure

```
config/
├── default.toml            Default application settings
├── telemetry-domains.txt   Domains to block via firewall
└── profiles/
    ├── gaming.toml         Gaming optimization profile
    ├── privacy.toml        Privacy-focused profile
    └── workstation.toml    Professional workstation profile
```

---

## default.toml

```toml
[general]
log_level = "info"
snapshot_dir = "C:\\ProgramData\\pieuvre\\snapshots"
default_profile = "gaming"

[audit]
services = true
hardware = true
appx = true
network = true
```

---

## Profiles

### Gaming Profile

Optimizes for minimum latency and maximum performance.

| Optimization | Value | Risk | Impact |
|-------------|-------|------|--------|
| Timer Resolution | 0.5ms | Low | +Power consumption |
| Power Plan | Ultimate Performance | Low | +Power consumption |
| DiagTrack | Disabled | None | - |
| SysMain (Superfetch) | Disabled | None | SSD only |
| Win32PrioritySeparation | 0x26 | None | - |
| MMCSS SystemResponsiveness | 10% | None | - |
| NetworkThrottlingIndex | OFF | None | - |
| GPU Priority | 8 (max) | None | - |
| GlobalTimerResolution | Permanent | Low | +Power |

### Privacy Profile

Minimizes telemetry and data collection.

| Optimization | Value | Risk | Impact |
|-------------|-------|------|--------|
| DiagTrack | Disabled | None | - |
| dmwappushservice | Disabled | None | - |
| Data Collection Level | 0 (Security) | None | - |
| Advertising ID | Disabled | None | - |
| Firewall Rules | Block 47 domains | Low | Some MS features |
| Firewall Rules | Block 17 IP ranges | Low | Some MS features |
| Copilot | Disabled | None | - |
| Hosts File | Block 50+ domains | Low | Some MS features |
| Scheduled Tasks | Disable 25 tasks | None | - |

### Workstation Profile

Balances performance with stability for professional use.

| Optimization | Value | Risk | Impact |
|-------------|-------|------|--------|
| Power Plan | High Performance | None | +Power |
| WSearch | Manual | Low | Slower search |
| Background Apps | Limited | Low | - |

---

## telemetry-domains.txt

Domains blocked via Windows Firewall:

```
# Microsoft Telemetry
vortex.data.microsoft.com
watson.telemetry.microsoft.com
settings-win.data.microsoft.com
telemetry.microsoft.com

# SmartScreen
smartscreen.microsoft.com
smartscreen-prod.microsoft.com

# Cortana
cortana.ai
www.bing.com/api/cortana

# Copilot
copilot.microsoft.com
sydney.bing.com

# Advertising
choice.microsoft.com
ads.msn.com
adnxs.com

# Office Telemetry
officeclient.microsoft.com
config.office.com
...
```

Full list: 47 domains

---

## Profile TOML Format

```toml
[profile]
name = "gaming"
description = "Optimizes for minimum latency"

[services]
DiagTrack = "disabled"
SysMain = "disabled"
WSearch = "manual"

[registry]
Win32PrioritySeparation = 0x26
SystemResponsiveness = 10
NetworkThrottlingIndex = 0xffffffff

[power]
plan = "ultimate"
timer_resolution = 0.5

[telemetry]
diagtrack = false
firewall_rules = true
hosts_blocking = true
scheduled_tasks = true
```

---

## Custom Profiles

Create custom profiles by copying an existing one:

```powershell
copy config\profiles\gaming.toml config\profiles\custom.toml
```

Then edit `custom.toml` and use:

```powershell
pieuvre sync --profile custom
```
