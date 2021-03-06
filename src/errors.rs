use std::io;
use error_chain::error_chain;

error_chain! {
  foreign_links {
    Io(io::Error);
    Json(json::Error);
    Reqwest(reqwest::Error);
    Toml(toml::de::Error);
  }
}
