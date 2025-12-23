//! Section and option definitions for interactive mode
//!
//! Module: Modular structure with explicit types.

use serde::{Deserialize, Serialize};

/// Optimization option with metadata
#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptItem {
    /// Unique identifier for the option
    pub id: &'static str,
    /// Label displayed to the user
    pub label: &'static str,
    /// Detailed description
    pub description: &'static str,
    /// Selected by default
    pub default: bool,
    /// Risk level
    pub risk: RiskLevel,
}

/// Risk level of an option
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// No risk, recommended
    Safe,
    /// Conditional, depends on context
    Conditional,
    /// Performance, may impact battery
    Performance,
    /// Caution required
    Warning,
    /// Critical risk - system security compromised
    Critical,
}

#[allow(dead_code)]
impl OptItem {
    /// Creates a safe option enabled by default
    pub const fn safe(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            description: "No description available.",
            default: true,
            risk: RiskLevel::Safe,
        }
    }

    #[allow(dead_code)]
    /// Creates a safe option disabled by default
    pub const fn safe_off(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            description: "No description available.",
            default: false,
            risk: RiskLevel::Safe,
        }
    }

    #[allow(dead_code)]
    /// Creates a conditional option
    pub const fn conditional(id: &'static str, label: &'static str, default: bool) -> Self {
        Self {
            id,
            label,
            description: "No description available.",
            default,
            risk: RiskLevel::Conditional,
        }
    }

    #[allow(dead_code)]
    /// Creates a performance option
    pub const fn perf(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            description: "No description available.",
            default: true,
            risk: RiskLevel::Performance,
        }
    }

    #[allow(dead_code)]
    /// Creates a warning option (laptop)
    pub const fn warning(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            description: "No description available.",
            default: false,
            risk: RiskLevel::Warning,
        }
    }

    #[allow(dead_code)]
    /// Creates a critical option (system security)
    pub const fn critical(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            description: "No description available.",
            default: false,
            risk: RiskLevel::Critical,
        }
    }
}

// ============================================================================
// SECTION 1: TELEMETRY
// ============================================================================

