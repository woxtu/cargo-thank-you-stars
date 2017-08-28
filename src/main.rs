#[macro_use]
extern crate error_chain;
extern crate toml;

use std::{env, io};
use std::io::Read;
use std::fs::File;

error_chain! {
  foreign_links {
    Io(io::Error);
    Toml(toml::de::Error);
  }
}

quick_main!(|| -> Result<()> {
  let path = env::current_dir()?.join("Cargo.lock");
  let mut contents = String::new();
  let _ = File::open(&path)?.read_to_string(&mut contents)?;

  let lockfile = toml::from_str::<toml::Value>(&contents)?;
  let dependencies = lockfile["root"]["dependencies"].as_array().unwrap();
  let dependencies = dependencies.into_iter().flat_map(|v| v.as_str());
  for dependency in dependencies {
    if dependency.ends_with("(registry+https://github.com/rust-lang/crates.io-index)") {
      println!("{}", dependency.split_whitespace().next().unwrap());
    }
  }

  Ok(())
});
