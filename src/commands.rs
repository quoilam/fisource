use std::collections::HashSet;

use colored::Colorize;

use crate::fish;
use crate::model::EnvFile;

/// source 模式：输出 fish 代码，由外壳函数 eval 执行。
pub fn cmd_source(file: &EnvFile) -> String {
    let entries: Vec<(String, String)> = file
        .entries
        .iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect();
    fish::emit_source(&entries)
}

/// `--keys`：列举所有 key（跨文件去重，保留首次出现顺序），脱敏。
pub fn cmd_keys(files: &[EnvFile]) -> String {
    let mut seen = HashSet::new();
    let mut out = String::new();
    for file in files {
        for key in file.keys() {
            if seen.insert(key.to_string()) {
                out.push_str(key);
                out.push('\n');
            }
        }
    }
    out
}

/// `--list`：带前缀的块结构，同名 key 不跨文件去重，脱敏。
pub fn cmd_list(files: &[EnvFile]) -> String {
    let mut out = String::new();
    for file in files {
        out.push_str("# file: ");
        out.push_str(&file.name);
        out.push('\n');
        if let Some(desc) = &file.description {
            out.push_str("# desc: ");
            out.push_str(desc);
            out.push('\n');
        }
        for key in file.keys() {
            out.push_str("key: ");
            out.push_str(key);
            out.push('\n');
        }
    }
    out
}

/// `--show`：env 矩阵，行标签为 env 文件，行内每个 key 独立标色：
/// 绿=匹配、红=值不匹配、黄=缺失。非 TTY 下颜色被禁用，但状态符号保留。
pub fn cmd_show(files: &[EnvFile]) -> String {
    let width = files
        .iter()
        .map(|f| f.name.chars().count())
        .max()
        .unwrap_or(0);

    let mut out = String::new();
    for file in files {
        out.push_str(&format!("{:<width$}  ", file.name, width = width));
        for entry in &file.entries {
            let current = std::env::var(&entry.key);
            let cell = match current {
                Ok(v) if v == entry.value => format!("✓{}", entry.key).green().to_string(),
                Ok(_) => format!("✗{}", entry.key).red().to_string(),
                Err(_) => format!("?{}", entry.key).yellow().to_string(),
            };
            out.push_str(&cell);
            out.push(' ');
        }
        out.push('\n');
    }
    out
}

/// fzf 候选列表：每行 `文件名\t描述\tKEY1 KEY2 ...`，供 fzf 展示与筛选。
pub fn cmd_fzf_list(files: &[EnvFile]) -> String {
    let mut out = String::new();
    for file in files {
        let desc = file.description.clone().unwrap_or_default();
        let keys: Vec<&str> = file.keys().collect();
        out.push_str(&file.name);
        out.push('\t');
        out.push_str(&desc);
        out.push('\t');
        out.push_str(&keys.join(" "));
        out.push('\n');
    }
    out
}

/// fzf preview：显示指定文件的描述与 key=value（含值）。仅用于用户 fzf 预览路径。
pub fn cmd_fzf_preview(file: &EnvFile) -> String {
    let mut out = String::new();
    if let Some(desc) = &file.description {
        out.push_str("# ");
        out.push_str(desc);
        out.push('\n');
    }
    for entry in &file.entries {
        out.push_str(&entry.key);
        out.push('=');
        out.push_str(&entry.value);
        out.push('\n');
    }
    out
}
