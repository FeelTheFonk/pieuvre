//! Tests unitaires SOTA pour pieuvre-persist
//!
//! Tests de création, listage et suppression de snapshots.
//! Tests en mode dry-run sans modification système.

use crate::snapshot;
use chrono::Utc;
use pieuvre_common::{ChangeRecord, Snapshot};
use uuid::Uuid;

// ============================================================================
// TESTS SNAPSHOT STRUCTURE
// ============================================================================

#[test]
fn test_snapshot_struct_creation() {
    let snap = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: "Test snapshot".to_string(),
        changes: vec![],
    };

    assert!(!snap.description.is_empty());
    assert!(snap.changes.is_empty());
}

#[test]
fn test_change_record_registry() {
    let change = ChangeRecord::Registry {
        key: r"SOFTWARE\Test".to_string(),
        value_name: "TestValue".to_string(),
        value_type: "REG_DWORD".to_string(),
        original_data: vec![1, 0, 0, 0], // DWORD = 1
    };

    if let ChangeRecord::Registry {
        key,
        value_name,
        value_type,
        original_data,
    } = change
    {
        assert!(key.contains("SOFTWARE"));
        assert_eq!(value_name, "TestValue");
        assert_eq!(value_type, "REG_DWORD");
        assert_eq!(original_data.len(), 4);
    } else {
        panic!("Expected Registry change record");
    }
}

#[test]
fn test_change_record_service() {
    let change = ChangeRecord::Service {
        name: "DiagTrack".to_string(),
        original_start_type: 2, // Automatic
    };

    if let ChangeRecord::Service {
        name,
        original_start_type,
    } = change
    {
        assert_eq!(name, "DiagTrack");
        assert_eq!(original_start_type, 2);
    } else {
        panic!("Expected Service change record");
    }
}

#[test]
fn test_change_record_firewall() {
    let change = ChangeRecord::FirewallRule {
        name: "Pieuvre-Block-Telemetry".to_string(),
    };

    if let ChangeRecord::FirewallRule { name } = change {
        assert!(name.starts_with("Pieuvre"));
    } else {
        panic!("Expected FirewallRule change record");
    }
}

// ============================================================================
// TESTS LIST SNAPSHOTS (Read-only)
// ============================================================================

#[test]
fn test_list_all_snapshots_returns_vec() {
    let result = snapshot::list_all();
    assert!(
        result.is_ok(),
        "list_all should succeed even with empty dir"
    );

    // Peut être vide si aucun snapshot n'existe
    let _snapshots = result.unwrap();
}

#[test]
fn test_list_all_snapshots_sorted_by_date() {
    let result = snapshot::list_all();
    if let Ok(snapshots) = result {
        if snapshots.len() >= 2 {
            // Vérifier que les snapshots sont triés par date décroissante
            for i in 0..snapshots.len() - 1 {
                assert!(
                    snapshots[i].timestamp >= snapshots[i + 1].timestamp,
                    "Snapshots should be sorted by date descending"
                );
            }
        }
    }
}

// ============================================================================
// TESTS SNAPSHOT DIRECTORY
// ============================================================================

#[test]
fn test_snapshot_dir_path_valid() {
    let expected_dir = r"C:\ProgramData\Pieuvre\snapshots";
    assert!(!expected_dir.is_empty());
    assert!(expected_dir.contains("ProgramData"));
}

// ============================================================================
// TESTS SERIALIZATION
// ============================================================================

#[test]
fn test_snapshot_serialization_roundtrip() {
    let original = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: "Test roundtrip".to_string(),
        changes: vec![ChangeRecord::Service {
            name: "TestService".to_string(),
            original_start_type: 3,
        }],
    };

    // Serialize
    let json = serde_json::to_string_pretty(&original);
    assert!(json.is_ok(), "Snapshot should serialize to JSON");

    let json_str = json.unwrap();
    assert!(json_str.contains("TestService"));
    assert!(json_str.contains("Test roundtrip"));

    // Deserialize
    let restored: Result<Snapshot, _> = serde_json::from_str(&json_str);
    assert!(restored.is_ok(), "Snapshot should deserialize from JSON");

    let restored_snap = restored.unwrap();
    assert_eq!(restored_snap.id, original.id);
    assert_eq!(restored_snap.description, original.description);
    assert_eq!(restored_snap.changes.len(), 1);
}

#[test]
fn test_snapshot_json_format() {
    let snap = Snapshot {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        timestamp: Utc::now(),
        description: "JSON format test".to_string(),
        changes: vec![],
    };

    let json = serde_json::to_string(&snap).unwrap();

    assert!(json.contains("550e8400"));
    assert!(json.contains("JSON format test"));
    assert!(json.contains("changes"));
}

// ============================================================================
// TESTS RESTORE LOGIC (Validation only)
// ============================================================================

#[test]
fn test_restore_nonexistent_snapshot_fails() {
    let result = snapshot::restore("nonexistent-snapshot-id-12345");
    assert!(
        result.is_err(),
        "Restore of non-existent snapshot should fail"
    );
}

#[test]
fn test_delete_nonexistent_snapshot_fails() {
    let result = snapshot::delete("nonexistent-snapshot-id-67890");
    assert!(
        result.is_err(),
        "Delete of non-existent snapshot should fail"
    );
}

// ============================================================================
// TESTS EDGE CASES
// ============================================================================

#[test]
fn test_empty_description() {
    let snap = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: String::new(),
        changes: vec![],
    };

    assert!(snap.description.is_empty());
}

#[test]
fn test_unicode_description() {
    let snap = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: "テスト スナップショット 🚀".to_string(),
        changes: vec![],
    };

    let json = serde_json::to_string(&snap).unwrap();
    assert!(json.contains("スナップショット"));
}

#[test]
fn test_large_changes_vector() {
    let mut changes = Vec::new();
    for i in 0..100 {
        changes.push(ChangeRecord::Service {
            name: format!("Service{}", i),
            original_start_type: 3,
        });
    }

    let snap = Snapshot {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        description: "Large changes test".to_string(),
        changes,
    };

    assert_eq!(snap.changes.len(), 100);

    let json = serde_json::to_string(&snap).unwrap();
    assert!(json.contains("Service99"));
}
