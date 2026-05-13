# Rust 学习计划（每天 2 小时）

> 目标：用 **49 天** 把 Rust 从零吃到能独立维护 `daily-tracker` 这样的桌面 / 服务端项目，
> 并形成自己的一套工程习惯（错误处理、测试、CI、打包）。
>
> 每天的可支配时间：**120 分钟**。如果某天有事，**顺延不补**，避免破窗。

---

## 时间分配模板（每天都这样）

| 段 | 时长 | 做什么 |
| --- | --- | --- |
| ① 复盘 | 20 min | 跑一遍前一天写的代码；把昨天 commit message + 笔记看一遍 |
| ② 输入 | 50 min | 读《The Rust Programming Language》对应章节，或看 Jon Gjengset / Let's Get Rusty 对应视频 |
| ③ 输出 | 50 min | 动手：写练习 / 改 `daily-tracker`，必须 **当天 commit 一次** |

> "输入 ≤ 输出" 是 **底线**。Rust 是肌肉记忆型语言，光看书没用。

---

## 总览

| 阶段 | 主题 | 天数 | 关键产出 |
| --- | --- | --- | --- |
| 0 | 起步 & 工具链 | D1 | 跑通 daily-tracker，熟悉 cargo |
| 1 | 基础语法 + 所有权 | D2 – D7 | 给 Entry 加字段，理解 `&` / `&mut` |
| 2 | 枚举、模式匹配、错误处理 | D8 – D14 | storage 改成 `Result + thiserror` |
| 3 | 集合、迭代器、闭包、测试 | D15 – D21 | 搜索 + 单元测试 |
| 4 | Trait、泛型、生命周期 | D22 – D28 | 抽象 `Exporter` trait |
| 5 | IO / serde / SQLite | D29 – D35 | 把 JSON 存储迁到 SQLite |
| 6 | 并发 & 异步 | D36 – D42 | 后台线程写盘；尝试 tokio |
| 7 | 工程化 & 发版 | D43 – D49 | workspace、CI、cargo-bundle，v1.0 |
| ∞ | 进阶选学 | 之后 | unsafe / FFI、性能、axum 服务端 |

---

## 阶段 0：起步（Day 1）

| 步骤 | 内容 |
| --- | --- |
| 输入 | Rust Book Ch 1（安装 / hello world）+ Ch 2（猜数字游戏，跟着敲一遍） |
| 输出 | ① `cargo run` 跑起 daily-tracker，添加 3 条记录；② 在自己应用里建一条记录："Day 1 完成"，类型 = 工作 |
| Checkpoint | 能解释 `cargo new` / `cargo run` / `cargo build` / `cargo check` 的区别 |

---

## 阶段 1：基础语法 + 所有权（Day 2 – Day 7）

这一阶段的核心是 **Ownership / Borrowing**，是 Rust 唯一真正陡的地方，硬啃过去后面就快了。

### Day 2 — 变量、类型、控制流
- 输入：Book Ch 3
- 输出：写 5 个小函数（`fizzbuzz`、`is_prime`、`fib`、`sum_of_digits`、`reverse_string`），放到 `daily-tracker/examples/day02.rs`，用 `cargo run --example day02` 跑

### Day 3 — Ownership（**最重要的一天**）
- 输入：Book Ch 4.1
- 输出：在 `models.rs` 给 `Entry` 加方法
  ```rust
  pub fn mark_done(&mut self) { self.done = true; }
  pub fn title(&self) -> &str { &self.title }
  ```
  并在 `app.rs` 里改用这两个方法。**用语言写下** "为什么 `mark_done` 要 `&mut self`、`title` 要 `&self`"。

### Day 4 — 引用与借用
- 输入：Book Ch 4.2
- 输出：阅读 `storage.rs`，找出里面 **每一个 `&` 和每一个 `.clone()`**，给自己解释：去掉它会怎样？

### Day 5 — Slice
- 输入：Book Ch 4.3
- 输出：在 `examples/day05.rs` 里实现：① `first_word(&str) -> &str`，② `longest(&str, &str) -> &str`（先报错，后面学生命周期再修）

### Day 6 — Struct
- 输入：Book Ch 5
- 输出：给 `Entry` 加 `pub tags: Vec<String>`，在 UI 上用一个简单的逗号分隔输入框（先不做花哨 UI），并能保存。**注意 `#[serde(default)]`**。

