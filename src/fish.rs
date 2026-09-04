/// 生成 fish 代码，把 key/value 安全地 export 为全局环境变量。
///
/// 直接输出 fish 单引号字面量（如 `set -gx KEY 'value'`），由外壳函数 eval 执行。
/// fish 单引号内 `$` 是字面量、不会展开，因此只需转义 `\` 和 `'` 两类字符。
pub fn emit_source(entries: &[(String, String)]) -> String {
    let mut out = String::new();
    for (key, value) in entries {
        out.push_str("set -gx ");
        out.push_str(key);
        out.push(' ');
        out.push_str(&single_quote(value));
        out.push('\n');
    }
    out
}

/// 把值包进 fish 单引号字面量。单引号内 `$` 不展开，仅 `\` 和 `'` 需要转义。
fn single_quote(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('\'', "\\'");
    format!("'{}'", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_in_single_quotes() {
        assert_eq!(single_quote("abc"), "'abc'");
        assert_eq!(single_quote("it's"), "'it\\'s'");
        assert_eq!(single_quote("a\\b"), "'a\\\\b'");
    }

    #[test]
    fn dollar_not_escaped() {
        assert_eq!(single_quote("has $dollar"), "'has $dollar'");
    }

    #[test]
    fn emits_set_global_export() {
        let entries = vec![("DB_HOST".to_string(), "localhost".to_string())];
        let out = emit_source(&entries);
        assert_eq!(out, "set -gx DB_HOST 'localhost'\n");
    }
}
