use ahash::AHashMap;
use serde::Serialize;
use std::sync::Arc;

use crate::dal::zotero::api::item::model::Item;
#[derive(Debug, Serialize)]
pub struct CollectionsData {
    pub name: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<CollectionsData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub father: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SimpleItemData {
    pub key: String,
    pub title: String,
}

pub struct Data {
    pub collections: Arc<Vec<CollectionsData>>,
    pub items: Arc<AHashMap<String, Item>>,

    /// empty key means the item is not in any collection
    pub collections_item_map: Arc<AHashMap<String, Arc<Vec<SimpleItemData>>>>,
}

pub const EMPTY_COLLECTION_KEY: &str = "";
