#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::time::Duration;
use anyhow::{anyhow, Result};
use ureq::serde_json::Value;

#[inline]
fn get_bool(json: &Value, key: &str) -> Result<bool> {
    json[key].as_bool().ok_or_else(|| anyhow!("Developer error: Require bool field: {}", key))
}

#[inline]
fn get_str<'a>(json: &'a Value, key: &str) -> Result<&'a str> {
    json[key].as_str().ok_or_else(|| anyhow!("Developer error: Require string field: {}", key))
}

/// Report your crate to the server.
///
/// This method returns an error if failed.
pub fn report(url: &str, timeout: Duration, name: &str, version: &str) -> Result<()> {
    let res = ureq::post(url).timeout(timeout)
        .send_json(ureq::json!({"crate": name, "ver": version}))?
        .into_json::<Value>()?;
    if get_bool(&res, "ok")? {
        if get_bool(&res, "res")? {
            Ok(())
        } else {
            Err(anyhow!("Internal error: Failed to report."))
        }
    } else {
        let err = get_str(&res, "err")?;
        Err(if get_bool(&res, "internal")? {
            anyhow!("Internal error: {}", err)
        } else {
            anyhow!("Developer error: {}", err)
        })
    }
}

/// Report your crate to the server.
///
/// This is a shortcut to be used in `build.rs`.
///
/// # Example
/// ```rust,no_run
/// use std::time::Duration;
/// use reportme::report_build;
///
/// fn main() {
///     report_build("https://<YourUrl>.pages.dev/metrics",
///                  Duration::from_secs(10),
///                  env!("CARGO_PKG_NAME"),
///                  env!("CARGO_PKG_VERSION"));
/// }
/// ```
pub fn report_build(url: &str, timeout: Duration, name: &str, version: &str) {
    println!("cargo:rerun-if-env-changed=DONT_REPORT_ME");
    if option_env!("DONT_REPORT_ME").is_none() {
        let res = report(url, timeout, name, version);
        if let Err(e) = res {
            println!("cargo:warning=ReportMe: {}", e);
        }
    }
}
