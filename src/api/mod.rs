// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use self::{cache::Cache, details::AppDetails};
use crate::Res;
use reqwest::Client;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};

pub mod cache;
pub mod details;

pub struct Api {
  cache: Cache,
  client: Client,
}

impl Api {
  pub fn new() -> Res<Self> {
    Ok(Self {
      cache: Cache::load()?,
      client: Default::default(),
    })
  }

  pub async fn get(&mut self, id: u64) -> Res<&AppDetails> {
    if !self.cache.0.contains_key(&id) {
      tokio::time::sleep(Duration::from_secs(1)).await;
      let url = format!("https://store.steampowered.com/api/appdetails?appids={id}&l=en");
      let json = self.client.get(url).send().await?.text().await?;
      let Ok(mut res) = serde_json::from_str::<HashMap<u64, ApiResult>>(&json) else {
        return Err(format!("Failed to parse response: {json}").into());
      };
      let Some(details) = res.remove(&id) else {
        return Err("API didn't return app details".into());
      };
      if !details.success {
        return Err("API returned failure".into());
      }
      let Some(data) = details.data else {
        return Err("Data is missing".into());
      };
      self.cache.0.insert(id, data);
      self.cache.save()?;
    }
    self.cache.0.get(&id).ok_or("Failed to get from api".into())
  }
}

#[derive(Deserialize, Debug)]
struct ApiResult {
  pub success: bool,
  pub data: Option<AppDetails>,
}