#[allow(dead_code)]
/// Returns options for the Telemetry section
pub fn telemetry_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "diagtrack",
            label: "Main Telemetry (DiagTrack)",
            description: "Disables the 'Connected User Experiences and Telemetry' service. This component is the core of Microsoft's data collection, managing the transmission of usage logs, performance reports, and system inventory data to remote servers. Disabling it drastically reduces unsolicited outgoing network traffic and frees up CPU cycles previously allocated to metadata processing.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "dmwappush",
            label: "WAP Push Service",
            description: "Neutralizes the 'dmwappushservice' (Device Management Wireless Application Protocol). Primarily used for routing push messages and telemetry related to mobile device management, this service is superfluous on a fixed workstation or standard laptop. Removing it eliminates a potential vector for behavioral data collection.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "wersvc",
            label: "Error Reporting (WerSvc)",
            description: "Disables the Windows Error Reporting (WER) service. Prevents the automatic generation and transmission of crash reports, memory dump files, and software error details to Microsoft. While useful for remote debugging, it consumes disk and network resources during every system incident.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "wercplsupport",
            label: "Error Reports Support",
            description: "Disables Control Panel support for problem reports. This component handles the user interface for viewing and managing archived error reports. Disabling it cleans the system of post-mortem error management processes.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "pcasvc",
            label: "Program Compatibility",
            description: "Disables the Program Compatibility Assistant (PCA). This service monitors software execution to detect known compatibility issues. While protective, it injects hooks into processes and can generate false positives or slowdowns when installing optimized or legacy software.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "wdisystem",
            label: "Diagnostic Host",
            description: "Disables the Diagnostic System Host (WdiSystemHost). This service is used by Windows to diagnose system problems and collect troubleshooting data. Disabling it prevents background diagnostic routines that can impact DPC latency.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "wdiservice",
            label: "Diagnostic Service",
            description: "Disables the Diagnostic Service Host (WdiServiceHost). Similar to the Diagnostic Host, it handles diagnostic services requiring local service privileges. Stopping it contributes to reducing the execution surface of non-essential system processes.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "lfsvc",
            label: "Geolocation Service",
            description: "Disables the geolocation service. Prevents Windows and third-party applications from determining the device's physical location via IP, Wi-Fi, or GPS. Crucial for privacy, it also avoids constant scanning of surrounding Wi-Fi networks for triangulation.",
            default: true,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "mapsbroker",
            label: "Maps Download Broker",
            description: "Disables the downloaded maps manager. This service handles access to offline maps for Windows applications. Useless if you don't use the native 'Maps' application, it avoids unnecessary background update checks.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "firewall",
            label: "Block Telemetry Domains",
            description: "Implements strict Windows Firewall rules to block known Microsoft IP addresses and domains dedicated to telemetry. Acts as an additional network security layer in case some services manage to reactivate or bypass registry settings.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "sched_tasks",
            label: "Disable Telemetry Tasks",
            description: "Disables over 50 scheduled tasks related to data collection, customer experience (CEIP), and telemetry maintenance. These tasks often run during system idle time, causing unpredictable disk and CPU usage spikes.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "hosts",
            label: "Hosts File Blocking",
            description: "Modifies the system 'hosts' file to redirect Microsoft telemetry domains to 0.0.0.0. This local DNS-level blocking method is extremely effective as it intercepts requests before they even leave the computer's network stack.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "onedrive",
            label: "Uninstall OneDrive",
            description: "Completely removes Microsoft OneDrive from the system. Eliminates constant synchronization processes, Explorer integrations, and login reminders. Frees up significant network and CPU resources, especially on configurations with many files.",
            default: true,
            risk: RiskLevel::Conditional,
        },
    ]
}

// ============================================================================
// SECTION 2: PRIVACY
// ============================================================================

#[allow(dead_code)]
/// Returns options for the Privacy section
pub fn privacy_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "telemetry_level",
            label: "Telemetry Level: Security Only",
            description: "Forces the telemetry level to 'Security' (0) via group policies. This level is normally reserved for Enterprise and Education editions, limiting the data sent to the bare minimum necessary to keep Windows updated and secure.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "advertising_id",
            label: "Disable Advertising ID",
            description: "Disables the unique advertising identifier assigned to each user. Prevents applications from tracking your behavior to provide targeted ads. Restores a level of anonymity within the Windows application ecosystem.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "location",
            label: "Disable System Location",
            description: "Globally disables location services at the operating system level. Prevents Windows from collecting your movement history and blocks access to position sensors for all applications, strengthening privacy.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "activity_history",
            label: "Disable Activity History",
            description: "Prevents Windows from collecting your activities (open applications, viewed files) for the 'Timeline' feature. Avoids local storage and cloud synchronization of your daily usage history.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cortana",
            label: "Disable Cortana Search",
            description: "Disables the Cortana assistant and integrated web search in the Start menu. Transforms the search bar into a purely local tool, speeding up result display and preventing your search queries from being sent to Bing.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "context_menu",
            label: "Classic Context Menu",
            description: "Restores the classic Windows 10 context menu on Windows 11. Eliminates the extra 'Show more options' step, speeding up the workflow and reducing the right-click menu display delay often slowed down by the new design.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "widgets",
            label: "Disable Win11 Widgets",
            description: "Removes the Widgets button from the taskbar and disables the associated service. Widgets consume RAM and network resources in the background to update news and weather feeds.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "pause_updates",
            label: "Pause Updates (35 days)",
            description: "Suspends automatic Windows updates for a 35-day period. Useful to avoid forced restarts or the installation of potentially unstable updates during critical work or gaming sessions.",
            default: false,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "driver_updates",
            label: "Disable Auto Driver Updates",
            description: "Prevents Windows Update from automatically installing hardware drivers. Crucial for users wanting full control over their driver versions (especially GPU), avoiding overwrites by generic or obsolete versions.",
            default: false,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "recall",
            label: "Block Windows Recall (AI)",
            description: "Disables the controversial 'Recall' feature of Windows AI. Prevents the system from taking constant screenshots of your activity and analyzing them via OCR. Major protection against exhaustive and automated local surveillance.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "group_policy_telem",
            label: "Enterprise Telemetry Policy",
            description: "Applies enterprise-level telemetry restrictions via the registry. Locks settings to prevent modification by system updates or third-party applications, ensuring long-term privacy.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

