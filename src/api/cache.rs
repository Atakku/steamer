// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use super::details::AppDetails;
use crate::Res;
use directories::ProjectDirs;
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[cfg(debug_assertions)]
const NAME: &str = "cache.json";

#[cfg(not(debug_assertions))]
const NAME: &str = "cache.bin";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Cache(pub HashMap<u64, AppDetails>);

impl Cache {
  pub fn root() -> Res<PathBuf> {
    let Some(dirs) = ProjectDirs::from("dev", "Atakku", "Steamer") else {
      return Err("Failed to get cache dir".into());
    };
    Ok(dirs.cache_dir().to_path_buf())
  }

  pub fn load() -> Res<Self> {
    let path = Self::root()?.join(NAME);

    if path.exists() {
      info!("Loading existing cache from {:?}", path);
      match fs::read(path) {
        Ok(data) => {
          #[cfg(debug_assertions)]
          let de = serde_json::from_slice(&data);
          #[cfg(not(debug_assertions))]
          let de = bincode::deserialize(&data);

          match de {
            Ok(config) => return Ok(config),
            Err(err) => log::warn!("Failed to deserialize cache: {err}"),
          }
        }
        Err(err) => log::warn!("Failed to read cache: {err}"),
      }
    }
    Ok(Self::default())
  }

  pub fn save(&self) -> Res<()> {
    let root = Self::root()?;
    if !root.exists() {
      fs::create_dir_all(&root)?;
    }
    #[cfg(debug_assertions)]
    let se = serde_json::to_vec(self);
    #[cfg(not(debug_assertions))]
    let se = bincode::serialize(self);
    Ok(fs::write(root.join(NAME), se?)?)
  }
}
