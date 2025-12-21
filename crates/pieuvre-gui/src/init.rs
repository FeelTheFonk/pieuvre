//! Initialisation de l'etat UI
//!
//! Setup initial au demarrage de l'application.

use crate::models::SystemInfoUI;

/// Configure l'etat initial de l'application
/// 
/// Note: Cette fonction sera appelee avec le MainWindow genere par Slint.
/// Pour l'instant, elle initialise les valeurs par defaut.
pub fn setup_initial_state<T>(_app: &T) {
    tracing::info!("Initialisation etat UI");
    
    // Detection laptop/desktop
    let is_laptop = detect_laptop();
    tracing::info!("Type systeme: {}", if is_laptop { "Laptop" } else { "Desktop" });
    
    // Les bindings Slint seront configures via les globals
    // AppState, SystemInfo, ProfileConfig sont accessibles depuis le code Slint
}

/// Recupere les informations systeme reelles
pub fn get_system_info() -> SystemInfoUI {
    let is_laptop = detect_laptop();
    
    // Recuperation hostname
    let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "UNKNOWN".into());
    
    // Recuperation OS via pieuvre_audit si disponible
    let os_version = get_os_version();
    let build_number = get_build_number();
    
    // Hardware info
    let (cpu_name, cpu_cores) = get_cpu_info();
    let ram_gb = get_ram_gb();
    let gpu_name = get_gpu_name();
    
    SystemInfoUI {
        os_version,
        build_number,
        hostname,
        cpu_name,
        cpu_cores,
        ram_gb,
        gpu_name,
        is_laptop,
    }
}

/// Verifie si le systeme est un laptop
pub fn detect_laptop() -> bool {
    pieuvre_audit::hardware::is_laptop()
}

/// Recupere la version Windows
fn get_os_version() -> String {
    use std::process::Command;
    
    let output = Command::new("cmd")
        .args(["/c", "ver"])
        .output();
    
    match output {
        Ok(o) => {
            let ver = String::from_utf8_lossy(&o.stdout);
            if ver.contains("10.0.22") {
                "Windows 11".into()
            } else if ver.contains("10.0") {
                "Windows 10".into()
            } else {
                "Windows".into()
            }
        }
        Err(_) => "Windows".into(),
    }
}

/// Recupere le build number
fn get_build_number() -> String {
    use std::process::Command;
    
    let output = Command::new("reg")
        .args(["query", r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion", "/v", "CurrentBuild"])
        .output();
    
    match output {
        Ok(o) => {
            let text = String::from_utf8_lossy(&o.stdout);
            for line in text.lines() {
                if line.contains("CurrentBuild") {
                    if let Some(val) = line.split_whitespace().last() {
                        return val.to_string();
                    }
                }
            }
            "Unknown".into()
        }
        Err(_) => "Unknown".into(),
    }
}

/// Recupere les infos CPU
fn get_cpu_info() -> (String, i32) {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(["cpu", "get", "Name,NumberOfCores", "/format:list"])
        .output();
    
    let mut name = "Unknown CPU".to_string();
    let mut cores = 4i32;
    
    if let Ok(o) = output {
        let text = String::from_utf8_lossy(&o.stdout);
        for line in text.lines() {
            if line.starts_with("Name=") {
                name = line.trim_start_matches("Name=").trim().to_string();
            } else if line.starts_with("NumberOfCores=") {
                if let Ok(n) = line.trim_start_matches("NumberOfCores=").trim().parse() {
                    cores = n;
                }
            }
        }
    }
    
    (name, cores)
}

/// Recupere la RAM en GB
fn get_ram_gb() -> i32 {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(["computersystem", "get", "TotalPhysicalMemory", "/format:list"])
        .output();
    
    if let Ok(o) = output {
        let text = String::from_utf8_lossy(&o.stdout);
        for line in text.lines() {
            if line.starts_with("TotalPhysicalMemory=") {
                if let Ok(bytes) = line.trim_start_matches("TotalPhysicalMemory=").trim().parse::<u64>() {
                    return (bytes / 1024 / 1024 / 1024) as i32;
                }
            }
        }
    }
    
    16 // Default
}

/// Recupere le nom du GPU
fn get_gpu_name() -> String {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(["path", "win32_VideoController", "get", "Name", "/format:list"])
        .output();
    
    if let Ok(o) = output {
        let text = String::from_utf8_lossy(&o.stdout);
        for line in text.lines() {
            if line.starts_with("Name=") {
                return line.trim_start_matches("Name=").trim().to_string();
            }
        }
    }
    
    "Unknown GPU".into()
}
