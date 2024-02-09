use crate::responses::video::VideoField;
use crate::responses::{error::Error, video::Video};
use crate::{apis::execute_api, error::Error as ApiError};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "https://open.tiktokapis.com/v2/video/query/";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Filters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    fields: HashSet<VideoField>,
    body: Body,
}

impl Api {
    pub fn new(fields: HashSet<VideoField>, body: Body) -> Self {
        Self { fields, body }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("fields", self.fields.iter().join(",")));
        let client = reqwest::Client::new();
        client
            .post(URL)
            .query(&query_parameters)
            .json(&self.body)
            .bearer_auth(bearer_code)
    }

    pub async fn execute(self, bearer_code: &str) -> Result<Response, ApiError> {
        execute_api(self.build(bearer_code)).await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Response {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .data
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true)
            && self
                .error
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Response {:?}", self.extra);
        }
        res
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<Video>>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .videos
                .as_ref()
                .map(|it| it.iter().all(|item| item.is_empty_extra()))
                .unwrap_or(true);
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