// ============================================================================
// SECTION 3: PERFORMANCE
// ============================================================================

#[allow(dead_code)]
/// Returns options for the Performance section
/// `is_laptop` adapts options for battery
pub fn performance_section(is_laptop: bool) -> Vec<OptItem> {
    let mut opts = Vec::with_capacity(20);

    // Timer 0.5ms
    if is_laptop {
        opts.push(OptItem {
            id: "timer",
            label: "Timer 0.5ms - Battery impact",
            description: "Forces the system timer resolution to 0.5ms. Drastically reduces input lag and improves the precision of micro-movements in games. On laptops, this prevents the CPU from entering deep power-saving states, reducing battery life.",
            default: false,
            risk: RiskLevel::Warning,
        });
    } else {
        opts.push(OptItem {
            id: "timer",
            label: "Timer Resolution 0.5ms (Input lag)",
            description: "Optimizes the temporal precision of the Windows kernel. By switching from 15.6ms to 0.5ms, the system processes interrupts more frequently, resulting in a smoother mouse feel and reduced overall rendering latency.",
            default: true,
            risk: RiskLevel::Safe,
        });
    }

    // Power Plan
    if is_laptop {
        opts.push(OptItem {
            id: "power_ultimate",
            label: "Ultimate Performance - Battery wear",
            description: "Activates the 'Ultimate Performance' power plan. Disables all power-saving techniques, forces the CPU to its maximum frequency, and reduces transition latencies. Caution: high heat and accelerated battery wear.",
            default: false,
            risk: RiskLevel::Warning,
        });
        opts.push(OptItem {
            id: "power_high",
            label: "High Performance - Recommended for laptop",
            description: "Balanced 'High Performance' plan for laptops. Offers excellent responsiveness while allowing the hardware to breathe during relative idle phases. The best compromise for mobile gaming.",
            default: false,
            risk: RiskLevel::Safe,
        });
    } else {
        opts.push(OptItem {
            id: "power_ultimate",
            label: "Power Plan: Ultimate Performance",
            description: "Unlocks the hidden 'Ultimate Performance' power plan in Windows. Eliminates micro-latencies related to power management, ensuring the hardware is always ready to deliver maximum power instantly.",
            default: false,
            risk: RiskLevel::Safe,
        });
    }

    // Common options
    opts.extend([
        OptItem {
            id: "cpu_throttle",
            label: "Disable CPU Throttling",
            description: "Disables software thermal and energy throttling of the processor. Prevents Windows from artificially lowering the CPU frequency, ensuring consistent performance even under prolonged heavy load.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "usb_suspend",
            label: "Disable USB Selective Suspend",
            description: "Prevents Windows from putting USB ports to sleep. Crucial for gaming peripherals (mice, keyboards, DACs) to avoid any wake-up delay or untimely disconnection during critical moments.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "msi",
            label: "Enable MSI (Message Signaled Interrupts)",
            description: "Switches compatible devices from classic interrupt mode (IRQ) to MSI mode. Reduces interrupt conflicts and CPU latency, allowing more efficient data processing by the GPU and network card.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "sysmain",
            label: "Disable SysMain (Superfetch)",
            description: "Disables the SysMain service which preloads applications into RAM. While useful on HDDs, it generates unnecessary disk writes and can cause micro-stutters on modern SSDs and configurations with plenty of RAM.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "wsearch",
            label: "Disable Windows Search Indexer",
            description: "Stops constant file indexing. Frees up considerable CPU and disk resources. Searching in Explorer will be slower, but overall system performance and latency will be improved.",
            default: true,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "edge_disable",
            label: "Disable Edge background/bloat",
            description: "Prevents Microsoft Edge from running in the background and disables its update and telemetry services. Frees up RAM and prevents 'ghost' processes from consuming CPU cycles.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "explorer_tweaks",
            label: "Windows Explorer optimizations",
            description: "Applies a series of Explorer tweaks: disabling history, speeding up folder display, and removing unnecessary animations for instant and fluid navigation.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "game_bar",
            label: "Disable Game Bar & DVR",
            description: "Disables the Windows game overlay and background recording (DVR). These features inject hooks into games, which can reduce FPS by 5-10% and significantly increase input lag.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "fullscreen_opt",
            label: "Disable Fullscreen Optimizations",
            description: "Globally disables fullscreen optimizations. While intended to help, they often force a hybrid 'borderless windowed' mode that adds composition latency via the DWM (Desktop Window Manager).",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "hags",
            label: "Disable HAGS (Hardware GPU Scheduling)",
            description: "Disables hardware-accelerated GPU scheduling. On some configurations, HAGS can cause instability or micro-stuttering. Disabling it gives the CPU direct control over the GPU queue.",
            default: false,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "nagle",
            label: "Disable Nagle Algorithm (Network Latency)",
            description: "Disables the Nagle algorithm (TCP No Delay). Prevents the system from waiting to group small network packets, which drastically reduces 'ping' in online games at the cost of a slight increase in traffic.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "power_throttle",
            label: "Disable Power Throttling",
            description: "Prevents Windows from limiting CPU resources for background applications. Ensures your streaming, voice chat, or music tools receive all necessary power without interruption.",
            default: true,
            risk: RiskLevel::Performance,
        },
    ]);

    // Advanced GPU - minimal input lag
    opts.extend([
        OptItem {
            id: "enable_game_mode",
            label: "Enable Windows Game Mode",
            description: "Activates Windows Game Mode. Prioritizes game processes for CPU and GPU access, while suspending driver updates and disruptive system notifications.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "prerendered_frames",
            label: "Low Latency: Pre-rendered frames = 1",
            description: "Forces the system to prepare only one frame in advance. Reduces the delay between user action and screen display, offering maximum responsiveness in competitive games.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "vrr_opt",
            label: "Disable VRR Optimizations",
            description: "Disables variable refresh rate (VRR) optimizations that can sometimes cause flickering or frame-time inconsistencies on certain G-Sync/FreeSync monitors.",
            default: true,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "shader_cache",
            label: "Shader Cache: 256MB (Stutter reduction)",
            description: "Sets a fixed and generous shader cache size. Avoids mid-game shader compilation that causes brutal stutters, ensuring constant visual fluidity when discovering new areas.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]);

    opts
}

// ============================================================================
// SECTION 4: SCHEDULER
// ============================================================================

#[allow(dead_code)]
/// Returns options for the Scheduler section
pub fn scheduler_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "priority_sep",
            label: "Win32PrioritySeparation 0x26 (Fixed)",
            description: "Optimizes CPU time allocation between foreground and background applications. The value 0x26 heavily favors the active application, ensuring your game receives absolute priority over processor resources.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "mmcss",
            label: "MMCSS Gaming Profile (Priority 6)",
            description: "Configures the Multimedia Class Scheduler Service (MMCSS) for the 'Games' profile. Increases the priority of game threads at the kernel level, preventing minor system tasks from interrupting the rendering flow.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "games_priority",
            label: "GPU Priority 8 / Scheduling 6",
            description: "Adjusts GPU scheduling priorities in the registry. Forces Windows to grant higher processing priority to game applications, reducing queue delays at the graphics driver level.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "global_timer",
            label: "Global Timer Resolution (Reboot)",
            description: "Applies a kernel-level modification to force a constant and global timer resolution. Eliminates timing variations that can cause 'jitter' and a lack of smoothness despite high FPS.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "startup_delay",
            label: "Disable Startup Delay (0ms)",
            description: "Removes the artificial delay imposed by Windows at startup before launching user applications. Allows you to reach the desktop and be operational several seconds faster.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "shutdown_timeout",
            label: "Shutdown Timeout 2000ms",
            description: "Reduces the time Windows waits before forcing applications to close during shutdown. Significantly speeds up the system shutdown and restart process.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

// ============================================================================
// SECTION 5: APPX BLOATWARE
// ============================================================================

#[allow(dead_code)]
/// Returns options for the AppX section
pub fn appx_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "bing_apps",
            label: "Bing Apps (News, Weather, Finance, Sports)",
            description: "Removes Bing-based Microsoft applications. These applications are often pre-installed and update in the background, consuming resources for services often better accessed via a browser.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "ms_productivity",
            label: "Productivity Apps (Todos, People, OfficeHub)",
            description: "Eliminates pre-installed 'productivity' tools that clutter the Start menu and system. Frees up disk space and cleans the user interface of superfluous elements.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "ms_media",
            label: "Media Apps (ZuneMusic, ZuneVideo, Clipchamp)",
            description: "Uninstalls Microsoft's default media players and video editing tools. Recommended if you use more powerful alternatives like VLC, MPC-HC, or professional editing suites.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "ms_communication",
            label: "Mail/Calendar, Skype, Teams",
            description: "Removes integrated communication applications. These applications tend to launch automatically at startup and maintain active connections, impacting memory and network.",
            default: true,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "ms_legacy",
            label: "Legacy Apps (Paint3D, 3DBuilder, Print3D)",
            description: "Cleans the system of obsolete or rarely used 3D applications. Reduces Windows' overall footprint and eliminates components that no longer meet the needs of most users.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "ms_tools",
            label: "Tools (FeedbackHub, GetHelp, QuickAssist)",
            description: "Removes assistance and feedback tools. Prevents Windows from prompting to rate the system and eliminates support services rarely used by advanced users.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "third_party",
            label: "Third-party (Spotify, Disney+, Facebook)",
            description: "Cleans up 'sponsored' third-party applications that Windows often installs without explicit consent during initial setup or major updates.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "copilot",
            label: "Microsoft Copilot - Disable AI",
            description: "Completely disables Microsoft Copilot integration in Windows. Removes the taskbar icon, disables associated background services, and frees up CPU/NPU resources dedicated to artificial intelligence.",
            default: true,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "cortana_app",
            label: "Cortana",
            description: "Removes the remaining Cortana application. Although largely replaced, the application may still reside on the system and consume minimal resources unnecessarily.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "xbox",
            label: "Xbox apps (caution Game Pass)",
            description: "Uninstalls the Xbox ecosystem (Game Bar, Provider, App). CAUTION: This option will make Xbox Game Pass unusable. Recommended only for players exclusively using Steam, Epic, or other platforms.",
            default: true,
            risk: RiskLevel::Conditional,
        },
    ]
}

