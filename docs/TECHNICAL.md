# Technical Documentation

Detailed technical documentation for Pieuvre's Windows integration layer.

---

## Timer Resolution

### Implementation

Uses `NtSetTimerResolution` from ntdll.dll for sub-millisecond timer control:

```rust
NtSetTimerResolution(5000, TRUE, &actual);  // 0.5ms
```

### Windows Behavior

- Default resolution: 15.6ms
- Minimum achievable: 0.5ms
- Global effect: affects all processes
- Power impact: increased CPU wake frequency

### References

- [Microsoft Timer Resolution](https://docs.microsoft.com/en-us/windows/win32/api/timeapi/)
- [Bruce Dawson Analysis](https://randomascii.wordpress.com/2020/10/04/windows-timer-resolution-the-great-rule-change/)

---

## Power Plans

### GUID Mapping

| Plan | GUID |
|------|------|
| Balanced | `381b4222-f694-41f0-9685-ff5bb260df2e` |
| High Performance | `8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c` |
| Ultimate Performance | `e9a42b02-d5df-448d-aa00-03f14749eb61` |

### Activation

```powershell
powercfg /setactive e9a42b02-d5df-448d-aa00-03f14749eb61
```

### Ultimate Performance

Hidden by default. Must be enabled via:

```powershell
powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61
```

---

## Service Control Manager

### API Usage

```rust
OpenSCManagerW(NULL, NULL, SC_MANAGER_ALL_ACCESS)
OpenServiceW(hSCManager, service_name, SERVICE_CHANGE_CONFIG)
ChangeServiceConfigW(hService, SERVICE_NO_CHANGE, SERVICE_DISABLED, ...)
```

### Target Services

| Service | Purpose | Safe to Disable |
|---------|---------|-----------------|
| DiagTrack | Telemetry | Yes |
| dmwappushservice | Push notifications | Yes |
| SysMain | Superfetch | Yes (SSD) |
| WSearch | Windows Search | Conditional |

---

## Windows Firewall

### Rule Injection

```powershell
netsh advfirewall firewall add rule name="Pieuvre-Block-Telemetry" dir=out action=block remoteip=X.X.X.X
```

### Blocked Domains (47)

See [telemetry-domains.txt](../config/telemetry-domains.txt)

### Blocked IP Ranges (17)

Microsoft telemetry endpoints including:
- `13.64.0.0/11` - Azure
- `20.33.0.0/16` - Microsoft
- `52.96.0.0/12` - Office 365

---

## MSI Mode

### Detection

Registry path:
```
HKLM\SYSTEM\CurrentControlSet\Enum\PCI\*\Device Parameters\Interrupt Management\MessageSignaledInterruptProperties
```

### Enabling

Set `MSISupported` DWORD to `1` for:
- GPU devices
- NVMe controllers
- Network adapters

### Benefits

- Reduced interrupt latency
- Better multi-core scaling
- Required for optimal gaming performance

---

## Registry Operations

### Atomic Writes

All registry modifications use transactional API when available:

```rust
RegCreateKeyExW(..., REG_OPTION_NON_VOLATILE, ...)
RegSetValueExW(hKey, value_name, 0, REG_DWORD, &data, size)
RegCloseKey(hKey)
```

### Backup Strategy

Original values captured in snapshot before any modification.

---

## Scheduled Tasks

### Disabled Tasks (25)

Categories:
- Microsoft Compatibility Appraiser
- Customer Experience Improvement Program (CEIP)
- Disk Diagnostics
- Family Safety
- Feedback
- Maps telemetry
- Office telemetry

### Implementation

```powershell
schtasks /Change /TN "TaskPath" /Disable
```

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
