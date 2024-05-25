pub mod get_v2_user_info;
pub mod post_v2_video_list;
pub mod post_v2_video_query;

use std::time::Duration;

use crate::{error::Error, URL_PREFIX};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

const ENV_KEY: &str = "TICTOK_V2_PREFIX_API";

pub fn clear_prefix_url() {
    std::env::set_var(ENV_KEY, URL_PREFIX);
}

pub fn setup_prefix_url(url: &str) {
    std::env::set_var(ENV_KEY, url);
}

pub(crate) fn make_url(postfix_url: &str, options: &Option<ApiOptions>) -> String {
    make_url_with_prefix(
        &std::env::var(ENV_KEY).unwrap_or(URL_PREFIX.to_owned()),
        options,
        postfix_url,
    )
}

fn make_url_with_prefix(
    default_perfix_url: &str,
    options: &Option<ApiOptions>,
    postfix_url: &str,
) -> String {
    let prefix_url = if let Some(options) = options {
        if let Some(prefix_url) = options.prefix_url.as_ref() {
            prefix_url
        } else {
            default_perfix_url
        }
    } else {
        default_perfix_url
    };
    format!("{}{}", prefix_url, postfix_url)
}

#[derive(Debug, Clone, Default)]
pub struct ApiOptions {
    pub prefix_url: Option<String>,
    pub timeout: Option<Duration>,
}

pub async fn execute_api<T>(builder: RequestBuilder) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = builder.send().await?;
    let status_code = response.status();

    if status_code.is_success() {
        Ok(response.json::<T>().await?)
    } else {
        let text = match response.text().await {
            Ok(text) => text,
            Err(err) => return Err(Error::Other(format!("{:?}", err), status_code)),
        };

        let json = match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(json) => json,
            Err(_err) => return Err(Error::Other(text, status_code)),
        };

        match serde_json::from_value::<crate::responses::error::Error>(json["error"].clone()) {
            Ok(err) => Err(Error::Api(err, status_code)),
            Err(_) => Err(Error::Other(text, status_code)),
        }
    }
}

pub(crate) fn apply_options(
    client: RequestBuilder,
    options: &Option<ApiOptions>,
) -> RequestBuilder {
    let Some(options) = options else {
        return client;
    };
    let Some(timeout) = options.timeout else {
        return client;
    };
    client.timeout(timeout)
}
