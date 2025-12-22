# Technical Documentation

Detailed technical documentation for pieuvre's Windows integration layer.

---

## Timer Resolution

### Implementation

Uses `NtSetTimerResolution` from `ntdll.dll` for sub-millisecond timer control:

```rust
NtSetTimerResolution(5000, TRUE, &actual);  // 0.5ms (5000 * 100ns)
```

### Windows Behavior

- Default resolution: 15.6ms
- Minimum achievable: 0.5ms
- Global effect: affects all processes
- Power impact: increased CPU wake frequency

---

## Power Plans

### Native API Usage

pieuvre bypasses `powercfg.exe` and uses `PowrProf.dll` directly:

```rust
PowerGetActiveScheme(None, &mut scheme_guid);
PowerSetActiveScheme(None, Some(&target_guid));
```

### GUID Mapping

| Plan | GUID |
|------|------|
| Balanced | `381b4222-f694-41f0-9685-ff5bb260df2e` |
| High Performance | `8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c` |
| Ultimate Performance | `e9a42b02-d5df-448d-aa00-03f14749eb61` |

---

## Service Control Manager

### Native API Usage

```rust
OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS);
OpenServiceW(hSCManager, service_name, SERVICE_CHANGE_CONFIG);
ChangeServiceConfigW(hService, SERVICE_NO_CHANGE, SERVICE_DISABLED, ...);
```

### Target Services

| Service | Purpose | Safe to Disable |
|---------|---------|-----------------|
| DiagTrack | Telemetry | Yes |
| dmwappushservice | Push notifications | Yes |
| SysMain | Superfetch | Yes (SSD) |
| WSearch | Windows Search | Conditional |

---

## Sentinel Engine (Self-Healing)

### Implementation

Uses `RegNotifyChangeKeyValue` for event-driven monitoring of critical registry keys.

- **Mode**: Event-driven (0% CPU idle)
- **Reaction**: Instantaneous restoration upon modification
- **Scope**: IFEO, AppInit_DLLs, Winlogon, ShellServiceObjectDelayLoad

---

## ETW Engine (Latency Monitoring)

### Implementation

Uses `EventTrace` APIs to capture kernel events in real-time.

- **DriverResolver**: Maps kernel routine addresses to `.sys` filenames using `EnumDeviceDrivers`.
- **Metrics**: Captures DPC/ISR duration per driver.
- **Feedback Loop**: Automatically adjusts interrupt affinity for high-latency drivers via `interrupts.rs`.
- **DriverResolver**: Implementation using `EnumDeviceDrivers` to map kernel addresses to real driver names.

---

## Registry Operations

### Atomic Writes

All registry modifications use native APIs:

```rust
RegCreateKeyExW(..., REG_OPTION_NON_VOLATILE, ...);
RegSetValueExW(hKey, value_name, 0, REG_DWORD, Some(&data));
```

### Backup Strategy

Original values captured in `zstd` compressed snapshots with SHA256 integrity checks.

---

## References

### Microsoft Documentation

- [Windows Internals (Sysinternals)](https://docs.microsoft.com/en-us/sysinternals/)
- [Service Control Manager](https://docs.microsoft.com/en-us/windows/win32/services/service-control-manager)
- [Registry Functions](https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-functions)

### Community Research

- [Bruce Dawson - Timer Resolution](https://randomascii.wordpress.com/2020/10/04/windows-timer-resolution-the-great-rule-change/)
- [Sophia Script](https://github.com/farag2/Sophia-Script-for-Windows)
- [privacy.sexy](https://github.com/undergroundwires/privacy.sexy)
