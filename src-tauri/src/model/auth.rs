use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

pub type UserName = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Secret(String);

impl Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[secret: ********]")
    }
}

impl From<String> for Secret {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Secret {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<Secret> for String {
    fn from(p: Secret) -> Self {
        p.0
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Secret {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
