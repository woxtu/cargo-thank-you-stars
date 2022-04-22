use crate::errors::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
}

pub fn read(path: &Path) -> Result<Config> {
    Ok(json::from_str(&fs::read_to_string(&path)?)?)
}
