//! Détection matérielle
//!
//! Probe CPU, GPU, RAM, stockage via APIs Windows.

use pieuvre_common::{CpuInfo, GpuInfo, HardwareInfo, MemoryInfo, Result, StorageInfo};
use windows::Win32::System::SystemInformation::{
    GetLogicalProcessorInformationEx, GlobalMemoryStatusEx, MEMORYSTATUSEX,
    RelationProcessorCore, SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
};
use std::arch::x86_64::__cpuid;

/// Collecte toutes les informations matérielles
pub fn probe_hardware() -> Result<HardwareInfo> {
    Ok(HardwareInfo {
        cpu: probe_cpu()?,
        memory: probe_memory()?,
        storage: probe_storage()?,
        gpu: probe_gpu()?,
    })
}

fn probe_cpu() -> Result<CpuInfo> {
    let mut p_cores = Vec::new();
    let mut e_cores = Vec::new();
    let mut physical_cores = 0u32;
    let is_hybrid;

    // Détection via CPUID pour vendor/model
    let (vendor, model_name) = detect_cpu_via_cpuid();

    // Détection via GetLogicalProcessorInformationEx pour topology
    unsafe {
        let mut length = 0u32;
        let _ = GetLogicalProcessorInformationEx(RelationProcessorCore, None, &mut length);
        
        if length > 0 {
            let mut buffer = vec![0u8; length as usize];
            let ptr = buffer.as_mut_ptr() as *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX;
            
            if GetLogicalProcessorInformationEx(
                RelationProcessorCore,
                Some(ptr),
                &mut length,
            ).is_ok() {
                let mut offset = 0usize;
                while offset < length as usize {
                    let info = &*(buffer.as_ptr().add(offset) as *const SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX);
                    physical_cores += 1;
                    
                    // EfficiencyClass: 0 = E-Core, 1+ = P-Core (Intel 12th+)
                    let efficiency = info.Anonymous.Processor.EfficiencyClass;
                    if efficiency == 0 {
                        e_cores.push(physical_cores - 1);
                    } else {
                        p_cores.push(physical_cores - 1);
                    }
                    
                    offset += info.Size as usize;
                }
            }
        }
    }
    
    let logical_cores = std::thread::available_parallelism()
        .map(|p| p.get() as u32)
        .unwrap_or(1);
    
    // Hybrid detection: both P and E cores present
    is_hybrid = !p_cores.is_empty() && !e_cores.is_empty();
    
    Ok(CpuInfo {
        vendor,
        model_name,
        logical_cores,
        physical_cores,
        is_hybrid,
        p_cores,
        e_cores,
    })
}

/// Détection CPU via instruction CPUID
fn detect_cpu_via_cpuid() -> (String, String) {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            // Get vendor string (EAX=0)
            let result = __cpuid(0);
            let vendor_bytes: [u8; 12] = [
                result.ebx as u8, (result.ebx >> 8) as u8, (result.ebx >> 16) as u8, (result.ebx >> 24) as u8,
                result.edx as u8, (result.edx >> 8) as u8, (result.edx >> 16) as u8, (result.edx >> 24) as u8,
                result.ecx as u8, (result.ecx >> 8) as u8, (result.ecx >> 16) as u8, (result.ecx >> 24) as u8,
            ];
            let vendor = String::from_utf8_lossy(&vendor_bytes).to_string();
            
            // Get brand string (EAX=0x80000002-0x80000004)
            let mut brand = String::new();
            for i in 0x80000002u32..=0x80000004u32 {
                let result = __cpuid(i);
                for reg in [result.eax, result.ebx, result.ecx, result.edx] {
                    for j in 0..4 {
                        let c = ((reg >> (j * 8)) & 0xFF) as u8;
                        if c != 0 {
                            brand.push(c as char);
                        }
                    }
                }
            }
            
            let vendor_name = match vendor.trim() {
                s if s.contains("GenuineIntel") => "Intel".to_string(),
                s if s.contains("AuthenticAMD") => "AMD".to_string(),
                _ => vendor.trim().to_string(),
            };
            
            (vendor_name, brand.trim().to_string())
        }
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        ("Unknown".to_string(), std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "Unknown".into()))
    }
}

