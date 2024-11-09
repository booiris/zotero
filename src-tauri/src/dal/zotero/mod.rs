use std::sync::{Arc, LazyLock};

use error::ZoteroError;
use model::{KeyResp, ZoteroApiKey};
use reqwest::{Client, ClientBuilder};
use tauri::http::HeaderMap;
pub mod api;
pub mod error;
pub mod model;
use tracing::error;

use crate::model::auth::UserName;

const BASE_URL: &str = "https://api.zotero.org";

fn client() -> &'static Client {
    static CLIENT: LazyLock<Client> = LazyLock::new(|| {
        let mut client_builder = ClientBuilder::new();

        let mut headers = HeaderMap::new();
        headers.insert("Zotero-API-Version", "3".parse().unwrap());
        client_builder = client_builder.default_headers(headers);

        client_builder
            .build()
            .expect("failed to create zotero client")
    });
    &CLIENT
}

#[derive(Debug, Clone)]
pub struct Zotero {
    client: &'static Client,
    api_key: Arc<ZoteroApiKey>,
    user_id: i64,
    pub user_name: Arc<UserName>,
}

impl Zotero {
    pub async fn new(api_key: ZoteroApiKey) -> Result<Self, ZoteroError> {
        let client = client();
        let url = format!("{}/keys/{}", BASE_URL, api_key);
        let response = client.get(url).send().await?;

        if !response.status().is_success() && response.status() != reqwest::StatusCode::NOT_MODIFIED
        {
            return Err(ZoteroError::RequestInvalid(response.status()));
        }

        let response = response.text().await?;

        let key_resp: KeyResp = match serde_json::from_str(&response) {
            Ok(key_resp) => key_resp,
            Err(e) => {
                error!(
                    "[zotero client new] resp data: {}, error: {:?}",
                    response, e
                );
                return Err(ZoteroError::ApiKey(response));
            }
        };

        if !key_resp.access.user.files
            || !key_resp.access.user.library
            || !key_resp.access.user.notes
        {
            return Err(ZoteroError::ApiKey(
                "no access to files, library or notes".into(),
            ));
        }

        Ok(Self {
            client,
            api_key: Arc::new(api_key),
            user_id: key_resp.user_id,
            user_name: Arc::new(key_resp.user_name),
        })
    }

    pub async fn user_get<T>(&self, path: impl AsRef<str>) -> Result<T, ZoteroError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/users/{}{}", BASE_URL, self.user_id, path.as_ref());
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.api_key.as_ref())
            .send()
            .await?
            .text()
            .await?;
        match serde_json::from_str(&response) {
            Ok(resp) => Ok(resp),
            Err(e) => {
                error!("resp data: {}, url: {}, error: {:?}", response, url, e);
                Err(ZoteroError::Data(e))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy_macro::dotenv;

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_get_user_id() {
        let client = client();

        let url = format!("{}/keys/{}", BASE_URL, dotenv!("ZOTERO_API_KEY"));
        let response = client.get(url).send().await.unwrap();
        println!("{:?}", response.text().await);
    }

    #[tokio::test]
    async fn test_no_auth() {
        let zotero = Zotero::new("".into()).await;
        println!("{:?}", zotero);
        assert!(matches!(
            zotero.unwrap_err(),
            ZoteroError::RequestInvalid(reqwest::StatusCode::FORBIDDEN)
        ));
    }

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_new() {
        let zotero = Zotero::new(dotenv!("ZOTERO_API_KEY").into()).await.unwrap();
        println!("{:?}", zotero);
    }
}
