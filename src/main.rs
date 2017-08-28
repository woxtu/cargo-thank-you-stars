#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod errors;
mod lockfile;

use std::env;

use errors::*;

quick_main!(|| -> Result<()> {
  let path = env::current_dir()?.join("Cargo.lock");
  if !path.exists() {
    bail!("Run `cargo install` before")
  }
  let lockfile = lockfile::read(&path)
    .chain_err(|| "Cannot read Cargo.lock")?;

  for dependency in lockfile.root.dependencies {
    if dependency.is_registry() {
      println!("{}", dependency.crate_id())
    }
  }

  Ok(())
});
