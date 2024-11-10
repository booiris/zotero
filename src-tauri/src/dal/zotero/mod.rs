use std::sync::{Arc, LazyLock};

use api::item::model::UploadAuthOk;
use error::ZoteroError;
use model::{KeyResp, ZoteroApiKey};
use reqwest::{Client, ClientBuilder, Response};
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

    pub async fn user_post<T>(
        &self,
        path: impl AsRef<str>,
        key: impl AsRef<str>,
        form: &[(&str, &str)],
        old_md5: impl AsRef<str>,
    ) -> Result<T, ZoteroError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!(
            "{}/users/{}{}/{}/file",
            BASE_URL,
            self.user_id,
            path.as_ref(),
            key.as_ref()
        );
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.api_key.as_ref())
            .header("If-Match", old_md5.as_ref())
            .form(form)
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

    pub async fn user_post_resp(
        &self,
        key: impl AsRef<str>,
        auth_resp: UploadAuthOk,
        old_md5: impl AsRef<str>,
    ) -> Result<Response, ZoteroError> {
        let mut form = reqwest::multipart::Form::new().text("params", "1");
        form = form.part("prefix", reqwest::multipart::Part::text(auth_resp.prefix));
        form = form.part("file", reqwest::multipart::Part::text("test"));
        form = form.part("suffix", reqwest::multipart::Part::text(auth_resp.suffix));
        form = form.part(
            "upload",
            reqwest::multipart::Part::text(auth_resp.upload_key),
        );
        form = form.part(
            "contentType",
            reqwest::multipart::Part::text(auth_resp.content_type.clone()),
        );
        let response = self
            .client
            .post(format!(
                "{}/users/{}/items/{}/file",
                BASE_URL,
                self.user_id,
                key.as_ref()
            ))
            .header("Content-Type", auth_resp.content_type)
            .body("test")
            .multipart(form)
            .bearer_auth(&self.api_key)
            .header("If-Match", old_md5.as_ref())
            .send()
            .await?;
        Ok(response)
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
