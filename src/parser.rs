use crate::model::{Entry, EnvFile};

/// 解析 bash env 语法的一个极简子集：
/// - `#` 注释（`#description:` 提取为文件描述）
/// - `KEY=VALUE`
/// - 可选 `export ` 前缀
/// - 单引号 / 双引号包裹值
///
/// 不做变量引用展开、不做命令替换、不支持多行值。
pub fn parse(name: &str, content: &str) -> EnvFile {
    let mut file = EnvFile {
        name: name.to_string(),
        description: None,
        entries: Vec::new(),
    };

    for raw in content.lines() {
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(comment) = trimmed.strip_prefix('#') {
            let comment = comment.trim();
            if let Some(desc) = comment.strip_prefix("description") {
                let desc = desc.trim_start_matches(':').trim();
                if !desc.is_empty() && file.description.is_none() {
                    file.description = Some(desc.to_string());
                }
            }
            continue;
        }

        let rest = trimmed
            .strip_prefix("export")
            .map(str::trim)
            .unwrap_or(trimmed);

        let Some(eq) = rest.find('=') else {
            continue;
        };

        let key = rest[..eq].trim();
        let raw_value = rest[eq + 1..].trim();

        if key.is_empty() {
            continue;
        }

        let value = unquote(raw_value);
        file.entries.push(Entry {
            key: key.to_string(),
            value,
        });
    }

    file
}

/// 去掉包裹值的外层引号。极简子集：只处理首尾成对出现的情况。
fn unquote(s: &str) -> String {
    let bytes = s.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        let last = bytes[bytes.len() - 1];
        if (first == b'"' && last == b'"') || (first == b'\'' && last == b'\'') {
            return s[1..s.len() - 1].to_string();
        }
    }
    s.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_kv() {
        let f = parse("app.env", "DB_HOST=localhost\nDB_PORT=5432\n");
        assert_eq!(f.entries.len(), 2);
        assert_eq!(f.entries[0].key, "DB_HOST");
        assert_eq!(f.entries[0].value, "localhost");
    }

    #[test]
    fn parses_export_prefix() {
        let f = parse("app.env", "export DB_HOST=localhost\n");
        assert_eq!(f.entries[0].key, "DB_HOST");
        assert_eq!(f.entries[0].value, "localhost");
    }

    #[test]
    fn parses_quotes() {
        let f = parse(
            "app.env",
            "PASSWORD=\"p@ss w0rd\"\nSINGLE='it has spaces'\n",
        );
        assert_eq!(f.entries[0].value, "p@ss w0rd");
        assert_eq!(f.entries[1].value, "it has spaces");
    }

    #[test]
    fn extracts_description() {
        let f = parse("app.env", "#description: 用于k8s集群的配置\nDB_HOST=x\n");
        assert_eq!(f.description.as_deref(), Some("用于k8s集群的配置"));
    }

    #[test]
    fn skips_plain_comments_and_blank() {
        let f = parse("app.env", "# 普通注释\n\nDB_HOST=x\n");
        assert_eq!(f.entries.len(), 1);
    }

    #[test]
    fn keeps_value_with_inner_equals() {
        let f = parse("app.env", "URL=http://x.com?a=b\n");
        assert_eq!(f.entries[0].value, "http://x.com?a=b");
    }
}
