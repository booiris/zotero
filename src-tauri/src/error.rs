#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Raw(#[from] anyhow::Error),

    #[error("[zotero client]: {0}")]
    Zotero(#[from] crate::dal::zotero::error::ZoteroError),

    #[error("[tokio runtime]: {0}")]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("[webdav client]: {0}")]
    WebDav(#[from] crate::dal::webdav::error::WebDavError),

    #[error("[file io]: {0}")]
    FileIo(#[from] std::io::Error),

    #[error("[tokio file]: {0}")]
    DownloadFile(#[from] reqwest::Error),

    #[error("[zip]: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("[shell]: {0}")]
    Shell(#[from] tauri_plugin_shell::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let err = self.to_string();
        let err_type = match self {
            Error::Zotero(_) => "zotero",
            Error::FileIo(_) => "file_io",
            Error::DownloadFile(_) => "download_file",
            Error::Zip(_) => "zip",
            Error::Shell(_) => "shell",
            Error::Raw(_) => "raw",
            Error::TokioJoin(_) => "tokio_join",
            Error::WebDav(_) => "webdav",
        };
        tracing::error!("[{}] get error: {:?}", err_type, err);
        serializer.serialize_str(&err)
    }
}
