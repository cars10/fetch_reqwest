use multimap::MultiMap;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct FetchResponse {
    pub ok: bool,
    pub status: u16,
    pub text: String,
    pub headers: MultiMap<String, String>,
}

impl FetchResponse {
    pub async fn from_response(response: reqwest::Response) -> Result<Self, super::Error> {
        let status = response.status().as_u16();
        let ok = (200..=299).contains(&status);
        let mut headers = MultiMap::new();
        for (key, value) in response.headers() {
            headers.insert(key.as_str().to_string(), value.to_str()?.to_string());
        }
        let text = response.text().await?;

        Ok(FetchResponse {
            ok,
            status,
            text,
            headers,
        })
    }
}
