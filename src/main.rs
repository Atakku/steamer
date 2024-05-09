// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use crate::api::Api;
use api::{cache::Cache, details::AppDetails};
use dioxus::{
  desktop::{Config, WindowBuilder},
  prelude::*,
};
use log::warn;
use rand::prelude::SliceRandom;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

pub mod api;

pub type Err = Box<dyn std::error::Error>;
pub type Res<T> = Result<T, Err>;

fn get_library_path() -> PathBuf {
  #[cfg(target_os = "windows")]
  let path = std::path::Path::new("C:\\Program Files (x86)\\Steam\\steamapps\\libraryfolders.vdf");
  #[cfg(target_os = "linux")]
  let path = directories::BaseDirs::new()
    .unwrap()
    .home_dir()
    .join(".steam/steam/steamapps/libraryfolders.vdf");

  path.to_path_buf()
}

const CDN: &str = "https://cdn.cloudflare.steamstatic.com/steam/apps";

fn main() {
  if !std::env::var("RUST_LOG").is_ok_and(|f| !f.is_empty()) {
    std::env::set_var("RUST_LOG", "warn");
  }
  pretty_env_logger::init();
  LaunchBuilder::desktop()
    .with_cfg(
      Config::new()
        .with_window(WindowBuilder::new().with_title("Steamer"))
        // TODO: Actually build tailwind
        .with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".into())
        .with_data_directory(Cache::root().unwrap()),
    )
    .launch(app);
}

#[derive(Deserialize, Debug)]
struct RawLibrary {
  pub apps: HashMap<u64, u64>,
}

fn app() -> Element {
  let data = use_signal(Vec::<AppDetails>::new);

  use_coroutine(|_: UnboundedReceiver<()>| {
    let mut data = data.to_owned();
    async move {
      let mut api = Api::new().unwrap();
      let raw: HashMap<u64, RawLibrary> = {
        let path = get_library_path();
        warn!("Reading library from {:?}", path);
        keyvalues_serde::from_str(&fs::read_to_string(path).unwrap()).unwrap()
      };
      let mut ids: Vec<u64> = raw
        .into_iter()
        .flat_map(|(_, l)| l.apps.into_keys().collect::<Vec<_>>())
        .collect();

      ids.sort();
      ids.dedup();

      ids.shuffle(&mut rand::thread_rng());

      for id in ids {
        match api.get(id).await {
          Ok(app) => {
            data.write().push(app.clone());
          }
          Result::Err(err) => warn!("Failed to get {id}: {err}"),
        }
      }
    }
  });

  let hover_id = use_signal(Option::<i64>::default);

  rsx! {
    body {
      class: "flex flex-wrap place-content-center gap-4 p-4 min-h-screen",
      background_color: "#1a1a1a",
      for app in data.read().clone() {
        draw_card {
          app,
          hover_id
        }
      }
    }
  }
}

#[derive(PartialEq, Clone, Props)]
struct ClickableProps {
  app: AppDetails,
  hover_id: Signal<Option<i64>>,
}

fn draw_card(mut s: ClickableProps) -> Element {
  rsx! {
    div {
      onmouseenter: move |_| *s.hover_id.write() = Some(s.app.id),
      onmouseleave: move |_| *s.hover_id.write() = None,
      class: "w-[460px] rounded-lg",
      background_color: "#212121",
      if *s.hover_id.read() == Some(s.app.id) && s.app.movies.len() > 0 {
        video {
          autoplay: true,
          r#loop: true,
          muted: false,
          class: "w-full h-[215px] rounded-t-lg bg-black",
          source {
            src: s.app.movies.get(0).unwrap().webm.low.clone(),
            r#type: "video/webm"
          },
          source {
            src: s.app.movies.get(0).unwrap().mp4.low.clone(),
            r#type: "video/mp4"
          }
        }
      } else {
        img {
          class: "w-full h-[215px] rounded-t-lg",
          src: "{CDN}/{s.app.id}/header.jpg",
        }
      }
      div {
        class: "p-4 text-white",
        dangerous_inner_html: s.app.short_description.clone()
      }
    }
  }
}
