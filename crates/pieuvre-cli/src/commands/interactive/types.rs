use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
    Performance,
    Conditional,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptItem {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default: bool,
    pub risk: RiskLevel,
}

pub struct ExecutionResult {
    pub _affected_count: usize,
    pub message: String,
}

impl ExecutionResult {
    pub fn ok(msg: impl Into<String>) -> Self {
        Self {
            _affected_count: 1,
            message: msg.into(),
        }
    }
    pub fn ok_count(count: usize, msg: impl Into<String>) -> Self {
        Self {
            _affected_count: count,
            message: msg.into(),
        }
    }
}

#[async_trait]
pub trait TweakCommand: Send + Sync {
    async fn execute(&self) -> Result<ExecutionResult>;
    async fn check_status(&self) -> Result<bool> {
        Ok(false) // Par défaut, on ne sait pas si c'est appliqué
    }
}
