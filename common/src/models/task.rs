use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Deserialize, Serialize, EnumString, Display, PartialEq, Eq, Clone, Debug)]
pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub user_id: String,
    pub task_id: String,
    pub task_type: String,
    pub state: TaskState,
    pub src_file: String,
    pub res_file: Option<String>,
}

impl Task {
    pub fn new(user_id: String, task_type: String, src_file: String) -> Self {
        Self {
            user_id,
            task_type,
            src_file,
            task_id: Uuid::new_v4().to_string(),
            state: TaskState::NotStarted,
            res_file: None,
        }
    }

    pub fn global_id(&self) -> String {
        format!("{}_{}", self.user_id, self.task_id)
    }

    pub fn can_transition_to(&self, state: &TaskState) -> bool {
        self.state != *state
    }
}
