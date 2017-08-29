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
mod lockfile;

use std::env;

use errors::*;

quick_main!(|| -> Result<()> {
  let path = env::home_dir()
    .expect("Cannot get home directory")
    .join(".thank-you-stars.json");
  if !path.exists() {
    bail!("Save your configuration as {:?}", path)
  }
  let config = config::read(&path)
    .chain_err(|| "Cannot read your configuration")?;

  println!("{:?}", config);

  let path = env::current_dir()?.join("Cargo.lock");
  if !path.exists() {
    bail!("Run `cargo install` before")
  }
  let lockfile = lockfile::read(&path)
    .chain_err(|| "Cannot read Cargo.lock")?;

  for dependency in lockfile.root.dependencies {
    if dependency.is_registry() {
      let krate = crates_io::get(&dependency.crate_id())
        .chain_err(|| "Cannot get crate data from crates.io")?;

      println!("{:?}", krate)
    }
  }

  Ok(())
});
