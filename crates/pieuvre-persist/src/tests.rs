#[cfg(test)]
mod tests {
    use crate::snapshot;
    use pieuvre_common::ChangeRecord;

    #[test]
    fn test_snapshot_lifecycle() {
        let description = "Test SOTA Snapshot";
        let changes = vec![ChangeRecord::Registry {
            key: "HKLM\\SOFTWARE\\PieuvreTest".to_string(),
            value_name: "TestValue".to_string(),
            value_type: "REG_DWORD".to_string(),
            original_data: vec![1, 0, 0, 0],
        }];

        // 1. Création
        let snap =
            snapshot::create(description, changes.clone()).expect("Failed to create snapshot");
        assert_eq!(snap.description, description);
        assert_eq!(snap.changes.len(), 1);

        // 2. Liste
        let list = snapshot::list_all().expect("Failed to list snapshots");
        assert!(list.iter().any(|s| s.id == snap.id));

        // 3. Chargement
        let loaded = snapshot::load(&snap.id.to_string()).expect("Failed to load snapshot");
        assert_eq!(loaded.id, snap.id);
        assert_eq!(loaded.description, description);

        // 4. Suppression (Nettoyage test)
        // Note: On pourrait ajouter une fonction de suppression si nécessaire pour le SOTA
    }
}
