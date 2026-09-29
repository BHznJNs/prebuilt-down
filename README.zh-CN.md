# Prebuilt Down

[English](README.md) | 简体中文

一个用于在项目中自动获取预编译二进制依赖的命令行工具。

## 安装

通过 npm 安装（推荐）：
```sh
npm install prebuilt-down --save-dev
```

通过 [binstall](https://github.com/cargo-bins/cargo-binstall) 安装：
```sh
cargo binstall prebuilt-down
```

通过 Cargo 从源码安装：
```sh
cargo install prebuilt-down
```

在 GitHub Actions 中安装：
```yaml
- uses: cargo-bins/cargo-binstall@main
- run: cargo binstall prebuilt-down --no-confirm
```

## 使用方法

默认读取当前工作目录中的 `prebuilt-down.toml`，并下载当前平台对应的二进制文件：
```sh
prebuilt-down
```

指定配置文件：
```sh
prebuilt-down --config config/binaries.toml
prebuilt-down -c config/binaries.toml
```

指定平台（不指定时使用当前平台）：
```sh
prebuilt-down --platform windows-x64
prebuilt-down -p windows-x64
```

## 配置文件示例

以下配置将下载文件并解压到各自的 `target` 目录；`root` 表示压缩包中需要提取的根目录。`hash` 可用于校验下载文件。

```toml
[node]
target = "bin/node/" # 解压目标目录

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

## 链接

- [Dais](https://github.com/Dais-Project/Dais)
- [LinuxDO](https://linux.do)
