# 统计报告

[![Crate](https://img.shields.io/crates/v/reportme.svg)](https://crates.io/crates/reportme)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/reportme)](https://github.com/xuxiaocheng0201/reportme/blob/master/LICENSE)

**其他语言版本：[English](README.md)，[简体中文](README_zh.md)。**

# 描述

一个用于构建环境的统计crate，用来了解crate的实际使用情况。
用来区分crate.io上的下载是人还是镜像服务。


# 用法

将以下内容添加到你的`Cargo.toml`：

```toml
[build.dependencies]
reportme = "~0.2"
```

# 示例

请参阅[Cloudflare](Cloudflare_zh.md) 来构建服务器（完全免费）！

```rust
use std::time::Duration;
use reportme::report_build;

fn main() {
    report_build("https://<你的网址>.pages.dev/metrics",
                 Duration::from_secs(10),
                 env!("CARGO_PKG_NAME"),
                 env!("CARGO_PKG_VERSION"));
}
```
