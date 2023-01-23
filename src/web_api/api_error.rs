use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug, Error)]
pub struct ApiError {
    status_code: u16,
    errors: Vec<String>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ApiError {}", &self.status_code))
    }
}

impl ApiError {
    pub fn new(status_code: u16, err: String) -> Self {
        Self {
            status_code,
            errors: vec![err],
        }
    }

    pub fn new_internal(err: String) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            errors: vec![err],
        }
    }

    pub fn new_bad_request(err: String) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            errors: vec![err],
        }
    }

    pub fn append_error(&mut self, err: String) {
        self.errors.push(err);
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::from_u16(self.status_code).unwrap(),
            serde_json::to_string(&self).unwrap(),
        )
            .into_response()
    }
}
