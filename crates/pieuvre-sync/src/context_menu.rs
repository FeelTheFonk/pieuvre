//! Context Menu Cleanup
//!
//! Remove Windows 11 context menu clutter.

use pieuvre_common::Result;
use std::process::Command;

/// Context menu items to remove (SOTA - Sophia Script reference)
const CONTEXT_MENU_CLSIDS: &[(&str, &str)] = &[
    // "Edit with Paint 3D"
    ("{D2B7917A-B138-4a96-9886-6A72F84CF60E}", "Edit with Paint 3D"),
    // "Edit with Photos"
    ("{FFE2A43C-56B9-4bf5-9A79-CC6D4285608A}", "Edit with Photos"),
    // "Edit with Clipchamp"
    ("{8AB635F8-9A67-4698-AB99-784AD929F3B4}", "Edit with Clipchamp"),
    // "Share" (modern share)
    ("{E2BF9676-5F8F-435C-97EB-11607A5BEDF7}", "Share"),
    // "Give access to"
    ("{F81E9010-6EA4-11CE-A7FF-00AA003CA9F5}", "Give access to"),
    // "Include in library"
    ("{7BA4C740-9E81-11CF-99D3-00AA004AE837}", "Include in library"),
    // "Pin to Quick Access"
    ("{679f85cb-0220-4080-b29b-5540cc05aab6}", "Pin to Quick Access"),
];

/// Remove context menu clutter entries
pub fn remove_context_menu_clutter() -> Result<u32> {
    let mut removed = 0u32;
    
    for (clsid, name) in CONTEXT_MENU_CLSIDS {
        // Remove from HKCR
        let key = format!(r"HKCR\*\shellex\ContextMenuHandlers\{}", clsid);
        let output = Command::new("reg")
            .args(["delete", &key, "/f"])
            .output();
        
        if output.is_ok() {
            tracing::info!("Removed context menu: {}", name);
            removed += 1;
        }
        
        // Also try HKCU
        let key_user = format!(r"HKCU\Software\Classes\*\shellex\ContextMenuHandlers\{}", clsid);
        let _ = Command::new("reg")
            .args(["delete", &key_user, "/f"])
            .output();
    }
    
    // Disable "Show more options" (restore classic context menu)
    let _ = Command::new("reg")
        .args([
            "add",
            r"HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32",
            "/ve",
            "/d", "",
            "/f"
        ])
        .output();
    
    tracing::info!("Removed {} context menu items", removed);
    Ok(removed)
}

/// Restore Windows 11 modern context menu
pub fn restore_modern_context_menu() -> Result<()> {
    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}",
            "/f"
        ])
        .output();
    
    tracing::info!("Restored modern context menu");
    Ok(())
}

/// Check if classic context menu is enabled
pub fn is_classic_context_menu() -> bool {
    let output = Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32"
        ])
        .output();
    
    output.map(|o| o.status.success()).unwrap_or(false)
}
