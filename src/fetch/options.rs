use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FetchOptions {
    pub method: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl FetchOptions {
    pub fn reqwest_method(&self) -> Method {
        if let Some(method) = &self.method {
            Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET)
        } else {
            Method::GET
        }
    }
}