// ============================================================================
// SECTION 6: CPU / MEMORY
// ============================================================================

#[allow(dead_code)]
/// Returns options for the CPU/Memory section
/// `is_laptop` adapts options for battery
pub fn cpu_section(is_laptop: bool) -> Vec<OptItem> {
    let mut opts = Vec::with_capacity(5);

    if is_laptop {
        opts.push(OptItem {
            id: "core_parking",
            label: "Disable Core Parking - Battery impact",
            description: "Prevents Windows from putting CPU cores to sleep (parking). Ensures all cores are instantly available for computation, eliminating wake-up latency. Significantly impacts battery life.",
            default: true,
            risk: RiskLevel::Warning,
        });
    } else {
        opts.push(OptItem {
            id: "core_parking",
            label: "Disable Core Parking - All cores active",
            description: "Disables 'Core Parking' at the kernel level. Forces the processor to maintain all physical and logical cores active, avoiding micro-stutters when redistributing workloads between cores.",
            default: true,
            risk: RiskLevel::Safe,
        });
    }

    opts.extend([
        OptItem {
            id: "memory_compression",
            label: "Disable Memory Compression (16GB+ RAM)",
            description: "Disables RAM compression. Compression saves RAM but consumes CPU cycles to compress/decompress data. On systems with 16GB+ RAM, disabling it improves overall responsiveness.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "superfetch_registry",
            label: "Disable Superfetch/Prefetch via registry",
            description: "Disables Prefetcher and Superfetch preloading mechanisms at the registry level. Reduces disk activity at startup and prevents Windows from trying to 'guess' your habits, freeing up resources for your actual tasks.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "static_pagefile",
            label: "Static Page File (1.5x RAM)",
            description: "Configures a fixed-size page file (1.5x RAM). Avoids file fragmentation and system slowdowns when Windows must dynamically resize virtual memory under heavy load.",
            default: true,
            risk: RiskLevel::Conditional,
        },
    ]);

    opts
}

