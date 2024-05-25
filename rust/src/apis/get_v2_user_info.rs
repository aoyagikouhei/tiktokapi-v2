use crate::responses::user::UserField;
use crate::responses::{error::Error, user::User};
use crate::{
    apis::{apply_options, execute_api, make_url, ApiOptions},
    error::Error as ApiError,
};
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const URL: &str = "/user/info/";

#[derive(Debug, Clone, Default)]
pub struct Api {
    options: Option<ApiOptions>,
    fields: HashSet<UserField>,
}

impl Api {
    pub fn new(fields: HashSet<UserField>, options: Option<ApiOptions>) -> Self {
        Self { options, fields }
    }

    #[allow(clippy::vec_init_then_push)]
    pub fn build(self, bearer_code: &str) -> RequestBuilder {
        let mut query_parameters = vec![];
        query_parameters.push(("fields", self.fields.iter().join(",")));
        let client = reqwest::Client::new()
            .get(make_url(URL, &self.options))
            .query(&query_parameters)
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
    pub user: Option<User>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Data {
    pub fn is_empty_extra(&self) -> bool {
        let res = self.extra.is_empty()
            && self
                .user
                .as_ref()
                .map(|it| it.is_empty_extra())
                .unwrap_or(true);
        if !res {
            println!("Data {:?}", self.extra);
        }
        res
    }
}
