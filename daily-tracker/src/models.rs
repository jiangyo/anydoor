use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 一条记录的类型：是工作任务，还是行程安排。
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKind {
    #[default]
    Work,
    Trip,
}

impl EntryKind {
    pub fn label(&self) -> &'static str {
        match self {
            EntryKind::Work => "工作",
            EntryKind::Trip => "行程",
        }
    }

    pub fn all() -> [EntryKind; 2] {
        [EntryKind::Work, EntryKind::Trip]
    }
}

/// 一条日常记录。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: Uuid,
    pub title: String,
    pub detail: String,
    pub kind: EntryKind,
    pub created_at: DateTime<Local>,
    pub done: bool,
}

impl Entry {
    pub fn new(title: impl Into<String>, detail: impl Into<String>, kind: EntryKind) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            detail: detail.into(),
            kind,
            created_at: Local::now(),
            done: false,
        }
    }
}

/// 整个应用持久化的数据。后面如果要加字段，加上 #[serde(default)] 就能保持向后兼容。
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppData {
    #[serde(default)]
    pub entries: Vec<Entry>,
}
