use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use anyhow::{anyhow, Result};
use reqwest::{ClientBuilder, IntoUrl};

fn get_field<T: FromStr>(map: &HashMap<String, String>, key: &str) -> Result<T> {
    Ok(map.get(key).ok_or_else(|| anyhow!("Missing field: {}", key))?.parse()?)
}

pub async fn report<U: IntoUrl>(timeout: Duration, url: U, name: &str, version: &str) -> Result<bool> {
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let mut body = HashMap::new();
    body.insert("crate", name);
    body.insert("ver", version);
    let res = client.execute(client.post(url).json(&body).build()?).await?
        .json::<HashMap<String, String>>().await?;
    if get_field::<bool>(&res, "ok")? {
        Ok(get_field::<bool>(&res, "success")?)
    } else {
        let err = get_field::<String>(&res, "err")?;
        Err(if get_field::<bool>(&res, "internal")? {
            anyhow!("Internal error: {}", err)
        } else {
            anyhow!("Error: {}", err)
        })
    }
}
