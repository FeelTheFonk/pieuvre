# pieuvre-persist

The persistence and recovery engine for pieuvre, featuring high-ratio compression and cryptographic integrity verification.

---

## Features

- **Atomic Snapshots**: Captures system state (Registry, Services, Firewall) before any modification.
- **Zstd Compression**: Achieves 3-10x compression ratios for efficient snapshot storage.
- **SHA256 Integrity**: Validates snapshot consistency to prevent corruption or tampering.
- **Granular Rollback**: Restore the entire system or specific components to a previous state.
- **Change Tracking**: Detailed logging of every modification with precise timestamps.

---

## Snapshot Schema

Snapshots are stored as compressed JSON files with the following structure:

```json
{
  "id": "7be4b13b-051a-4cb2-afb2-257c7a3aff2c",
  "timestamp": "2025-12-23T19:30:00Z",
  "description": "Gaming profile applied",
  "checksum": "sha256:...",
  "changes": [
    {
      "type": "service",
      "name": "DiagTrack",
      "original_value": "Automatic",
      "new_value": "Disabled"
    }
  ]
}
```

---

## API Usage

### Creating a Snapshot

```rust
use pieuvre_persist::snapshot;

let changes = vec![
    ChangeRecord::service("DiagTrack", "Automatic", "Disabled"),
];

let snapshot = snapshot::create("Optimization Profile", changes)?;
println!("Snapshot ID: {}", snapshot.id);
```

### Performing a Rollback

```rust
use pieuvre_persist::snapshot;

// Restore the most recent snapshot
let latest = snapshot::get_latest()?;
snapshot::restore(&latest.id)?;

// Restore a specific snapshot by ID
snapshot::restore("7be4b13b-051a-4cb2-afb2-257c7a3aff2c")?;
```

---

## Storage & Persistence

Snapshots are persisted in the following system directory:

`C:\ProgramData\pieuvre\snapshots\`

Files are named using their unique UUID and appended with the `.json.zst` extension.

---

## Recovery Capabilities

| Component | Rollback Support | Status |
|:---|:---|:---|
| **Services** | Full | Supported |
| **Registry** | Full | Supported |
| **Firewall** | Full | Supported |
| **Hosts File** | Full | Supported |
| **AppX Packages** | Limited | Reinstallation required via Store. |
| **OneDrive** | Manual | Requires manual installer execution. |
