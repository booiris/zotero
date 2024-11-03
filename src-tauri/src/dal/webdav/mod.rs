use std::sync::LazyLock;

use ahash::AHashMap;
use parking_lot::RwLock;
use reqwest_dav::{Auth, Client, ClientBuilder, Error};

pub struct WebDavAuth {
    pub username: String,
    pub password: String,
}

pub fn client(host: impl AsRef<str>, auth: Option<WebDavAuth>) -> Result<&'static Client, Error> {
    static CLIENTS: LazyLock<RwLock<AHashMap<String, &'static Client>>> =
        LazyLock::new(|| RwLock::new(AHashMap::new()));

    let mut key = host.as_ref().to_string();
    if let Some(auth) = auth.as_ref() {
        key += &format!(":{}:{}", auth.username, auth.password);
    }

    if let Some(c) = CLIENTS.read().get(&key) {
        return Ok(*c);
    }

    let mut client_builder = ClientBuilder::new().set_host(host.as_ref().to_string());

    if let Some(auth) = auth {
        client_builder = client_builder.set_auth(Auth::Basic(auth.username, auth.password));
    }

    let client = client_builder.build()?;
    let client_ref = Box::leak(Box::new(client));

    CLIENTS.write().insert(key, client_ref);
    Ok(client_ref)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy_macro::dotenv;
    use tracing::info;

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_no_auth_client() {
        let client = client(dotenv!("WEB_DAV_NO_AUTH_HOST"), None).unwrap();

        let resp = client.get("/").await.unwrap();
        assert!(resp.status().is_success());

        let text = resp.text().await;
        info!("{:?}", text);
        assert!(text.is_ok());
    }

    #[cfg(feature = "__local_test__")]
    #[tokio::test]
    async fn test_auth_client() {
        let client = client(
            dotenv!("WEB_DAV_AUTH_HOST"),
            Some(WebDavAuth {
                username: dotenv!("WEB_DAV_USERNAME").to_string(),
                password: dotenv!("WEB_DAV_PASSWORD").to_string(),
            }),
        )
        .unwrap();

        let resp = client.get("/").await.unwrap();
        assert!(resp.status().is_success());

        let text = resp.text().await;
        info!("{:?}", text);
        assert!(text.is_ok());
    }
}
