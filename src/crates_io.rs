use std::result;
use std::io::Read;

use json;
use serde::{Deserialize, Deserializer};
use reqwest;
use url::Url;

use errors::*;

#[derive(Clone, Debug)]
pub enum Repository {
  GitHub(String),
  Other(String),
  None,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
  pub id: String,
  #[serde(deserialize_with = "deserialize_url")]
  pub repository: Repository,
}

fn deserialize_url<'de, D: Deserializer<'de>>(deserializer: D) -> result::Result<Repository, D::Error> {
  Option::<String>::deserialize(deserializer)
    .map(|s| s.unwrap_or("".to_owned()))
    .map(|s| match Url::parse(&s) {
      Ok(ref url) if url.host_str() == Some("github.com") =>
        Repository::GitHub(url.path().trim_matches('/').to_owned()),
      Ok(_) => Repository::Other(s.to_owned()),
      Err(_) => Repository::None,
    })
}

#[derive(Deserialize, Debug)]
pub struct Crate {
  #[serde(rename = "crate")]
  pub meta: Metadata,
}

impl Crate {
  pub fn repository(&self) -> Repository {
    self.meta.repository.clone()
  }
}

pub fn get(crate_id: &str) -> Result<Crate> {
  let mut buffer = String::new();
  let _ = reqwest::Client::new()
    .get(&format!("https://crates.io/api/v1/crates/{}", crate_id))
    .send()?
    .read_to_string(&mut buffer)?;

  Ok(json::from_str(&buffer)?)
}
