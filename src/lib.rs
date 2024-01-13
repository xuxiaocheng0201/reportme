use std::time::Duration;
use anyhow::{anyhow, Result};
use ureq::serde_json::Value;

fn get_bool(json: &Value, key: &str) -> Result<bool> {
    Ok(json.get(key).ok_or_else(|| anyhow!("Missing field: {}", key))?
        .as_bool().ok_or_else(|| anyhow!("Except bool."))?)
}

fn get_str<'a>(json: &'a Value, key: &str) -> Result<&'a str> {
    Ok(json.get(key).ok_or_else(|| anyhow!("Missing field: {}", key))?
        .as_str().ok_or_else(|| anyhow!("Except str."))?)
}

pub fn report(url: &str, timeout: Duration, name: &str, version: &str) -> Result<()> {
    let res = ureq::post(url).timeout(timeout)
        .send_json(ureq::json!({"crate": name, "ver": version}))?
        .into_json::<Value>()?;
    if get_bool(&res, "ok")? {
        if get_bool(&res, "res")? {
            Ok(())
        } else {
            Err(anyhow!("Failed to report."))
        }
    } else {
        let err = get_str(&res, "err")?;
        Err(if get_bool(&res, "internal")? {
            anyhow!("Internal error: {}", err)
        } else {
            anyhow!("Error: {}", err)
        })
    }
}

pub fn report_build(url: &str, timeout: Duration, name: &str, version: &str) {
    println!("cargo:rerun-if-env-changed=DONT_REPORT_ME");
    if option_env!("DONT_REPORT_ME").is_none() {
        let res = report(url, timeout, name, version);
        if let Err(e) = res {
            println!("cargo:warning=ReportMe: {}", e);
        }
    }
}
