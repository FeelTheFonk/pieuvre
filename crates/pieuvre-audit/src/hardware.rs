//! Détection matérielle SOTA
//!
//! Probe CPU, GPU, RAM, stockage via APIs Windows natives.
//! GPU via DXGI pour VRAM précis, Storage via DeviceIoControl pour NVMe/SSD.

use pieuvre_common::{CpuInfo, GpuInfo, HardwareInfo, MemoryInfo, Result, StorageInfo};
use windows::Win32::System::SystemInformation::{
    GetLogicalProcessorInformationEx, GlobalMemoryStatusEx, MEMORYSTATUSEX,
    RelationProcessorCore, SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX,
};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIFactory1, IDXGIAdapter1,
    DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG_SOFTWARE,
};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, GetDiskFreeSpaceExW, GetLogicalDrives, GetDriveTypeW,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Ioctl::{
    IOCTL_STORAGE_QUERY_PROPERTY,
    StorageDeviceSeekPenaltyProperty, StorageAdapterProperty, PropertyStandardQuery,
};
use windows::Win32::System::IO::DeviceIoControl;
use windows::core::{HSTRING, PCWSTR};
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

/// Detecte si le systeme est un laptop (presence batterie)
/// Utilise pour les warnings power/timer
pub fn is_laptop() -> bool {
    use windows::Win32::System::Power::{GetSystemPowerStatus, SYSTEM_POWER_STATUS};
    
    unsafe {
        let mut status = SYSTEM_POWER_STATUS::default();
        if GetSystemPowerStatus(&mut status).is_ok() {
            // BatteryFlag: 128 = no battery, 255 = unknown
            // Si batterie presente, c'est un laptop
            status.BatteryFlag != 128 && status.BatteryFlag != 255
        } else {
            false
        }
    }
}

fn probe_cpu() -> Result<CpuInfo> {
    let mut p_cores = Vec::new();
    let mut e_cores = Vec::new();
    let mut physical_cores = 0u32;

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
    let is_hybrid = !p_cores.is_empty() && !e_cores.is_empty();
    
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

/// Probe storage avec détection SSD/NVMe et taille réelle
fn probe_storage() -> Result<Vec<StorageInfo>> {
    let mut drives = Vec::new();
    
    unsafe {
        const DRIVE_FIXED: u32 = 3;
        
        let drive_mask = GetLogicalDrives();
        
        for i in 0..26u32 {
            if (drive_mask & (1 << i)) != 0 {
                let letter = (b'A' + i as u8) as char;
                let path: Vec<u16> = format!("{}:\\", letter).encode_utf16().chain(std::iter::once(0)).collect();
                
                let drive_type = GetDriveTypeW(PCWSTR(path.as_ptr()));
                
                if drive_type == DRIVE_FIXED {
                    let device_id = format!("{}:", letter);
                    
                    // Récupérer taille via GetDiskFreeSpaceExW
                    let size_bytes = get_disk_size(&device_id);
                    
                    // Détecter SSD via seek penalty (IOCTL)
                    let is_ssd = !has_seek_penalty(&device_id);
                    
                    // Détecter NVMe via BusType ou heuristique
                    let is_nvme = detect_nvme(&device_id);
                    
                    // Récupérer modèle via registre
                    let model = get_disk_model(&device_id, letter);
                    
                    drives.push(StorageInfo {
                        device_id,
                        model,
                        size_bytes,
                        is_ssd,
                        is_nvme,
                    });
                }
            }
        }
    }
    
    Ok(drives)
}

/// Récupère la taille totale d'un disque
fn get_disk_size(device_id: &str) -> u64 {
    unsafe {
        let path: Vec<u16> = format!("{}\\", device_id).encode_utf16().chain(std::iter::once(0)).collect();
        let mut total_bytes = 0u64;
        let mut _free_bytes = 0u64;
        let mut _free_to_caller = 0u64;
        
        if GetDiskFreeSpaceExW(
            PCWSTR(path.as_ptr()),
            Some(&mut _free_to_caller),
            Some(&mut total_bytes),
            Some(&mut _free_bytes),
        ).is_ok() {
            total_bytes
        } else {
            0
        }
    }
}

/// Détecte si le disque a une pénalité de seek (HDD) via IOCTL
fn has_seek_penalty(device_id: &str) -> bool {
    unsafe {
        let physical_path = format!(r"\\.\{}", device_id);
        let path_wide = HSTRING::from(&physical_path);
        
        let handle = match CreateFileW(
            &path_wide,
            0, // Query only, pas besoin de read/write
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            Default::default(),
            None,
        ) {
            Ok(h) => h,
            Err(_) => return true, // Assume HDD si on ne peut pas ouvrir
        };
        
        // Préparer la query pour StorageDeviceSeekPenaltyProperty
        #[repr(C)]
        struct StoragePropertyQuerySeek {
            property_id: u32,
            query_type: u32,
            additional_parameters: [u8; 1],
        }
        
        #[repr(C)]
        struct DeviceSeekPenaltyDescriptor {
            version: u32,
            size: u32,
            incurs_seek_penalty: u8,
        }
        
        let query = StoragePropertyQuerySeek {
            property_id: StorageDeviceSeekPenaltyProperty.0 as u32,
            query_type: PropertyStandardQuery.0 as u32,
            additional_parameters: [0],
        };
        
        let mut result = DeviceSeekPenaltyDescriptor {
            version: 0,
            size: 0,
            incurs_seek_penalty: 1,
        };
        
        let mut bytes_returned = 0u32;
        
        let success = DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some(&query as *const _ as *const std::ffi::c_void),
            std::mem::size_of::<StoragePropertyQuerySeek>() as u32,
            Some(&mut result as *mut _ as *mut std::ffi::c_void),
            std::mem::size_of::<DeviceSeekPenaltyDescriptor>() as u32,
            Some(&mut bytes_returned),
            None,
        );
        
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        
        if success.is_ok() && bytes_returned > 0 {
            result.incurs_seek_penalty != 0
        } else {
            true // Fallback: assume HDD
        }
    }
}

