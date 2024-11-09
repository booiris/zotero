use serde::Deserialize;

use crate::model::auth::Secret;

#[derive(Debug, Deserialize)]
pub(super) struct KeyResp {
    // pub key: String,
    #[serde(rename = "userID")]
    pub user_id: i64,
    #[serde(rename = "username")]
    pub user_name: String,
    pub access: Access,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Access {
    pub user: User,
}

#[derive(Default, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct User {
    pub library: bool,
    pub files: bool,
    pub notes: bool,
    pub write: bool,
}

pub type ZoteroApiKey = Secret;
