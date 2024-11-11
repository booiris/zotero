use dal::zotero::Zotero;
use model::{auth::Secret, zotero_data::Data};
use parking_lot::Mutex;
use tauri::Manager;

mod api;
mod dal;
mod error;
mod model;

#[derive(Default)]
pub(crate) struct AppState {
    pub zotero: Option<Zotero>,
    pub data: Option<Data>,
    pub api_key: Option<Secret>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logger();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            api::login::login,
            api::get_collections::get_collections,
            api::refresh::refresh,
            api::get_items::get_items_by_collection,
            api::download_pdf::download_pdf,
            api::is_login::is_login,
        ])
        .setup(|app| {
            let api_path = app.path().app_data_dir().unwrap().join("api_key");
            let api_key = std::fs::read_to_string(api_path).ok().map(Secret::from);

            app.manage(Mutex::new(AppState {
                api_key,
                ..Default::default()
            }));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_logger() {
    use android_logger::Config;
    use log::LevelFilter;
    #[cfg(debug_assertions)]
    let level = LevelFilter::Debug;
    #[cfg(not(debug_assertions))]
    let level = LevelFilter::Info;
    android_logger::init_once(
        Config::default()
            .with_max_level(level)
            .with_tag("ZoteroClient"),
    );

    tracing::info!("init logger success!");
}

#[cfg(test)]
mod tests {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    #[ctor::ctor]
    fn init_test() {
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug"));

        let formatting_layer = tracing_subscriber::fmt::layer()
            .pretty()
            .with_writer(std::io::stderr);

        tracing_subscriber::Registry::default()
            .with(env_filter)
            .with(tracing_error::ErrorLayer::default())
            .with(formatting_layer)
            .init();
    }
}
