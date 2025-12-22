//! Network Optimizations
//!
//! Nagle algorithm disable and TCP optimizations for gaming.

use pieuvre_common::Result;

/// Disable Nagle's Algorithm for all network adapters
/// Reduces latency for online gaming by sending TCP packets immediately
pub fn disable_nagle_algorithm() -> Result<u32> {
    let mut modified = 0u32;

    // Get all network interfaces via registry
    let interfaces_key = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
    let subkeys = crate::registry::list_subkeys(interfaces_key)?;

    for guid in subkeys {
        let key_path = format!(r"{}\{}", interfaces_key, guid);

        // Set TcpNoDelay
        if crate::registry::set_dword_value(&key_path, "TcpNoDelay", 1).is_ok() {
            // Set TcpAckFrequency
            let _ = crate::registry::set_dword_value(&key_path, "TcpAckFrequency", 1);
            modified += 1;
        }
    }

    tracing::info!("Nagle disabled on {} interfaces", modified);
    Ok(modified)
}

/// Enable Nagle's Algorithm (restore default)
pub fn enable_nagle_algorithm() -> Result<()> {
    let interfaces_key = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
    let subkeys = crate::registry::list_subkeys(interfaces_key)?;

    for guid in subkeys {
        let key_path = format!(r"{}\{}", interfaces_key, guid);
        let _ = crate::registry::delete_value(&key_path, "TcpNoDelay");
        let _ = crate::registry::delete_value(&key_path, "TcpAckFrequency");
    }

    tracing::info!("Nagle re-enabled (default)");
    Ok(())
}

/// Check if Nagle is disabled on primary adapter
pub fn is_nagle_disabled() -> bool {
    let interfaces_key = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces";
    if let Ok(subkeys) = crate::registry::list_subkeys(interfaces_key) {
        for guid in subkeys {
            let key_path = format!(r"{}\{}", interfaces_key, guid);
            if crate::registry::read_dword_value(&key_path, "TcpNoDelay").unwrap_or(0) == 1 {
                return true;
            }
        }
    }
    false
}

/// Disable Interrupt Moderation on all network adapters
/// Reduces network latency at cost of higher CPU usage
pub fn disable_interrupt_moderation() -> Result<u32> {
    set_advanced_property("*InterruptModeration", "0")
}

/// Enable Interrupt Moderation (restore default)
pub fn enable_interrupt_moderation() -> Result<()> {
    set_advanced_property("*InterruptModeration", "1").map(|_| ())
}

/// Disable Large Send Offload (LSO) for reduced latency
pub fn disable_lso() -> Result<()> {
    let _ = set_advanced_property("*LsoV2IPv4", "0");
    let _ = set_advanced_property("*LsoV2IPv6", "0");
    tracing::info!("Large Send Offload disabled");
    Ok(())
}

/// Enable LSO (restore default)
pub fn enable_lso() -> Result<()> {
    let _ = set_advanced_property("*LsoV2IPv4", "1");
    let _ = set_advanced_property("*LsoV2IPv6", "1");
    tracing::info!("Large Send Offload enabled");
    Ok(())
}

/// Disable Energy Efficient Ethernet for consistent performance
pub fn disable_eee() -> Result<()> {
    let _ = set_advanced_property("*EEE", "0");
    tracing::info!("Energy Efficient Ethernet disabled");
    Ok(())
}

/// Enable Receive Side Scaling (RSS) across all cores
pub fn enable_rss() -> Result<()> {
    let _ = set_advanced_property("*RSS", "1");
    tracing::info!("Receive Side Scaling enabled");
    Ok(())
}

/// Disable Receive Segment Coalescing for lower latency
pub fn disable_rsc() -> Result<()> {
    let _ = set_advanced_property("*RscIPv4", "0");
    let _ = set_advanced_property("*RscIPv6", "0");
    tracing::info!("Receive Segment Coalescing disabled");
    Ok(())
}

/// Advanced TCP Stack Hardening (SOTA)
/// Tweaks for MaxFreeTcbs, MaxHashTableSize, and TcpWindowSize
pub fn apply_tcp_stack_hardening() -> Result<()> {
    let tcp_path = r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters";

    // MaxFreeTcbs: 65536 (High performance)
    crate::registry::set_dword_value(tcp_path, "MaxFreeTcbs", 65536)?;

    // MaxHashTableSize: 16384
    crate::registry::set_dword_value(tcp_path, "MaxHashTableSize", 16384)?;

    // TcpWindowSize: 65535 (Classic SOTA value for low latency)
    crate::registry::set_dword_value(tcp_path, "TcpWindowSize", 65535)?;

    // Disable TCP Chimney Offload (can cause latency spikes)
    crate::registry::set_dword_value(tcp_path, "EnableTCPChimney", 0)?;

    // Disable RSS Queues auto-tuning
    crate::registry::set_dword_value(tcp_path, "EnableRSS", 1)?;

    tracing::info!("Advanced TCP Stack Hardening applied");
    Ok(())
}

/// Helper interne pour modifier les propriétés avancées via le registre (SOTA Native)
fn set_advanced_property(property_name: &str, value: &str) -> Result<u32> {
    let mut modified = 0u32;
    let class_key =
        r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}";

    let subkeys = crate::registry::list_subkeys(class_key)?;
    for subkey in subkeys {
        if subkey == "Properties" {
            continue;
        }
        let key_path = format!(r"{}\{}", class_key, subkey);

        // Vérifier si c'est une carte réseau valide (DriverDesc présent)
        if crate::registry::read_string_value(&key_path, "DriverDesc").is_ok()
            && crate::registry::set_string_value(&key_path, property_name, value).is_ok()
        {
            modified += 1;
        }
    }

    tracing::debug!(
        "Property {} set to {} on {} adapters",
        property_name,
        value,
        modified
    );
    Ok(modified)
}

/// Apply all network latency optimizations
pub fn apply_all_network_optimizations() -> Result<u32> {
    let mut count = 0u32;

    count += disable_nagle_algorithm()?;
    let _ = disable_interrupt_moderation();
    let _ = disable_lso();
    let _ = disable_eee();
    let _ = enable_rss();
    let _ = disable_rsc();

    tracing::info!("All network latency optimizations applied");
    Ok(count)
}
