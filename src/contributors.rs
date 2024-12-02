use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Contributor {
    pub login: String,
    pub avatar_url: String,
}

pub async fn fetch_contributors(user: &str, repo: &str) -> Result<Vec<Contributor>, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{}/{}/contributors", user, repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Rust-Readme-Generator")
        .send()
        .await?
        .json::<Vec<Contributor>>()
        .await?;
    Ok(response)
}
