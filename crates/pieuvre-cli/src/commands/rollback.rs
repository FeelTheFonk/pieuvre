//! Commande rollback

use pieuvre_common::Result;

pub fn run(list: bool, last: bool, id: Option<String>) -> Result<()> {
    let snapshots = pieuvre_persist::list_snapshots()?;

    if list {
        if snapshots.is_empty() {
            println!("No snapshots available");
        } else {
            println!("Available snapshots:\n");
            for s in &snapshots {
                println!(
                    "  {} - {} ({} changes)",
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
            println!("Restoring last snapshot: {}", &snapshot.id.to_string()[..8]);
            pieuvre_persist::restore_snapshot(&snapshot.id.to_string())?;
            println!("Restoration completed");
        } else {
            println!("No snapshots available");
        }
        return Ok(());
    }

    if let Some(snapshot_id) = id {
        println!("Restoring snapshot: {}", snapshot_id);
        pieuvre_persist::restore_snapshot(&snapshot_id)?;
        println!("Restoration completed");
        return Ok(());
    }

    println!("Usage: pieuvre rollback --list | --last | --id <ID>");
    Ok(())
}
