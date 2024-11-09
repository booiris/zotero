use model::Collection;
use tracing::debug;

use crate::dal::zotero::{error::ZoteroError, Zotero};

pub mod model;

impl Zotero {
    // TODO: Returns up to 100 collections per page, Add pagination support for large result sets
    pub async fn get_all_collections(&self) -> Result<Vec<Collection>, ZoteroError> {
        let resp = self.user_get("/collections").await?;
        debug!("get collections: {}", serde_json::to_string(&resp).unwrap());
        Ok(resp)
    }

    #[allow(dead_code)]
    pub async fn get_collection_top(&self) -> Result<Vec<Collection>, ZoteroError> {
        let resp = self.user_get("/collections/top").await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy_macro::dotenv;

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_get_collections() {
        let zotero = Zotero::new(dotenv!("ZOTERO_API_KEY").into()).await.unwrap();
        let collections = zotero.get_all_collections().await.unwrap();
        println!("{:?}", collections);
    }

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_get_collection_top() {
        let zotero = Zotero::new(dotenv!("ZOTERO_API_KEY").into()).await.unwrap();
        let collection = zotero.get_collection_top().await.unwrap();
        println!("{}", serde_json::to_string(&collection).unwrap());
    }
}
