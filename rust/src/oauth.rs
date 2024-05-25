use crate::error::{Error, OAuthError};
use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use itertools::Itertools;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use rand::Rng;
use reqwest::header::CACHE_CONTROL;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

const AUTH_URL: &str = "https://www.tiktok.com/v2/auth/authorize/";
const TOKEN_URL: &str = "https://open.tiktokapis.com/v2/oauth/token/";
const REVOKE_URL: &str = "https://open.tiktokapis.com/v2/oauth/revoke/";

pub enum TiktokScope {
    ResearchAdlibBasic,
    ResearchDataBasic,
    UserInfoBasic,
    UserInfoProfile,
    UserInfoStats,
    VideoList,
    VideoPublish,
    VideoUpload,
}

impl TiktokScope {
    pub fn all() -> Vec<Self> {
        vec![
            Self::ResearchAdlibBasic,
            Self::ResearchDataBasic,
            Self::UserInfoBasic,
            Self::UserInfoProfile,
            Self::UserInfoStats,
            Self::VideoList,
            Self::VideoPublish,
            Self::VideoUpload,
        ]
    }
}

impl std::fmt::Display for TiktokScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ResearchAdlibBasic => write!(f, "research.adlib.basic"),
            Self::ResearchDataBasic => write!(f, "research.data.basic"),
            Self::UserInfoBasic => write!(f, "user.info.basic"),
            Self::UserInfoProfile => write!(f, "user.info.profile"),
            Self::UserInfoStats => write!(f, "user.info.stats"),
            Self::VideoList => write!(f, "video.list"),
            Self::VideoPublish => write!(f, "video.publish"),
            Self::VideoUpload => write!(f, "video.upload"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OAuthUrlResult {
    pub oauth_url: String,
    pub csrf_token: String,
}

#[derive(Debug, Clone, Default)]
pub struct TiktokOauthOptions {
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResult {
    pub open_id: String,
    pub scope: String,
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub refresh_expires_in: u64,
    pub token_type: String,
}

pub struct TiktokOauth {
    scopes: Vec<TiktokScope>,
    client_key: String,
    client_secret: String,
    callback_url: String,
    options: Option<TiktokOauthOptions>,
}

impl TiktokOauth {
    pub fn new(
        client_key: &str,
        client_secret: &str,
        callback_url: &str,
        scopes: Vec<TiktokScope>,
    ) -> Self {
        Self::new_with_options(client_key, client_secret, callback_url, scopes, None)
    }

    pub fn new_with_options(
        client_key: &str,
        client_secret: &str,
        callback_url: &str,
        scopes: Vec<TiktokScope>,
        options: Option<TiktokOauthOptions>,
    ) -> Self {
        Self {
            callback_url: callback_url.to_owned(),
            scopes,
            client_key: client_key.to_owned(),
            client_secret: client_secret.to_owned(),
            options,
        }
    }

    pub fn oauth_url(&self, state: Option<String>) -> OAuthUrlResult {
        let csrf_token = state.unwrap_or(csrf_token());
        let scope = self.scopes.iter().map(|it| it.to_string()).join(",");
        let redirect_uri = utf8_percent_encode(&self.callback_url, NON_ALPHANUMERIC);
        let oauth_url = format!(
            "{}?client_key={}&response_type=code&scope={}&redirect_uri={}&state={}",
            AUTH_URL, self.client_key, scope, redirect_uri, csrf_token
        );
        OAuthUrlResult {
            oauth_url,
            csrf_token,
        }
    }

    pub async fn token(&self, code: &str) -> Result<TokenResult, Error> {
        let mut form = HashMap::new();
        form.insert("client_key", self.client_key.as_str());
        form.insert("client_secret", self.client_secret.as_str());
        form.insert("grant_type", "authorization_code");
        form.insert("code", code);
        form.insert("redirect_uri", self.callback_url.as_str());
        execute_token(form, &self.options).await
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenResult, Error> {
        let mut form = HashMap::new();
        form.insert("client_key", self.client_key.as_str());
        form.insert("client_secret", self.client_secret.as_str());
        form.insert("grant_type", "refresh_token");
        form.insert("refresh_token", refresh_token);
        execute_token(form, &self.options).await
    }

    pub async fn revoke(&self, access_token: &str) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("client_key", self.client_key.as_str());
        form.insert("client_secret", self.client_secret.as_str());
        form.insert("token", access_token);
        let response = execute_send(REVOKE_URL, &form, &self.options).await?;
        let status_code = response.status();
        if status_code.is_success() {
            Ok(())
        } else {
            let json = response.json().await?;
            let token_error: OAuthError = serde_json::from_value(json)?;
            Err(Error::OAuth(token_error, status_code))
        }
    }
}

async fn execute_send(
    url: &str,
    form: &HashMap<&str, &str>,
    options: &Option<TiktokOauthOptions>,
) -> Result<reqwest::Response, reqwest::Error> {
    let builder = reqwest::Client::new()
        .post(url)
        .header(CACHE_CONTROL, "no-cache")
        .form(form);
    let builder = if let Some(options) = options {
        if let Some(timeout) = options.timeout {
            builder.timeout(timeout)
        } else {
            builder
        }
    } else {
        builder
    };
    builder.send().await
}

async fn execute_token(
    form: HashMap<&str, &str>,
    options: &Option<TiktokOauthOptions>,
) -> Result<TokenResult, Error> {
    let response = execute_send(TOKEN_URL, &form, options).await?;
    let status_code = response.status();
    let json = response.json().await?;
    if status_code.is_success() {
        let token_result: TokenResult = serde_json::from_value(json)?;
        Ok(token_result)
    } else {
        let token_error: OAuthError = serde_json::from_value(json)?;
        Err(Error::OAuth(token_error, status_code))
    }
}

fn csrf_token() -> String {
    let random_bytes: Vec<u8> = (0..16).map(|_| rand::thread_rng().gen::<u8>()).collect();
    BASE64_URL_SAFE_NO_PAD.encode(random_bytes)
}
