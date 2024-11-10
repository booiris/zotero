use std::io;
use std::path::PathBuf;

use anyhow::Context;
use dotenvy_macro::dotenv;
use futures_util::StreamExt;
use parking_lot::Mutex;
use tauri::ipc::Channel;
use tauri::State;
use tauri_plugin_shell::ShellExt;
use tokio::fs;

use crate::dal::webdav::{client, WebDavAuth};
use crate::dal::zotero::error::ZoteroError;
use crate::error::Error;
use crate::AppState;

const DOCUMENT_PATH: &str = "/storage/emulated/0/Download/zotero";

#[tauri::command(rename_all = "snake_case")]
pub async fn download_pdf(
    key: &str,
    state: State<'_, Mutex<AppState>>,
    app: tauri::AppHandle,
    downloaded_size: Channel<usize>,
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
    let size = resp.content_length().unwrap_or(0);

    let path = DOCUMENT_PATH.parse::<PathBuf>().unwrap().join(key);

    fs::create_dir_all(&path)
        .await
        .context("create dir failed")?;

    let mut stream = resp.bytes_stream();
    let mut data = Vec::with_capacity(size as usize);
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                data.extend(bytes);
                downloaded_size.send(data.len()).ok();
            }
            Err(e) => return Err(e.into()),
        };
    }

    let pdf_path = tokio::task::spawn_blocking(move || -> Result<Option<PathBuf>, Error> {
        let reader = std::io::BufReader::new(std::io::Cursor::new(data));
        let mut archive = zip::ZipArchive::new(reader)?;
        let mut pdf_path = None;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let mut out_path = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            out_path = if out_path.extension() == Some("pdf".as_ref()) {
                "/storage/emulated/0/Download/"
                    .parse::<PathBuf>()
                    .unwrap()
                    .join(out_path)
            } else {
                path.join(out_path)
            };

            if file.is_dir() {
                std::fs::create_dir_all(&out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p)?;
                    }
                }

                if out_path.extension() == Some("pdf".as_ref()) {
                    pdf_path = Some(out_path.clone());
                }

                let mut out_file = std::fs::File::create(&out_path).with_context(|| {
                    format!("create unzip file failed: {:?}", out_path.display())
                })?;

                io::copy(&mut file, &mut out_file)?;
            }
        }
        Ok(pdf_path)
    })
    .await??;

    if let Some(pdf_path) = pdf_path {
        tracing::info!("file_path: {:?}", pdf_path);
        app.shell()
            .open(pdf_path.to_str().unwrap().to_string(), None)?;
    }

    Ok(())
}
