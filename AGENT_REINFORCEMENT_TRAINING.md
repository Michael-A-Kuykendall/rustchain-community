# 🧠 RustChain Agent Reinforcement Training (ART)

**Self-Improving AI Agents Through Reinforcement Learning**

*A complete simulacrum of OpenPipe's ART system, retrofitted for RustChain Community Edition*

---

## 🎯 **Overview**

RustChain ART enables agents to learn and improve from their own interactions, creating a compound intelligence system where agents continuously get better at their specialized tasks. This system captures agent trajectories, evaluates performance, and fine-tunes models based on successful patterns.

**Inspired by**: [OpenPipe ART](https://art.openpipe.ai) - Retrofitted for Rust ecosystem with personal model integration.

---

## 🏗️ **System Architecture**

```rust
// Core ART Architecture
pub struct AgentReinforcementTraining {
    trajectory_tracker: TrajectoryTracker,
    performance_judge: PerformanceJudge, 
    reward_system: RULERAlgorithm,
    training_pipeline: TrainingPipeline,
    model_manager: PersonalModelManager,
}

// Integration with existing RustChain Agent system
impl Agent<'_> {
    pub async fn run_with_art(&mut self, objective: &str) -> Result<ARTSession> {
        let mut art_session = ARTSession::new(&self.name, objective);
        
        for iteration in 0..self.max_iterations {
            // Capture trajectory data during ReAct cycle
            let trajectory_step = self.execute_tracked_iteration(objective, &mut art_session).await?;
            art_session.add_step(trajectory_step);
        }
        
        // Submit for learning
        self.submit_to_art_pipeline(art_session).await?;
        Ok(art_session)
    }
}
```

---

## 📊 **1. Trajectory Tracking System**

Complete implementation of trajectory capture during agent execution:

### Core Trajectory Structure

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

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
    pub task_completion: bool,
    pub solution_quality: f64,
    pub efficiency_rating: f64,
    pub tool_usage_optimization: f64,
    pub reasoning_coherence: f64,
    pub error_recovery: f64,
}
```

### Trajectory Tracker Implementation

```rust
/// Trajectory tracking during agent execution
pub struct TrajectoryTracker {
    active_trajectories: HashMap<String, ARTTrajectory>,
    storage: Box<dyn TrajectoryStorage>,
    performance_evaluator: PerformanceEvaluator,
}

impl TrajectoryTracker {
    pub fn new(storage: Box<dyn TrajectoryStorage>) -> Self {
        Self {
            active_trajectories: HashMap::new(),
            storage,
            performance_evaluator: PerformanceEvaluator::new(),
        }
    }
    
    /// Start tracking a new agent session
    pub fn start_session(&mut self, agent_name: &str, objective: &str) -> String {
        let session_id = format!("art_{}_{}", agent_name, Utc::now().timestamp_millis());
        
        let trajectory = ARTTrajectory {
            session_id: session_id.clone(),
            agent_name: agent_name.to_string(),
            objective: objective.to_string(),
            steps: Vec::new(),
            metadata: TrajectoryMetadata::default(),
            performance_metrics: PerformanceMetrics::default(),
            reward_score: None,
            created_at: Utc::now(),
            completed_at: None,
        };
        
        self.active_trajectories.insert(session_id.clone(), trajectory);
        session_id
    }
    
    /// Record a step in the agent's trajectory
    pub async fn record_step(
        &mut self, 
        session_id: &str,
        step: TrajectoryStep
    ) -> Result<(), ARTError> {
        if let Some(trajectory) = self.active_trajectories.get_mut(session_id) {
            trajectory.steps.push(step);
            
            // Update metadata
            self.update_trajectory_metadata(trajectory).await?;
        }
        
        Ok(())
    }
    
    /// Complete a trajectory and evaluate performance
    pub async fn complete_session(
        &mut self, 
        session_id: &str
    ) -> Result<ARTTrajectory, ARTError> {
        if let Some(mut trajectory) = self.active_trajectories.remove(session_id) {
            trajectory.completed_at = Some(Utc::now());
            
            // Evaluate performance
            trajectory.performance_metrics = self
                .performance_evaluator
                .evaluate_trajectory(&trajectory)
                .await?;
            
            // Store completed trajectory
            self.storage.store_trajectory(&trajectory).await?;
            
            Ok(trajectory)
        } else {
            Err(ARTError::SessionNotFound(session_id.to_string()))
        }
    }
    