// ============================================================================
// SECTION 7: DPC LATENCY
// ============================================================================

#[allow(dead_code)]
/// Returns options for the DPC Latency section (micro-stuttering)
pub fn dpc_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "paging_executive",
            label: "DisablePagingExecutive - Kernel in RAM",
            description: "Forces Windows to keep the kernel and drivers in physical RAM rather than moving them to the page file on disk. Drastically reduces system latencies when accessing critical kernel functions.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "dynamic_tick",
            label: "Disable Dynamic Tick - Reboot required",
            description: "Disables 'Dynamic Ticking', a power-saving feature that varies the system timer interval. Disabling it stabilizes the processor rhythm, crucial for audio synchronization and game fluidity.",
            default: true,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "tsc_sync",
            label: "TSC Sync Enhanced - Precision timer",
            description: "Improves Time Stamp Counter (TSC) synchronization between different processor cores. Ensures ultra-precise and consistent time measurement, eliminating a common source of micro-stutters in game engines.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "hpet",
            label: "Disable HPET - Test with LatencyMon",
            description: "Disables the High Precision Event Timer (HPET). On many modern architectures, HPET can introduce additional latency. Disabling it forces the use of faster hardware timers integrated into the CPU.",
            default: true,
            risk: RiskLevel::Conditional,
        },
        OptItem {
            id: "interrupt_affinity",
            label: "Interrupt Affinity Spread - Core distribution",
            description: "Intelligently distributes hardware interrupts (GPU, Network, USB) across different CPU cores. Prevents a single core from being saturated by interrupts, improving overall smoothness and system responsiveness under load.",
            default: true,
            risk: RiskLevel::Performance,
        },
    ]
}

