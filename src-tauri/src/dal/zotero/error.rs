use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum ZoteroError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("resp data error: {0}")]
    Data(#[from] serde_json::Error),
    #[error("api key error: {0}")]
    ApiKey(String),
    #[error("request invalid: http code: {0}, message: {0}.canonical_reason()")]
    RequestInvalid(StatusCode),
    #[error("not login")]
    NotLogin,
    #[error("no data, please login first and refresh")]
    NoData,
    #[error("no pdf found")]
    NoPdf,
}
