use async_trait::async_trait;
use reqwest::Client;
use crate::fortnite::error::FortniteError;
use crate::fortnite::structs::ShopJson;
use crate::get_io_key;
use super::{get_headers, ShopApi};

pub struct FortniteIo {
    client: Client
}
impl FortniteIo {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }
}

#[async_trait]
impl ShopApi for FortniteIo {
    async fn get_upcoming(&self) -> Result<ShopJson, FortniteError> {
        let res = self.client
            .get("https://fortniteapi.io/v2/items/upcoming?lang=en")
            .header("Authorization", get_io_key().await)
            .send()
            .await?;
        
        let json: ShopJson = serde_json::from_str(&res.text().await?)?;
        return Ok(json);
    }
    
    async fn get_daily(&self) -> Result<ShopJson, FortniteError> {
        let res = self.client
            .get("https://fortniteapi.io/v2/shop?lang=en")
            .header("Authorization", get_io_key().await)
            .headers(get_headers())
            .send()
            .await?;

        let json: ShopJson = serde_json::from_str(&res.text().await?)?;
        return Ok(json);
    }
}
