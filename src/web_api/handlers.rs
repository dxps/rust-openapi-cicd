use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{app::app_state::AppState, domain::model::thought::Thought};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateThoughtInput {
    pub idea: String,
    pub tags: Vec<String>,
}

pub async fn create_thought(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateThoughtInput>,
) -> (StatusCode, Json<Value>) {
    let thought = state.thoughts_repo.add(input);
    (StatusCode::OK, Json(serde_json::to_value(thought).unwrap()))
}

pub async fn get_all_thoughts(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<Thought>>) {
    (StatusCode::OK, Json(state.thoughts_repo.get_all()))
}
