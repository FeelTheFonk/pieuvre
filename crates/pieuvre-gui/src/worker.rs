//! Worker thread pour operations lourdes
//!
//! Execution des taches longues en arriere-plan.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// Messages envoyes au worker
#[derive(Debug)]
pub enum WorkerCommand {
    RunAudit { full: bool },
    ApplyOptimizations { dry_run: bool },
    LoadProfile { name: String },
    RollbackSnapshot { id: String },
    CreateSnapshot,
    Shutdown,
}

/// Messages de retour du worker
#[derive(Debug)]
pub enum WorkerResult {
    AuditComplete { success: bool, message: String, services_count: usize },
    OptimizationsApplied { success: bool, message: String },
    ProfileLoaded { success: bool, message: String },
    RollbackComplete { success: bool, message: String },
    SnapshotCreated { success: bool, id: String },
    Error { message: String },
}

/// Handle pour communiquer avec le worker
pub struct WorkerHandle {
    sender: Sender<WorkerCommand>,
    receiver: Receiver<WorkerResult>,
}

impl WorkerHandle {
    /// Cree un nouveau worker thread
    pub fn spawn() -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<WorkerCommand>();
        let (res_tx, res_rx) = mpsc::channel::<WorkerResult>();

        thread::spawn(move || {
            worker_loop(cmd_rx, res_tx);
        });

        Self {
            sender: cmd_tx,
            receiver: res_rx,
        }
    }

    /// Envoie une commande au worker
    pub fn send(&self, cmd: WorkerCommand) -> Result<(), mpsc::SendError<WorkerCommand>> {
        self.sender.send(cmd)
    }

    /// Tente de recevoir un resultat (non-bloquant)
    pub fn try_recv(&self) -> Option<WorkerResult> {
        self.receiver.try_recv().ok()
    }

    /// Recoit un resultat (bloquant avec timeout)
    pub fn recv_timeout(&self, timeout: std::time::Duration) -> Option<WorkerResult> {
        self.receiver.recv_timeout(timeout).ok()
    }
}

impl Drop for WorkerHandle {
    fn drop(&mut self) {
        let _ = self.sender.send(WorkerCommand::Shutdown);
    }
}

/// Boucle principale du worker
fn worker_loop(rx: Receiver<WorkerCommand>, tx: Sender<WorkerResult>) {
    tracing::info!("Worker thread demarre");

    while let Ok(cmd) = rx.recv() {
        match cmd {
            WorkerCommand::RunAudit { full: _ } => {
                tracing::info!("Worker: execution audit");
                
                match pieuvre_audit::full_audit() {
                    Ok(report) => {
                        let services_count = report.services.len();
                        tracing::info!("Audit termine: {} services detectes", services_count);
                        let _ = tx.send(WorkerResult::AuditComplete {
                            success: true,
                            message: format!("Audit termine: {} services", services_count),
                            services_count,
                        });
                    }
                    Err(e) => {
                        tracing::error!("Erreur audit: {}", e);
                        let _ = tx.send(WorkerResult::Error {
                            message: format!("Erreur audit: {}", e),
                        });
                    }
                }
            }
            
            WorkerCommand::ApplyOptimizations { dry_run } => {
                tracing::info!("Worker: application optimisations (dry_run={})", dry_run);
                
                if dry_run {
                    let _ = tx.send(WorkerResult::OptimizationsApplied {
                        success: true,
                        message: "Dry run: aucune modification".into(),
                    });
                } else {
                    // Creation snapshot avant modification
                    match pieuvre_persist::snapshot::create("Pre-optimization", vec![]) {
                        Ok(snapshot) => {
                            tracing::info!("Snapshot cree: {}", snapshot.id);
                            
                            // Application profil par defaut (workstation)
                            match pieuvre_sync::apply_profile("workstation", false) {
                                Ok(_) => {
                                    let _ = tx.send(WorkerResult::OptimizationsApplied {
                                        success: true,
                                        message: format!("Optimisations appliquees (snapshot: {})", &snapshot.id.to_string()[..8]),
                                    });
                                }
                                Err(e) => {
                                    let _ = tx.send(WorkerResult::Error {
                                        message: format!("Erreur application: {}", e),
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(WorkerResult::Error {
                                message: format!("Erreur snapshot: {}", e),
                            });
                        }
                    }
                }
            }
            
            WorkerCommand::LoadProfile { name } => {
                tracing::info!("Worker: chargement profil {}", name);
                
                let profile_path = format!("config/profiles/{}.toml", name);
                if std::path::Path::new(&profile_path).exists() {
                    let _ = tx.send(WorkerResult::ProfileLoaded {
                        success: true,
                        message: format!("Profil {} charge", name),
                    });
                } else {
                    let _ = tx.send(WorkerResult::ProfileLoaded {
                        success: false,
                        message: format!("Profil {} introuvable", name),
                    });
                }
            }
            
            WorkerCommand::RollbackSnapshot { id } => {
                tracing::info!("Worker: rollback snapshot {}", id);
                
                match pieuvre_persist::snapshot::restore(&id) {
                    Ok(_) => {
                        let _ = tx.send(WorkerResult::RollbackComplete {
                            success: true,
                            message: format!("Snapshot {} restaure", &id[..8.min(id.len())]),
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(WorkerResult::Error {
                            message: format!("Erreur rollback: {}", e),
                        });
                    }
                }
            }
            
            WorkerCommand::CreateSnapshot => {
                tracing::info!("Worker: creation snapshot");
                
                match pieuvre_persist::snapshot::create("Manual snapshot", vec![]) {
                    Ok(snapshot) => {
                        let _ = tx.send(WorkerResult::SnapshotCreated {
                            success: true,
                            id: snapshot.id.to_string(),
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(WorkerResult::Error {
                            message: format!("Erreur creation snapshot: {}", e),
                        });
                    }
                }
            }
            
            WorkerCommand::Shutdown => {
                tracing::info!("Worker: arret");
                break;
            }
        }
    }

    tracing::info!("Worker thread termine");
}
