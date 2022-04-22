use crate::errors::*;
use serde::Deserialize;
use std::path::Path;
use std::{fs, result};

#[derive(Debug)]
pub struct Dependency {
    pub raw: String,
}

impl<'de> serde::Deserialize<'de> for Dependency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> result::Result<Self, D::Error> {
        String::deserialize(deserializer).map(|s| Dependency { raw: s })
    }
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    #[serde(default = "Vec::new")]
    pub dependencies: Vec<Dependency>,
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
