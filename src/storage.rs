use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::models::AppData;

/// 数据存放路径管理。
///
/// 跨平台地把数据放在标准的"应用数据目录"下，例如：
/// - Linux:   ~/.local/share/daily-tracker/data.json
/// - macOS:   ~/Library/Application Support/com.example.daily-tracker/data.json
/// - Windows: %APPDATA%\example\daily-tracker\data\data.json
pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let file_path = default_data_file().unwrap_or_else(|| PathBuf::from("daily-tracker.json"));
        Self { file_path }
    }

    pub fn path(&self) -> &Path {
        &self.file_path
    }

    /// 从磁盘读取数据；如果文件不存在或者解析失败，就返回空数据。
    pub fn load(&self) -> AppData {
        match fs::read_to_string(&self.file_path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(err) if err.kind() == io::ErrorKind::NotFound => AppData::default(),
            Err(err) => {
                eprintln!("读取数据失败: {err}");
                AppData::default()
            }
        }
    }

    /// 把数据写回磁盘。失败时打印一条日志，不让 GUI 整个崩掉。
    pub fn save(&self, data: &AppData) {
        if let Some(parent) = self.file_path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                eprintln!("创建数据目录失败: {err}");
                return;
            }
        }

        match serde_json::to_string_pretty(data) {
            Ok(json) => {
                if let Err(err) = fs::write(&self.file_path, json) {
                    eprintln!("写入数据失败: {err}");
                }
            }
            Err(err) => eprintln!("序列化数据失败: {err}"),
        }
    }
}

fn default_data_file() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("com", "example", "daily-tracker")?;
    Some(dirs.data_dir().join("data.json"))
}
