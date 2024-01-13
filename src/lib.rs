use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use anyhow::{anyhow, Result};

fn get_field<T: FromStr>(map: &HashMap<String, String>, key: &str) -> Result<T> where T::Err: Error + Send + Sync + 'static {
    Ok(map.get(key).ok_or_else(|| anyhow!("Missing field: {}", key))?.parse()?)
}

pub fn report(url: &str, timeout: Duration, name: &str, version: &str) -> Result<()> {
    let res = ureq::post(url).timeout(timeout)
        .send_json(ureq::json!({"crate": name, "ver": version}))?
        .into_json::<HashMap<String, String>>()?;
    if get_field::<bool>(&res, "ok")? {
        if get_field::<bool>(&res, "success")? {
            Ok(())
        } else {
            Err(anyhow!("Failed to report."))
        }
    } else {
        let err = get_field::<String>(&res, "err")?;
        Err(if get_field::<bool>(&res, "internal")? {
            anyhow!("Internal error: {}", err)
        } else {
            anyhow!("Error: {}", err)
        })
    }
}
