use log::{debug, error, info};
use reqwest::{
    header::{HeaderMap, HeaderName},
    Client,
};

mod error;
mod options;
mod random;
mod response;

pub use error::Error;
pub use options::FetchOptions;
pub use response::FetchResponse;

pub type FetchResponseResult = Result<FetchResponse, Error>;

pub async fn fetch<U>(resource: U, init: Option<FetchOptions>) -> FetchResponseResult
where
    U: reqwest::IntoUrl + std::fmt::Debug,
{
    let client_builder = Client::builder()
        .danger_accept_invalid_certs(true)
        .use_rustls_tls();
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

    info!(
        "[{req_id}] {method} {url}",
        method = request.method(),
        url = request.url().to_string()
    );
    let response = client.execute(request).await?;
    info!("[{req_id}] {status}", status = response.status());

    let fetch_response = FetchResponse::from_response(response).await;

    if let Ok(fetch_response) = &fetch_response {
        if fetch_response.ok {
            debug!("[{req_id}] {text}", text = fetch_response.text);
        } else {
            error!("[{req_id}] {text}", text = fetch_response.text);
        }
    }

    fetch_response
}
