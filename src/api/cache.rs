use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use log::info;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use crate::Res;
use directories::ProjectDirs;

use super::details::AppDetails;

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
          match serde_json::from_reader(ZlibDecoder::new(data.as_slice())) {
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
    let mut buf = ZlibEncoder::new(Vec::new(), Compression::default());
    buf.write_all(&serde_json::to_vec(self)?)?;
    Ok(fs::write(root.join(NAME), buf.finish()?)?)
  }
}
