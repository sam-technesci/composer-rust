use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PersistedApplication {
    pub id: String,
    pub version: String,
    pub timestamp: i64,
    pub state: ApplicationState,
    pub app_name: String,
    pub compose_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ApplicationState {
    STARTING,
    RUNNING,
    ERROR,
}
