use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Timeout")]
    Timeout,

    #[error("Other {0}")]
    Other(String, StatusCode),

    #[error("OAuth {0:?}, {1}")]
    OAuth(OAuthError, StatusCode),

    #[error("ApiError {0:?}, {1}")]
    Api(crate::responses::error::Error, StatusCode),

    #[error("reqwest {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("serde json {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthError {
    pub error: String,
    pub error_description: String,
    pub log_id: String,
}
