use reqwest::header::AUTHORIZATION;
use crate::errors::*;

pub fn star(token: &str, repository: &str) -> Result<()> {
  let _ = reqwest::Client::new()
    .put(&format!(
      "https://api.github.com/user/starred/{}",
      repository
    ))
    .header(AUTHORIZATION, format!("token {}", token))
    .send()?;

  Ok(())
}
