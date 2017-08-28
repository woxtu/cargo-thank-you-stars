use std::io;

use json;
use reqwest;
use toml;

error_chain! {
  foreign_links {
    Io(io::Error);
    Json(json::Error);
    Reqwest(reqwest::Error);
    Toml(toml::de::Error);
  }
}