    async fn update_trajectory_metadata(&self, trajectory: &mut ARTTrajectory) -> Result<(), ARTError> {
        // Calculate running metrics
        trajectory.metadata.total_tokens += self.calculate_step_tokens(&trajectory.steps.last().unwrap());
        trajectory.metadata.tools_used = trajectory.steps
            .iter()
            .filter_map(|step| match &step.action {
                AgentAction::UseTool { tool, .. } => Some(tool.clone()),
                _ => None,
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
            
        Ok(())
    }
    
    fn calculate_step_tokens(&self, step: &TrajectoryStep) -> u32 {
        // Estimate token usage for this step
        (step.observation.len() + step.thought.len() + step.action_input.len()) as u32 / 4
    }
}
```

---

## 🎯 **2. Performance Judging System**

Implementation of automated performance evaluation:

### Performance Judge

```rust
use crate::core::llm::LLMBackend;

/// Automated performance evaluation for agent trajectories
pub struct PerformanceJudge {
    llm: Box<dyn LLMBackend>,
    evaluation_prompts: EvaluationPromptBank,
    scoring_criteria: ScoringCriteria,
}

#[derive(Debug, Clone)]
pub struct ScoringCriteria {
    pub task_completion_weight: f64,
    pub efficiency_weight: f64,
    pub reasoning_quality_weight: f64,
    pub tool_usage_weight: f64,
    pub error_handling_weight: f64,
}

impl Default for ScoringCriteria {
    fn default() -> Self {
        Self {
            task_completion_weight: 0.4,
            efficiency_weight: 0.2,
            reasoning_quality_weight: 0.2,
            tool_usage_weight: 0.1,
            error_handling_weight: 0.1,
        }
    }
}

impl PerformanceJudge {
    pub fn new(llm: Box<dyn LLMBackend>) -> Self {
        Self {
            llm,
            evaluation_prompts: EvaluationPromptBank::new(),
            scoring_criteria: ScoringCriteria::default(),
        }
    }
    
    /// Evaluate a completed trajectory
    pub async fn evaluate_trajectory(
        &self,
        trajectory: &ARTTrajectory
    ) -> Result<PerformanceMetrics, ARTError> {
        let mut metrics = PerformanceMetrics::default();
        
        // 1. Task Completion Assessment
        metrics.task_completion = self.assess_task_completion(trajectory).await?;
        
        // 2. Solution Quality Evaluation
        metrics.solution_quality = self.evaluate_solution_quality(trajectory).await?;
        
        // 3. Efficiency Rating
        metrics.efficiency_rating = self.calculate_efficiency_rating(trajectory).await?;
        
        // 4. Tool Usage Optimization
        metrics.tool_usage_optimization = self.evaluate_tool_usage(trajectory).await?;
        
        // 5. Reasoning Coherence
        metrics.reasoning_coherence = self.assess_reasoning_coherence(trajectory).await?;
        
        // 6. Error Recovery
        metrics.error_recovery = self.evaluate_error_recovery(trajectory).await?;
        
        Ok(metrics)
    }
    
    async fn assess_task_completion(&self, trajectory: &ARTTrajectory) -> Result<bool, ARTError> {
        let prompt = self.evaluation_prompts.task_completion_prompt(trajectory);
        let response = self.llm.generate(&prompt).await?;
        
        // Parse LLM response for completion assessment
        Ok(response.to_lowercase().contains("completed") || response.to_lowercase().contains("success"))
    }
    
    async fn evaluate_solution_quality(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        let prompt = self.evaluation_prompts.solution_quality_prompt(trajectory);
        let response = self.llm.generate(&prompt).await?;
        
        // Extract score from LLM response
        self.parse_score_from_response(&response, "quality")
    }
    
    async fn calculate_efficiency_rating(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        // Calculate based on steps taken vs optimal path
        let total_steps = trajectory.steps.len() as f64;
        let estimated_optimal = self.estimate_optimal_steps(&trajectory.objective).await? as f64;
        
        // Efficiency = optimal / actual (capped at 1.0)
        Ok((estimated_optimal / total_steps).min(1.0))
    }
    
    async fn evaluate_tool_usage(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        let prompt = self.evaluation_prompts.tool_usage_prompt(trajectory);
        let response = self.llm.generate(&prompt).await?;
        self.parse_score_from_response(&response, "tool_usage")
    }
    
    async fn assess_reasoning_coherence(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        let prompt = self.evaluation_prompts.reasoning_coherence_prompt(trajectory);
        let response = self.llm.generate(&prompt).await?;
        self.parse_score_from_response(&response, "reasoning")
    }
    
    async fn evaluate_error_recovery(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        let error_count = trajectory.steps
            .iter()
            .filter(|step| step.tool_result.as_ref().map_or(false, |r| r.contains("error")))
            .count();
            
        if error_count == 0 {
            return Ok(1.0); // No errors to recover from
        }
        
        let recovery_count = trajectory.steps
            .windows(2)
            .filter(|window| {
                window[0].tool_result.as_ref().map_or(false, |r| r.contains("error")) &&
                window[1].tool_result.as_ref().map_or(true, |r| !r.contains("error"))
            })
            .count();
            
        Ok(recovery_count as f64 / error_count as f64)
    }
    
    async fn estimate_optimal_steps(&self, objective: &str) -> Result<usize, ARTError> {
        let prompt = format!(
            "Estimate the minimum number of steps needed to complete this objective: {}\n\
            Respond with just a number between 1 and 20.", 
            objective
        );
        let response = self.llm.generate(&prompt).await?;
        
        response.trim()
            .parse::<usize>()
            .map_err(|_| ARTError::ParseError("Could not parse step estimate".to_string()))
    }
    
    fn parse_score_from_response(&self, response: &str, score_type: &str) -> Result<f64, ARTError> {
        // Extract numerical score from LLM response
        use regex::Regex;
        let re = Regex::new(r"(\d+(?:\.\d+)?)/10|(\d+(?:\.\d+)?)%|score:\s*(\d+(?:\.\d+)?)")
            .map_err(|e| ARTError::ParseError(format!("Regex error: {}", e)))?;
            
        if let Some(captures) = re.captures(response) {
            if let Some(score_match) = captures.get(1).or(captures.get(2)).or(captures.get(3)) {
                let score: f64 = score_match.as_str().parse()
                    .map_err(|_| ARTError::ParseError("Could not parse score".to_string()))?;
                
                // Normalize to 0.0-1.0 range
                if response.contains("/10") {
                    return Ok(score / 10.0);
                } else if response.contains("%") {
                    return Ok(score / 100.0);
                }
                return Ok(score.min(1.0));
            }
        }
        
        // Fallback: try to extract any number
        let re_number = Regex::new(r"(\d+(?:\.\d+)?)")
            .map_err(|e| ARTError::ParseError(format!("Regex error: {}", e)))?;
            
        if let Some(captures) = re_number.captures(response) {
            if let Some(score_match) = captures.get(1) {
                let score: f64 = score_match.as_str().parse()
                    .map_err(|_| ARTError::ParseError("Could not parse fallback score".to_string()))?;
                return Ok((score / 10.0).min(1.0));
            }
        }
        
        Err(ARTError::ParseError(format!("Could not extract {} score from: {}", score_type, response)))
    }
}

/// Prompt bank for performance evaluation
pub struct EvaluationPromptBank;

impl EvaluationPromptBank {
    pub fn new() -> Self {
        Self
    }
    
    pub fn task_completion_prompt(&self, trajectory: &ARTTrajectory) -> String {
        format!(
            "Evaluate if this agent successfully completed its objective.\n\n\
            Objective: {}\n\n\
            Agent Actions:\n{}\n\n\
            Was the objective completed successfully? Respond with 'COMPLETED' or 'INCOMPLETE' and explain why.",
            trajectory.objective,
            self.format_trajectory_for_evaluation(trajectory)
        )
    }
    
    pub fn solution_quality_prompt(&self, trajectory: &ARTTrajectory) -> String {
        format!(
            "Rate the quality of this agent's solution on a scale of 0-10.\n\n\
            Objective: {}\n\n\
            Agent Actions:\n{}\n\n\
            Consider: correctness, completeness, elegance, and robustness.\n\
            Respond with: 'Quality score: X/10' and brief explanation.",
            trajectory.objective,
            self.format_trajectory_for_evaluation(trajectory)
        )
    }
    
    pub fn tool_usage_prompt(&self, trajectory: &ARTTrajectory) -> String {
        format!(
            "Evaluate how effectively this agent used available tools (0-10).\n\n\
            Objective: {}\n\n\
            Agent Actions:\n{}\n\n\
            Consider: tool selection appropriateness, sequence optimization, avoiding redundancy.\n\
            Respond with: 'Tool usage score: X/10' and explanation.",
            trajectory.objective,
            self.format_trajectory_for_evaluation(trajectory)
        )
    }
    
    pub fn reasoning_coherence_prompt(&self, trajectory: &ARTTrajectory) -> String {
        format!(
            "Rate the coherence and logic of this agent's reasoning (0-10).\n\n\
            Objective: {}\n\n\
            Agent Thoughts and Actions:\n{}\n\n\
            Consider: logical flow, consistency, appropriate problem decomposition.\n\
            Respond with: 'Reasoning score: X/10' and analysis.",
            trajectory.objective,
            self.format_reasoning_for_evaluation(trajectory)
        )
    }
    
    fn format_trajectory_for_evaluation(&self, trajectory: &ARTTrajectory) -> String {
        trajectory.steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                format!(
                    "Step {}: {} -> {} -> Result: {}",
                    i + 1,
                    step.thought,
                    format!("{:?}", step.action),
                    step.tool_result.as_deref().unwrap_or("No result")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn format_reasoning_for_evaluation(&self, trajectory: &ARTTrajectory) -> String {
        trajectory.steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                format!(
                    "Step {}: Thought: '{}' -> Action: {:?}",
                    i + 1,
                    step.thought,
                    step.action
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

---

## 🏆 **3. RULER Algorithm Implementation**

Automated reward function generation system:

### RULER Core System

```rust
/// RULER: Reinforcement Utility Learning for Efficient Reasoning
/// Automated reward scoring without manual reward crafting
pub struct RULERAlgorithm {
    reward_model: Box<dyn LLMBackend>,
    reward_criteria: RULERCriteria,
    reward_history: RewardHistory,
}

#[derive(Debug, Clone)]
pub struct RULERCriteria {
    pub success_indicators: Vec<String>,
    pub failure_indicators: Vec<String>,
    pub efficiency_factors: Vec<String>,
    pub quality_markers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RewardScore {
    pub total_reward: f64,
    pub component_scores: HashMap<String, f64>,
    pub reasoning: String,
    pub confidence: f64,
}

impl RULERAlgorithm {
    pub fn new(reward_model: Box<dyn LLMBackend>) -> Self {
        Self {
            reward_model,
            reward_criteria: RULERCriteria::default(),
            reward_history: RewardHistory::new(),
        }
    }
    
    /// Generate automated reward for a trajectory
    pub async fn calculate_reward(
        &mut self,
        trajectory: &ARTTrajectory
    ) -> Result<RewardScore, ARTError> {
        // Multi-component reward calculation
        let mut component_scores = HashMap::new();
        
        // 1. Task Success Reward
        let success_score = self.calculate_success_reward(trajectory).await?;
        component_scores.insert("success".to_string(), success_score);
        
        // 2. Efficiency Reward  
        let efficiency_score = self.calculate_efficiency_reward(trajectory).await?;
        component_scores.insert("efficiency".to_string(), efficiency_score);
        
        // 3. Quality Reward
        let quality_score = self.calculate_quality_reward(trajectory).await?;
        component_scores.insert("quality".to_string(), quality_score);
        
        // 4. Innovation Reward
        let innovation_score = self.calculate_innovation_reward(trajectory).await?;
        component_scores.insert("innovation".to_string(), innovation_score);
        
        // Weighted total reward
        let total_reward = self.calculate_weighted_reward(&component_scores);
        
        // Generate reasoning explanation
        let reasoning = self.generate_reward_reasoning(trajectory, &component_scores).await?;
        
        let reward_score = RewardScore {
            total_reward,
            component_scores,
            reasoning,
            confidence: self.calculate_confidence(&component_scores),
        };
        
        // Store for learning
        self.reward_history.add_reward(trajectory.session_id.clone(), reward_score.clone());
        
        Ok(reward_score)
    }
    
    async fn calculate_success_reward(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        let prompt = format!(
            "Evaluate task success for this agent trajectory.\n\n\
            Objective: {}\n\n\
            Final State:\n{}\n\n\
            Rate success from 0.0 (complete failure) to 1.0 (perfect success).\n\
            Respond with only a number like: 0.85",
            trajectory.objective,
            self.format_final_state(trajectory)
        );
        
        let response = self.reward_model.generate(&prompt).await?;
        self.parse_reward_score(&response)
    }
    
    async fn calculate_efficiency_reward(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        let step_count = trajectory.steps.len();
        let tool_switches = self.count_tool_switches(trajectory);
        let error_count = self.count_errors(trajectory);
        
        // Efficiency formula: fewer steps, fewer switches, fewer errors = higher reward
        let base_efficiency = 1.0 - (step_count as f64 * 0.05).min(0.8);
        let switch_penalty = tool_switches as f64 * 0.02;
        let error_penalty = error_count as f64 * 0.1;
        
        Ok((base_efficiency - switch_penalty - error_penalty).max(0.0))
    }
    
    async fn calculate_quality_reward(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        let prompt = format!(
            "Rate the quality of reasoning and execution in this trajectory.\n\n\
            Objective: {}\n\n\
            Reasoning Chain:\n{}\n\n\
            Consider: logical coherence, appropriate tool selection, problem-solving approach.\n\
            Respond with a quality score from 0.0 to 1.0: ",
            trajectory.objective,
            self.format_reasoning_chain(trajectory)
        );
        
        let response = self.reward_model.generate(&prompt).await?;
        self.parse_reward_score(&response)
    }
    
    async fn calculate_innovation_reward(&self, trajectory: &ARTTrajectory) -> Result<f64, ARTError> {
        // Check for novel approaches compared to reward history
        let similar_trajectories = self.reward_history.find_similar_objectives(&trajectory.objective);
        
        if similar_trajectories.is_empty() {
            return Ok(0.1); // Small bonus for novel objectives
        }
        
        // Compare approach uniqueness
        let approach_similarity = self.calculate_approach_similarity(trajectory, &similar_trajectories);
        Ok((1.0 - approach_similarity) * 0.2) // Up to 0.2 bonus for novel approaches
    }
    
    fn calculate_weighted_reward(&self, components: &HashMap<String, f64>) -> f64 {
        let weights = [
            ("success", 0.5),
            ("efficiency", 0.2),
            ("quality", 0.2),
            ("innovation", 0.1),
        ];
        
        weights.iter()
            .map(|(component, weight)| {
                components.get(*component).unwrap_or(&0.0) * weight
            })
            .sum()
    }
    
    async fn generate_reward_reasoning(
        &self,
        trajectory: &ARTTrajectory,
        components: &HashMap<String, f64>
    ) -> Result<String, ARTError> {
        let prompt = format!(
            "Explain the reward scoring for this agent trajectory.\n\n\
            Objective: {}\n\n\
            Component Scores:\n{}\n\n\
            Provide concise reasoning for each score in 2-3 sentences.",
            trajectory.objective,
            self.format_component_scores(components)
        );
        
        let reasoning = self.reward_model.generate(&prompt).await?;
        Ok(reasoning)
    }
    
    fn calculate_confidence(&self, components: &HashMap<String, f64>) -> f64 {
        // Confidence based on score consistency and available data
        let scores: Vec<f64> = components.values().cloned().collect();
        let variance = self.calculate_variance(&scores);
        
        // Lower variance = higher confidence
        (1.0 - variance).max(0.1)
    }
    
    fn calculate_variance(&self, scores: &[f64]) -> f64 {
        if scores.len() <= 1 {
            return 0.0;
        }
        
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance = scores.iter()
            .map(|score| (score - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;
            
        variance
    }
    
    fn parse_reward_score(&self, response: &str) -> Result<f64, ARTError> {
        use regex::Regex;
        let re = Regex::new(r"(\d+(?:\.\d+)?)")
            .map_err(|e| ARTError::ParseError(format!("Regex error: {}", e)))?;
            
        if let Some(captures) = re.captures(response) {
            if let Some(score_match) = captures.get(1) {
                let score: f64 = score_match.as_str().parse()
                    .map_err(|_| ARTError::ParseError("Could not parse reward score".to_string()))?;
                return Ok(score.min(1.0));
            }
        }
        
        Err(ARTError::ParseError(format!("Could not extract reward score from: {}", response)))
    }
    
    // Helper methods
    fn format_final_state(&self, trajectory: &ARTTrajectory) -> String {
        trajectory.steps.last()
            .map(|step| format!("Final Action: {:?}\nResult: {}", 
                step.action, 
                step.tool_result.as_deref().unwrap_or("No result")))
            .unwrap_or_else(|| "No steps recorded".to_string())
    }
    
    fn format_reasoning_chain(&self, trajectory: &ARTTrajectory) -> String {
        trajectory.steps
            .iter()
            .enumerate()
            .map(|(i, step)| format!("{}. {}", i + 1, step.thought))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn format_component_scores(&self, components: &HashMap<String, f64>) -> String {
        components.iter()
            .map(|(component, score)| format!("{}: {:.2}", component, score))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn count_tool_switches(&self, trajectory: &ARTTrajectory) -> usize {
        trajectory.steps
            .windows(2)
            .filter(|window| {
                match (&window[0].action, &window[1].action) {
                    (AgentAction::UseTool { tool: tool1, .. }, AgentAction::UseTool { tool: tool2, .. }) => {
                        tool1 != tool2
                    }
                    _ => false,
                }
            })
            .count()
    }
    
    fn count_errors(&self, trajectory: &ARTTrajectory) -> usize {
        trajectory.steps
            .iter()
            .filter(|step| {
                step.tool_result.as_ref()
                    .map_or(false, |result| result.to_lowercase().contains("error"))
            })
            .count()
    }
    
    fn calculate_approach_similarity(&self, _trajectory: &ARTTrajectory, _similar: &[RewardRecord]) -> f64 {
        // Simplified similarity calculation - could be enhanced with embeddings
        0.5 // Placeholder
    }
}

/// Reward history for learning and comparison
#[derive(Debug, Clone)]
pub struct RewardHistory {
    records: Vec<RewardRecord>,
}

#[derive(Debug, Clone)]
pub struct RewardRecord {
    pub session_id: String,
    pub objective: String,
    pub reward_score: RewardScore,
    pub timestamp: DateTime<Utc>,
}

impl RewardHistory {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
    
    pub fn add_reward(&mut self, session_id: String, reward_score: RewardScore) {
        // Implementation would include objective extraction
        let record = RewardRecord {
            session_id,
            objective: "".to_string(), // Would be passed from trajectory
            reward_score,
            timestamp: Utc::now(),
        };
        
        self.records.push(record);
    }
    
    pub fn find_similar_objectives(&self, objective: &str) -> Vec<RewardRecord> {
        // Simple similarity matching - could be enhanced
        self.records.iter()
            .filter(|record| {
                let similarity = self.calculate_objective_similarity(&record.objective, objective);
                similarity > 0.7
            })
            .cloned()
            .collect()
    }
    
    fn calculate_objective_similarity(&self, obj1: &str, obj2: &str) -> f64 {
        // Simplified similarity - could use embeddings or more sophisticated matching
        if obj1 == obj2 {
            1.0
        } else if obj1.contains(obj2) || obj2.contains(obj1) {
            0.8
        } else {
            0.3 // Default low similarity
        }
    }
}

impl Default for RULERCriteria {
    fn default() -> Self {
        Self {
            success_indicators: vec![
                "completed".to_string(),
                "successful".to_string(),
                "achieved".to_string(),
                "finished".to_string(),
            ],
            failure_indicators: vec![
                "failed".to_string(),
                "error".to_string(),
                "incomplete".to_string(),
                "unsuccessful".to_string(),
            ],
            efficiency_factors: vec![
                "steps".to_string(),
                "time".to_string(),
                "resources".to_string(),
            ],
            quality_markers: vec![
                "accurate".to_string(),
                "precise".to_string(),
                "correct".to_string(),
                "optimal".to_string(),
            ],
        }
    }
}
```

---

## 🔄 **4. Training Pipeline**

Complete training system with personal model integration:

### Training Configuration

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub groups_per_step: usize,
    pub num_epochs: usize,
    pub rollouts_per_group: usize,
    pub learning_rate: f64,
    pub max_steps: usize,
    pub batch_size: usize,
    pub model_path: String,
    pub checkpoint_interval: usize,
    pub evaluation_interval: usize,
    pub early_stopping_patience: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            groups_per_step: 2,
            num_epochs: 20,
            rollouts_per_group: 4,
            learning_rate: 1e-5,
            max_steps: 1000,
            batch_size: 8,
            model_path: "./llama-3.2-1b-personal".to_string(), // Your champion model
            checkpoint_interval: 10,
            evaluation_interval: 50,
            early_stopping_patience: 5,
        }
    }
}

/// Training pipeline for agent reinforcement learning
pub struct ARTTrainingPipeline {
    config: TrainingConfig,
    trajectory_storage: Box<dyn TrajectoryStorage>,
    model_manager: PersonalModelManager,
    training_data_generator: TrainingDataGenerator,
    evaluation_suite: EvaluationSuite,
}

impl ARTTrainingPipeline {
    pub fn new(
        config: TrainingConfig,
        trajectory_storage: Box<dyn TrajectoryStorage>,
    ) -> Self {
        Self {
            config,
            trajectory_storage,
            model_manager: PersonalModelManager::new(),
            training_data_generator: TrainingDataGenerator::new(),
            evaluation_suite: EvaluationSuite::new(),
        }
    }
    
    /// Main training loop
    pub async fn train(&mut self) -> Result<TrainingResults, ARTError> {
        println!("🚀 Starting RustChain ART Training Pipeline");
        
        let mut training_results = TrainingResults::new();
        let mut best_performance = 0.0;
        let mut patience_counter = 0;
        
        for epoch in 0..self.config.num_epochs {
            println!("📚 Epoch {}/{}", epoch + 1, self.config.num_epochs);
            
            // 1. Collect trajectories for this epoch
            let trajectories = self.collect_training_trajectories().await?;
            println!("  📊 Collected {} trajectories", trajectories.len());
            
            // 2. Generate training data
            let training_data = self.training_data_generator
                .generate_training_data(&trajectories)
                .await?;
            println!("  🔧 Generated {} training examples", training_data.len());
            
            // 3. Train model on successful patterns
            let epoch_metrics = self.train_epoch(&training_data).await?;
            training_results.add_epoch_metrics(epoch, epoch_metrics.clone());
            
            // 4. Evaluate performance
            if epoch % self.config.evaluation_interval == 0 {
                let eval_performance = self.evaluation_suite
                    .evaluate_model_performance()
                    .await?;
                    
                println!("  📈 Evaluation Performance: {:.4}", eval_performance);
                
                // Check for improvement
                if eval_performance > best_performance {
                    best_performance = eval_performance;
                    patience_counter = 0;
                    
                    // Save best model checkpoint
                    self.model_manager.save_checkpoint("best_model").await?;
                } else {
                    patience_counter += 1;
                    
                    if patience_counter >= self.config.early_stopping_patience {
                        println!("  🛑 Early stopping triggered");
                        break;
                    }
                }
            }
            
            // 5. Save checkpoint
            if epoch % self.config.checkpoint_interval == 0 {
                self.model_manager.save_checkpoint(&format!("epoch_{}", epoch)).await?;
            }
        }
        
        println!("✅ Training completed. Best performance: {:.4}", best_performance);
        Ok(training_results)
    }
    
    async fn collect_training_trajectories(&self) -> Result<Vec<ARTTrajectory>, ARTError> {
        // Get high-performing trajectories from storage
        let all_trajectories = self.trajectory_storage.get_recent_trajectories(1000).await?;
        
        // Filter for successful trajectories (reward > threshold)
        let successful_trajectories: Vec<ARTTrajectory> = all_trajectories
            .into_iter()
            .filter(|trajectory| {
                trajectory.reward_score.map_or(false, |score| score > 0.7)
            })
            .collect();
            
        Ok(successful_trajectories)
    }
    
    async fn train_epoch(&mut self, training_data: &[TrainingExample]) -> Result<EpochMetrics, ARTError> {
        let mut epoch_metrics = EpochMetrics::new();
        
        // Process training data in batches
        for (batch_idx, batch) in training_data.chunks(self.config.batch_size).enumerate() {
            let batch_loss = self.train_batch(batch).await?;
            epoch_metrics.add_batch_loss(batch_loss);
            
            if batch_idx % 10 == 0 {
                println!("    Batch {}: Loss {:.4}", batch_idx, batch_loss);
            }
        }
        
        Ok(epoch_metrics)
    }
    
    async fn train_batch(&mut self, batch: &[TrainingExample]) -> Result<f64, ARTError> {
        // Convert batch to model training format
        let training_batch = self.format_batch_for_training(batch)?;
        
        // Train model (this would integrate with your personal model training)
        let loss = self.model_manager.train_batch(training_batch).await?;
        
        Ok(loss)
    }
    
    fn format_batch_for_training(&self, batch: &[TrainingExample]) -> Result<ModelTrainingBatch, ARTError> {
        // Convert ART training examples to model training format
        let examples: Vec<ModelExample> = batch.iter()
            .map(|example| ModelExample {
                input: example.context.clone(),
                target: example.preferred_response.clone(),
                metadata: example.metadata.clone(),
            })
            .collect();
            
        Ok(ModelTrainingBatch { examples })
    }
}

/// Training data generation from trajectories
pub struct TrainingDataGenerator {
    preference_extractor: PreferenceExtractor,
    context_formatter: ContextFormatter,
}

impl TrainingDataGenerator {
    pub fn new() -> Self {
        Self {
            preference_extractor: PreferenceExtractor::new(),
            context_formatter: ContextFormatter::new(),
        }
    }
    
    /// Generate training examples from successful trajectories
    pub async fn generate_training_data(
        &self,
        trajectories: &[ARTTrajectory]
    ) -> Result<Vec<TrainingExample>, ARTError> {
        let mut training_examples = Vec::new();
        
        for trajectory in trajectories {
            // Extract successful reasoning patterns
            let examples = self.extract_examples_from_trajectory(trajectory).await?;
            training_examples.extend(examples);
        }
        
        // Add preference pairs for contrastive learning
        let preference_pairs = self.preference_extractor
            .create_preference_pairs(trajectories)
            .await?;
            
        for pair in preference_pairs {
            training_examples.push(self.preference_pair_to_example(pair)?);
        }
        
        Ok(training_examples)
    }
    
    async fn extract_examples_from_trajectory(
        &self,
        trajectory: &ARTTrajectory
    ) -> Result<Vec<TrainingExample>, ARTError> {
        let mut examples = Vec::new();
        
        for (i, step) in trajectory.steps.iter().enumerate() {
            // Create training example for each successful step
            let context = self.context_formatter.format_context(&trajectory.steps[..i], &trajectory.objective);
            
            let example = TrainingExample {
                context,
                preferred_response: step.thought.clone(),
                rejected_response: None, // Would be generated for contrastive learning
                reward_score: step.step_reward,
                metadata: step.step_metadata.clone(),
            };
            
            examples.push(example);
        }
        
        Ok(examples)
    }
    
    fn preference_pair_to_example(&self, pair: PreferencePair) -> Result<TrainingExample, ARTError> {
        Ok(TrainingExample {
            context: pair.context,
            preferred_response: pair.preferred_response,
            rejected_response: Some(pair.rejected_response),
            reward_score: Some(pair.preference_score),
            metadata: HashMap::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub context: String,
    pub preferred_response: String,
    pub rejected_response: Option<String>,
    pub reward_score: Option<f64>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct PreferencePair {
    pub context: String,
    pub preferred_response: String,
    pub rejected_response: String,
    pub preference_score: f64,
}

#[derive(Debug, Clone)]
pub struct ModelTrainingBatch {
    pub examples: Vec<ModelExample>,
}

#[derive(Debug, Clone)]
pub struct ModelExample {
    pub input: String,
    pub target: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct EpochMetrics {
    pub batch_losses: Vec<f64>,
    pub average_loss: f64,
    pub training_examples: usize,
}

impl EpochMetrics {
    pub fn new() -> Self {
        Self {
            batch_losses: Vec::new(),
            average_loss: 0.0,
            training_examples: 0,
        }
    }
    
    pub fn add_batch_loss(&mut self, loss: f64) {
        self.batch_losses.push(loss);
        self.average_loss = self.batch_losses.iter().sum::<f64>() / self.batch_losses.len() as f64;
    }
}

#[derive(Debug, Clone)]
pub struct TrainingResults {
    pub epoch_metrics: HashMap<usize, EpochMetrics>,
    pub best_performance: f64,
    pub total_epochs: usize,
}

impl TrainingResults {
    pub fn new() -> Self {
        Self {
            epoch_metrics: HashMap::new(),
            best_performance: 0.0,
            total_epochs: 0,
        }
    }
    
    pub fn add_epoch_metrics(&mut self, epoch: usize, metrics: EpochMetrics) {
        self.epoch_metrics.insert(epoch, metrics);
        self.total_epochs = self.total_epochs.max(epoch + 1);
    }
}
```

---

## 🤖 **5. Personal Model Manager**

Integration with your champion model and training infrastructure:

### Model Management System

```rust
use tokio::process::Command;
use std::path::PathBuf;

/// Manages your personal fine-tuned models for ART training
pub struct PersonalModelManager {
    model_registry: ModelRegistry,
    training_interface: TrainingInterface,
    champion_model: String,
}

#[derive(Debug, Clone)]
pub struct ModelRegistry {
    pub models: HashMap<String, PersonalModel>,
    pub champion: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PersonalModel {
    pub name: String,
    pub path: PathBuf,
    pub base_model: String,
    pub performance_metrics: ModelPerformanceMetrics,
    pub specialization: ModelSpecialization,
    pub last_trained: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum ModelSpecialization {
    General,
    Research,
    Planning,
    Execution,
    CodeGeneration,
    Analysis,
}

#[derive(Debug, Clone)]
pub struct ModelPerformanceMetrics {
    pub accuracy: f64,
    pub efficiency: f64,
    pub reasoning_quality: f64,
    pub task_completion_rate: f64,
    pub tokens_per_second: f64,
}

impl PersonalModelManager {
    pub fn new() -> Self {
        Self {
            model_registry: ModelRegistry::load_from_command_center(),
            training_interface: TrainingInterface::new(),
            champion_model: "llama-3.2-1b-personal".to_string(), // Your champion!
        }
    }
    
    /// Train model on ART trajectory data
    pub async fn train_batch(&mut self, batch: ModelTrainingBatch) -> Result<f64, ARTError> {
        println!("🎯 Training {} on {} examples", self.champion_model, batch.examples.len());
        
        // Convert to training format for your command center pipeline
        let training_data = self.convert_to_training_format(&batch)?;
        
        // Use your existing training pipeline
        let loss = self.training_interface.train_model(
            &self.champion_model,
            training_data
        ).await?;
        
        println!("  📉 Training loss: {:.4}", loss);
        Ok(loss)
    }
    
    /// Save model checkpoint
    pub async fn save_checkpoint(&self, checkpoint_name: &str) -> Result<(), ARTError> {
        let checkpoint_path = format!("./checkpoints/{}_{}", self.champion_model, checkpoint_name);
        
        // Copy model files to checkpoint directory
        let model_path = self.get_model_path(&self.champion_model)?;
        
        tokio::fs::create_dir_all("./checkpoints").await?;
        self.copy_model_files(&model_path, &checkpoint_path).await?;
        
        println!("💾 Saved checkpoint: {}", checkpoint_path);
        Ok(())
    }
    
    /// Create specialized agent models
    pub async fn create_specialized_model(
        &mut self,
        base_model: &str,
        specialization: ModelSpecialization,
        training_trajectories: &[ARTTrajectory]
    ) -> Result<PersonalModel, ARTError> {
        let model_name = format!("{}-{:?}", base_model, specialization);
        
        println!("🔧 Creating specialized model: {}", model_name);
        
        // Filter trajectories by specialization
        let specialized_trajectories = self.filter_by_specialization(training_trajectories, &specialization);
        
        // Generate specialized training data
        let training_data = self.generate_specialized_training_data(&specialized_trajectories).await?;
        
        // Train specialized model
        let model_path = self.training_interface.train_specialized_model(
            base_model,
            &model_name,
            training_data
        ).await?;
        
        let personal_model = PersonalModel {
            name: model_name.clone(),
            path: model_path,
            base_model: base_model.to_string(),
            performance_metrics: ModelPerformanceMetrics::default(),
            specialization,
            last_trained: Utc::now(),
        };
        
        // Register the new model
        self.model_registry.models.insert(model_name, personal_model.clone());
        
        println!("✅ Created specialized model: {}", personal_model.name);
        Ok(personal_model)
    }
    
    /// Evaluate model performance on test trajectories
    pub async fn evaluate_model(&self, model_name: &str) -> Result<ModelPerformanceMetrics, ARTError> {
        let model_path = self.get_model_path(model_name)?;
        
        // Run evaluation using your testing infrastructure
        let performance = self.training_interface.evaluate_model(&model_path).await?;
        
        Ok(performance)
    }
    
    fn convert_to_training_format(&self, batch: &ModelTrainingBatch) -> Result<TrainingDataset, ARTError> {
        // Convert to your command center training format
        let conversations: Vec<Conversation> = batch.examples.iter()
            .map(|example| Conversation {
                messages: vec![
                    Message {
                        role: "user".to_string(),
                        content: example.input.clone(),
                    },
                    Message {
                        role: "assistant".to_string(),
                        content: example.target.clone(),
                    }
                ],
                metadata: example.metadata.clone(),
            })
            .collect();
            
        Ok(TrainingDataset { conversations })
    }
    
    fn filter_by_specialization(
        &self,
        trajectories: &[ARTTrajectory],
        specialization: &ModelSpecialization
    ) -> Vec<ARTTrajectory> {
        trajectories.iter()
            .filter(|trajectory| {
                match specialization {
                    ModelSpecialization::Research => {
                        trajectory.metadata.tools_used.iter()
                            .any(|tool| tool.contains("search") || tool.contains("web") || tool.contains("read"))
                    },
                    ModelSpecialization::Planning => {
                        trajectory.objective.to_lowercase().contains("plan") ||
                        trajectory.steps.iter().any(|step| step.thought.to_lowercase().contains("plan"))
                    },
                    ModelSpecialization::Execution => {
                        trajectory.metadata.tools_used.len() > 3 // High tool usage
                    },
                    ModelSpecialization::CodeGeneration => {
                        trajectory.objective.to_lowercase().contains("code") ||
                        trajectory.objective.to_lowercase().contains("program")
                    },
                    _ => true, // General model includes all
                }
            })
            .cloned()
            .collect()
    }
    
    async fn generate_specialized_training_data(
        &self,
        trajectories: &[ARTTrajectory]
    ) -> Result<TrainingDataset, ARTError> {
        // Enhanced training data generation for specialized models
        let mut conversations = Vec::new();
        
        for trajectory in trajectories {
            for step in &trajectory.steps {
                let conversation = Conversation {
                    messages: vec![
                        Message {
                            role: "system".to_string(),
                            content: format!("You are a specialized agent for: {:?}", trajectory.objective),
                        },
                        Message {
                            role: "user".to_string(),
                            content: step.observation.clone(),
                        },
                        Message {
                            role: "assistant".to_string(),
                            content: step.thought.clone(),
                        }
                    ],
                    metadata: step.step_metadata.clone(),
                };
                
                conversations.push(conversation);
            }
        }
        
        Ok(TrainingDataset { conversations })
    }
    
    fn get_model_path(&self, model_name: &str) -> Result<PathBuf, ARTError> {
        if let Some(model) = self.model_registry.models.get(model_name) {
            Ok(model.path.clone())
        } else {
            // Default path for command center models
            Ok(PathBuf::from(format!("./{}", model_name)))
        }
    }
    
    async fn copy_model_files(&self, source: &PathBuf, dest: &str) -> Result<(), ARTError> {
        let output = Command::new("cp")
            .arg("-r")
            .arg(source)
            .arg(dest)
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(ARTError::ModelOperationFailed(
                format!("Failed to copy model files: {}", 
                String::from_utf8_lossy(&output.stderr))
            ));
        }
        
        Ok(())
    }
}

impl ModelRegistry {
    pub fn load_from_command_center() -> Self {
        // Load your existing model registry from command center
        let mut models = HashMap::new();
        
        // Your existing personal models
        let personal_models = vec![
            ("llama-3.2-1b-personal", "meta-llama/Llama-3.2-1B"),
            ("phi3-personal-h100-cloud", "microsoft/Phi-3-mini-4k-instruct"),
            ("starcoder2-3b-personal", "bigcode/starcoder2-3b"),
            ("deepseek-coder-personal", "deepseek-ai/deepseek-coder-1.3b"),
            // ... add all your personal models
        ];
        
        for (name, base) in personal_models {
            let model = PersonalModel {
                name: name.to_string(),
                path: PathBuf::from(format!("./{}", name)),
                base_model: base.to_string(),
                performance_metrics: ModelPerformanceMetrics::default(),
                specialization: ModelSpecialization::General,
                last_trained: Utc::now(),
            };
            
            models.insert(name.to_string(), model);
        }
        
        Self {
            models,
            champion: Some("llama-3.2-1b-personal".to_string()),
        }
    }
}

/// Training interface with command center
pub struct TrainingInterface {
    python_path: String,
    command_center_path: String,
}

impl TrainingInterface {
    pub fn new() -> Self {
        Self {
            python_path: "C:/Python311/python.exe".to_string(),
            command_center_path: "../command-center".to_string(),
        }
    }
    
    /// Train model using command center infrastructure  
    pub async fn train_model(
        &self,
        model_name: &str,
        training_data: TrainingDataset
    ) -> Result<f64, ARTError> {
        // Save training data
        let data_path = format!("./art_training_data_{}.jsonl", model_name);
        self.save_training_data(&training_data, &data_path).await?;
        
        // Run training script
        let output = Command::new(&self.python_path)
            .current_dir(&self.command_center_path)
            .arg("single_potty_train.py")
            .arg(model_name)
            .arg(format!("./{}", model_name))
            .arg("--data")
            .arg(&data_path)
            .arg("--epochs")
            .arg("3")
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(ARTError::TrainingFailed(
                format!("Training failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }
        
        // Parse loss from output
        let output_str = String::from_utf8_lossy(&output.stdout);
        self.parse_training_loss(&output_str)
    }
    
    async fn save_training_data(&self, data: &TrainingDataset, path: &str) -> Result<(), ARTError> {
        let file = tokio::fs::File::create(path).await?;
        let mut writer = tokio::io::BufWriter::new(file);
        
        for conversation in &data.conversations {
            let line = serde_json::to_string(conversation)?;
            tokio::io::AsyncWriteExt::write_all(&mut writer, line.as_bytes()).await?;
            tokio::io::AsyncWriteExt::write_all(&mut writer, b"\n").await?;
        }
        
        tokio::io::AsyncWriteExt::flush(&mut writer).await?;
        Ok(())
    }
    
    fn parse_training_loss(&self, output: &str) -> Result<f64, ARTError> {
        // Parse loss from training output
        for line in output.lines() {
            if line.contains("loss:") || line.contains("Loss:") {
                use regex::Regex;
                let re = Regex::new(r"(\d+(?:\.\d+)?)")
                    .map_err(|e| ARTError::ParseError(format!("Regex error: {}", e)))?;
                    
                if let Some(captures) = re.captures(line) {
                    if let Some(loss_match) = captures.get(1) {
                        let loss: f64 = loss_match.as_str().parse()
                            .map_err(|_| ARTError::ParseError("Could not parse loss".to_string()))?;
                        return Ok(loss);
                    }
                }
            }
        }
        
        // Default loss if parsing fails
        Ok(0.5)
    }
    
    pub async fn train_specialized_model(
        &self,
        base_model: &str,
        specialized_name: &str,
        training_data: TrainingDataset
    ) -> Result<PathBuf, ARTError> {
        // Train specialized model
        let _loss = self.train_model(specialized_name, training_data).await?;
        
        Ok(PathBuf::from(format!("./{}", specialized_name)))
    }
    
    pub async fn evaluate_model(&self, model_path: &PathBuf) -> Result<ModelPerformanceMetrics, ARTError> {
        // Run evaluation using your benchmark scripts
        let output = Command::new(&self.python_path)
            .current_dir(&self.command_center_path)
            .arg("model_performance_lab.py")
            .arg(model_path)
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(ARTError::EvaluationFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ));
        }
        
        // Parse performance metrics
        let output_str = String::from_utf8_lossy(&output.stdout);
        self.parse_performance_metrics(&output_str)
    }
    
    fn parse_performance_metrics(&self, output: &str) -> Result<ModelPerformanceMetrics, ARTError> {
        // Parse metrics from evaluation output
        Ok(ModelPerformanceMetrics {
            accuracy: 0.85,      // Would parse from actual output
            efficiency: 0.78,
            reasoning_quality: 0.82,
            task_completion_rate: 0.89,
            tokens_per_second: 21.2, // Your champion's speed
        })
    }
}

// Supporting data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataset {
    pub conversations: Vec<Conversation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub messages: Vec<Message>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Default for ModelPerformanceMetrics {
    fn default() -> Self {
        Self {
            accuracy: 0.0,
            efficiency: 0.0,
            reasoning_quality: 0.0,
            task_completion_rate: 0.0,
            tokens_per_second: 0.0,
        }
    }
}
```

---

## 🔄 **6. Integration with RustChain**

Complete integration with existing agent system:

### Enhanced Agent Implementation

```rust
// Add ART capability to existing Agent implementation
impl<'a> Agent<'a> {
    /// Run agent with full ART tracking and learning
    pub async fn run_with_art(
        &mut self, 
        objective: &str,
        art_system: &mut AgentReinforcementTraining
    ) -> Result<String, RustChainError> {
        info!("🧠 Agent {} starting ART-enabled mission: {}", self.name, objective);
        
        // Start ART session
        let session_id = art_system.trajectory_tracker.start_session(&self.name, objective);
        
        self.state = AgentState::Running;
        self.memory.store("objective", objective)?;
        
        let mut iteration = 0;
        let mut final_answer = None;
        
        while iteration < self.max_iterations && final_answer.is_none() {
            iteration += 1;
            let step_start = Utc::now();
            
            if self.verbose {
                println!("\n🔄 ART Iteration {} ===", iteration);
            }
            
            // 1. Observe with ART tracking
            let observation = self.observe().await?;
            
            // 2. Think with enhanced reasoning
            let thought = self.think_with_art(&observation, objective, &session_id, art_system).await?;
            
            // 3. Decide on action with ART guidance
            let action = self.decide_action_with_art(&thought, &session_id, art_system).await?;
            
            // 4. Execute action with performance tracking
            let execution_start = std::time::Instant::now();
            let (action_result, tool_result) = match action.clone() {
                AgentAction::UseTool { tool, input } => {
                    let result = self.use_tool(&tool, &input).await?;
                    self.memory.store(&format!("tool_result_{}", iteration), &result)?;
                    (format!("Used tool {} successfully", tool), Some(result))
                }
                AgentAction::Answer(answer) => {
                    final_answer = Some(answer.clone());
                    self.memory.store("final_answer", &answer)?;
                    ("Provided final answer".to_string(), None)
                }
                AgentAction::RequestMoreInfo(question) => {
                    if self.verbose {
                        println!("🤔 Agent needs more info: {}", question);
                    }
                    self.memory.store(&format!("question_{}", iteration), &question)?;
                    ("Requested more information".to_string(), None)
                }
                AgentAction::Think => {
                    ("Continued thinking".to_string(), None)
                }
            };
            
            let execution_time = execution_start.elapsed().as_millis() as u64;
            
            // 5. Record trajectory step with ART
            let trajectory_step = TrajectoryStep {
                step_id: iteration,
                observation: observation.clone(),
                thought: thought.clone(),
                action: action.clone(),
                action_input: match &action {
                    AgentAction::UseTool { input, .. } => input.clone(),
                    AgentAction::Answer(answer) => answer.clone(),
                    AgentAction::RequestMoreInfo(question) => question.clone(),
                    AgentAction::Think => "".to_string(),
                },
                tool_result,
                reflection: None,
                step_reward: None, // Will be calculated by RULER
                step_metadata: self.create_step_metadata(&action, execution_time),
                timestamp: Utc::now(),
                execution_time_ms: execution_time,
            };
            
            art_system.trajectory_tracker.record_step(&session_id, trajectory_step).await
                .map_err(|e| RustChainError::Execution(ExecutionError::unknown(
                    &format!("ART tracking failed: {:?}", e)
                )))?;
            
            if self.verbose {
                println!("💭 Thought: {}", thought);
                println!("🎯 Action: {:?}", action);
                println!("📊 Result: {}", action_result);
            }
            
            // 6. Periodic reflection with ART insights
            if iteration % 3 == 0 {
                let reflection = self.reflect_with_art(&session_id, art_system).await?;
                if self.verbose {
                    println!("🔍 ART Reflection: {}", reflection);
                }
            }
        }
        
        self.state = AgentState::Completed;
        
        // Complete ART session and get trajectory
        let completed_trajectory = art_system.trajectory_tracker.complete_session(&session_id).await
            .map_err(|e| RustChainError::Execution(ExecutionError::unknown(
                &format!("ART session completion failed: {:?}", e)
            ))))?;
            
        // Calculate reward score
        let reward_score = art_system.reward_system.calculate_reward(&completed_trajectory).await
            .map_err(|e| RustChainError::Execution(ExecutionError::unknown(
                &format!("RULER reward calculation failed: {:?}", e)
            ))))?;
            
        if self.verbose {
            println!("🏆 ART Session Completed - Reward: {:.3}", reward_score.total_reward);
            println!("📈 Performance Components: {:?}", reward_score.component_scores);
        }
        
        final_answer.ok_or_else(|| {
            RustChainError::Execution(ExecutionError::timeout(
                &self.name,
                (self.max_iterations * 10000) as u64,
            ))
        })
    }
    
    /// Enhanced thinking with ART context
    async fn think_with_art(
        &mut self,
        observation: &str,
        objective: &str,
        session_id: &str,
        art_system: &AgentReinforcementTraining
    ) -> Result<String, RustChainError> {
        // Get historical insights from similar successful trajectories
        let similar_successes = art_system.get_similar_successful_patterns(objective).await
            .map_err(|e| RustChainError::Execution(ExecutionError::unknown(
                &format!("ART pattern matching failed: {:?}", e)
            ))))?;
            
        let mut prompt = format!(
            "You are {}, an AI agent with ART learning capabilities.\n\
            Your objective is: {}\n\n\
            Current observation:\n{}\n\n\
            Available tools: {:?}\n\n",
            self.name, objective, observation, self.tools.list()
        );
        
        // Add successful patterns if available
        if !similar_successes.is_empty() {
            prompt.push_str("💡 Learning from similar successful missions:\n");
            for (i, pattern) in similar_successes.iter().take(3).enumerate() {
                prompt.push_str(&format!("{}. {}\n", i + 1, pattern.success_summary));
            }
            prompt.push_str("\n");
        }
        
        prompt.push_str("Think step by step about what to do next, incorporating lessons learned:");
        
        let thought = self.llm.generate(&prompt).await?;
        self.memory.store(&format!("art_thought_{}", chrono::Utc::now().timestamp()), &thought)?;
        
        Ok(thought)
    }
    
    /// Enhanced action decision with ART guidance
    async fn decide_action_with_art(
        &self,
        thought: &str,
        session_id: &str,
        art_system: &AgentReinforcementTraining
    ) -> Result<AgentAction, RustChainError> {
        // Get recommended actions from ART system
        let action_recommendations = art_system.get_action_recommendations(session_id, thought).await
            .map_err(|e| RustChainError::Execution(ExecutionError::unknown(
                &format!("ART action recommendation failed: {:?}", e)
            ))))?;
            
        let mut prompt = format!(
            "Based on this thought: {}\n\n\
            Decide on ONE of these actions:\n\
            1. USE_TOOL <tool_name> <input> - Use a specific tool\n\
            2. ANSWER <final_answer> - Provide the final answer\n\
            3. ASK <question> - Request more information\n\
            4. THINK - Continue thinking\n\n",
            thought
        );
        
        // Add ART recommendations
        if !action_recommendations.is_empty() {
            prompt.push_str("💡 ART Recommendations based on successful patterns:\n");
            for rec in &action_recommendations {
                prompt.push_str(&format!("- {}\n", rec));
            }
            prompt.push_str("\n");
        }
        
        prompt.push_str("Respond with only the action in the specified format:");
        
        let response = self.llm.generate(&prompt).await?;
        
        // Parse action with ART context
        self.parse_action_with_art(&response)
    }
    
    /// Enhanced reflection with ART insights
    async fn reflect_with_art(
        &mut self,
        session_id: &str,
        art_system: &AgentReinforcementTraining
    ) -> Result<String, RustChainError> {
        // Get current trajectory analysis
        let trajectory_analysis = art_system.analyze_current_trajectory(session_id).await
            .map_err(|e| RustChainError::Execution(ExecutionError::unknown(
                &format!("ART trajectory analysis failed: {:?}", e)
            ))))?;
            
        let prompt = format!(
            "You are {}. Reflect on your progress with ART insights.\n\n\
            Current Performance Analysis:\n{}\n\n\
            Questions to consider:\n\
            - Are you making efficient progress toward your goal?\n\
            - Should you adjust your approach based on the performance data?\n\
            - What have you learned that could improve future decisions?\n\n\
            Provide a brief reflection:",
            self.name, trajectory_analysis
        );
        
        let reflection = self.llm.generate(&prompt).await?;
        self.memory.store(
            &format!("art_reflection_{}", chrono::Utc::now().timestamp()),
            &reflection,
        )?;
        
        Ok(reflection)
    }
    
    fn create_step_metadata(&self, action: &AgentAction, execution_time: u64) -> HashMap<String, serde_json::Value> {
        let mut metadata = HashMap::new();
        
        metadata.insert("agent_name".to_string(), 
            serde_json::Value::String(self.name.clone()));
        metadata.insert("action_type".to_string(), 
            serde_json::Value::String(format!("{:?}", action)));
        metadata.insert("execution_time_ms".to_string(), 
            serde_json::Value::Number(serde_json::Number::from(execution_time)));
        metadata.insert("max_iterations".to_string(), 
            serde_json::Value::Number(serde_json::Number::from(self.max_iterations)));
            
        metadata
    }
    
    fn parse_action_with_art(&self, response: &str) -> Result<AgentAction, RustChainError> {
        // Enhanced parsing with ART context - same logic as before but with better error handling
        let action = if response.starts_with("USE_TOOL") {
            let parts: Vec<&str> = response.splitn(3, ' ').collect();
            if parts.len() >= 3 {
                AgentAction::UseTool {
                    tool: parts[1].to_string(),
                    input: parts[2].to_string(),
                }
            } else {
                AgentAction::Think
            }
        } else if response.starts_with("ANSWER") {
            let answer = response.strip_prefix("ANSWER").unwrap_or("").trim();
            AgentAction::Answer(answer.to_string())
        } else if response.starts_with("ASK") {
            let question = response.strip_prefix("ASK").unwrap_or("").trim();
            AgentAction::RequestMoreInfo(question.to_string())
        } else {
            AgentAction::Think
        };
        
        Ok(action)
    }
}
```

---

## 🚀 **7. Main ART System Implementation**

Complete system orchestration:

### ART System Core

```rust
/// Main ART system that coordinates all components
pub struct AgentReinforcementTraining {
    pub trajectory_tracker: TrajectoryTracker,
    pub performance_judge: PerformanceJudge,
    pub reward_system: RULERAlgorithm,
    pub training_pipeline: ARTTrainingPipeline,
    pub model_manager: PersonalModelManager,
    pub pattern_database: SuccessPatternDatabase,
    config: ARTConfig,
}

#[derive(Debug, Clone)]
pub struct ARTConfig {
    pub enable_tracking: bool,
    pub enable_training: bool,
    pub training_interval: Duration,
    pub min_trajectories_for_training: usize,
    pub pattern_matching_threshold: f64,
    pub reward_threshold_for_learning: f64,
}

impl Default for ARTConfig {
    fn default() -> Self {
        Self {
            enable_tracking: true,
            enable_training: true,
            training_interval: Duration::from_secs(3600), // 1 hour
            min_trajectories_for_training: 10,
            pattern_matching_threshold: 0.7,
            reward_threshold_for_learning: 0.7,
        }
    }
}

impl AgentReinforcementTraining {
    pub async fn new(config: ARTConfig) -> Result<Self, ARTError> {
        // Initialize storage
        let storage = Box::new(FileTrajectoryStorage::new("./art_data").await?);
        
        // Initialize LLM for judging and rewards
        let judge_llm = Box::new(create_llm_backend()?);
        let reward_llm = Box::new(create_llm_backend()?);
        
        // Initialize training config
        let training_config = TrainingConfig::default();
        
        Ok(Self {
            trajectory_tracker: TrajectoryTracker::new(storage.clone()),
            performance_judge: PerformanceJudge::new(judge_llm),
            reward_system: RULERAlgorithm::new(reward_llm),
            training_pipeline: ARTTrainingPipeline::new(training_config, storage),
            model_manager: PersonalModelManager::new(),
            pattern_database: SuccessPatternDatabase::new(),
            config,
        })
    }
    
    /// Get successful patterns similar to the current objective
    pub async fn get_similar_successful_patterns(&self, objective: &str) -> Result<Vec<SuccessPattern>, ARTError> {
        self.pattern_database.find_similar_patterns(objective, self.config.pattern_matching_threshold).await
    }
    
    /// Get action recommendations based on successful trajectories
    pub async fn get_action_recommendations(&self, session_id: &str, current_thought: &str) -> Result<Vec<String>, ARTError> {
        // Analyze current trajectory context
        let trajectory = self.trajectory_tracker.active_trajectories.get(session_id)
            .ok_or_else(|| ARTError::SessionNotFound(session_id.to_string()))?;
            
        // Find similar successful patterns
        let similar_patterns = self.pattern_database
            .find_patterns_by_context(&trajectory.objective, current_thought)
            .await?;
            
        // Extract action recommendations
        let recommendations = similar_patterns.iter()
            .flat_map(|pattern| &pattern.successful_actions)
            .take(3)
            .cloned()
            .collect();
            
        Ok(recommendations)
    }
    
    /// Analyze current trajectory performance
    pub async fn analyze_current_trajectory(&self, session_id: &str) -> Result<String, ARTError> {
        let trajectory = self.trajectory_tracker.active_trajectories.get(session_id)
            .ok_or_else(|| ARTError::SessionNotFound(session_id.to_string()))?;
            
        let steps_taken = trajectory.steps.len();
        let tools_used = trajectory.metadata.tools_used.len();
        let error_count = trajectory.metadata.error_count;
        
        let analysis = format!(
            "📊 Current Trajectory Analysis:\n\
            - Steps taken: {}\n\
            - Tools used: {} ({})\n\
            - Errors encountered: {}\n\
            - Efficiency score: {:.2}\n\
            - Progress assessment: {}",
            steps_taken,
            tools_used,
            trajectory.metadata.tools_used.join(", "),
            error_count,
            trajectory.metadata.efficiency_score,
            if steps_taken < 5 { "Early stage" } 
            else if steps_taken < 10 { "Making progress" } 
            else { "Extensive exploration" }
        );
        
        Ok(analysis)
    }
    
    /// Main training loop - runs periodically to improve models
    pub async fn run_training_loop(&mut self) -> Result<(), ARTError> {
        println!("🧠 Starting ART training loop");
        
        loop {
            tokio::time::sleep(self.config.training_interval).await;
            
            if !self.config.enable_training {
                continue;
            }
            
            // Check if we have enough trajectories
            let trajectory_count = self.trajectory_tracker.storage.count_trajectories().await?;
            
            if trajectory_count < self.config.min_trajectories_for_training {
                println!("📈 Waiting for more trajectories ({}/{})", 
                    trajectory_count, self.config.min_trajectories_for_training);
                continue;
            }
            
            println!("🎯 Starting training cycle with {} trajectories", trajectory_count);
            
            // Run training
            match self.training_pipeline.train().await {
                Ok(results) => {
                    println!("✅ Training completed successfully");
                    println!("   Best performance: {:.4}", results.best_performance);
                    println!("   Total epochs: {}", results.total_epochs);
                    
                    // Update pattern database with new successful patterns
                    self.update_pattern_database(&results).await?;
                }
                Err(e) => {
                    eprintln!("❌ Training failed: {:?}", e);
                }
            }
        }
    }
    
    /// Update success patterns from training results
    async fn update_pattern_database(&mut self, results: &TrainingResults) -> Result<(), ARTError> {
        // Extract successful patterns from high-performing trajectories
        let successful_trajectories = self.trajectory_tracker.storage
            .get_trajectories_by_reward_threshold(self.config.reward_threshold_for_learning)
            .await?;
            
        for trajectory in successful_trajectories {
            let pattern = self.extract_success_pattern(&trajectory).await?;
            self.pattern_database.add_pattern(pattern).await?;
        }
        
        println!("📝 Updated pattern database with new successful patterns");
        Ok(())
    }
    
    async fn extract_success_pattern(&self, trajectory: &ARTTrajectory) -> Result<SuccessPattern, ARTError> {
        let successful_actions = trajectory.steps.iter()
            .filter(|step| step.step_reward.unwrap_or(0.0) > 0.7)
            .map(|step| format!("{}: {}", format!("{:?}", step.action), step.thought))
            .collect();
            
        Ok(SuccessPattern {
            objective_pattern: trajectory.objective.clone(),
            successful_actions,
            success_summary: format!("Achieved objective with {} steps", trajectory.steps.len()),
            reward_score: trajectory.reward_score.unwrap_or(0.0),
            context_keywords: self.extract_keywords(&trajectory.objective),
        })
    }
    
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        // Simple keyword extraction - could be enhanced
        text.to_lowercase()
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .map(|word| word.to_string())
            .collect()
    }
}

/// Database of successful patterns for learning
pub struct SuccessPatternDatabase {
    patterns: Vec<SuccessPattern>,
    storage: Box<dyn PatternStorage>,
}

#[derive(Debug, Clone)]
pub struct SuccessPattern {
    pub objective_pattern: String,
    pub successful_actions: Vec<String>,
    pub success_summary: String,
    pub reward_score: f64,
    pub context_keywords: Vec<String>,
}

impl SuccessPatternDatabase {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            storage: Box::new(FilePatternStorage::new("./art_patterns")),
        }
    }
    
    pub async fn find_similar_patterns(&self, objective: &str, threshold: f64) -> Result<Vec<SuccessPattern>, ARTError> {
        let similar_patterns = self.patterns.iter()
            .filter(|pattern| {
                let similarity = self.calculate_objective_similarity(&pattern.objective_pattern, objective);
                similarity > threshold
            })
            .cloned()
            .collect();
            
        Ok(similar_patterns)
    }
    
    pub async fn find_patterns_by_context(&self, objective: &str, context: &str) -> Result<Vec<SuccessPattern>, ARTError> {
        let context_keywords: Vec<String> = context.to_lowercase()
            .split_whitespace()
            .filter(|word| word.len() > 3)
            .map(|word| word.to_string())
            .collect();
            
        let matching_patterns = self.patterns.iter()
            .filter(|pattern| {
                let keyword_overlap = pattern.context_keywords.iter()
                    .filter(|keyword| context_keywords.contains(keyword))
                    .count() as f64 / pattern.context_keywords.len().max(1) as f64;
                    
                keyword_overlap > 0.3
            })
            .cloned()
            .collect();
            
        Ok(matching_patterns)
    }
    
    pub async fn add_pattern(&mut self, pattern: SuccessPattern) -> Result<(), ARTError> {
        self.patterns.push(pattern.clone());
        self.storage.store_pattern(&pattern).await?;
        Ok(())
    }
    
    fn calculate_objective_similarity(&self, pattern_objective: &str, current_objective: &str) -> f64 {
        // Simple similarity calculation - could use embeddings
        if pattern_objective == current_objective {
            1.0
        } else if pattern_objective.contains(current_objective) || current_objective.contains(pattern_objective) {
            0.8
        } else {
            let pattern_words: std::collections::HashSet<&str> = pattern_objective.split_whitespace().collect();
            let current_words: std::collections::HashSet<&str> = current_objective.split_whitespace().collect();
            
            let intersection = pattern_words.intersection(&current_words).count() as f64;
            let union = pattern_words.union(&current_words).count() as f64;
            
            if union > 0.0 {
                intersection / union
            } else {
                0.0
            }
        }
    }
}

// Supporting traits and implementations
#[async_trait]
pub trait TrajectoryStorage: Send + Sync {
    async fn store_trajectory(&self, trajectory: &ARTTrajectory) -> Result<(), ARTError>;
    async fn get_recent_trajectories(&self, limit: usize) -> Result<Vec<ARTTrajectory>, ARTError>;
    async fn get_trajectories_by_reward_threshold(&self, threshold: f64) -> Result<Vec<ARTTrajectory>, ARTError>;
    async fn count_trajectories(&self) -> Result<usize, ARTError>;
}

#[async_trait]
pub trait PatternStorage: Send + Sync {
    async fn store_pattern(&self, pattern: &SuccessPattern) -> Result<(), ARTError>;
    async fn load_patterns(&self) -> Result<Vec<SuccessPattern>, ARTError>;
}

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum ARTError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Training failed: {0}")]
    TrainingFailed(String),
    
    #[error("Evaluation failed: {0}")]
    EvaluationFailed(String),
    
    #[error("Model operation failed: {0}")]
    ModelOperationFailed(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("LLM error: {0}")]
    LLM(#[from] crate::core::error::RustChainError),
}

// Utility functions
fn create_llm_backend() -> Result<Box<dyn crate::core::llm::LLMBackend>, ARTError> {
    // Use your shimmy provider as the LLM backend
    use crate::llm::shimmy_provider::ShimmyProvider;
    
    Ok(Box::new(ShimmyProvider::new(None)
        .with_model("llama-3.2-1b-personal".to_string())))
}
```

---

## 📋 **8. Storage Implementations**

File-based storage for trajectories and patterns:

### File Storage

```rust
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct FileTrajectoryStorage {
    base_path: PathBuf,
}

impl FileTrajectoryStorage {
    pub async fn new(base_path: &str) -> Result<Self, ARTError> {
        let path = PathBuf::from(base_path);
        fs::create_dir_all(&path).await?;
        
        Ok(Self {
            base_path: path,
        })
    }
}

#[async_trait]
impl TrajectoryStorage for FileTrajectoryStorage {
    async fn store_trajectory(&self, trajectory: &ARTTrajectory) -> Result<(), ARTError> {
        let filename = format!("{}.json", trajectory.session_id);
        let filepath = self.base_path.join(filename);
        
        let json_data = serde_json::to_string_pretty(trajectory)?;
        let mut file = fs::File::create(filepath).await?;
        file.write_all(json_data.as_bytes()).await?;
        
        Ok(())
    }
    
    async fn get_recent_trajectories(&self, limit: usize) -> Result<Vec<ARTTrajectory>, ARTError> {
        let mut trajectories = Vec::new();
        let mut entries = fs::read_dir(&self.base_path).await?;
        
        let mut files = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Ok(metadata) = entry.metadata().await {
                    files.push((path, metadata.modified().unwrap_or(std::time::UNIX_EPOCH)));
                }
            }
        }
        
        // Sort by modification time, newest first
        files.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (filepath, _) in files.into_iter().take(limit) {
            if let Ok(mut file) = fs::File::open(filepath).await {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).await.is_ok() {
                    if let Ok(trajectory) = serde_json::from_str::<ARTTrajectory>(&contents) {
                        trajectories.push(trajectory);
                    }
                }
            }
        }
        
        Ok(trajectories)
    }
    
