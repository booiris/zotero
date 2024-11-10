use model::{Item, UploadAuthOk, UploadAuthResponse};

use crate::dal::zotero::{error::ZoteroError, Zotero};

pub mod model;

impl Zotero {
    // TODO: Returns up to 100 items per page, Add pagination support for large result sets
    pub async fn get_all_items(&self) -> Result<Vec<Item>, ZoteroError> {
        let resp = self.user_get("/items").await?;
        Ok(resp)
    }

    #[allow(dead_code)]
    pub async fn get_collection_top_items(
        &self,
        collection_key: impl AsRef<str>,
    ) -> Result<Vec<Item>, ZoteroError> {
        let resp = self
            .user_get(&format!(
                "/collections/{}/items/top",
                collection_key.as_ref()
            ))
            .await?;
        Ok(resp)
    }

    pub async fn start_upload(
        &self,
        key: impl AsRef<str>,
        form: &[(&str, &str)],
        old_md5: &str,
    ) -> Result<UploadAuthResponse, ZoteroError> {
        let resp = self.user_post("/items", key, form, old_md5).await?;
        Ok(resp)
    }

    pub async fn finish_upload(
        &self,
        key: impl AsRef<str>,
        auth_resp: UploadAuthOk,
        old_md5: &str,
    ) -> Result<(), ZoteroError> {
        let resp = self.user_post_resp(key, auth_resp, old_md5).await?;
        tracing::info!("finish upload resp: {:?}", resp);
        tracing::info!("finish upload resp: {:?}", resp.text().await);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy_macro::dotenv;

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_get_items() {
        let zotero = Zotero::new(dotenv!("ZOTERO_API_KEY").into()).await.unwrap();
        let items = zotero.get_all_items().await.unwrap();
        println!("{}", serde_json::to_string(&items).unwrap());
    }

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_get_collection_items() {
        let zotero = Zotero::new(dotenv!("ZOTERO_API_KEY").into()).await.unwrap();
        let items = zotero
            .get_collection_top_items(dotenv!("ZOTERO_COLLECTION_KEY"))
            .await
            .unwrap();
        println!("{}", serde_json::to_string(&items).unwrap());
    }

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_get_upload_auth() {
        let zotero = Zotero::new(dotenv!("ZOTERO_API_KEY").into()).await.unwrap();
        let form = [
            ("md5", dotenv!("TEST_ITEM_NEW_MD5")),
            ("filename", "test"),
            ("filesize", "1000"),
            ("mtime", "1000"),
        ];
        let resp = zotero
            .start_upload(
                dotenv!("TEST_ITEM_KEY"),
                &form,
                dotenv!("TEST_ITEM_OLD_MD5"),
            )
            .await
            .unwrap();

        println!("{}", serde_json::to_string(&resp).unwrap());
        if let UploadAuthResponse::Ok(new) = resp {
            zotero
                .finish_upload(dotenv!("TEST_ITEM_KEY"), new, dotenv!("TEST_ITEM_OLD_MD5"))
                .await
                .unwrap();
        }
    }
}
