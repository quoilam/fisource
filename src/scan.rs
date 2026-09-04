use std::fs;
use std::path::{Path, PathBuf};

use crate::model::EnvFile;
use crate::parser;

/// 扫描 `~/.fisource` 顶层目录下的 `*.env` 文件，按文件名排序。
pub fn scan_all() -> Vec<EnvFile> {
    let dir = match default_dir() {
        Some(d) => d,
        None => return Vec::new(),
    };
    let mut files = Vec::new();

    let Ok(entries) = fs::read_dir(&dir) else {
        return Vec::new();
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.ends_with(".env") {
            continue;
        }
        if let Ok(content) = fs::read_to_string(&path) {
            files.push(parser::parse(name, &content));
        }
    }

    files.sort_by(|a, b| a.name.cmp(&b.name));
    files
}

/// 解析指定 env 文件的完整路径并读取。
pub fn load_one(arg: &str) -> Option<EnvFile> {
    let path = resolve_path(arg);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(arg)
        .to_string();
    let content = fs::read_to_string(&path).ok()?;
    Some(parser::parse(&name, &content))
}

/// 把用户给的参数解析为绝对路径：若已是绝对路径则原样返回，否则相对于 `~/.fisource`。
fn resolve_path(arg: &str) -> PathBuf {
    let p = Path::new(arg);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        default_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(arg)
    }
}

fn default_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(Path::new(&home).join(".fisource"))
}
