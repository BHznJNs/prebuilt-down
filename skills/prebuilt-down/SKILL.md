---
name: prebuilt-down
description: >
  Help users write and troubleshoot `prebuilt-down.toml` configuration files for the prebuilt-down CLI tool.
  Use this skill whenever the user needs to configure prebuilt binary downloads, set up CI/CD binary dependencies,
  write or edit a prebuilt-down.toml file, fix configuration errors (hash mismatch, archive extraction failure,
  platform not found), or understand prebuilt-down's behavior (lock files, caching, supported platforms/archives/hashes).
  Also use when the user mentions "prebuilt-down", "prebuilt binary", "binary dependency", or asks how to
  automate downloading external tools like node, ripgrep, or similar in their project.
---

# prebuilt-down

prebuilt-down is a CLI tool that automates downloading, verifying, and extracting prebuilt external binaries
based on a declarative TOML configuration file.

## CLI Reference

```
prebuilt-down [OPTIONS]
```

| Flag | Short | Description |
|------|-------|-------------|
| `--config` | `-c` | Path to config file (default: `prebuilt-down.toml`) |
| `--platform` | `-p` | Target platform; if omitted, auto-detected from OS/arch |
| `--verbose` | `-v` | Increase log verbosity (repeatable: `-v`, `-vv`, `-vvv`) |
| `--force` | `-f` | Skip lock-file check, force re-download and re-extract |

## Configuration File Format

The config file is TOML. Each top-level section defines one binary dependency.

### Full Schema

```toml
[binary-name]
target = "path/to/extract/"        # directory where files land after extraction

[binary-name.<platform>]
url = "https://..."                # download URL
root = "archive-root-dir/"         # prefix inside the archive to strip (use "." for flat archives)
archive = "zip"                    # optional: "zip", "tar-gz", or "tar-xz"
hash = { algorithm = "sha256", digest = "abcdef..." }  # optional integrity check
```

- If `archive` is omitted, the downloaded file is copied directly into `target` (no extraction).
- If the archive has no root directory (flat structure), set `root = "."`.
- `hash` is optional but recommended for CI/CD reproducibility. Supported algorithms: `sha256`, `sha512`.

### Supported Platforms

| Platform string | OS | Architecture |
|----------------|-----|-------------|
| `windows-x64` | Windows | x86_64 |
| `windows-arm64` | Windows | aarch64 |
| `linux-x64` | Linux | x86_64 |
| `linux-arm64` | Linux | aarch64 |
| `darwin-x64` | macOS | x86_64 |
| `darwin-arm64` | macOS | aarch64 |

### Supported Archive Types

| Value | Format |
|-------|--------|
| `zip` | ZIP |
| `tar-gz` | tar.gz / tgz |
| `tar-xz` | tar.xz |

### Hash Configuration

Two forms are equivalent (TOML inline table vs. section):

```toml
# Inline
hash = { algorithm = "sha256", digest = "bb1518746cab560370fb402c3fe17ddd527141a2a341043d5e7db5d39b98d4be" }

# Section
[binary-name.windows-x64.hash]
algorithm = "sha256"
digest = "bb1518746cab560370fb402c3fe17ddd527141a2a341043d5e7db5d39b98d4be"
```

## How It Works (Internal Pipeline)

For each binary defined in the config, prebuilt-down executes this pipeline:

1. **Load config** → read and parse the TOML file.
2. **Lock check** → if the URL, hash, and `target` directory contents match the lock file, skip.
3. **Download** → fetch the file into `.prebuilt-down/` cache directory.
4. **Hash verify** → if `hash` is configured, compute the digest and compare (skips on mismatch).
5. **Extract / Copy** → if `archive` is set, strip the `root` prefix and extract into `target`; otherwise copy the file directly into `target`.
6. **Record lock** → write an entry into `.prebuilt-down/prebuilt-down.lock` (JSON).

### Cache & Lock File

- Cache directory: `.prebuilt-down/` (auto-created, gitignored)
- Lock file: `.prebuilt-down/prebuilt-down.lock` (JSON)
- The lock file records the URL, hash digest, and extracted file list per platform for each binary. This prevents redundant downloads when nothing changed.
- Use `--force` to bypass the lock check.

