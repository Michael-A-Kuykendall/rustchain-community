// Concurrency utilities for RustChain
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};

/// Manages concurrent execution with resource limits
pub struct ConcurrencyManager {
    pub max_parallel: usize,
    pub semaphore: Arc<Semaphore>,
    pub active_tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub id: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub task_type: String,
}

impl ConcurrencyManager {
    pub fn new(max_parallel: usize) -> Self {
        Self {
            max_parallel,
            semaphore: Arc::new(Semaphore::new(max_parallel)),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn acquire_slot(
        &self,
        task_id: String,
        task_type: String,
    ) -> Result<tokio::sync::SemaphorePermit<'_>, tokio::sync::AcquireError> {
        let permit = self.semaphore.acquire().await?;

        let task_info = TaskInfo {
            id: task_id.clone(),
            started_at: chrono::Utc::now(),
            task_type,
        };

        self.active_tasks.write().await.insert(task_id, task_info);
        Ok(permit)
    }

    pub async fn release_slot(&self, task_id: &str) {
        self.active_tasks.write().await.remove(task_id);
    }

    pub async fn get_active_tasks(&self) -> Vec<TaskInfo> {
        self.active_tasks.read().await.values().cloned().collect()
    }
}
