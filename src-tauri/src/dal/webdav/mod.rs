use std::sync::LazyLock;

use ahash::AHashMap;
use error::WebDavError;
use parking_lot::RwLock;
use reqwest::{Body, Response};
use reqwest_dav::{Auth, Client, ClientBuilder};
use tracing::debug;

use crate::model::auth::{Secret, UserName};

pub mod error;

pub struct WebDavAuth {
    pub username: UserName,
    pub password: Secret,
}

pub struct WebDavClient {
    client: Client,
}

impl WebDavClient {
    pub async fn get(&self, path: impl AsRef<str>) -> Result<Response, WebDavError> {
        debug!("get: {}", path.as_ref());
        Ok(self.client.get(path.as_ref()).await?)
    }

    pub async fn put(
        &self,
        path: impl AsRef<str>,
        body: impl Into<Body>,
    ) -> Result<(), WebDavError> {
        debug!("put: {}", path.as_ref());
        Ok(self.client.put(path.as_ref(), body).await?)
    }
}

pub fn client(
    host: impl AsRef<str>,
    auth: Option<WebDavAuth>,
) -> Result<&'static WebDavClient, WebDavError> {
    static CLIENTS: LazyLock<RwLock<AHashMap<String, &'static WebDavClient>>> =
        LazyLock::new(|| RwLock::new(AHashMap::new()));

    let mut key = host.as_ref().to_string();
    if let Some(auth) = auth.as_ref() {
        key += &format!(":{}:{}", auth.username, auth.password);
    }

    if let Some(c) = CLIENTS.read().get(&key) {
        return Ok(*c);
    }

    let mut client_builder = ClientBuilder::new().set_host(host.as_ref().to_string());

    if let Some(auth) = auth {
        client_builder = client_builder.set_auth(Auth::Basic(auth.username, auth.password.into()));
    }

    let client = client_builder.build()?;
    let client = WebDavClient { client };
    let client_ref = Box::leak(Box::new(client));

    CLIENTS.write().insert(key, client_ref);
    Ok(client_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy_macro::dotenv;
    use tracing::info;

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_no_auth_client() {
        let client = client(dotenv!("WEB_DAV_HELLO_HOST"), None).unwrap();

        let resp = client.get("/").await.unwrap();
        assert!(resp.status().is_success());

        let text = resp.text().await;
        info!("{:?}", text);
        assert!(text.is_ok());
    }

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_auth_client_data() {
        let client = client(
            dotenv!("WEB_DAV_AUTH_HOST"),
            Some(WebDavAuth {
                username: dotenv!("WEB_DAV_USERNAME").to_string(),
                password: dotenv!("WEB_DAV_PASSWORD").into(),
            }),
        )
        .unwrap();

        let resp = client.get(dotenv!("WEB_DAV_DATA_PATH")).await.unwrap();
        assert!(resp.status().is_success());

        let text = resp.text().await;
        info!("{:?}", text.unwrap().len());
        // assert!(text.is_ok());
    }

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_auth_client() {
        let client = client(
            dotenv!("WEB_DAV_AUTH_HOST"),
            Some(WebDavAuth {
                username: dotenv!("WEB_DAV_USERNAME").to_string(),
                password: dotenv!("WEB_DAV_PASSWORD").into(),
            }),
        )
        .unwrap();

        let resp = client.get("/").await.unwrap();
        assert!(resp.status().is_success());

        let text = resp.text().await;
        info!("{:?}", text);
        assert!(text.is_ok());
    }
}
