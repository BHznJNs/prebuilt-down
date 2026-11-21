# Features

English | [简体中文](Features.zh-CN.md)

[Back to README](../README.md)

## Archive Formats

Set `archive` in each platform entry to one of these values:

| `archive` | File format |
| --- | --- |
| `zip` | `.zip` |
| `tar-gz` | `.tar.gz` |
| `tar-xz` | `.tar.xz` |

When `archive` is omitted, the downloaded file is copied directly to `target` without extraction.

## Hash Verification

The optional platform-level `hash` table supports `algorithm = "sha256"` (SHA-256) and `algorithm = "sha512"` (SHA-512). Set `digest` to the expected hexadecimal hash of the downloaded file. Verification runs before extraction; a mismatch causes that download item to fail.

## Download Authorization

Set `authorization_headers` on each download item. The headers apply to that item's downloads on all platforms:

```toml
[node]
target = "bin/node/"

[node.authorization_headers]
Authorization = "Bearer ${GITHUB_TOKEN}"
```

Downloads send `User-Agent: prebuilt-down/<version>` by default. Set `User-Agent` in an item's `authorization_headers` to override the value for that item.

The equivalent inline TOML table uses `=` between keys and values:

```toml
[node]
target = "bin/node/"
authorization_headers = { Authorization = "Bearer ${GITHUB_TOKEN}" }
```

Keep the platform entries such as `[node.windows-x64]` from the [example configuration](../README.md#example-configuration). Header values support `${VARIABLE}` and `$VARIABLE` placeholders, plus `${VARIABLE:default}` for a fallback. An unset variable without a fallback causes that download to fail with an error. Variable values are inserted without recursively expanding them.

Variable names use ASCII letters, digits, and underscores. Escape a literal `$` as `\$` (write `\\$` inside double-quoted TOML strings), for example `Authorization = 'Bearer literal-\$token'`. See the [template syntax](https://docs.rs/subst/0.3.8/subst/struct.Template.html) for details.

Variables are read from the process environment and an optional dotenv file:

- By default, only `.env` in the current working directory is read.
- With `--env-file PATH`, only the specified file is read. Relative paths are resolved from the current working directory; the default `.env` is not loaded.
- If the selected file does not exist, one warning is logged and the process environment is used. Other read or parse errors still cause a failure.
- Values from the selected file override process environment variables with the same name. Variables absent from the file fall back to the process environment.

```dotenv
# .env or secrets/download.env
GITHUB_TOKEN=your-token
```

```sh
prebuilt-down --env-file secrets/download.env
```
