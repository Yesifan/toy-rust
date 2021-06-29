use reqwest::{get};

pub async fn oauth() -> Result<String, Box<dyn std::error::Error>> {
  let body = get("https://www.rust-lang.org")
  .await?.text().await?;
  Ok(body)
}