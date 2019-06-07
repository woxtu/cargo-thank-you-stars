use std::fs;
use std::path::Path;
use serde::Deserialize;
use crate::errors::*;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub token: String,
}

pub fn read(path: &Path) -> Result<Config> {
  Ok(json::from_str(&fs::read_to_string(&path)?)?)
}