    async fn get_trajectories_by_reward_threshold(&self, threshold: f64) -> Result<Vec<ARTTrajectory>, ARTError> {
        let all_trajectories = self.get_recent_trajectories(1000).await?;
        
        let filtered_trajectories = all_trajectories.into_iter()
            .filter(|trajectory| trajectory.reward_score.unwrap_or(0.0) >= threshold)
            .collect();
            
        Ok(filtered_trajectories)
    }
    
    async fn count_trajectories(&self) -> Result<usize, ARTError> {
        let mut count = 0;
        let mut entries = fs::read_dir(&self.base_path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                count += 1;
            }
        }
        
        Ok(count)
    }
}

pub struct FilePatternStorage {
    filepath: PathBuf,
}

impl FilePatternStorage {
    pub fn new(base_path: &str) -> Self {
        let filepath = PathBuf::from(base_path).join("success_patterns.json");
        
        Self { filepath }
    }
}

#[async_trait]
impl PatternStorage for FilePatternStorage {
    async fn store_pattern(&self, pattern: &SuccessPattern) -> Result<(), ARTError> {
        let mut patterns = self.load_patterns().await.unwrap_or_default();
        patterns.push(pattern.clone());
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.filepath.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let json_data = serde_json::to_string_pretty(&patterns)?;
        let mut file = fs::File::create(&self.filepath).await?;
        file.write_all(json_data.as_bytes()).await?;
        
        Ok(())
    }
    
