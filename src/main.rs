extern crate serde_json as json;
use error_chain::quick_main;

mod config;
mod crates_io;
mod errors;
mod github;
mod lockfile;

use std::env;

use crates_io::Repository;
use errors::*;

quick_main!(|| -> Result<()> {
    let home_dir = dirs::home_dir().expect("Cannot get home directory");

    let config = config::read(&home_dir.join(".thank-you-stars.json"))
        .chain_err(|| "Save your configuration as `.thank-you-stars.json`")?;

    let lockfile = lockfile::read(&env::current_dir()?.join("Cargo.lock"))
        .chain_err(|| "Run `cargo install` before")?;

    let mut starred: Vec<String> = vec![];

    for package in lockfile.packages {
        if !starred.contains(&package.name) && package.is_registry() {
            starred.append(&mut vec![package.name.to_owned()]);

            let krate = crates_io::get(&package.crate_id())
                .chain_err(|| "Cannot get crate data from crates.io")?;

            if let Repository::GitHub(repository) = krate.repository() {
                match github::star(&config.token, &repository) {
                    Ok(_) => println!("Starred! https://github.com/{}", &repository),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }

    Ok(())
});