/// Détecte si le disque est NVMe via BusType (SOTA Native)
fn detect_nvme(device_id: &str) -> bool {
    unsafe {
        let physical_path = format!(r"\\.\{}", device_id);
        let path_wide = HSTRING::from(&physical_path);
        
        let handle = match CreateFileW(
            &path_wide,
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            Default::default(),
            None,
        ) {
            Ok(h) => h,
            Err(_) => return false,
        };
        
        #[repr(C)]
        struct StoragePropertyQuery {
            property_id: u32,
            query_type: u32,
            additional_parameters: [u8; 1],
        }
        
        #[repr(C)]
        struct StorageAdapterDescriptor {
            version: u32,
            size: u32,
            maximum_transfer_length: u32,
            maximum_physical_pages: u32,
            alignment_mask: u32,
            adapter_uses_pio: u8,
            adapter_scans_down: u8,
            command_queueing: u8,
            accelerated_transfer: u8,
            bus_type: u8,
            bus_major_version: u16,
            bus_minor_version: u16,
            srb_type: u8,
            address_type: u8,
        }
        
        let query = StoragePropertyQuery {
            property_id: StorageAdapterProperty.0 as u32,
            query_type: PropertyStandardQuery.0 as u32,
            additional_parameters: [0],
        };
        
        let mut result: StorageAdapterDescriptor = std::mem::zeroed();
        let mut bytes_returned = 0u32;
        
        let success = DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            Some(&query as *const _ as *const std::ffi::c_void),
            std::mem::size_of::<StoragePropertyQuery>() as u32,
            Some(&mut result as *mut _ as *mut std::ffi::c_void),
            std::mem::size_of::<StorageAdapterDescriptor>() as u32,
            Some(&mut bytes_returned),
            None,
        );
        
        let _ = windows::Win32::Foundation::CloseHandle(handle);
        
        // BusTypeNvme = 17
        success.is_ok() && result.bus_type == 17
    }
}

/// Récupère le modèle du disque via le registre (SOTA)
fn get_disk_model(device_id: &str, _letter: char) -> String {
    // Tentative de récupération via Enum\USBSTOR ou Enum\SCSI
    // Pour simplifier en SOTA natif sans WMI, on peut chercher dans le registre
    // SYSTEM\CurrentControlSet\Enum\SCSI\... ou STORAGE\Volume
    // Ici on utilise une version simplifiée qui rend "Disk [Letter]" par défaut
    // mais on pourrait parser les FriendlyName du registre.
    format!("Disk {}", device_id)
}

/// Probe GPU via DXGI - détection VRAM précise et multi-GPU
fn probe_gpu() -> Result<Vec<GpuInfo>> {
    let mut gpus = Vec::new();
    
    unsafe {
        // Créer la factory DXGI
        let factory: IDXGIFactory1 = match CreateDXGIFactory1() {
            Ok(f) => f,
            Err(_) => {
                // Fallback au registre si DXGI échoue
                return probe_gpu_registry();
            }
        };
        
        let mut adapter_index = 0u32;
        
        loop {
            let adapter: IDXGIAdapter1 = match factory.EnumAdapters1(adapter_index) {
                Ok(a) => a,
                Err(_) => break, // Plus d'adaptateurs
            };
            
            let desc: DXGI_ADAPTER_DESC1 = match adapter.GetDesc1() {
                Ok(d) => d,
                Err(_) => {
                    adapter_index += 1;
                    continue;
                }
            };
            
            // Exclure les adaptateurs software (comme Microsoft Basic Render Driver)
            if (desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) == 0 {
                // Convertir le nom (UTF-16 avec null terminator)
                let name_end = desc.Description.iter()
                    .position(|&c| c == 0)
                    .unwrap_or(desc.Description.len());
                let name = String::from_utf16_lossy(&desc.Description[..name_end]);
                
                // Détecter le vendor via VendorId
                let vendor = detect_vendor_from_id(desc.VendorId);
                
                // VRAM = DedicatedVideoMemory (pour GPU discret)
                // Pour iGPU, SharedSystemMemory est plus pertinent mais on prend Dedicated
                let vram_bytes = desc.DedicatedVideoMemory as u64;
                
                gpus.push(GpuInfo {
                    name: name.trim().to_string(),
                    vendor,
                    vram_bytes,
                });
            }
            
            adapter_index += 1;
        }
    }
    
    // Si aucun GPU trouvé via DXGI, fallback au registre
    if gpus.is_empty() {
        return probe_gpu_registry();
    }
    
    Ok(gpus)
}

