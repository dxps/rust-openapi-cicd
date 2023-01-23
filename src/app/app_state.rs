use std::sync::Arc;

use axum::extract::FromRef;

use crate::repos::thoughts_repo::ThoughtsRepo;

/// The (global) state of the app.
#[derive(FromRef)]
pub struct AppState {
    pub thoughts_repo: Arc<ThoughtsRepo>,
}

impl AppState {
    pub fn new(thoughts_repo: ThoughtsRepo) -> Self {
        Self {
            thoughts_repo: Arc::new(thoughts_repo),
        }
    }
}
