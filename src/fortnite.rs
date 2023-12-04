use std::collections::HashMap;
use async_trait::async_trait;
use reqwest::header::{ACCEPT, CONNECTION, HeaderMap, HeaderValue, USER_AGENT};

mod api_com; use api_com::*;
mod api_io; use api_io::*;
mod error; pub use error::*;
mod structs; pub use structs::*;
mod skins; pub use skins::*;

pub use structs::*;

#[async_trait]
pub trait ShopApi {
    async fn get_upcoming(&self) -> Result<ShopJson, FortniteError>;
    async fn get_daily(&self) -> Result<ShopJson, FortniteError>;
}

pub struct Shop {
    io: FortniteIo,
    com: FortniteCom
}
impl Shop {
    pub fn new() -> Self {
        Self {
            io: FortniteIo::new(),
            com: FortniteCom::new()
        }
    }
}

impl Shop {
    pub(crate) async fn get_upcoming(&self) -> Result<Vec<ShopJson>, FortniteError> {
        log::info!("Getting the upcoming shop..");
        let io = self.io.get_upcoming().await?;
        let com = self.com.get_upcoming().await?;
        return Ok(vec![io, com]);
    }
    
    pub(crate) async fn get_daily(&self) -> Result<Vec<ShopJson>, FortniteError> {
        log::info!("Getting the daily shop..");
        let io = self.io.get_daily().await?;
        let com = self.com.get_daily().await?;
        return Ok(vec![io, com]);
    }
}

pub fn get_headers() -> HeaderMap {
    let mut map = HashMap::new();
    map.insert(USER_AGENT, "MeWant/RavenTeamLeader");
    map.insert(ACCEPT, "*/*");
    map.insert(CONNECTION, "keep-alive");

    let mut header_map = HeaderMap::new();
    for (name, value) in map {
        let Ok(value) = HeaderValue::from_str(value) else { continue };
        header_map.insert(name, value);
    }
    header_map
}
