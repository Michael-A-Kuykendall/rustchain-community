// Agent Reinforcement Training - Session Management Module
use crate::core::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSession {
    pub id: String,
    pub agent_id: String,
    pub started_at: u64,
    pub completed_at: Option<u64>,
    pub trajectory_count: usize,
}

impl TrainingSession {
    pub fn new(agent_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            agent_id,
            started_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            completed_at: None,
            trajectory_count: 0,
        }
    }
}

pub fn start_session(agent_id: &str) -> Result<TrainingSession> {
    Ok(TrainingSession::new(agent_id.to_string()))
}

pub fn add_trajectory_to_session(session: &mut TrainingSession) -> Result<()> {
    session.trajectory_count += 1;
    Ok(())
}

pub fn complete_session(session: &mut TrainingSession) -> Result<()> {
    session.completed_at = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    );
    Ok(())
}

pub fn get_session_metrics(session: &TrainingSession) -> Option<SessionMetrics> {
    Some(SessionMetrics {
        trajectory_count: session.trajectory_count,
        duration_seconds: session.completed_at
            .map(|end| end - session.started_at)
            .unwrap_or(0),
        completion_rate: if session.completed_at.is_some() { 1.0 } else { 0.0 },
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub trajectory_count: usize,
    pub duration_seconds: u64,
    pub completion_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session() {
        let session = TrainingSession::new("test_agent".to_string());
        assert_eq!(session.agent_id, "test_agent");
        assert!(session.completed_at.is_none());
        assert_eq!(session.trajectory_count, 0);
    }

    #[test]
    fn test_start_session() {
        let result = start_session("test_agent");
        assert!(result.is_ok());
        let session = result.unwrap();
        assert_eq!(session.agent_id, "test_agent");
    }

    #[test]
    fn test_add_trajectory_to_session() {
        let mut session = TrainingSession::new("test_agent".to_string());
        let result = add_trajectory_to_session(&mut session);
        assert!(result.is_ok());
        assert_eq!(session.trajectory_count, 1);
    }

    #[test]
    fn test_complete_session() {
        let mut session = TrainingSession::new("test_agent".to_string());
        let result = complete_session(&mut session);
        assert!(result.is_ok());
        assert!(session.completed_at.is_some());
    }

    #[test]
    fn test_get_session_metrics() {
        let session = TrainingSession::new("test_agent".to_string());
        let result = get_session_metrics(&session);
        assert!(result.is_some());
        let metrics = result.unwrap();
        assert_eq!(metrics.trajectory_count, 0);
    }
}