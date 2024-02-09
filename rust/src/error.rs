use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Timeout")]
    Timeout,

    #[error("Other {0}")]
    Other(String, Option<reqwest::StatusCode>),

    #[error("reqwest {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("serde json {0}")]
    Json(#[from] serde_json::Error),

    #[error("Api {0:?}")]
    Api(crate::responses::error::Error),
}
