use crate::errors::*;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
}

impl Package {
    pub fn crate_id(&self) -> String {
        self.name.split_whitespace().next().unwrap_or("").to_owned()
    }

    pub fn is_registry(&self) -> bool {
        match &self.source {
            None => false,
            Some(source) => {
                source.ends_with("registry+https://github.com/rust-lang/crates.io-index")
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Lockfile {
    #[serde(rename = "package")]
    pub packages: Vec<Package>,
}

pub fn read(path: &Path) -> Result<Lockfile> {
    Ok(toml::from_str(&fs::read_to_string(&path)?)?)
}
