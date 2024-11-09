use std::sync::Arc;

use ahash::{AHashMap, AHashSet};
use parking_lot::Mutex;
use tauri::State;
use tracing::{error, info};

use crate::dal::zotero::api::collection::model::Collection;
use crate::dal::zotero::api::item::model::Item;
use crate::dal::zotero::error::ZoteroError;
use crate::dal::zotero::Zotero;
use crate::error::Error;
use crate::model::zotero_data::{CollectionsData, Data, SimpleItemData, EMPTY_COLLECTION_KEY};
use crate::AppState;

#[tauri::command(rename_all = "snake_case")]
pub async fn refresh(state: State<'_, Mutex<AppState>>) -> Result<(), Error> {
    info!("refreshing zotero data");

    let zotero = state.lock().zotero.clone();

    if let Some(zotero) = zotero {
        state.lock().data = Some(get_data(zotero).await?);
        Ok(())
    } else {
        Err(Error::Zotero(ZoteroError::NotLogin))
    }
}

async fn get_data(client: Zotero) -> Result<Data, Error> {
    let c1 = client.clone();
    let collections = tokio::spawn(async move {
        let collections = c1.get_all_collections().await?;
        Ok::<_, Error>(parse_collections(collections, AHashSet::new()))
    });
    let items = tokio::spawn(async move {
        let items = client.get_all_items().await?;
        Ok::<_, Error>(parse_items(items))
    });

    let (collections, items) = tokio::try_join!(collections, items)?;
    let collections = collections?;
    let items = items?;
    Ok(Data {
        collections: Arc::new(collections),
        items: Arc::new(items.0),
        collections_item_map: Arc::new(items.1),
    })
}

fn parse_collections(collections: Vec<Collection>, next: AHashSet<String>) -> Vec<CollectionsData> {
    if collections.is_empty() {
        return vec![];
    }
    let mut roots = vec![];
    let mut rest_collections = vec![];
    let mut index = AHashMap::new();
    for collection in collections {
        if collection.data.parent_collection.is_none() || next.contains(&collection.data.key) {
            index.insert(collection.data.key.clone(), roots.len());
            roots.push(CollectionsData {
                key: collection.data.key,
                name: collection.data.name,
                children: None,
                father: collection.data.parent_collection,
            });
        } else {
            rest_collections.push(collection);
        }
    }
    let mut next = AHashSet::new();
    for r in &rest_collections {
        if let Some(father) = &r.data.parent_collection {
            if index.contains_key(father) {
                next.insert(r.key.clone());
            }
        } else {
            error!("collection has no father: {}", r.data.key);
        }
    }

    let children = parse_collections(rest_collections, next);
    for child in children {
        if let Some(father) = &child.father {
            if let Some(father_index) = index.get(father) {
                roots[*father_index]
                    .children
                    .get_or_insert_with(Vec::new)
                    .push(child);
            } else {
                error!(
                    "father collection not found: {} now key: {}",
                    father, child.key
                );
            }
        } else {
            error!("children collection has no father: {}", child.key);
        }
    }

    for r in &mut roots {
        if let Some(children) = r.children.as_mut() {
            children.sort_by(|a, b| a.name.cmp(&b.name));
        }
    }

    roots.sort_by(|a, b| a.name.cmp(&b.name));

    roots
}

#[allow(clippy::type_complexity)]
fn parse_items(
    items: Vec<Item>,
) -> (
    AHashMap<String, Item>,
    AHashMap<String, Arc<Vec<SimpleItemData>>>,
) {
    let mut collections_item_map = AHashMap::new();

    for item in &items {
        if let Some(father_collection) = &item.data.collections {
            if father_collection.is_empty() {
                collections_item_map
                    .entry(EMPTY_COLLECTION_KEY.to_string())
                    .or_insert_with(Vec::new)
                    .push(SimpleItemData {
                        key: item.key.clone(),
                        title: item
                            .data
                            .title
                            .clone()
                            .unwrap_or("unknown name".to_string()),
                    });
            } else {
                for collection in father_collection {
                    collections_item_map
                        .entry(collection.clone())
                        .or_insert_with(Vec::new)
                        .push(SimpleItemData {
                            key: item.key.clone(),
                            title: item
                                .data
                                .title
                                .clone()
                                .unwrap_or("unknown name".to_string()),
                        });
                }
            }
        }
    }

    let collections_item_map = collections_item_map
        .into_iter()
        .map(|x| (x.0, Arc::new(x.1)))
        .collect();

    let mut items_map = AHashMap::new();
    let mut items_temp = vec![];

    for item in items {
        if item.data.parent_item.is_none() {
            items_map.insert(item.key.clone(), item);
        } else {
            items_temp.push(item);
        }
    }

    for item in items_temp {
        let father_item_key = item.data.parent_item.clone().unwrap();
        if !items_map.contains_key(&father_item_key) {
            error!(
                "father item not found: {}, key: {}",
                father_item_key, item.key
            );
            continue;
        }
        let father_item = items_map.get_mut(&father_item_key).unwrap();
        father_item.sub_items.push(item);
    }

    (items_map, collections_item_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy_macro::dotenv;

    #[test]
    fn test_parse_collections() {
        let test_collection_data =
            std::fs::read_to_string(dotenv!("TEST_COLLECTION_DATA_PATH")).unwrap();
        let collections: Vec<Collection> = serde_json::from_str(&test_collection_data).unwrap();
        let parsed = parse_collections(collections, AHashSet::new());

        println!("{}", serde_json::to_string(&parsed).unwrap());
    }

    #[cfg(feature = "__local_test__")]
    #[test]
    fn test_parse_items() {
        let test_item_data = std::fs::read_to_string(dotenv!("TEST_ITEM_DATA_PATH")).unwrap();

        let items: Vec<Item> = serde_json::from_str(&test_item_data).unwrap();
        let parsed = parse_items(items);

        println!("{}", serde_json::to_string(&parsed.0).unwrap());
        println!("{}", serde_json::to_string(&parsed.1).unwrap());
    }
}
