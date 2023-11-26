use reqwest;
use serde_yaml;
use serde::{Serialize, Deserialize};

async fn build_res (res: &str) -> serde_yaml::Result<serde_yaml::Value> {
    let res = serde_yaml::from_str(res);
    res
}

async fn request_seele (url: &str) -> Result<String, reqwest::Error>{
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}