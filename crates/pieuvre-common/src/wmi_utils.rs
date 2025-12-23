use crate::error::{PieuvreError, Result};
use windows::core::BSTR;
use windows::Win32::System::Variant::{
    VariantChangeType, VariantClear, VARIANT, VAR_CHANGE_FLAGS, VT_BSTR, VT_I4,
};
use windows::Win32::System::Wmi::IWbemClassObject;

/// Wrapper pour faciliter l'extraction de propriétés WMI de manière sécurisée.
pub struct WmiObject<'a> {
    inner: &'a IWbemClassObject,
}

impl<'a> WmiObject<'a> {
    pub fn new(inner: &'a IWbemClassObject) -> Self {
        Self { inner }
    }

    /// Extrait une propriété sous forme de String.
    pub fn get_string(&self, name: &str) -> Result<String> {
        unsafe {
            let mut vt = VARIANT::default();
            self.inner
                .Get(&BSTR::from(name), 0, &mut vt, None, None)
                .map_err(|e| PieuvreError::System(format!("WMI Get failed for {}: {}", name, e)))?;

            let mut vt_str = VARIANT::default();
            let res = VariantChangeType(&mut vt_str, &vt, VAR_CHANGE_FLAGS(0), VT_BSTR);

            // Nettoyage immédiat de la source
            let _ = VariantClear(&mut vt);

            res.map_err(|e| {
                PieuvreError::System(format!("VariantChangeType failed for {}: {}", name, e))
            })?;

            let bstr_val = &vt_str.Anonymous.Anonymous.Anonymous.bstrVal;
            let result = bstr_val.to_string();

            // Nettoyage de la destination
            let _ = VariantClear(&mut vt_str);

            Ok(result)
        }
    }

    /// Extrait une propriété sous forme de i32.
    pub fn get_i32(&self, name: &str) -> Result<i32> {
        unsafe {
            let mut vt = VARIANT::default();
            self.inner
                .Get(&BSTR::from(name), 0, &mut vt, None, None)
                .map_err(|e| PieuvreError::System(format!("WMI Get failed for {}: {}", name, e)))?;

            let mut vt_i4 = VARIANT::default();
            let res = VariantChangeType(&mut vt_i4, &vt, VAR_CHANGE_FLAGS(0), VT_I4);

            // Nettoyage immédiat de la source
            let _ = VariantClear(&mut vt);

            res.map_err(|e| {
                PieuvreError::System(format!("VariantChangeType failed for {}: {}", name, e))
            })?;

            let val = vt_i4.Anonymous.Anonymous.Anonymous.lVal;

            // Nettoyage de la destination
            let _ = VariantClear(&mut vt_i4);

            Ok(val)
        }
    }
}

/// Helper pour les appels ConnectServer (windows-rs 0.62.2 exige des BSTR non nulles).
pub fn bstr_empty() -> BSTR {
    BSTR::new()
}