## Examples

### Single binary (node) with hash verification

```toml
[node]
target = "bin/node/"

[node.windows-x64]
url = "https://nodejs.org/dist/v25.8.1/node-v25.8.1-win-x64.zip"
root = "node-v25.8.1-win-x64/"
archive = "zip"
hash = { algorithm = "sha256", digest = "bb1518746cab560370fb402c3fe17ddd527141a2a341043d5e7db5d39b98d4be" }

[node.linux-x64]
url = "https://nodejs.org/dist/v25.8.1/node-v25.8.1-linux-x64.tar.xz"
root = "node-v25.8.1-linux-x64/"
archive = "tar-xz"
hash = { algorithm = "sha256", digest = "abc123..." }
```

### Multiple binaries in one file

```toml
[node]
target = "bin/node/"

[node.windows-x64]
url = "https://nodejs.org/dist/v25.8.1/node-v25.8.1-win-x64.zip"
root = "node-v25.8.1-win-x64/"
archive = "zip"

[ripgrep]
target = "bin/ripgrep/"

[ripgrep.windows-x64]
url = "https://github.com/BurntSushi/ripgrep/releases/download/15.1.0/ripgrep-15.1.0-x86_64-pc-windows-msvc.zip"
root = "ripgrep-15.1.0-x86_64-pc-windows-msvc/"
archive = "zip"

[ripgrep.linux-x64]
url = "https://github.com/BurntSushi/ripgrep/releases/download/15.1.0/ripgrep-15.1.0-x86_64-unknown-linux-musl.tar.gz"
root = "ripgrep-15.1.0-x86_64-unknown-linux-musl/"
archive = "tar-gz"
```

### Single executable (no archive)

For a binary that is distributed as a standalone file (no archive):

```toml
[my-tool]
target = "bin/"

[my-tool.linux-x64]
url = "https://github.com/example/my-tool/releases/download/v1.0.0/my-tool-linux-amd64"

[my-tool.windows-x64]
url = "https://github.com/example/my-tool/releases/download/v1.0.0/my-tool-windows-amd64.exe"
```

When no `archive` is specified, the downloaded file is copied as-is into `target`.

## Troubleshooting

### "platform X not configured for Y, skipping"

The config file has no section for the target platform. Add a `[binary-name.<platform>]` section for the needed platform. Use `--platform` to explicitly target a different platform.

### "Hash verification failed"

The downloaded file's hash does not match the configured `digest`. Common causes:
- The remote file was updated (check the release page for new hashes)
- The `digest` value has extra whitespace or wrong case (digests are compared case-insensitively after trimming)
- Wrong algorithm (sha256 vs sha512)

### "Root path not found in archive"

The `root` value does not match the actual directory structure inside the archive. Inspect the archive manually:
```bash
# For zip
unzip -l downloaded.zip | head -20

# For tar.gz
tar -tzf downloaded.tar.gz | head -20

# For tar.xz
tar -tJf downloaded.tar.xz | head -20
```
Set `root` to the common prefix directory, or `"."` if files are at the archive root.

### Config file not found

By default prebuilt-down looks for `prebuilt-down.toml` in the current working directory. Specify an explicit path with `-c`:
```
prebuilt-down -c path/to/config.toml
```

### Skipped due to unchanged lock

This is normal behavior — prebuilt-down skips binaries whose URL, hash, and `target` directory files haven't changed since the last successful run. Use `--force` to override.

### Determining the correct `root` value

The `root` is the top-level directory inside the archive that should be stripped. For example, if the archive contains:
```
ripgrep-15.1.0-x86_64-pc-windows-msvc/
  rg.exe
  LICENSE
```
Then `root = "ripgrep-15.1.0-x86_64-pc-windows-msvc/"` and after extraction `target/` will contain `rg.exe` and `LICENSE` directly.

### Unsupported platform auto-detection

If prebuilt-down panics with "Unsupported platform", the current OS/arch combination is not one of the six supported platforms. Use `--platform` to explicitly select a supported platform.
