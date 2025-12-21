//! Commande rollback

use anyhow::Result;

pub fn run(list: bool, last: bool, id: Option<String>) -> Result<()> {
    let snapshots = pieuvre_persist::list_snapshots()?;
    
    if list {
        if snapshots.is_empty() {
            println!("Aucun snapshot disponible");
        } else {
            println!("Snapshots disponibles:\n");
            for s in &snapshots {
                println!("  {} - {} ({} changements)", 
                    &s.id.to_string()[..8],
                    s.timestamp.format("%Y-%m-%d %H:%M"),
                    s.changes.len()
                );
                println!("    {}", s.description);
            }
        }
        return Ok(());
    }
    
    if last {
        if let Some(snapshot) = snapshots.first() {
            println!("Restauration du dernier snapshot: {}", &snapshot.id.to_string()[..8]);
            pieuvre_persist::restore_snapshot(&snapshot.id.to_string())?;
            println!("Restauration terminée");
        } else {
            println!("Aucun snapshot disponible");
        }
        return Ok(());
    }
    
    if let Some(snapshot_id) = id {
        println!("Restauration snapshot: {}", snapshot_id);
        pieuvre_persist::restore_snapshot(&snapshot_id)?;
        println!("Restauration terminée");
        return Ok(());
    }
    
    println!("Usage: pieuvre rollback --list | --last | --id <ID>");
    Ok(())
}
