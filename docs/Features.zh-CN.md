# 功能说明

[English](Features.md) | 简体中文

[返回 README](../README.zh-CN.md)

## 压缩包类型

在各平台配置中通过 `archive` 指定压缩包类型：

| `archive` | 文件格式 |
| --- | --- |
| `zip` | `.zip` |
| `tar-gz` | `.tar.gz` |
| `tar-xz` | `.tar.xz` |

不配置 `archive` 时，下载的文件会直接复制到 `target`，不进行解压。

## Hash 校验

各平台配置中可选的 `hash` 表支持 `algorithm = "sha256"`（SHA-256）和 `algorithm = "sha512"`（SHA-512）。`digest` 填写下载文件的预期十六进制哈希值。校验在解压前进行，校验不匹配时该下载项会报错。

## 下载认证

在每个下载项中配置 `authorization_headers`，请求头会用于该项在所有平台上的下载：

```toml
[node]
target = "bin/node/"

[node.authorization_headers]
Authorization = "Bearer ${GITHUB_TOKEN}"
```

下载请求默认发送 `User-Agent: prebuilt-down/<版本号>`。在下载项的 `authorization_headers` 中设置 `User-Agent`，即可覆盖该项的默认值。

也可以使用等价的 TOML 行内表，键值之间使用 `=`：

```toml
[node]
target = "bin/node/"
authorization_headers = { Authorization = "Bearer ${GITHUB_TOKEN}" }
```

仍需保留[配置文件示例](../README.zh-CN.md#配置文件示例)中的 `[node.windows-x64]` 等平台配置。请求头的值支持 `${变量名}`、`$变量名` 占位符，也可以通过 `${变量名:默认值}` 设置回退值；变量未定义且没有默认值时，该下载项会报错。读取到的变量值不会被递归展开。

变量名由 ASCII 字母、数字和下划线组成。普通文本中的 `$` 需写为 `\$`（在 TOML 双引号字符串中写为 `\\$`），例如 `Authorization = 'Bearer literal-\$token'`。完整规则见[模板语法说明](https://docs.rs/subst/0.3.8/subst/struct.Template.html)。

变量从进程环境变量和可选的 dotenv 文件中读取：

- 默认只读取当前工作目录中的 `.env`。
- 指定 `--env-file 路径` 后，只读取指定文件，不再读取默认的 `.env`；相对路径基于当前工作目录解析。
- 所选文件不存在时，打印一条 warning 并继续使用系统环境变量；其他读取错误或文件解析错误仍会报错。
- 所读文件中的值覆盖系统同名环境变量；文件中未定义的变量仍从系统环境变量中读取。

```dotenv
# .env 或 secrets/download.env
GITHUB_TOKEN=your-token
```

```sh
prebuilt-down --env-file secrets/download.env
```
