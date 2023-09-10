
use reqwest;
use serde_json;

pub async fn get_mod_files(mod_id: &str) -> Result<serde_json::Value, reqwest::Error> {
    let api_url = format!("https://api.nexusmods.com/v1/games/skyrimspecialedition/mods/{}/files.json", mod_id);
    let client = reqwest::Client::new();
    let res = client.get(&api_url)
        .header("apikey", "YOUR_API_KEY_HERE")
        .send()
        .await?
        .json()
        .await?;

    Ok(res)
}
