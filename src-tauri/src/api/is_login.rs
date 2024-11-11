use crate::dal::zotero::Zotero;
use crate::error::Error;
use crate::AppState;
use parking_lot::Mutex;
use tauri::State;

#[tauri::command(rename_all = "snake_case")]
pub async fn is_login(state: State<'_, Mutex<AppState>>) -> Result<bool, Error> {
    let api_key = state.lock().api_key.clone();
    if let Some(api_key) = api_key {
        if state.lock().zotero.is_none() {
            let zotero = Zotero::new(api_key).await?;
            state.lock().zotero = Some(zotero);
        }
        Ok(true)
    } else {
        Ok(false)
    }
}
