use model::Item;

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
        println!("{:?}", items);
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
}
