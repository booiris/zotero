use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Collection {
    pub key: String,
    pub version: i64,
    pub library: Library,
    pub links: Links,
    pub meta: Meta,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Library {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub name: String,
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: Option<LinkType>,
    pub alternate: Option<LinkType>,
    pub up: Option<LinkType>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LinkType {
    pub href: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Meta {
    #[serde(rename = "numCollections")]
    pub num_collections: i64,
    #[serde(rename = "numItems")]
    pub num_items: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(default)]
pub struct Data {
    pub key: String,
    pub version: i64,
    pub name: String,
    #[serde(rename = "parentCollection")]
    pub parent_collection: Option<String>,
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            key: String,
            version: i64,
            name: String,
            #[serde(rename = "parentCollection")]
            parent_collection: Value,
        }

        let helper = Helper::deserialize(deserializer)?;
        let parent_collection = match helper.parent_collection {
            Value::String(s) => Some(s),
            Value::Bool(false) => None,
            _ => None,
        };

        Ok(Data {
            key: helper.key,
            version: helper.version,
            name: helper.name,
            parent_collection,
        })
    }
}
