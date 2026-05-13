# Daily Tracker

一个用 **Rust** 编写的桌面端小应用，用来记录日常 **工作** 与 **行程**。
项目本身是为了 **学习 Rust** 而搭建的，刻意保持简单、模块化，方便逐步扩展。

> 配套学习计划：[`LEARNING_PLAN.md`](./LEARNING_PLAN.md)（每天 2 小时，49 天，每个新概念都回到本项目里落地）。

## 技术选型

| 用途 | 选型 | 理由 |
| --- | --- | --- |
| GUI | [`eframe`](https://crates.io/crates/eframe) + [`egui`](https://crates.io/crates/egui) | 纯 Rust 即时模式 GUI，**没有 JS/HTML**，可以完全聚焦在 Rust 本身 |
| 序列化 | `serde` + `serde_json` | Rust 生态最常用的序列化方案 |
| 时间 | `chrono` | 处理日期/时间，配合 `serde` 直接落盘 |
| 路径 | `directories` | 跨平台获取数据目录 |
| ID | `uuid` | 每条记录一个稳定 ID |

## 目录结构

```
daily-tracker/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs       # 程序入口：创建窗口、装配应用
    ├── app.rs        # GUI 主结构 & 渲染逻辑（实现 eframe::App）
    ├── models.rs     # 数据模型：Entry / EntryKind / AppData
    └── storage.rs    # 本地 JSON 持久化
```

## 功能

- 添加一条记录（标题、详细描述、类型：工作 / 行程）
- 列表展示，按时间倒序
- 一键勾选已完成 / 删除
- 按类型 / 完成状态筛选
- 数据自动保存到本地 JSON 文件，重启不丢失

## 运行

```bash
cd daily-tracker
cargo run
```

> 第一次运行需要下载并编译依赖，会慢一些；之后增量编译会快很多。

Linux 桌面环境下，eframe 依赖一些系统库；如果在精简环境里编译失败，参考 [eframe 文档](https://github.com/emilk/egui/tree/master/crates/eframe#what-is-eframe) 安装对应的开发包（例如 `libxkbcommon-dev`、`libgl1-mesa-dev` 等）。

## 数据存放位置

应用使用 `directories` crate 选择标准目录，运行时也可以在左侧栏 "数据位置" 里看到具体路径：

- Linux: `~/.local/share/daily-tracker/data.json`
- macOS: `~/Library/Application Support/com.example.daily-tracker/data.json`
- Windows: `%APPDATA%\example\daily-tracker\data\data.json`

## 学习 Rust 的扩展方向

这个项目刻意留了很多可以继续练手的口子，按从易到难：

1. **加字段**：给 `Entry` 加 `tags: Vec<String>`、`due_at: Option<DateTime<Local>>`，体会 `serde(default)` 的向后兼容。
2. **错误处理**：把 `storage.rs` 里的 `eprintln!` 改成返回 `Result`，配合 [`thiserror`](https://crates.io/crates/thiserror) 定义错误类型。
3. **编辑功能**：点击列表项后进入编辑模式（学习 `Option<Uuid>` 表示"正在编辑哪一条"）。
4. **搜索**：加一个搜索框，按标题/描述过滤（练习字符串处理 + 迭代器）。
5. **导出**：导出成 Markdown / CSV（练习 trait 抽象 "导出器"）。
6. **测试**：给 `models` 和 `storage` 写单元测试，体验 `#[cfg(test)]` 和 `tempfile`。
7. **异步**：把存储改成异步写入（`tokio` + channel），避免主线程卡顿。
8. **打包**：用 `cargo-bundle` 或 `cargo dist` 打成 `.app` / `.msi` / `.AppImage`。

## License

MIT
