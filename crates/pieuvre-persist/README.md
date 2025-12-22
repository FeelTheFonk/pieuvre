# pieuvre-persist

Snapshot and rollback management engine.

---

## Features

- Automatic snapshot creation before modifications
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
  "changes": [
    {
      "type": "service",
      "name": "DiagTrack",
      "original_value": "Automatic",
      "new_value": "Disabled"
    },
    {
      "type": "registry",
      "path": "HKLM\\SYSTEM\\...",
      "original_value": "0",
      "new_value": "1"
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
    ChangeRecord::registry(path, "0", "1"),
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
├── 7be4b13b-051a-4cb2-afb2-257c7a3aff2c.json
├── a1b2c3d4-e5f6-7890-abcd-ef1234567890.json
└── ...
```

---

## Rollback Capabilities

| Type | Rollback Support |
|------|------------------|
| Service state | ✅ Full |
| Registry values | ✅ Full |
| Firewall rules | ✅ Full |
| Hosts file | ✅ Full |
| AppX packages | ❌ Cannot reinstall |
| OneDrive | ⚠️ Manual reinstall |