### Day 7 — 周复盘
- 输入：不读新内容，把 D1–D6 笔记和 commit 重新过一遍
- 输出：写一篇 `daily-tracker/notes/week1.md`，包含：
  - 我这周学到的 3 个最重要的点
  - 我目前还**不**明白的 3 个问题（贴出来，下周回头看）

---

## 阶段 2：枚举、模式匹配、错误处理（Day 8 – Day 14）

### Day 8 — Enum / Option
- 输入：Book Ch 6.1 + 6.2
- 输出：把所有 `if let Some(_)` / `match` 用法在 `app.rs` 里找出来，标注它们的作用

### Day 9 — match 模式
- 输入：Book Ch 6.3 + Ch 18（模式匹配大全）
- 输出：给 `Entry` 加 `priority: Priority { Low, Mid, High }` 枚举字段，在 UI 上展示成颜色不同的小标签

### Day 10 — 模块系统
- 输入：Book Ch 7
- 输出：把 `app.rs` 拆成 `app/mod.rs` + `app/sidebar.rs` + `app/list.rs`，体会 `pub(crate)` / `pub(super)` 的差别

### Day 11 — 集合（Vec / String / HashMap）
- 输入：Book Ch 8
- 输出：`examples/day11.rs`：用 HashMap 实现"按 tag 统计每个 tag 出现多少次"

### Day 12 — 错误处理 panic vs Result
- 输入：Book Ch 9
- 输出：纸面练习：把 `storage.rs` 里所有 `eprintln!` 列出来，写下如果改成返回 `Result`，函数签名分别会变成什么

