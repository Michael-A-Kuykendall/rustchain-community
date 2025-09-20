//! Trajectory Tracking System for Agent Reinforcement Training
//! 
//! Captures and analyzes agent behavior for learning improvements

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::core::agent::AgentAction;

/// Complete trajectory capture for agent learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARTTrajectory {
    pub session_id: String,
    pub agent_name: String,
    pub objective: String,
    pub steps: Vec<TrajectoryStep>,
    pub metadata: TrajectoryMetadata,
    pub performance_metrics: PerformanceMetrics,
    pub reward_score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryStep {
    pub step_id: usize,
    pub observation: String,
    pub thought: String,
    pub action: AgentAction,
    pub action_input: String,
    pub tool_result: Option<String>,
    pub reflection: Option<String>,
    pub step_reward: Option<f64>,
    pub step_metadata: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryMetadata {
    pub model_used: String,
    pub total_tokens: u32,
    pub total_cost: f64,
    pub tools_used: Vec<String>,
    pub success_indicators: HashMap<String, bool>,
    pub error_count: u32,
    pub efficiency_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub task_completion_rate: f64,
    pub average_step_time: f64,
    pub tool_success_rate: f64,
    pub reasoning_quality_score: f64,
    pub memory_efficiency: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            task_completion_rate: 0.0,
            average_step_time: 0.0,
            tool_success_rate: 0.0,
            reasoning_quality_score: 0.0,
            memory_efficiency: 0.0,
        }
    }
}