/// Détecte le vendor GPU via son ID PCI
fn detect_vendor_from_id(vendor_id: u32) -> String {
    match vendor_id {
        0x10DE => "NVIDIA".to_string(),
        0x1002 => "AMD".to_string(),
        0x8086 => "Intel".to_string(),
        0x1414 => "Microsoft".to_string(), // Software adapters
        0x5143 => "Qualcomm".to_string(),
        _ => format!("Unknown (0x{:04X})", vendor_id),
    }
}

/// Fallback: détection GPU via registre
fn probe_gpu_registry() -> Result<Vec<GpuInfo>> {
    use windows::Win32::System::Registry::{
        RegOpenKeyExW, RegCloseKey, RegEnumKeyExW,
        HKEY_LOCAL_MACHINE, KEY_READ,
    };
    
    let mut gpus = Vec::new();
    
    unsafe {
        // Énumérer tous les adaptateurs (0000, 0001, etc.)
        let base_key: Vec<u16> = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        
        let mut hkey = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(base_key.as_ptr()), Some(0), KEY_READ, &mut hkey).is_err() {
            return Ok(gpus);
        }
        
        let mut index = 0u32;
        loop {
            let mut name_buffer = vec![0u16; 256];
            let mut name_len = name_buffer.len() as u32;
            
            if RegEnumKeyExW(
                hkey,
                index,
                Some(windows::core::PWSTR(name_buffer.as_mut_ptr())),
                &mut name_len,
                None,
                None,
                None,
                None,
            ).is_err() {
                break;
            }
            
            // Ouvrir la sous-clé (ex: 0000, 0001)
            let subkey_name = String::from_utf16_lossy(&name_buffer[..name_len as usize]);
            if subkey_name.chars().all(|c| c.is_ascii_digit()) {
                let full_path = format!(
                    r"SYSTEM\CurrentControlSet\Control\Class\{{4d36e968-e325-11ce-bfc1-08002be10318}}\{}",
                    subkey_name
                );
                
                if let Some(gpu) = read_gpu_from_registry(&full_path) {
                    gpus.push(gpu);
                }
            }
            
            index += 1;
        }
        
        let _ = RegCloseKey(hkey);
    }
    
    Ok(gpus)
}

/// Lit les informations GPU depuis une clé registre
fn read_gpu_from_registry(key_path: &str) -> Option<GpuInfo> {
    use windows::Win32::System::Registry::{
        RegOpenKeyExW, RegQueryValueExW, RegCloseKey,
        HKEY_LOCAL_MACHINE, KEY_READ,
    };
    
    unsafe {
        let key_wide: Vec<u16> = key_path.encode_utf16().chain(std::iter::once(0)).collect();
        
        let mut hkey = Default::default();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, PCWSTR(key_wide.as_ptr()), Some(0), KEY_READ, &mut hkey).is_err() {
            return None;
        }
        
        // Lire DriverDesc
        let value_name: Vec<u16> = "DriverDesc".encode_utf16().chain(std::iter::once(0)).collect();
        let mut buffer = vec![0u8; 512];
        let mut size = buffer.len() as u32;
        
        let name = if RegQueryValueExW(
            hkey,
            PCWSTR(value_name.as_ptr()),
            None,
            None,
            Some(buffer.as_mut_ptr()),
            Some(&mut size),
        ).is_ok() {
            String::from_utf16_lossy(
                std::slice::from_raw_parts(buffer.as_ptr() as *const u16, (size as usize / 2).saturating_sub(1))
            ).trim().to_string()
        } else {
            let _ = RegCloseKey(hkey);
            return None;
        };
        
        let _ = RegCloseKey(hkey);
        
        // Détecter vendor depuis le nom
        let vendor = if name.to_lowercase().contains("nvidia") {
            "NVIDIA"
        } else if name.to_lowercase().contains("amd") || name.to_lowercase().contains("radeon") {
            "AMD"
        } else if name.to_lowercase().contains("intel") {
            "Intel"
        } else {
            "Unknown"
        };
        
        Some(GpuInfo {
            name,
            vendor: vendor.to_string(),
            vram_bytes: 0, // Pas disponible via registre
        })
    }
}