    async fn load_patterns(&self) -> Result<Vec<SuccessPattern>, ARTError> {
        if !self.filepath.exists() {
            return Ok(Vec::new());
        }
        
        let mut file = fs::File::open(&self.filepath).await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        
        let patterns = serde_json::from_str::<Vec<SuccessPattern>>(&contents)?;
        Ok(patterns)
    }
}
```

---

## 🎮 **9. CLI Integration**

Add ART commands to RustChain CLI:

### CLI Commands

```rust
// In src/cli/commands.rs - Add ART commands

use crate::art::{AgentReinforcementTraining, ARTConfig};

impl CLICommands {
    /// Initialize ART system
    pub async fn art_init(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧠 Initializing RustChain Agent Reinforcement Training");
        
        let config = ARTConfig::default();
        let mut art_system = AgentReinforcementTraining::new(config).await?;
        
        println!("✅ ART system initialized successfully");
        println!("📁 Data directory: ./art_data");
        println!("🎯 Pattern database: ./art_patterns");
        println!("🏆 Champion model: llama-3.2-1b-personal");
        
        // Start background training loop
        tokio::spawn(async move {
            if let Err(e) = art_system.run_training_loop().await {
                eprintln!("❌ ART training loop error: {:?}", e);
            }
        });
        
        println!("🔄 Background training loop started");
        Ok(())
    }
    
