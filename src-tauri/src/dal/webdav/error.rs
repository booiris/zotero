#[derive(Debug, thiserror::Error)]
pub enum WebDavError {
    #[error("client error: {0}")]
    Client(#[from] reqwest_dav::Error),

    #[error("response error: {0}")]
    Response(#[from] reqwest::Error),
}
