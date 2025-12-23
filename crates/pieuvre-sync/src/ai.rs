//! Management of AI features (Recall, CoPilot)
//!
//! Registry keys and policies management for AI features.

use crate::operation::{RegistryDwordOperation, SyncOperation};
use async_trait::async_trait;
use pieuvre_common::{ChangeRecord, Result};

/// Operation to disable Windows Recall
pub struct DisableRecallOperation;

#[async_trait]
impl SyncOperation for DisableRecallOperation {
    fn name(&self) -> &str {
        "Disable Windows Recall"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let mut changes = Vec::new();

        // 1. Policy: Disable Recall
        let op = RegistryDwordOperation {
            key: r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI".to_string(),
            value: "DisableAIDataAnalysis".to_string(),
            target_data: 1,
        };
        changes.extend(op.apply().await?);

        // 2. User Settings: Disable Recall
        let op_user = RegistryDwordOperation {
            key: r"SOFTWARE\Microsoft\Windows\CurrentVersion\AI\DataAnalysis".to_string(),
            value: "Allowed".to_string(),
            target_data: 0,
        };
        changes.extend(op_user.apply().await?);

        Ok(changes)
    }

    async fn is_applied(&self) -> Result<bool> {
        let val1 = crate::registry::read_dword_value(
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsAI",
            "DisableAIDataAnalysis",
        )
        .unwrap_or(0);
        let val2 = crate::registry::read_dword_value(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\AI\DataAnalysis",
            "Allowed",
        )
        .unwrap_or(1);
        Ok(val1 == 1 && val2 == 0)
    }
}

/// Operation to disable CoPilot
pub struct DisableCoPilotOperation;

#[async_trait]
impl SyncOperation for DisableCoPilotOperation {
    fn name(&self) -> &str {
        "Disable Windows CoPilot"
    }

    async fn apply(&self) -> Result<Vec<ChangeRecord>> {
        let mut changes = Vec::new();

        // 1. Policy: Disable CoPilot
        let op = RegistryDwordOperation {
            key: r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot".to_string(),
            value: "TurnOffWindowsCopilot".to_string(),
            target_data: 1,
        };
        changes.extend(op.apply().await?);

        // 2. Taskbar: Hide CoPilot button
        let op_taskbar = RegistryDwordOperation {
            key: r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced".to_string(),
            value: "ShowCopilotButton".to_string(),
            target_data: 0,
        };
        changes.extend(op_taskbar.apply().await?);

        // 3. Edge: Disable CoPilot
        let op_edge = RegistryDwordOperation {
            key: r"SOFTWARE\Policies\Microsoft\Edge".to_string(),
            value: "HubsSidebarEnabled".to_string(),
            target_data: 0,
        };
        changes.extend(op_edge.apply().await?);

        Ok(changes)
    }

    async fn is_applied(&self) -> Result<bool> {
        let val1 = crate::registry::read_dword_value(
            r"SOFTWARE\Policies\Microsoft\Windows\WindowsCopilot",
            "TurnOffWindowsCopilot",
        )
        .unwrap_or(0);
        let val2 = crate::registry::read_dword_value(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "ShowCopilotButton",
        )
        .unwrap_or(1);
        Ok(val1 == 1 && val2 == 0)
    }
}
