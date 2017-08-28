use std::result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde;
use toml;

use errors::*;

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
pub struct Root {
  pub name: String,
  pub version: String,
  pub dependencies: Vec<Dependency>,
}

#[derive(Deserialize, Debug)]
pub struct Lockfile {
  pub root: Root,
}

pub fn read(path: &Path) -> Result<Lockfile> {
  let mut buffer = String::new();
  let _ = File::open(&path)?.read_to_string(&mut buffer)?;
  Ok(toml::from_str(&buffer)?)
}
