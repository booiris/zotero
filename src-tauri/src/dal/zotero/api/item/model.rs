use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub key: String,
    pub version: i32,
    pub library: Library,
    pub links: Links,
    #[serde(default)]
    pub meta: Meta,
    pub data: ItemData,

    #[serde(skip_deserializing)]
    pub sub_items: Vec<Item>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    #[serde(rename = "type")]
    pub library_type: String,
    pub id: i64,
    pub name: String,
    pub links: HashMap<String, Link>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub href: String,
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_size: Option<i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: Link,
    pub alternate: Link,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enclosure: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Link>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_children: Option<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct ItemData {
    pub key: String,
    pub version: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_item: Option<String>,
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default)]
    pub creators: Vec<Creator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abstract_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_date: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub collections: Option<Vec<String>>,
    pub relations: HashMap<String, String>,
    pub date_added: String,
    pub date_modified: String,
    pub content_type: String,
    // 其他可选字段...
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub creator_type: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub tag: String,
    #[serde(rename = "type")]
    pub tag_type: Option<i32>,
}
