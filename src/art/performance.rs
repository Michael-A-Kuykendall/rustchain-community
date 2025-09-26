//! Performance Judging and RULER Algorithm for ART System
//! 
//! Implements automated reward scoring for agent trajectories


#[derive(Debug, Clone)]
pub struct PerformanceJudge {
    pub evaluation_prompts: EvaluationPromptBank,
    pub scoring_criteria: ScoringCriteria,
}

#[derive(Debug, Clone)]
pub struct ScoringCriteria {
    pub task_completion_weight: f64,
    pub efficiency_weight: f64,
    pub reasoning_quality_weight: f64,
    pub tool_usage_weight: f64,
}

#[derive(Debug, Clone)]
pub struct EvaluationPromptBank {
    pub success_prompts: Vec<String>,
    pub failure_prompts: Vec<String>,
}

impl Default for ScoringCriteria {
    fn default() -> Self {
        Self {
            task_completion_weight: 0.4,
            efficiency_weight: 0.2,
            reasoning_quality_weight: 0.25,
            tool_usage_weight: 0.15,
        }
    }
}
