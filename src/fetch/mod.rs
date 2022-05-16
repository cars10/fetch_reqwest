mod error;
mod options;
mod response;

pub use error::Error;
pub use options::FetchOptions;
pub use response::FetchResponse;

pub type FetchResponseResult = Result<FetchResponse, Error>;

pub async fn fetch<U>(resource: U, init: Option<FetchOptions>) -> FetchResponseResult
where
    U: reqwest::IntoUrl + std::fmt::Debug,
{
    let client_builder = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .use_rustls_tls();
    let client = client_builder.build()?;
    let some_init = init.unwrap_or_default();

    let mut headers = reqwest::header::HeaderMap::new();
    for (name, value) in &some_init.headers {
        headers.insert(
            reqwest::header::HeaderName::from_bytes(name.as_bytes())?,
            value.parse()?,
        );
    }
    let request_builder = client
        .request(some_init.reqwest_method(), resource)
        .headers(headers)
        .body(some_init.body.unwrap_or_default());

    let response = request_builder.send().await?;
    response::FetchResponse::from_response(response).await
}
