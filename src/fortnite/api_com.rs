use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::fortnite::error::FortniteError;
use super::{get_headers, ShopApi};
use crate::fortnite::structs::ShopJson;

pub struct FortniteCom {
    client: Client
}
impl FortniteCom {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
}

#[async_trait]
impl ShopApi for FortniteCom {
    async fn get_upcoming(&self) -> Result<ShopJson, FortniteError> {
        return Ok(ShopJson::empty());
    }

    async fn get_daily(&self) -> Result<ShopJson, FortniteError> {
        // TODO: Add authorization for this, maybe?
        let res = self.client
            .get("https://fortnite-api.com/v2/shop/br/combined")
            .headers(get_headers())
            .send()
            .await?;

        let json: ShopJsonWrapper = serde_json::from_str(&res.text().await?)?;
        return Ok(json.data.featured);
    }
}

#[derive(Serialize, Deserialize)]
struct ShopJsonWrapper {
    data: ShopJsonWrapperIn
}
#[derive(Serialize, Deserialize)]
struct ShopJsonWrapperIn {
    featured: ShopJson
}
