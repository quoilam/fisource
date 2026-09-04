#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvFile {
    /// 文件名，不含路径，如 "app.env"
    pub name: String,
    /// 来自 `#description:` 注释
    pub description: Option<String>,
    /// 有序 kv 对
    pub entries: Vec<Entry>,
}

impl EnvFile {
    pub fn keys(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.key.as_str())
    }
}
