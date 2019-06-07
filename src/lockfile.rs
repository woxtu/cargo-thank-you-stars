use std::{fs, result};
use std::path::Path;
use serde::Deserialize;
use crate::errors::*;

#[derive(Debug)]
pub struct Dependency {
  raw: String,
}

impl Dependency {
  pub fn crate_id(&self) -> String {
    self.raw.split_whitespace().next().unwrap_or("").to_owned()
  }

  pub fn is_registry(&self) -> bool {
    self.raw.ends_with("(registry+https://github.com/rust-lang/crates.io-index)")
  }
}

impl<'de> serde::Deserialize<'de> for Dependency {
  fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> result::Result<Self, D::Error> {
    String::deserialize(deserializer).map(|s| {
      Dependency { raw: s.to_owned() }
    })
  }
}

#[derive(Deserialize, Debug)]
pub struct Package {
  pub name: String,
  pub version: String,
  #[serde(default = "Vec::new")]
  pub dependencies: Vec<Dependency>,
}

#[derive(Deserialize, Debug)]
pub struct Lockfile {
  #[serde(rename = "package")]
  pub packages: Vec<Package>,
}

pub fn read(path: &Path) -> Result<Lockfile> {
  Ok(toml::from_str(&fs::read_to_string(&path)?)?)
}
