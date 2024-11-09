use parking_lot::Mutex;
use tauri::State;
use tracing::info;

use crate::dal::zotero::Zotero;
use crate::error::Error;
use crate::model::auth::Secret;
use crate::AppState;

#[tauri::command(rename_all = "snake_case")]
pub async fn login(api_key: &str, state: State<'_, Mutex<AppState>>) -> Result<(), Error> {
    info!("logging in with api key");
    let api_key: Secret = api_key.into();

    let zotero = Zotero::new(api_key).await?;

    info!("{} log success!", zotero.user_name);

    state.lock().zotero = Some(zotero);

    Ok(())
}