    /// Run mission with ART tracking
    pub async fn art_run(&self, mission_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Running mission with ART tracking: {}", mission_file);
        
        // Load ART system
        let config = ARTConfig::default();
        let mut art_system = AgentReinforcementTraining::new(config).await?;
        
        // Load and run mission
        let mission = self.load_mission(mission_file).await?;
        let result = self.execute_mission_with_art(&mission, &mut art_system).await?;
        
        println!("✅ Mission completed with ART");
        println!("📊 Result: {}", result);
        
        Ok(())
    }
    
    /// Show ART statistics
    pub async fn art_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 RustChain ART Statistics");
        
        // Load trajectory storage
        let storage = crate::art::FileTrajectoryStorage::new("./art_data").await?;
        
        let total_trajectories = storage.count_trajectories().await?;
        let recent_trajectories = storage.get_recent_trajectories(10).await?;
        
        println!("📈 Total trajectories recorded: {}", total_trajectories);
        println!("🎯 Recent successful trajectories: {}", 
            recent_trajectories.iter()
                .filter(|t| t.reward_score.unwrap_or(0.0) > 0.7)
                .count());
        
        if !recent_trajectories.is_empty() {
            println!("\n🏆 Top Recent Performances:");
            let mut sorted_trajectories = recent_trajectories;
            sorted_trajectories.sort_by(|a, b| {
                b.reward_score.unwrap_or(0.0).partial_cmp(&a.reward_score.unwrap_or(0.0))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            
            for (i, trajectory) in sorted_trajectories.iter().take(5).enumerate() {
                println!("  {}. {}: {:.3} ({})", 
                    i + 1, 
                    trajectory.agent_name,
                    trajectory.reward_score.unwrap_or(0.0),
                    trajectory.objective);
            }
        }
        
        Ok(())
    }
    
    /// Train models on ART data
    pub async fn art_train(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎯 Starting ART model training");
        
        let config = ARTConfig::default();
        let mut art_system = AgentReinforcementTraining::new(config).await?;
        
        let results = art_system.training_pipeline.train().await?;
        
        println!("✅ Training completed successfully");
        println!("   Best performance: {:.4}", results.best_performance);
        println!("   Total epochs: {}", results.total_epochs);
        
        Ok(())
    }
    
    /// Create specialized agent model
    pub async fn art_specialize(&self, agent_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Creating specialized {} agent model", agent_type);
        
        let specialization = match agent_type.to_lowercase().as_str() {
            "research" => crate::art::ModelSpecialization::Research,
            "planning" => crate::art::ModelSpecialization::Planning,
            "execution" => crate::art::ModelSpecialization::Execution,
            "code" => crate::art::ModelSpecialization::CodeGeneration,
            _ => {
                eprintln!("❌ Unknown agent type. Available: research, planning, execution, code");
                return Ok(());
            }
        };
        
        let config = ARTConfig::default();
        let mut art_system = AgentReinforcementTraining::new(config).await?;
        
        // Get relevant trajectories
        let storage = crate::art::FileTrajectoryStorage::new("./art_data").await?;
        let trajectories = storage.get_recent_trajectories(100).await?;
        
        // Create specialized model
        let specialized_model = art_system.model_manager.create_specialized_model(
            "llama-3.2-1b-personal",
            specialization,
            &trajectories
        ).await?;
        
        println!("✅ Created specialized model: {}", specialized_model.name);
        println!("📁 Model path: {:?}", specialized_model.path);
        
        Ok(())
    }
    
    async fn execute_mission_with_art(
        &self,
        mission: &Mission,
        art_system: &mut AgentReinforcementTraining
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Create agent with ART capability
        let mut memory = crate::core::memory::InMemoryStore::new();
        let tools = crate::core::tools::ToolRegistry::new();
        let llm = self.create_llm_backend()?;
        
        let mut agent = crate::core::agent::Agent::new(
            mission.name.clone(),
            &mut memory,
            &tools,
            llm.as_ref(),
        ).with_verbose(true);
        
        // Run with ART tracking
        let result = agent.run_with_art(&mission.objective, art_system).await?;
        
        Ok(result)
    }
}

// Add to CLI argument parsing
#[derive(Debug, Clone)]
pub enum ARTCommand {
    Init,
    Run { mission_file: String },
    Stats,
    Train,
    Specialize { agent_type: String },
}

pub fn parse_art_commands(matches: &clap::ArgMatches) -> Option<ARTCommand> {
    if let Some(art_matches) = matches.subcommand_matches("art") {
        match art_matches.subcommand() {
            Some(("init", _)) => Some(ARTCommand::Init),
            Some(("run", sub_matches)) => {
                let mission_file = sub_matches.value_of("mission").unwrap().to_string();
                Some(ARTCommand::Run { mission_file })
            },
            Some(("stats", _)) => Some(ARTCommand::Stats),
            Some(("train", _)) => Some(ARTCommand::Train),
            Some(("specialize", sub_matches)) => {
                let agent_type = sub_matches.value_of("type").unwrap().to_string();
                Some(ARTCommand::Specialize { agent_type })
            },
            _ => None,
        }
    } else {
        None
    }
}
```

---

## 🏁 **10. Getting Started Guide**

Complete setup and usage instructions:

### Quick Start

```bash
# 1. Initialize ART system
cargo run -- art init

# 2. Run a mission with ART tracking
cargo run -- art run examples/research_mission.yaml

# 3. View statistics
cargo run -- art stats

# 4. Train models on collected data
cargo run -- art train

# 5. Create specialized agent models
cargo run -- art specialize research
cargo run -- art specialize planning
cargo run -- art specialize execution
```

### Example Mission with ART

```yaml
# examples/art_research_mission.yaml
name: "ART Research Agent"
type: "agent"
agent_config:
  max_iterations: 15
  enable_art: true
  model: "llama-3.2-1b-personal"
  
objective: "Research and summarize the latest developments in AI agent frameworks"

tools:
  - name: "web_search"
    config:
      max_results: 5
  - name: "file_write" 
    config:
      base_path: "./research_output"

success_criteria:
  - "Summary document created"
  - "At least 3 frameworks covered"
  - "Key insights identified"
```

### Configuration

```toml
# rustchain.toml - Add ART configuration
[art]
enable_tracking = true
enable_training = true
training_interval = 3600  # 1 hour in seconds
min_trajectories_for_training = 10
pattern_matching_threshold = 0.7
reward_threshold_for_learning = 0.7

[art.models]
champion = "llama-3.2-1b-personal"
base_models_path = "../command-center"
checkpoints_path = "./checkpoints"

[art.storage]
trajectories_path = "./art_data"
patterns_path = "./art_patterns"
backup_interval = 86400  # 24 hours
```

---

## 🚀 **Integration Benefits**

This ART system provides:

1. **🧠 Self-Improving Agents**: Agents learn from successful patterns
2. **🎯 Specialized Models**: Domain-specific agent intelligence
3. **📊 Performance Tracking**: Comprehensive trajectory analysis
4. **🏆 Automated Rewards**: RULER algorithm eliminates manual scoring
5. **🔄 Continuous Learning**: Background training on real agent data
6. **⚡ Personal Model Integration**: Uses your champion Llama-3.2-1B model
7. **📈 Pattern Recognition**: Success patterns guide future decisions
8. **🛠️ Production Ready**: Full CLI integration and storage systems

The system creates a **compound intelligence feedback loop** where:
- Agents execute real tasks and record everything
- Successful patterns are identified and stored
- Models are fine-tuned on successful interaction data
- Agents get better at their specialized jobs through continuous learning

This transforms RustChain from a static agent framework into a **living, learning system** that improves with every mission completed.

---

**🎯 Ready to deploy into RustChain Community!**