use std::sync::Arc;

use crate::dal::zotero::error::ZoteroError;
use crate::error::Error;
use crate::model::zotero_data::{CollectionsData, EMPTY_COLLECTION_KEY};
use crate::AppState;
use parking_lot::Mutex;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_collections(
    state: State<'_, Mutex<AppState>>,
) -> Result<(Arc<Vec<CollectionsData>>, usize, usize), Error> {
    if let Some(data) = &state.lock().data {
        Ok((
            data.collections.clone(),
            data.items.len(),
            data.collections_item_map
                .get(EMPTY_COLLECTION_KEY)
                .map(|x| x.len())
                .unwrap_or_default(),
        ))
    } else {
        Err(Error::Zotero(ZoteroError::NoData))
    }
}
