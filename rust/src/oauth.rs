use std::time::Duration;

use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use thiserror::Error;

const AUTH_URL: &str = "https://www.tiktok.com/v2/auth/authorize/";
const TOKEN_URL: &str = "https://open.tiktokapis.com/v2/oauth/token/";

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

#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("Url {0}")]
    Url(#[from] oauth2::url::ParseError),

    #[error("Token {0}")]
    Token(String),
}

#[derive(Debug, Clone)]
pub struct OAuthUrlResult {
    pub oauth_url: String,
    pub csrf_token: String,
}

#[derive(Debug, Clone)]
pub struct TokenResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<Duration>,
}

pub struct TiktokOauth {
    basic_client: BasicClient,
    redirect_url: RedirectUrl,
    scopes: Vec<Scope>,
}

impl TiktokOauth {
    pub fn new(
        client_key: &str,
        client_secret: &str,
        callback_url: &str,
        scopes: Vec<TiktokScope>,
    ) -> Result<Self, OAuthError> {
        let basic_client = BasicClient::new(
            ClientId::new(client_key.to_owned()),
            Some(ClientSecret::new(client_secret.to_owned())),
            AuthUrl::new(AUTH_URL.to_owned())?,
            Some(TokenUrl::new(TOKEN_URL.to_owned())?),
        );
        let redirect_url = RedirectUrl::new(callback_url.to_string())?;
        let scopes: Vec<Scope> = scopes
            .into_iter()
            .map(|it| Scope::new(it.to_string()))
            .collect();
        Ok(Self {
            basic_client,
            redirect_url,
            scopes,
        })
    }

    pub fn oauth_url(&self) -> OAuthUrlResult {
        let (auth_url, csrf_token) = self
            .basic_client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .authorize_url(CsrfToken::new_random)
            .add_scopes(self.scopes.clone())
            .url();

        OAuthUrlResult {
            oauth_url: auth_url.to_string(),
            csrf_token: csrf_token.to_string(),
        }
    }

    pub async fn token(&self, code: &str) -> Result<TokenResult, OAuthError> {
        let token = self
            .basic_client
            .clone()
            .set_redirect_uri(self.redirect_url.clone())
            .exchange_code(AuthorizationCode::new(code.to_owned()))
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuthError::Token(format!("{:?}", e)))?;
        Ok(TokenResult {
            access_token: token.access_token().secret().to_string(),
            refresh_token: token.refresh_token().map(|it| it.secret().to_string()),
            expires_in: token.expires_in(),
        })
    }
}
