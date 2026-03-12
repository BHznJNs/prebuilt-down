---
name: "prebuilt-down"
description: "一个用于在 CI/CD 中自动获取项目中依赖的外部二进制文件的 CLI 工具"
author: "BHznJNs"
authorUrl: "https://github.com/BHznJNs/prebuilt-down"
tags: ["ci-cd", "binary", "cli"]
---

这是一个用于在软件项目中，基于配置文件自动下载依赖的外部二进制文件的 CLI 工具。

## CLI 调用方法

直接基于当前目录的 prebuilt-down.toml 配置文件下载文件
```
prebuilt-down
```

指定配置文件路径
```
prebuilt-down --config config/binaries.toml
prebuilt-down -c config/binaries.toml
```

指定下载的二进制文件的平台（不指定则默认下载当前平台的二进制文件）
```
prebuilt-down --platform windows-x64
prebuilt-down -p windows-x64
```

## 配置文件示例

```toml
[node]
target = "bin/node/" # extracted path

[node.windows-x64]
url = "https://nodejs.org/dist/v25.8.1/node-v25.8.1-win-x64.zip"
root = "node-v25.8.1-win-x64/"
archive = "zip"
hash = {
    algorithm = "sha256",
    digest = "bb1518746cab560370fb402c3fe17ddd527141a2a341043d5e7db5d39b98d4be"
}

[node.linux-x64]
url = "https://nodejs.org/dist/v25.8.1/node-v25.8.1-linux-x64.tar.xz"
root = "node-v25.8.1-linux-x64/"
archive = "tar-xz"

[ripgrep]
target = "bin/ripgrep/"

[ripgrep.windows-x64]
url = "https://github.com/BurntSushi/ripgrep/releases/download/15.1.0/ripgrep-15.1.0-x86_64-pc-windows-msvc.zip"
root = "ripgrep-15.1.0-x86_64-pc-windows-msvc/"
archive = "zip"
```
