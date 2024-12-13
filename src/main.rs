use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Deserialize)]
struct ProviderItem {
    model: Vec<Model>,
    key: String,
    base: String,
}

#[derive(Deserialize)]
struct Model {
    group: String,
    name: String,
}

#[derive(Deserialize)]
struct SecurityItem {
    key: String,
}

#[derive(Serialize)]
struct ProviderConfig {
    group: String,
    name: String,
    key: String,
    base: String,
}

#[derive(Serialize)]
struct Config {
    providers: Vec<ProviderConfig>,
    security: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = env::var("SUPABASE_URL")?;
    let key = env::var("SUPABASE_KEY")?;
    let cfg = env::var("OPENBRIDGE_CONFIG_FILENAME")?;

    let mut config = Config {
        providers: Vec::new(),
        security: Vec::new(),
    };

    let client = reqwest::Client::new();

    let providers: Vec<ProviderItem> = client
        .get(format!("{}/rest/v1/{}", url, "providers"))
        .header("apikey", &key)
        .send()
        .await?
        .json()
        .await?;

    for item in providers {
        for model in item.model {
            config.providers.push(ProviderConfig {
                group: model.group,
                name: model.name,
                key: item.key.clone(),
                base: item.base.clone(),
            });
        }
    }

    let securities: Vec<SecurityItem> = client
        .get(format!("{}/rest/v1/{}", url, "securities"))
        .header("apikey", &key)
        .send()
        .await?
        .json()
        .await?;

    for item in securities {
        config.security.push(item.key);
    }

    let mut file = File::create(cfg)?;
    file.write_all(serde_json::to_string(&config)?.as_bytes())?;

    Ok(())
}