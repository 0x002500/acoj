use reqwest;

async fn request_seele (url: &str) -> Result<String, reqwest::Error>{
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}