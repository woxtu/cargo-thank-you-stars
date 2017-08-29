use reqwest;
use reqwest::header::Authorization;

use errors::*;

pub fn star(token: &str, repository: &str) -> Result<()> {
  let _ = reqwest::Client::new()?
    .put(&format!("https://api.github.com/user/starred/{}", repository))?
    .header(Authorization(format!("token {}", token)))
    .send()?;

  Ok(())
}
