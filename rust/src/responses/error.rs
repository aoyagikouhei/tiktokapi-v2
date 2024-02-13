use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Error {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Error {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty();
        if !res {
            println!("Error {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Code {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "access_token_invalid")]
    AccessTokenInvalid,
    #[serde(rename = "internal_error")]
    InternalError,
    #[serde(rename = "invalid_file_upload")]
    InvalidFileUpload,
    #[serde(rename = "invalid_params")]
    InvalidParams,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "scope_not_authorized")]
    ScopeNotAuthorized,
    #[serde(rename = "scope_permission_missed")]
    ScopePermissionMissed,
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "ok"),
            Self::AccessTokenInvalid => write!(f, "access_token_invalid"),
            Self::InternalError => write!(f, "internal_error"),
            Self::InvalidFileUpload => write!(f, "invalid_file_upload"),
            Self::InvalidParams => write!(f, "invalid_params"),
            Self::RateLimitExceeded => write!(f, "rate_limit_exceeded"),
            Self::ScopeNotAuthorized => write!(f, "scope_not_authorized"),
            Self::ScopePermissionMissed => write!(f, "scope_permission_missed"),
        }
    }
}

impl Default for Code {
    fn default() -> Self {
        Self::Ok
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ErrorField {
    Code,
    Message,
    LogId,
}

impl ErrorField {
    pub fn all() -> HashSet<Self> {
        let mut set = HashSet::new();
        set.insert(ErrorField::Code);
        set.insert(ErrorField::Message);
        set.insert(ErrorField::LogId);
        set
    }
}

impl std::fmt::Display for ErrorField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Code => write!(f, "code"),
            Self::Message => write!(f, "message"),
            Self::LogId => write!(f, "log_id"),
        }
    }
}
