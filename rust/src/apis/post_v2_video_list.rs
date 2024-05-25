use crate::responses::video::VideoField;
use crate::responses::{error::Error, video::Video};
use crate::{
    apis::{apply_options, execute_api, make_url, ApiOptions},
    error::Error as ApiError,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "/video/list/";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    fields: HashSet<VideoField>,
    body: Body,
}

impl Api {
    pub fn new(fields: HashSet<VideoField>, body: Body, options: Option<ApiOptions>) -> Self {
        Self {
            options,
            fields,
            body,
        }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("fields", self.fields.iter().join(",")));
        let client = reqwest::Client::new()
            .post(make_url(URL, &self.options))
            .query(&query_parameters)
            .json(&self.body)
            .bearer_auth(bearer_code);
        apply_options(client, &self.options)
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
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
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
