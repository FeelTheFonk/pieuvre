//! Worker thread pour opérations lourdes
//!
//! Exécution des tâches longues en arrière-plan.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

/// Messages envoyés au worker
#[derive(Debug)]
pub enum WorkerCommand {
    RunAudit { full: bool },
    ApplyOptimizations { dry_run: bool },
    LoadProfile { name: String },
    RollbackSnapshot { id: String },
    Shutdown,
}

/// Messages de retour du worker
#[derive(Debug)]
pub enum WorkerResult {
    AuditComplete { success: bool, message: String },
    OptimizationsApplied { success: bool, message: String },
    ProfileLoaded { success: bool, message: String },
    RollbackComplete { success: bool, message: String },
    Error { message: String },
}

/// Handle pour communiquer avec le worker
pub struct WorkerHandle {
    sender: Sender<WorkerCommand>,
    receiver: Receiver<WorkerResult>,
}

impl WorkerHandle {
    /// Crée un nouveau worker thread
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

    /// Tente de recevoir un résultat (non-bloquant)
    pub fn try_recv(&self) -> Option<WorkerResult> {
        self.receiver.try_recv().ok()
    }
}

/// Boucle principale du worker
fn worker_loop(rx: Receiver<WorkerCommand>, tx: Sender<WorkerResult>) {
    tracing::info!("Worker thread démarré");

    while let Ok(cmd) = rx.recv() {
        match cmd {
            WorkerCommand::RunAudit { full } => {
                tracing::info!("Worker: exécution audit (full={})", full);
                // TODO: Appeler pieuvre_audit::run()
                let _ = tx.send(WorkerResult::AuditComplete {
                    success: true,
                    message: "Audit terminé".into(),
                });
            }
            WorkerCommand::ApplyOptimizations { dry_run } => {
                tracing::info!("Worker: application optimisations (dry_run={})", dry_run);
                // TODO: Appeler pieuvre_sync::apply()
                let _ = tx.send(WorkerResult::OptimizationsApplied {
                    success: true,
                    message: "Optimisations appliquées".into(),
                });
            }
            WorkerCommand::LoadProfile { name } => {
                tracing::info!("Worker: chargement profil {}", name);
                // TODO: Charger profil
                let _ = tx.send(WorkerResult::ProfileLoaded {
                    success: true,
                    message: format!("Profil {} chargé", name),
                });
            }
            WorkerCommand::RollbackSnapshot { id } => {
                tracing::info!("Worker: rollback snapshot {}", id);
                // TODO: Appeler pieuvre_persist::rollback()
                let _ = tx.send(WorkerResult::RollbackComplete {
                    success: true,
                    message: "Rollback effectué".into(),
                });
            }
            WorkerCommand::Shutdown => {
                tracing::info!("Worker: arrêt");
                break;
            }
        }
    }

    tracing::info!("Worker thread terminé");
}
