#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json as json;
extern crate toml;
extern crate url;

mod config;
mod crates_io;
mod errors;
mod github;
mod lockfile;

use std::env;

use crates_io::Repository;
use errors::*;

quick_main!(|| -> Result<()> {
  let home_dir = env::home_dir().expect("Cannot get home directory");

  let config = config::read(&home_dir.join(".thank-you-stars.json"))
    .chain_err(|| "Save your configuration as `.thank-you-stars.json`")?;

  let lockfile = lockfile::read(&env::current_dir()?.join("Cargo.lock"))
    .chain_err(|| "Run `cargo install` before")?;

  if let Some(package) = lockfile
    .packages
    .iter()
    .find(|package| package.name == env!("CARGO_PKG_NAME"))
  {
    for dependency in &package.dependencies {
      if dependency.is_registry() {
        let krate = crates_io::get(&dependency.crate_id())
          .chain_err(|| "Cannot get crate data from crates.io")?;

        if let Repository::GitHub(repository) = krate.repository() {
          match github::star(&config.token, &repository) {
            Ok(_) => println!("Starred! https://github.com/{}", &repository),
            Err(e) => eprintln!("{}", e.to_string()),
          }
        }
      }
    }
  }

  Ok(())
});
