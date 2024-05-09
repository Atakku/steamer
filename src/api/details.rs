// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DefaultOnError;
use serde_with::DisplayFromStr;
use serde_with::PickFirst;

type Text = String;
type Num = i64;

nestruct::flatten! {
  #[serde_as]
  #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
  #[serde(rename_all = "snake_case")]
  AppDetails {
    #[serde(alias = "type")]
    app_type: {
      Game,
      Mod,
      Advertising
    },
    name: Text,
    #[serde(alias = "steam_appid")]
    id: Num,
    #[serde_as(as = "Option<PickFirst<(_, DisplayFromStr)>>")]
    required_age: Num?,
    is_free: bool,
    controller_support: Text?,
    #[serde(default)]
    dlc: [Num],
    detailed_description: Text,
    about_the_game: Text,
    short_description: Text,
    fullgame: {
      #[serde_as(as = "PickFirst<(_, DisplayFromStr)>")]
      appid: Num,
      name: Text,
    }?,
    supported_languages: Text?,
    header_image: Text,
    capsule_image: Text,
    capsule_imagev5: Text,
    website: Text?,
    #[serde_as(as = "DefaultOnError")]
    pc_requirements: Requirements?,
    #[serde_as(as = "DefaultOnError")]
    mac_requirements: Requirements?,
    #[serde_as(as = "DefaultOnError")]
    linux_requirements: Requirements?,
    legal_notice: Text?,
    #[serde(default)]
    developers: [Text],
    #[serde(default)]
    publishers: [Text],
    #[serde(default)]
    demos: [{
      appid: Num,
      description: String
    }],
    price_overview: {
      currency: Text,
      initial: Num,
      #[serde(alias = "final")]
      current: Num,
      discount_percent: Num,
      initial_formatted: Text,
      #[serde(alias = "final_formatted")]
      current_formatted: Text,
    }?,
    #[serde(default)]
    packages: [Num],
    #[serde(default)]
    package_groups: [{
      name: Text,
      title: Text,
      description: Text,
      selection_text: Text,
      save_text: Text,
      //display_type: Num,
      //is_recurring_subscription: bool,
      #[serde(default)]
      subs: [{
        packageid: Num,
        percent_savings_text: Text,
        percent_savings: Num,
        option_text: Text,
        option_description: Text,
        //can_get_free_license: Num,
        is_free_license: bool,
        price_in_cents_with_discount: Num,
      }],
    }],
    reviews: Text?,
    platforms: {
      windows: bool,
      mac: bool,
      linux: bool,
    },
    metacritic: {
      score: Num,
      url: Text,
    }?,
    #[serde(default)]
    categories: [{
      id: Num,
      description: Text,
    }],
    #[serde(default)]
    genres: [{
      id: Text,
      description: Text,
    }],
    #[serde(default)]
    screenshots: [{
      id: Num,
      path_thumbnail: Text,
      path_full: Text,
    }],
    #[serde(default)]
    movies: [{
      id: Num,
      name: Text,
      //thumbnail: redundant, use "https://cdn.akamai.steamstatic.com/steam/apps/{id}/movie.293x165.jpg
      webm: {
        #[serde(alias = "480")]
        low: String,
        max: String,
      },
      mp4: {
        #[serde(alias = "480")]
        low: String,
        max: String,
      },
      highlight: bool,
    }],
    recommendations: {
      total: Num
    }?,
    achievements: {
      total: Num,
      #[serde(default)]
      highlighted: [{
        name: Text,
        path: Text
      }]
    }?,
    release_date: {
      coming_soon: bool,
      date: Text
    },
    support_info: {
      url: Text,
      email: Text,
    },
    //background: Text: redundant, use "https://cdn.akamai.steamstatic.com/steam/apps/{id}/page_bg_generated_v6b.jpg"
    //background_raw: Text: redundant, use "https://cdn.akamai.steamstatic.com/steam/apps/{id}/page_bg_generated.jpg"
    //content_descriptors
    //ratings
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Requirements {
  pub minimum: Option<Text>,
  pub recommended: Option<Text>,
}
