# pieuvre-persist

Snapshot and rollback management engine with **zstd compression** and **SHA256 integrity**.

---

## Features

- Automatic snapshot creation before modifications
- **zstd compression** (3-10x ratio)
- **SHA256 checksums** for integrity validation
- Rollback to any previous state
- Change record tracking with timestamps
- JSON export for external analysis

---

## Snapshot Format

```json
{
  "id": "7be4b13b-051a-4cb2-afb2-257c7a3aff2c",
  "timestamp": "2025-12-22T07:30:00Z",
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

## API

### Create Snapshot

```rust
use pieuvre_persist::snapshot;

let changes = vec![
    ChangeRecord::service("DiagTrack", "Automatic", "Disabled"),
];

let snapshot = snapshot::create("Gaming profile", changes)?;
println!("Snapshot ID: {}", snapshot.id);
```

### List Snapshots

```rust
use pieuvre_persist::snapshot;

let snapshots = snapshot::list_snapshots()?;

for s in snapshots {
    println!("{}: {} ({})", s.id, s.description, s.timestamp);
}
```

### Rollback

```rust
use pieuvre_persist::snapshot;

// Rollback to specific snapshot
snapshot::restore(&snapshot_id)?;

// Rollback to most recent
let latest = snapshot::get_latest()?;
snapshot::restore(&latest.id)?;
```

---

## Storage Location

```
C:\ProgramData\Pieuvre\snapshots\
├── 7be4b13b-051a-4cb2-afb2-257c7a3aff2c.json.zst
├── a1b2c3d4-e5f6-7890-abcd-ef1234567890.json.zst
└── ...
```

---

## Rollback Capabilities

| Type | Rollback Support |
|------|------------------|
| Service state | [OK] Full |
| Registry values | [OK] Full |
| Firewall rules | [OK] Full |
| Hosts file | [OK] Full |
| AppX packages | [ERR] Cannot reinstall |
| OneDrive | [WARN] Manual reinstall |