// ============================================================================
// SECTION 8: SECURITY (CAUTION)
// ============================================================================

#[allow(dead_code)]
/// Returns options for the Security section
/// WARNING: Security risk options - isolated gaming systems only
pub fn security_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "hvci",
            label: "Memory Integrity (HVCI) - Off (+5% FPS)",
            description: "Disables hypervisor-based memory integrity. This security feature uses virtualization to protect processes but heavily impacts CPU performance and latency. Disabling it frees up raw power.",
            default: true,
            risk: RiskLevel::Warning,
        },
        OptItem {
            id: "vbs",
            label: "Virtualization Based Security (VBS) - Off",
            description: "Disables Virtualization-Based Security (VBS). VBS creates an isolated memory zone for security, which can reduce gaming performance by 10-25% on some processors. Recommended for pure gaming.",
            default: true,
            risk: RiskLevel::Warning,
        },
        OptItem {
            id: "spectre",
            label: "Spectre/Meltdown Mitigations - Off (RISK)",
            description: "Disables software protections against Spectre and Meltdown CPU vulnerabilities. CAUTION: Major security risk. However, it restores lost performance (up to 15%) on older Intel/AMD processors.",
            default: false,
            risk: RiskLevel::Critical,
        },
        OptItem {
            id: "defender_realtime",
            label: "Windows Defender Real-time Protection",
            description: "Disables Windows Defender real-time scanning. Eliminates file scans during every read/write, reducing CPU usage and untimely disk access during gaming or intensive work.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "uac_level",
            label: "UAC Level: Notify only on apps",
            description: "Adjusts User Account Control (UAC) to be less intrusive. Prevents screen dimming during elevation requests, avoiding temporary freezes and workflow interruptions.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "firewall_status",
            label: "Windows Firewall: Active/Optimized",
            description: "Optimizes Windows Firewall settings to reduce local packet inspection while maintaining perimeter protection. Slightly improves internal network latency.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "smartscreen",
            label: "Disable SmartScreen (Advanced users)",
            description: "Disables Windows SmartScreen which checks every downloaded file and executed application via Microsoft servers. Speeds up the launch of new programs and strengthens privacy.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

// ============================================================================
// SECTION 9: ADVANCED NETWORK
// ============================================================================

#[allow(dead_code)]
/// Returns options for the Advanced Network section
pub fn network_advanced_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "interrupt_moderation",
            label: "Disable Interrupt Moderation - Latency",
            description: "Disables network card interrupt moderation. Instead of grouping packets to save CPU, the card processes each packet instantly, reducing ping to the absolute minimum.",
            default: true,
            risk: RiskLevel::Performance,
        },
        OptItem {
            id: "lso",
            label: "Disable Large Send Offload (LSO)",
            description: "Disables large send offload. LSO can cause network instability and unpredictable latencies by delegating too much work to the network card hardware. Disabling it stabilizes the flow.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "eee",
            label: "Disable Energy Efficient Ethernet",
            description: "Disables Energy Efficient Ethernet (EEE). Prevents the network card from entering low-power mode, avoiding micro-wake-up delays when receiving data after a short pause.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "rss",
            label: "Enable Receive Side Scaling (RSS)",
            description: "Enables Receive Side Scaling. Allows network traffic processing to be distributed across multiple CPU cores, preventing a single core from becoming a bottleneck during massive downloads.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "rsc",
            label: "Disable Receive Segment Coalescing",
            description: "Disables receive segment coalescing. Like interrupt moderation, this ensures each packet is processed as soon as it arrives, crucial for online game responsiveness.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

// ============================================================================
// SECTION 10: DNS
// ============================================================================

#[allow(dead_code)]
/// Returns options for the DNS section
pub fn dns_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "doh_cloudflare",
            label: "Enable DNS-over-HTTPS (Cloudflare)",
            description: "Enables secure DNS via Cloudflare (1.1.1.1). Encrypts your DNS queries to prevent your ISP from tracking your browsing and often offers faster resolution than default servers.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "doh_google",
            label: "Enable DNS-over-HTTPS (Google)",
            description: "Enables secure DNS via Google (8.8.8.8). A robust and fast alternative for domain name resolution with HTTPS encryption.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "doh_quad9",
            label: "Enable DNS-over-HTTPS (Quad9)",
            description: "Enables secure DNS via Quad9 (9.9.9.9). Focuses on security by blocking access to known malicious domains while respecting privacy.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "dns_flush",
            label: "Flush DNS Resolver Cache",
            description: "Clears the local DNS resolver cache. Useful for resolving connection issues or ensuring DNS configuration changes take effect immediately.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

#[allow(dead_code)]
/// Returns options for the System Audit section
pub fn audit_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "audit_hardware",
            label: "Hardware Inventory (CPU/RAM/GPU)",
            description: "Generates a detailed report on physical components. Includes precise specifications, firmware versions, and health status, crucial for diagnosing hardware bottlenecks.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "audit_security",
            label: "Security Baseline Audit",
            description: "Compares the current system configuration to recommended security standards (CIS/Microsoft). Identifies configuration weaknesses that could be exploited.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "audit_services",
            label: "Critical Services Status",
            description: "Checks the operational status of vital Windows services. Ensures protection and update components are active and optimally configured.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "audit_network",
            label: "Network Configuration Audit",
            description: "Analyzes network interfaces, open ports, and active connections. Helps detect potential data leaks or unnecessarily exposed services on the network.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "audit_software",
            label: "Installed Software Analysis",
            description: "Lists all installed software and searches for outdated or vulnerable versions. Helps maintain rigorous software hygiene to reduce the attack surface.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

#[allow(dead_code)]
/// Returns options for the Settings section
pub fn settings_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "ui_theme_minimal",
            label: "Theme: Dark Minimalist (Default)",
            description: "Applies a clean dark interface designed to reduce eye strain and maximize readability of essential technical information.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "ui_theme_classic",
            label: "Theme: Classic Terminal",
            description: "Switches to a retro-computing aesthetic, reminiscent of classic terminals. Ideal for command-line purists and high-visibility environments.",
            default: false,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "ui_animations",
            label: "Enable Micro-animations",
            description: "Enables smooth transitions and subtle visual feedback during interactions. Improves user experience without impacting TUI rendering performance.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "sys_auto_elevate",
            label: "Auto-elevate on Startup",
            description: "Attempts to automatically obtain Administrator privileges when launching the application. Avoids interruptions and ensures all optimizations can be applied without failure.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "sys_log_level",
            label: "Verbose Logging Mode",
            description: "Enables detailed logging of all operations performed. Essential for troubleshooting and precisely understanding system modifications.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

#[allow(dead_code)]
/// Returns options for the Cleanup section
pub fn cleanup_section() -> Vec<OptItem> {
    vec![
        OptItem {
            id: "cleanup_temp",
            label: "Clean Temporary Files (System/User)",
            description: "Deletes temporary files accumulated by Windows and applications. Frees up disk space and eliminates obsolete files that can cause software conflicts.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cleanup_winsxs",
            label: "Clean WinSxS (Windows Update)",
            description: "Analyzes and cleans the Windows component store (WinSxS). Removes old component versions after update installation, often freeing several gigabytes.",
            default: true,
            risk: RiskLevel::Safe,
        },
        OptItem {
            id: "cleanup_edge",
            label: "Clean Edge Browser Cache",
            description: "Clears browsing data, cache, and cookies from Microsoft Edge. Improves privacy and can resolve browser display or slowness issues.",
            default: true,
            risk: RiskLevel::Safe,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_section_not_empty() {
        let section = telemetry_section();
        assert!(!section.is_empty());
    }

    #[test]
    fn test_all_sections_have_unique_ids() {
        let mut all_ids = Vec::new();
        all_ids.extend(telemetry_section().iter().map(|o| o.id));
        all_ids.extend(privacy_section().iter().map(|o| o.id));
        all_ids.extend(performance_section(false).iter().map(|o| o.id));
        all_ids.extend(scheduler_section().iter().map(|o| o.id));
        all_ids.extend(appx_section().iter().map(|o| o.id));
        all_ids.extend(cpu_section(false).iter().map(|o| o.id));
        all_ids.extend(dpc_section().iter().map(|o| o.id));
        all_ids.extend(security_section().iter().map(|o| o.id));
        all_ids.extend(network_advanced_section().iter().map(|o| o.id));

        let unique_count = {
            let mut sorted = all_ids.clone();
            sorted.sort();
            sorted.dedup();
            sorted.len()
        };

        assert_eq!(
            all_ids.len(),
            unique_count,
            "Duplicate IDs found in sections"
        );
    }
}
