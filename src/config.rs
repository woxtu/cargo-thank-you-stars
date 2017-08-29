use std::fs::File;
use std::io::Read;
use std::path::Path;

use json;

use errors::*;

#[derive(Deserialize, Debug)]
pub struct Config {
  pub token: String,
}

pub fn read(path: &Path) -> Result<Config> {
  let mut buffer = String::new();
  let _ = File::open(&path)?.read_to_string(&mut buffer)?;
  Ok(json::from_str(&buffer)?)
}
