pub mod post_v2_video_list;
pub mod post_v2_video_query;

use crate::error::Error;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

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
