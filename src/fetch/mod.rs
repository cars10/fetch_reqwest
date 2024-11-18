use log::{debug, error, info};
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, Proxy,
};

mod error;
mod options;
mod random;
mod response;

pub use error::Error;
pub use options::FetchOptions;
pub use response::FetchResponse;

pub type FetchResponseResult = Result<FetchResponse, Error>;

fn build_proxy() -> Option<Proxy> {
    let url = std::env::var("http_proxy")
        .or_else(|_| std::env::var("https_proxy"))
        .ok()?;
    Proxy::https(url).ok()
}

pub async fn fetch<U>(resource: U, init: Option<FetchOptions>) -> FetchResponseResult
where
    U: reqwest::IntoUrl + std::fmt::Debug,
{
    let mut client_builder = Client::builder()
        .danger_accept_invalid_certs(true)
        .use_rustls_tls();

    if let Some(proxy) = build_proxy() {
        client_builder = client_builder.proxy(proxy);
    }
    let client = client_builder.build()?;
    let init = init.unwrap_or_default();

    let mut headers = HeaderMap::new();
    for (name, value) in &init.headers {
        headers.insert(HeaderName::from_bytes(name.as_bytes())?, value.parse()?);
    }
    let method = init.reqwest_method();
    let request = client
        .request(method, resource)
        .headers(headers)
        .body(init.body.unwrap_or_default())
        .build()?;

    let req_id = random::random_string(12).to_lowercase();
    let request_url = request.url().to_string();

    info!("[{}] {} {}", req_id, request.method(), request_url);
    let response = client.execute(request).await?;
    info!("[{}] {}", req_id, response.status());

    let fetch_response = FetchResponse::from_response(response).await?;

    if fetch_response.ok {
        debug!("[{}] {}", req_id, fetch_response.text);
    } else {
        error!("[{}] {}", req_id, fetch_response.text);
    }

    Ok(fetch_response)
}
