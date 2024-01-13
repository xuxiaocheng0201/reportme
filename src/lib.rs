use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use anyhow::{anyhow, Result};

pub extern crate ureq;

fn get_field<T: FromStr>(map: &HashMap<String, String>, key: &str) -> Result<T> {
    Ok(map.get(key).ok_or_else(|| anyhow!("Missing field: {}", key))?.parse()?)
}

pub fn report(url: &str, timeout: Duration, name: &str, version: &str) -> Result<bool> {
    let res = ureq::post(url).timeout(timeout)
        .send_json(ureq::json!({"crate": name, "ver": version}))?
        .into_json::<HashMap<String, String>>()?;
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
