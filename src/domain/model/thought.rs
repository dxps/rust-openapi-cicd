use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Thought {
    pub id: String,
    pub idea: String,
    pub tags: Vec<String>,
}
