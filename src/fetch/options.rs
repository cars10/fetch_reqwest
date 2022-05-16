use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FetchOptions {
    pub method: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl FetchOptions {
    pub fn reqwest_method(&self) -> reqwest::Method {
        if let Some(method) = &self.method {
            reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::GET)
        } else {
            reqwest::Method::GET
        }
    }
}
