# ReportMe

[![Crate](https://img.shields.io/crates/v/reportme.svg)](https://crates.io/crates/reportme)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/blob/master/LICENSE)

**Read this in other languages: [English](README.md), [简体中文](README_zh.md).**

# Description

A Rust metrics crate used in build environment. Used to know the actual usage of your crates.


# Usage

Add this to your `Cargo.toml`:

```toml
[build.dependencies]
reportme = "~0.2"
```

# Example

Use [Cloudflare](Cloudflare_zh.md) to build server. **IT IS FREE!**

```rust,no_run
use std::time::Duration;
use reportme::report_build;

fn main() {
    report_build("https://<YourUrl>.pages.dev/metrics",
                 Duration::from_secs(10),
                 env!("CARGO_PKG_NAME"),
                 env!("CARGO_PKG_VERSION"));
}
```
