use std::sync::Arc;

use parking_lot::Mutex;
use tauri::State;

use crate::dal::zotero::error::ZoteroError;
use crate::error::Error;
use crate::model::zotero_data::SimpleItemData;
use crate::AppState;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_items_by_collection(
    collection_key: &str,
    state: State<'_, Mutex<AppState>>,
) -> Result<Arc<Vec<SimpleItemData>>, Error> {
    if let Some(data) = &state.lock().data {
        if collection_key == "all-items" {
            return Ok(Arc::new(
                data.items
                    .values()
                    .map(|x| SimpleItemData {
                        key: x.key.clone(),
                        title: x.data.title.clone().unwrap_or_default(),
                    })
                    .collect(),
            ));
        }
        Ok(data
            .collections_item_map
            .get(collection_key)
            .cloned()
            .unwrap_or_default())
    } else {
        Err(Error::Zotero(ZoteroError::NoData))
    }
}
