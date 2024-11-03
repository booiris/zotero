use anyhow::Context;
use dal::webdav::client;
use error::Error;

mod api;
mod dal;
mod error;

async fn temp(url: String) -> Result<String, Error> {
    let resp = client(&url, None)
        .context("init client error")?
        .get("/")
        .await
        .context("get error")?
        .text()
        .await;
    Ok(format!("calling url: {}, response: {:?}", url, resp))
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(url: String) -> String {
    match temp(url).await {
        Ok(resp) => resp,
        Err(e) => format!("error: {:?}", e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
