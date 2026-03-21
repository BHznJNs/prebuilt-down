use reqwest::blocking::Response;
use std::path::Path;

pub trait ResponseExt {
    fn resolve_filename(&self, fallback_name: &str) -> String;
}

impl ResponseExt for Response {
    fn resolve_filename(&self, fallback_name: &str) -> String {
        if let Some(filename) = parse_content_disposition(self) {
            let sanitized = sanitize_filename(&filename);
            if !sanitized.is_empty() {
                return sanitized;
            }
        }

        // get filename from url
        if let Some(filename) = parse_url_filename(self) {
            let sanitized = sanitize_filename(&filename);
            if !sanitized.is_empty() {
                return sanitized;
            }
        }

        // guess file extension from Content-Type
        let ext = guess_extension(self).unwrap_or_default();
        return format!("{fallback_name}{ext}");
    }
}

#[cfg(test)]
mod tests {
    use super::ResponseExt;
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use reqwest::blocking::Client;

    #[test]
    fn resolve_filename_prefers_content_disposition() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/download");
            then.status(200)
                .header("Content-Disposition", "attachment; filename=\"report.pdf\"")
                .body("ok");
        });

        let client = Client::new();
        let response = client
            .get(format!("{}/download", server.base_url()))
            .send()
            .unwrap();

        assert_eq!(response.resolve_filename("fallback"), "report.pdf");
    }

    #[test]
    fn resolve_filename_uses_url_when_no_header() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/files/archive.zip");
            then.status(200).body("ok");
        });

        let client = Client::new();
        let response = client
            .get(format!("{}/files/archive.zip", server.base_url()))
            .send()
            .unwrap();

        assert_eq!(response.resolve_filename("fallback"), "archive.zip");
    }

    #[test]
    fn resolve_filename_falls_back_to_content_type() {
        let server = MockServer::start();
        let _mock = server.mock(|when, then| {
            when.method(GET).path("/resource");
            then.status(200)
                .header("Content-Type", "application/json")
                .body("{}");
        });

        let client = Client::new();
        let response = client
            .get(format!("{}/resource", server.base_url()))
            .send()
            .unwrap();

        assert_eq!(response.resolve_filename("fallback"), "fallback.json");
    }
}

/// 解析 Content-Disposition 响应头
/// 支持 filename* (RFC 5987, UTF-8 编码) 和 filename 两种形式
fn parse_content_disposition(response: &Response) -> Option<String> {
    let header_value = response
        .headers()
        .get("Content-Disposition")?
        .to_str()
        .ok()?;

    // 优先尝试 filename*=UTF-8''<encoded> (RFC 5987)
    if let Some(filename) = extract_filename_star(header_value) {
        return Some(filename);
    }

    // 回退到普通 filename="..."
    extract_filename(header_value)
}

/// 解析 filename*=UTF-8''<percent-encoded> 格式 (RFC 5987)
fn extract_filename_star(header: &str) -> Option<String> {
    // 找到 filename*= 字段（大小写不敏感）
    let lower = header.to_lowercase();
    let key = "filename*=";
    let pos = lower.find(key)?;
    let value = header[pos + key.len()..].trim();

    // 去掉可能的分号结尾
    let value = value.split(';').next()?.trim();

    // 格式：charset'language'encoded_value，通常是 UTF-8''<encoded>
    let parts: Vec<&str> = value.splitn(3, '\'').collect();
    if parts.len() != 3 {
        return None;
    }

    let encoded = parts[2];
    percent_decode(encoded).ok()
}

/// 解析普通 filename="..." 或 filename=... 格式
fn extract_filename(header: &str) -> Option<String> {
    let lower = header.to_lowercase();
    let key = "filename=";
    let pos = lower.find(key)?;
    let value = header[pos + key.len()..].trim();

    // 去掉引号
    let value = if value.starts_with('"') {
        value
            .trim_start_matches('"')
            .split('"')
            .next()
            .unwrap_or("")
    } else {
        // 无引号时截到分号或末尾
        value.split(';').next().unwrap_or("").trim()
    };

    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

/// 从请求 URL 的最后一段路径中提取文件名
fn parse_url_filename(response: &Response) -> Option<String> {
    let url = response.url();
    let path = url.path();

    let last_segment = path.split('/').filter(|s| !s.is_empty()).last()?;

    // URL decode
    let decoded = percent_decode(last_segment).ok()?;

    // 过滤掉没有意义的路径段，比如纯数字 ID 或者无扩展名的单词
    if decoded.contains('.') {
        Some(decoded)
    } else {
        None
    }
}

/// 根据 Content-Type 猜测文件扩展名
fn guess_extension(response: &Response) -> Option<String> {
    let content_type = response
        .headers()
        .get("Content-Type")?
        .to_str()
        .ok()?
        .split(';')
        .next()?
        .trim();

    let ext = match content_type {
        "application/pdf" => ".pdf",
        "application/zip" => ".zip",
        "application/json" => ".json",
        "application/octet-stream" => ".bin",
        "text/plain" => ".txt",
        "text/html" => ".html",
        "text/csv" => ".csv",
        "image/jpeg" => ".jpg",
        "image/png" => ".png",
        "image/gif" => ".gif",
        "image/webp" => ".webp",
        "video/mp4" => ".mp4",
        "audio/mpeg" => ".mp3",
        _ => return None,
    };

    Some(ext.to_string())
}

fn sanitize_filename(name: &str) -> String {
    // 只取文件名部分（防止路径穿越攻击，如 ../../etc/passwd）
    let name = Path::new(name)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(name);

    // 替换 Windows/Unix 文件名非法字符
    let sanitized: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();

    // 去掉首尾空格和点（Windows 不允许文件名以点或空格结尾）
    sanitized.trim().trim_matches('.').to_string()
}

/// 简单的 percent decode（%XX → char）
fn percent_decode(input: &str) -> Result<String, std::num::ParseIntError> {
    let mut output = String::new();
    let mut bytes = input.bytes();

    while let Some(b) = bytes.next() {
        if b == b'%' {
            let hi = bytes.next().unwrap_or(0) as char;
            let lo = bytes.next().unwrap_or(0) as char;
            let hex = format!("{hi}{lo}");
            let byte = u8::from_str_radix(&hex, 16)?;
            output.push(byte as char);
        } else if b == b'+' {
            output.push(' ');
        } else {
            output.push(b as char);
        }
    }

    Ok(output)
}
