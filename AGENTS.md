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

[node.windows-x64.hash]
algorithm = "sha256"
digest = "bb1518746cab560370fb402c3fe17ddd527141a2a341043d5e7db5d39b98d4be"

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

## Rust 代码规范

### 何时保留命名空间？

1. 函数
这是最重要的惯例，引入函数时保留其父模块，避免混淆函数来源：

```
// ❌ 不推荐：直接引入函数，来源不明
use std::fs::read_to_string;
read_to_string("file.txt")?;

// ✅ 推荐：保留模块，来源清晰
use std::fs;
fs::read_to_string("file.txt")?;
```

2. 同名冲突 - 用路径区分

```rust
// 两个 Result 类型同时用，必须保留路径
use std::fmt;
use std::io;

fn foo() -> fmt::Result { ... }
fn bar() -> io::Result<()> { ... }
```

3. 路径本身有语义价值

```
rust// ✅ 保留路径，语义更丰富
std::mem::drop(val);       // "内存操作的 drop"，一目了然
std::thread::spawn(|| {}); // 明确是线程操作

// vs 引入后反而显得突兀
use std::mem::drop; // drop 是内置名，这样写还会 shadow 掉它！
```
