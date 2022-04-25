use crate::errors::*;
use reqwest::header::AUTHORIZATION;

pub fn check_if_starred(token: &str, repository: &str) -> Result<bool> {
    let result = reqwest::Client::new()
        .get(&format!(
            "https://api.github.com/user/starred/{}",
            repository
        ))
        .header(AUTHORIZATION, format!("token {}", token))
        .send()?;

    if result.status() == 204 {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn star(token: &str, repository: &str) -> Result<()> {
    reqwest::Client::new()
        .put(&format!(
            "https://api.github.com/user/starred/{}",
            repository
        ))
        .header(AUTHORIZATION, format!("token {}", token))
        .send()?;

    Ok(())
}
