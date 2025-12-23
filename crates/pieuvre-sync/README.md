# pieuvre-sync

The core execution engine of pieuvre, containing over 24 specialized modules for system optimization and hardening.

---

## Optimization Modules

| Module | Description |
|:---|:---|
| **Services** | Granular control over service states and start types. |
| **Timer** | High-precision timer resolution forcing (0.5ms). |
| **Power** | Advanced power plan configuration and energy throttling control. |
| **Firewall** | Native rule injection for telemetry domain blocking. |
| **MSI** | Migration of PCI devices to Message Signaled Interrupts. |
| **Registry** | Atomic writes with security descriptor (SDDL) locking. |
| **AppX** | Clean removal of pre-installed Windows applications. |
| **Hardening** | System protection via ACL locking and IFEO protection. |
| **Network** | TCP stack tuning, Nagle algorithm, and interrupt moderation. |
| **Security** | VBS, HVCI, and Memory Integrity management. |
| **DPC/ISR** | Latency reduction via paging control and TSC synchronization. |
| **Sentinel** | Background monitoring and automatic restoration of critical settings. |

---

## API Usage

### Security & Performance Hardening

```rust
use pieuvre_sync::security;

// Disable Memory Integrity (HVCI) for significant FPS gains
security::disable_memory_integrity()?;

// Disable Virtualization-Based Security (VBS)
security::disable_vbs()?;
```

### Latency Optimization

```rust
use pieuvre_sync::dpc;

// Prevent kernel paging to disk
dpc::disable_paging_executive()?;

// Stabilize system timer by disabling dynamic tick
dpc::disable_dynamic_tick()?;

// Distribute hardware interrupts across cores
dpc::set_interrupt_affinity_spread()?;
```

### Network Tuning

```rust
use pieuvre_sync::network;

// Apply all gaming-focused network optimizations
network::apply_all_network_optimizations()?;

// Individual controls
network::disable_nagle_algorithm()?;
network::disable_interrupt_moderation()?;
```

---

## Safety & Rollback

Every modification performed by `pieuvre-sync` follows a strict safety protocol:

1. **State Capture**: The original value is read and stored.
2. **Snapshot Recording**: A `ChangeRecord` is generated and sent to the persistence engine.
3. **Atomic Application**: The change is applied using native APIs to ensure consistency.

### Risk Assessment

| Feature | Risk Level | Impact |
|:---|:---|:---|
| `disable_memory_integrity()` | Medium | Reduced kernel-level protection. |
| `disable_vbs()` | High | Disables Credential Guard. |
| `disable_spectre_meltdown()` | Critical | Exposes CPU to known vulnerabilities. |

---

## Build Instructions

```powershell
cargo build -p pieuvre-sync --release
```
