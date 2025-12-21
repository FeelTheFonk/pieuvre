//! Pieuvre GUI - Point d'entree principal

slint::include_modules!();

use anyhow::Result;
use std::sync::{Arc, Mutex};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use pieuvre_gui::{WorkerHandle, WorkerCommand, WorkerResult};

// Les globals Slint sont generes par slint::include_modules!()
// SystemInfo et AppState sont disponibles directement

fn main() -> Result<()> {
    // Configuration logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("info"))
        .init();

    tracing::info!("Demarrage Pieuvre GUI");

    // Creation fenetre principale
    let app = MainWindow::new()?;
    
    // Worker thread pour operations async
    let worker = WorkerHandle::spawn();
    let worker = Arc::new(Mutex::new(worker));

    // Initialisation etat systeme
    setup_system_info(&app);

    // Configuration callbacks
    setup_callbacks(&app, worker.clone());

    // Timer pour poll worker results
    setup_worker_poll(&app, worker);

    // Lancement event loop
    app.run()?;

    Ok(())
}

/// Configure les informations systeme au demarrage
fn setup_system_info(app: &MainWindow) {
    let info = pieuvre_gui::get_system_info();
    
    tracing::info!("Systeme: {} {} ({})", info.os_version, info.build_number, info.hostname);
    tracing::info!("CPU: {} ({} cores)", info.cpu_name, info.cpu_cores);
    tracing::info!("RAM: {} GB, GPU: {}", info.ram_gb, info.gpu_name);
    tracing::info!("Type: {}", if info.is_laptop { "Laptop" } else { "Desktop" });
    
    // Mise a jour proprietes systeme sur MainWindow
    app.set_sys_os_version(info.os_version.into());
    app.set_sys_build_number(info.build_number.into());
    app.set_sys_hostname(info.hostname.into());
    app.set_sys_cpu_name(info.cpu_name.into());
    app.set_sys_cpu_cores(info.cpu_cores);
    app.set_sys_ram_gb(info.ram_gb);
    app.set_sys_gpu_name(info.gpu_name.into());
    app.set_sys_is_laptop(info.is_laptop);
}

/// Configure tous les callbacks UI -> Rust
fn setup_callbacks(app: &MainWindow, worker: Arc<Mutex<WorkerHandle>>) {
    // Callback: Run Audit
    {
        let worker = worker.clone();
        let app_weak = app.as_weak();
        app.on_run_audit(move || {
            tracing::info!("Callback: run-audit");
            if let Ok(w) = worker.lock() {
                let _ = w.send(WorkerCommand::RunAudit { full: true });
            }
            if let Some(app) = app_weak.upgrade() {
                app.set_is_auditing(true);
                app.set_audit_progress(0.0);
            }
        });
    }

    // Callback: Apply Optimizations
    {
        let worker = worker.clone();
        let app_weak = app.as_weak();
        app.on_apply_optimizations(move |dry_run| {
            tracing::info!("Callback: apply-optimizations (dry_run={})", dry_run);
            if let Ok(w) = worker.lock() {
                let _ = w.send(WorkerCommand::ApplyOptimizations { dry_run });
            }
            if let Some(app) = app_weak.upgrade() {
                app.set_is_applying(true);
            }
        });
    }

    // Callback: Load Profile
    {
        let worker = worker.clone();
        app.on_load_profile(move |name| {
            tracing::info!("Callback: load-profile ({})", name);
            if let Ok(w) = worker.lock() {
                let _ = w.send(WorkerCommand::LoadProfile { 
                    name: name.to_string() 
                });
            }
        });
    }

    // Callback: Restore Snapshot
    {
        let worker = worker.clone();
        app.on_restore_snapshot(move |id| {
            tracing::info!("Callback: restore-snapshot ({})", id);
            if let Ok(w) = worker.lock() {
                let _ = w.send(WorkerCommand::RollbackSnapshot { 
                    id: id.to_string() 
                });
            }
        });
    }

    // Callback: Create Snapshot
    {
        let worker = worker.clone();
        app.on_create_snapshot(move || {
            tracing::info!("Callback: create-snapshot");
            if let Ok(w) = worker.lock() {
                let _ = w.send(WorkerCommand::CreateSnapshot);
            }
        });
    }

    // Callback: Save Settings
    {
        app.on_save_settings(move || {
            tracing::info!("Callback: save-settings");
            if let Err(e) = pieuvre_gui::handle_save_settings() {
                tracing::error!("Erreur sauvegarde: {}", e);
            }
        });
    }
}

/// Configure le timer pour poll les resultats du worker
fn setup_worker_poll(app: &MainWindow, worker: Arc<Mutex<WorkerHandle>>) {
    let app_weak = app.as_weak();
    
    // Timer 100ms pour poll worker
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(100),
        move || {
            if let Ok(w) = worker.lock() {
                while let Some(result) = w.try_recv() {
                    if let Some(app) = app_weak.upgrade() {
                        handle_worker_result(&app, result);
                    }
                }
            }
        },
    );
    
    // Keep timer alive
    std::mem::forget(timer);
}

/// Traite les resultats du worker
fn handle_worker_result(app: &MainWindow, result: WorkerResult) {
    match result {
        WorkerResult::AuditComplete { success, message, services_count } => {
            tracing::info!("Audit complete: {} ({} services)", message, services_count);
            app.set_is_auditing(false);
            app.set_audit_progress(1.0);
            show_toast(app, &message, success);
        }
        
        WorkerResult::OptimizationsApplied { success, message } => {
            tracing::info!("Optimizations: {}", message);
            app.set_is_applying(false);
            show_toast(app, &message, success);
        }
        
        WorkerResult::ProfileLoaded { success, message } => {
            tracing::info!("Profile: {}", message);
            show_toast(app, &message, success);
        }
        
        WorkerResult::RollbackComplete { success, message } => {
            tracing::info!("Rollback: {}", message);
            show_toast(app, &message, success);
        }
        
        WorkerResult::SnapshotCreated { success, id } => {
            tracing::info!("Snapshot cree: {}", id);
            let msg = format!("Snapshot cree: {}", &id[..8.min(id.len())]);
            show_toast(app, &msg, success);
            app.set_snapshot_count(app.get_snapshot_count() + 1);
        }
        
        WorkerResult::Error { message } => {
            tracing::error!("Erreur worker: {}", message);
            show_toast(app, &message, false);
        }
    }
}

/// Affiche un toast notification
fn show_toast(app: &MainWindow, message: &str, _success: bool) {
    app.set_toast_message(message.into());
    app.set_toast_visible(true);
    
    // Auto-hide apres 3 secondes
    let app_weak = app.as_weak();
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::SingleShot,
        std::time::Duration::from_secs(3),
        move || {
            if let Some(app) = app_weak.upgrade() {
                app.set_toast_visible(false);
            }
        },
    );
    std::mem::forget(timer);
}