fn probe_memory() -> Result<MemoryInfo> {
    unsafe {
        let mut status = MEMORYSTATUSEX {
            dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
            ..Default::default()
        };
        
        GlobalMemoryStatusEx(&mut status)?;
        
        Ok(MemoryInfo {
            total_bytes: status.ullTotalPhys,
            available_bytes: status.ullAvailPhys,
        })
    }
}

fn probe_storage() -> Result<Vec<StorageInfo>> {
    // Use win32 GetLogicalDrives + GetDriveTypeW for drive detection
    let mut drives = Vec::new();
    
    unsafe {
        use windows::Win32::Storage::FileSystem::{GetLogicalDrives, GetDriveTypeW};
        use windows::core::PCWSTR;
        
        const DRIVE_FIXED: u32 = 3; // Fixed drive
        
        let drive_mask = GetLogicalDrives();
        
        for i in 0..26u32 {
            if (drive_mask & (1 << i)) != 0 {
                let letter = (b'A' + i as u8) as char;
                let path: Vec<u16> = format!("{}:\\", letter).encode_utf16().chain(std::iter::once(0)).collect();
                
                let drive_type = GetDriveTypeW(PCWSTR(path.as_ptr()));
                
                if drive_type == DRIVE_FIXED {
                    drives.push(StorageInfo {
                        device_id: format!("{}:", letter),
                        model: format!("Drive {}", letter),
                        size_bytes: 0, // Would need DeviceIoControl for real size
                        is_ssd: true,  // Assume SSD for now
                        is_nvme: false,
                    });
                }
            }
        }
    }
    
    Ok(drives)
}

fn probe_gpu() -> Result<Vec<GpuInfo>> {
    // Minimal GPU detection via registry
    let mut gpus = Vec::new();
    
    // Read from registry: HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}
    // For now, use environment variable as fallback
    if let Ok(gpu_name) = std::env::var("GPU_NAME") {
        gpus.push(GpuInfo {
            name: gpu_name,
            vendor: "Unknown".to_string(),
            vram_bytes: 0,
        });
    }
    
    // Try to detect via Display adapter registry
    use windows::Win32::System::Registry::{RegOpenKeyExW, RegQueryValueExW, RegCloseKey, HKEY_LOCAL_MACHINE, KEY_READ};
    use windows::core::PCWSTR;
    
    unsafe {
        let subkey: Vec<u16> = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        
        let mut hkey = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(subkey.as_ptr()), 0, KEY_READ, &mut hkey).is_ok() {
            let value_name: Vec<u16> = "DriverDesc".encode_utf16().chain(std::iter::once(0)).collect();
            let mut buffer = vec![0u8; 512];
            let mut size = buffer.len() as u32;
            
            if RegQueryValueExW(
                hkey,
                PCWSTR(value_name.as_ptr()),
                None,
                None,
                Some(buffer.as_mut_ptr()),
                Some(&mut size),
            ).is_ok() {
                let name = String::from_utf16_lossy(
                    std::slice::from_raw_parts(buffer.as_ptr() as *const u16, (size as usize / 2).saturating_sub(1))
                );
                
                let vendor = if name.to_lowercase().contains("nvidia") {
                    "NVIDIA"
                } else if name.to_lowercase().contains("amd") || name.to_lowercase().contains("radeon") {
                    "AMD"
                } else if name.to_lowercase().contains("intel") {
                    "Intel"
                } else {
                    "Unknown"
                };
                
                gpus.push(GpuInfo {
                    name: name.trim().to_string(),
                    vendor: vendor.to_string(),
                    vram_bytes: 0, // Would need DXGI for VRAM
                });
            }
            
            let _ = RegCloseKey(hkey);
        }
    }
    
    Ok(gpus)
}
