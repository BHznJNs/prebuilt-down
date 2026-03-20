# Prebuilt Down

A CLI tool that helps automatically resolve the external prebuilt binary dependencies in your codebases.

## Usage

This will automatically read the `prebuilt-down.toml` file in the cwd and download prebuilts:
```
prebuilt-down
```

Specify the config file:
```
prebuilt-down --config config/binaries.toml
prebuilt-down -c config/binaries.toml
```

Specify the platform (prebuilt-down downloads the binary for the current platform by default):
```
prebuilt-down --platform windows-x64
prebuilt-down -p windows-x64
```

## Example Configuration

```
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
