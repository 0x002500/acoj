use reqwest;
use serde::{Deserialize,Serialize};

pub struct GoJudge;

async fn request (url: &str) -> Result<String, reqwest::Error>{
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}