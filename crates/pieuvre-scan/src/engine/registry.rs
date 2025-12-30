use crate::engine::walker::FastFilter;
use crate::engine::{Threat, ThreatSeverity};
use crate::Result;
use std::ptr;
use windows_sys::Win32::System::Registry::{
    RegCloseKey, RegEnumValueW, RegOpenKeyExW, HKEY, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    KEY_READ,
};

#[derive(Debug, Clone, Copy)]
pub struct SendHKey(pub HKEY);
unsafe impl Send for SendHKey {}
unsafe impl Sync for SendHKey {}

pub struct RegistryWalker {
    asep_keys: Vec<(&'static str, SendHKey)>,
    fast_filter: FastFilter,
}

impl Default for RegistryWalker {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistryWalker {
    pub fn new() -> Self {
        Self {
            asep_keys: vec![
                (
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
                    SendHKey(HKEY_LOCAL_MACHINE),
                ),
                (
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
                    SendHKey(HKEY_CURRENT_USER),
                ),
                (
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
                    SendHKey(HKEY_LOCAL_MACHINE),
                ),
                (
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
                    SendHKey(HKEY_CURRENT_USER),
                ),
                (
                    r"SOFTWARE\Wow6432Node\Microsoft\Windows\CurrentVersion\Run",
                    SendHKey(HKEY_LOCAL_MACHINE),
                ),
                (
                    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Image File Execution Options",
                    SendHKey(HKEY_LOCAL_MACHINE),
                ),
                (
                    r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Windows",
                    SendHKey(HKEY_LOCAL_MACHINE),
                ),
                (
                    r"SYSTEM\CurrentControlSet\Services",
                    SendHKey(HKEY_LOCAL_MACHINE),
                ),
                (
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Winlogon",
                    SendHKey(HKEY_LOCAL_MACHINE),
                ),
                (
                    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Shell Folders",
                    SendHKey(HKEY_CURRENT_USER),
                ),
            ],
            fast_filter: FastFilter::default(),
        }
    }

    pub fn scan_asep(&self) -> Result<Vec<Threat>> {
        let mut findings = Vec::new();
        let mut name_buffer = [0u16; 16384];
        let mut data_buffer = [0u8; 16384];

        for (path, root) in &self.asep_keys {
            if let Ok(values) =
                self.enumerate_values(root.0, path, &mut name_buffer, &mut data_buffer)
            {
                for val in values {
                    if self.fast_filter.is_suspicious(&val) {
                        findings.push(Threat {
                            name: "Persistence Registry Hijack".to_string(),
                            description: format!("Clé de registre suspecte trouvée dans {}", path),
                            severity: ThreatSeverity::High,
                            source: "Registry".to_string(),
                            location: val,
                        });
                    }
                }
            }
        }
        Ok(findings)
    }

    fn enumerate_values(
        &self,
        hkey: HKEY,
        subkey: &str,
        name_buf: &mut [u16],
        data_buf: &mut [u8],
    ) -> Result<Vec<String>> {
        let mut results = Vec::new();
        let subkey_u16: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let mut h_key: HKEY = ptr::null_mut();

        unsafe {
            if RegOpenKeyExW(hkey, subkey_u16.as_ptr(), 0, KEY_READ, &mut h_key) != 0 {
                return Ok(results);
            }

            let mut index = 0;
            loop {
                let mut name_len = name_buf.len() as u32;
                let mut data_len = data_buf.len() as u32;
                let mut val_type = 0u32;

                let status = RegEnumValueW(
                    h_key,
                    index,
                    name_buf.as_mut_ptr(),
                    &mut name_len,
                    ptr::null_mut(),
                    &mut val_type,
                    data_buf.as_mut_ptr(),
                    &mut data_len,
                );

                if status == 0 {
                    let val_name = String::from_utf16_lossy(&name_buf[..name_len as usize]);
                    results.push(format!(r"{}\{}", subkey, val_name));
                    index += 1;
                } else {
                    break;
                }
            }

            RegCloseKey(h_key);
        }

        Ok(results)
    }
}
