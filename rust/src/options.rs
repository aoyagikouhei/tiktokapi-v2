use std::time::Duration;

use reqwest::RequestBuilder;

const URL_PREFIX: &str = "https://open.tiktokapis.com/v2";
const ENV_KEY: &str = "TICTOK_V2_PREFIX_API";

#[derive(Debug, Clone, Default)]
pub struct TiktokOptions {
    pub prefix_url: Option<String>,
    pub timeout: Option<Duration>,
}

pub fn clear_prefix_url() {
    std::env::set_var(ENV_KEY, URL_PREFIX);
}

pub fn setup_prefix_url(url: &str) {
    std::env::set_var(ENV_KEY, url);
}

pub(crate) fn make_url(postfix_url: &str, options: &Option<TiktokOptions>) -> String {
    make_url_with_prefix(
        &std::env::var(ENV_KEY).unwrap_or(URL_PREFIX.to_owned()),
        options,
        postfix_url,
    )
}

fn make_url_with_prefix(
    default_perfix_url: &str,
    options: &Option<TiktokOptions>,
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

pub(crate) fn apply_options(
    client: RequestBuilder,
    options: &Option<TiktokOptions>,
) -> RequestBuilder {
    let Some(options) = options else {
        return client;
    };
    let Some(timeout) = options.timeout else {
        return client;
    };
    client.timeout(timeout)
}
