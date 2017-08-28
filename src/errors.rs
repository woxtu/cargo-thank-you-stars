use std::io;

use reqwest;
use toml;

error_chain! {
  foreign_links {
    Io(io::Error);
    Reqwest(reqwest::Error);
    Toml(toml::de::Error);
  }
}
