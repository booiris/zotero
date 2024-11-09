use std::io;
use std::path::PathBuf;

use anyhow::Context;
use dotenvy_macro::dotenv;
use parking_lot::Mutex;
use tauri::State;
use tauri_plugin_dialog::DialogExt;
use tokio::fs;
use tracing::info;

use crate::dal::webdav::{client, WebDavAuth};
use crate::dal::zotero::error::ZoteroError;
use crate::error::Error;
use crate::AppState;

const DOCUMENT_PATH: &str = "/storage/emulated/0/Documents/zotero";

#[tauri::command(rename_all = "snake_case")]
pub async fn download_pdf(
    key: &str,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
) -> Result<(), Error> {
    let client = client(
        dotenv!("WEB_DAV_AUTH_HOST"),
        Some(WebDavAuth {
            username: dotenv!("WEB_DAV_USERNAME").into(),
            password: dotenv!("WEB_DAV_PASSWORD").into(),
        }),
    )?;

    let data = state
        .lock()
        .data
        .as_ref()
        .map(|x| x.items.clone())
        .ok_or(ZoteroError::NoData)?;

    let item = data.get(key).ok_or(ZoteroError::NoData)?;
    let key = item
        .sub_items
        .iter()
        .find(|x| x.data.content_type == "application/pdf")
        .ok_or(ZoteroError::NoPdf)?
        .key
        .as_ref();

    let resp = client.get("/zotero/".to_string() + key + ".zip").await?;
    let path = DOCUMENT_PATH.parse::<PathBuf>().unwrap().join(key);

    fs::create_dir_all(&path)
        .await
        .context("create dir failed")?;

    let bytes = resp.bytes().await?;

    let pdf_path = tokio::task::spawn_blocking(move || -> Result<Option<PathBuf>, Error> {
        let reader = std::io::BufReader::new(std::io::Cursor::new(bytes));
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut pdf_path = None;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let out_path = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };
            let out_path = path.join(out_path);

            if file.is_dir() {
                std::fs::create_dir_all(&out_path).context("create unzip dir failed")?;
            } else {
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).context("create unzip parent dir failed")?;
                    }
                }

                if out_path.extension() == Some("pdf".as_ref()) {
                    pdf_path = Some(out_path.clone());
                }

                let mut out_file =
                    std::fs::File::create(&out_path).context("create unzip file failed")?;
                io::copy(&mut file, &mut out_file).context("copy unzip file failed")?;
            }
        }
        Ok(pdf_path)
    })
    .await??;

    if let Some(_pdf_path) = pdf_path {
        let file_path = app.dialog().file().blocking_pick_file();
        info!("file_path: {:?}", file_path);
    }

    Ok(())
}