### Day 13 — 实战：thiserror 改造 storage
- 输入：[thiserror docs](https://docs.rs/thiserror)
- 输出：把 `storage.rs` 改成：
  ```rust
  pub enum StorageError {
      Io(#[from] std::io::Error),
      Parse(#[from] serde_json::Error),
  }
  pub fn load(&self) -> Result<AppData, StorageError> { ... }
  ```
  调用方用 `?` 传播，UI 层把错误显示成 toast

### Day 14 — 周复盘
- `notes/week2.md`，并把 D7 留下的 3 个问题尝试回答

---

## 阶段 3：集合、迭代器、闭包、测试（Day 15 – Day 21）

### Day 15 — 闭包
- 输入：Book Ch 13.1
- 输出：在 `examples/day15.rs` 里写 3 个高阶函数：`retry(n, f)`、`measure(label, f)`、`memoize(f)`

### Day 16 — 迭代器
- 输入：Book Ch 13.2 + 13.3
- 输出：把 `app.rs` 里的 `for idx in (0..n).rev() { if !filter { continue; } ... }` 改写成 `.iter().rev().filter(...).enumerate()`，对比可读性

### Day 17 — 实战：搜索框
- 输入：—
- 输出：在左侧面板加一个搜索框，按 `title`/`detail`/`tags` 模糊匹配，**必须用迭代器链式实现**，禁止显式 for 循环

### Day 18 — 测试基础
- 输入：Book Ch 11.1 + 11.2
- 输出：给 `models.rs` 写 5 个单元测试（覆盖 `Entry::new`、`EntryKind::label`）

### Day 19 — 实战：storage 测试
- 输入：[tempfile docs](https://docs.rs/tempfile)
- 输出：写 `storage::tests`，每个测试用 `tempfile::TempDir` 创建一个临时目录，验证 save/load 往返一致

### Day 20 — 实战：Markdown 导出
- 输入：—
- 输出：增加菜单 "导出 → Markdown"，把当前筛选结果导出成一份 `.md` 文件。**必须用 `Iterator + fold/format!` 实现**，不要先 `String::new()` 再 push。

### Day 21 — 周复盘
- `notes/week3.md` + 把项目跑一遍，发现的任何小 bug 都开一个 GitHub Issue 给自己

---

## 阶段 4：Trait、泛型、生命周期（Day 22 – Day 28）

这一阶段后你会"开始觉得 Rust 很爽"。

### Day 22 — 泛型
- 输入：Book Ch 10.1
- 输出：`examples/day22.rs`：实现一个 `largest<T: PartialOrd>(&[T]) -> &T`

### Day 23 — Trait + 实战
- 输入：Book Ch 10.2
- 输出：定义
  ```rust
  pub trait Exporter {
      fn extension(&self) -> &'static str;
      fn export(&self, entries: &[Entry]) -> String;
  }
  ```
  把 Day 20 的 Markdown 导出改成实现这个 trait 的 `MarkdownExporter`

### Day 24 — 生命周期
- 输入：Book Ch 10.3
- 输出：回到 Day 5 那个 `longest`，这次写对它的签名 `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str`，并解释为什么

### Day 25 — Trait Object（dyn）
- 输入：Book Ch 17.2
- 输出：在 `app.rs` 里维护 `exporters: Vec<Box<dyn Exporter>>`，再加一个 `CsvExporter` 实现，菜单变成下拉

### Day 26 — From / Into / Display / Debug
- 输入：[std::convert](https://doc.rust-lang.org/std/convert/)
- 输出：给 `EntryKind` 实现 `Display`，给 `EntryKind::from_str` 实现 `FromStr`，重写 `label()` 为 `Display::fmt`

### Day 27 — 自由练习日
- 任选一条之前留下的 TODO 或 Issue 实现掉

### Day 28 — 周复盘
- `notes/week4.md`，给自己拍一张 daily-tracker 当前界面的截图，对比 Day 1 的状态

---

## 阶段 5：IO / serde / SQLite（Day 29 – Day 35）

### Day 29 — std::fs / std::io
- 输入：[std::fs docs](https://doc.rust-lang.org/std/fs/) + [std::io docs](https://doc.rust-lang.org/std/io/)
- 输出：写一个 CLI 工具 `cargo run --bin exporter -- --in data.json --out report.md`，复用 Exporter trait（学习把 lib + 多个 bin 放一起）

### Day 30 — serde 深入
- 输入：[serde derive 文档](https://serde.rs/derive.html) + tagged enum
- 输出：给 `EntryKind` 加 `#[serde(rename_all = "lowercase")]`，并写一个数据迁移测试：旧 JSON（字段缺失）也能成功 load

### Day 31 — 导入功能
- 输出：实现"从 CSV 导入"，处理各种边界（缺列、编码、引号）

### Day 32 — 引入 rusqlite
- 输入：[rusqlite README](https://github.com/rusqlite/rusqlite)
- 输出：在 `examples/day32.rs` 里跑通：建表 → 插入 → 查询

### Day 33 — 事务 & 迁移
- 输出：写一个 `migrations.rs`，幂等地从 `v0` → `v1`（建表）→ `v2`（加 priority 列）

### Day 34 — 实战：切换到 SQLite
- 输出：新增 `storage::sqlite::SqliteStorage`，实现和现有 `JsonStorage` **一样的方法签名**（提前抽出 `trait Storage`）。在 app 里通过环境变量 `DT_BACKEND=sqlite|json` 选择后端。

### Day 35 — 周复盘
- `notes/week5.md`：写一段"为什么 SQLite 比 JSON 好 / 差"，自己回答

---

## 阶段 6：并发 & 异步（Day 36 – Day 42）

### Day 36 — 线程 + channel
- 输入：Book Ch 16.1 + 16.2
- 输出：`examples/day36.rs`：主线程把任务通过 `mpsc::channel` 发到 worker 线程

### Day 37 — Mutex / Arc
- 输入：Book Ch 16.3
- 输出：`examples/day37.rs`：4 个线程并发往同一个 `Arc<Mutex<HashMap>>` 写，对比不加锁的崩溃

### Day 38 — 实战：后台写盘
- 输出：把 `storage.save` 改成"发到 channel，后台线程消费"，UI 操作完全不阻塞。重点：**优雅停机**，关窗时 flush。

### Day 39 — async 起步
- 输入：[async book](https://rust-lang.github.io/async-book/) 1–3 章
- 输出：`examples/day39.rs`：用 `tokio` 写一个并发抓 5 个 URL 的小程序（如果不能联网，就 `tokio::time::sleep` 模拟）

### Day 40 — async/await 深入
- 输入：async book 4–5 章
- 输出：自己实现一个 `async fn retry<F, Fut, T>(...)`

### Day 41 — sqlx + tokio
- 输入：[sqlx README](https://github.com/launchbadge/sqlx)
- 输出：再加一个 `SqlxStorage`，和现有 trait 对齐。**这一步可能要把整个 app 改成 async runtime**，遇到困难就把它单独放到一个新 example 里先打通

### Day 42 — 周复盘 + 决定方向
- 写下：daily-tracker 接下来想做"纯桌面 + SQLite"，还是"桌面 + 云端同步"？这决定 Week 7 重点

---

## 阶段 7：工程化 & 发版（Day 43 – Day 49）

### Day 43 — Cargo Workspace
- 输出：把 daily-tracker 拆成
  ```
  daily-tracker/
  ├── Cargo.toml      # workspace
  ├── crates/
  │   ├── core/       # models + storage trait
  │   ├── storage-json/
  │   ├── storage-sqlite/
  │   └── app/        # 二进制 bin
  ```

### Day 44 — Feature flag
- 输出：让 `storage-sqlite` 是可选 feature，`cargo build --no-default-features` 只编 JSON 版本

### Day 45 — 打包
- 输入：[cargo-bundle](https://github.com/burtonageo/cargo-bundle) 或 [cargo-dist](https://opensource.axo.dev/cargo-dist/)
- 输出：能在本机产出一个 `.AppImage` 或 `.app` 双击就能跑

### Day 46 — CI
- 输出：新建 `.github/workflows/ci.yml`：`cargo fmt --check` + `cargo clippy -D warnings` + `cargo test`，三个 OS（ubuntu / macos / windows）跑通

### Day 47 — pre-commit / 规范
- 输出：装 `rustfmt` + `clippy` 做 pre-commit hook；在 README 加 "Contributing" 章节（哪怕只有你一个 contributor，也是练手工程素养）

### Day 48 — 完善 README + 截图
- 输出：给 README 加 GIF / 截图（用 `peek` 或系统截图工具），写"为什么我做这个项目"

### Day 49 — v1.0.0
- 输出：`git tag v1.0.0` + 在 GitHub Release 页面上传 Day 45 的安装包，**发布**。给自己写一篇博客（哪怕只发给自己看），标题就叫 *"我用 49 天学 Rust，做了一个 daily-tracker"*。

---

## 之后：进阶选学（按兴趣挑）

| 主题 | 关键资源 | 适合做什么 |
| --- | --- | --- |
| 智能指针 Rc / Arc / RefCell / Cell | Book Ch 15 | 在 UI 里做一个父子组件共享状态的小练习 |
| unsafe / FFI | Nomicon | 调一个 C 库（比如 SQLite 原生 API、libnotify） |
| 性能 | `cargo-flamegraph`, `criterion` | 给 daily-tracker 写 benchmark |
| 宏 | Book Ch 19.5, proc-macro2 / syn / quote | 写一个 `#[derive(Exporter)]` |
| Web 后端 | axum / sqlx | 给 daily-tracker 加云端同步 server |
| 嵌入式 | embedded-book + ESP32-Rust | 把 daily-tracker 的"今日任务"显示在一个 e-ink 上 |

---

## 推荐资料（按优先级，**不要贪多**）

1. **The Rust Programming Language**（"the Book"）— https://doc.rust-lang.org/book/ — **唯一一本必读**
2. **Rust By Example** — https://doc.rust-lang.org/rust-by-example/ — 当成第 1 本的练习册
3. **Rustlings** — https://github.com/rust-lang/rustlings — 边读 Book 边做，刚好对应章节
4. *Programming Rust*（O'Reilly，第 2 版） — 想深入时再读
5. *Rust for Rustaceans*（Jon Gjengset） — Day 28 以后再翻
6. Jon Gjengset 的 [YouTube "Crust of Rust"](https://www.youtube.com/c/JonGjengset) — Day 35 以后看，会爽到

> **不推荐** 一上来就刷 leetcode-rust 或者啃《Rust 死灵书》，前者偏算法，后者讲 unsafe，都不解决"我能写出一个完整 app"这件事。

---

## 怎么检查进度

每个周末 (Day 7, 14, 21, 28, 35, 42, 49) 必须做这 3 件事：

1. `git log --since="last sunday" --oneline` 看看自己这周有没有 commit
2. 把 `daily-tracker` 跑一遍，截图 vs. 上周对比
3. 写一段 `notes/weekN.md`，回答：
   - 这周最大的 *aha* 是什么？
   - 这周我**没**学完的是什么？为什么？
   - 下周第一个要解决的问题是什么？

如果连续两周没 commit，说明计划没在跑——回到 Day 1，**降低当天目标**（比如 30 分钟也行），先把习惯捡回来。

---

## 紧急情况处理

| 症状 | 处方 |
| --- | --- |
| 借用检查器报错看不懂 | 把 borrow 关系画在纸上，谁拥有谁，谁借给谁。不要去强行 `.clone()` |
| 写得太慢，进度卡住 | 看 Day 计划里 **"输出"** 一栏，砍到 50%，让今天能 commit 就行 |
| 觉得 Rust 太繁琐想跑路 | 去把 daily-tracker 跑起来，添加一条记录"我学 Rust 第 N 天"。再回去 |
| 学了忘 | 这是正常的。每周末必须复盘，不复盘等于没学 |
