#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::agent::AgentAction;
    use crate::art::tracker::TrajectoryTracker;
    
    #[tokio::test]
    async fn test_trajectory_creation() {
        let mut tracker = TrajectoryTracker::new("./test_data".to_string());
        
        tracker.start_trajectory(
            "test_session_1".to_string(),
            "test_agent".to_string(), 
            "Test objective".to_string()
        );
        
        let step = TrajectoryStep {
            step_id: 1,
            observation: "Test observation".to_string(),
            thought: "Test thought".to_string(),
            action: AgentAction::ToolUse {
                tool_name: "test_tool".to_string(),
                parameters: HashMap::new(),
            },
            action_input: "test input".to_string(),
            tool_result: Some("test result".to_string()),
            reflection: Some("test reflection".to_string()),
            step_reward: Some(0.8),
            step_metadata: HashMap::new(),
            timestamp: Utc::now(),
            execution_time_ms: 1500,
        };
        
        tracker.add_step(step);
        
        let trajectory = tracker.complete_trajectory().await.unwrap();
        assert_eq!(trajectory.steps.len(), 1);
        assert!(trajectory.completed_at.is_some());
    }
}
