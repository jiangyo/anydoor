# 把 `daily-tracker` 迁移到独立仓库

> 目标：在 GitHub 上得到一个独立的仓库 **`jiangyo/daily-tracker`**，里面只放 Rust 项目，并且保留现有的两个 commit 历史。

我（Cloud Agent）没有创建 GitHub 仓库的权限，所以**第 1 步必须由你在浏览器或本地 `gh` CLI 完成**。第 2、3 步可以一键复制粘贴执行。

---

## 第 1 步：在 GitHub 上创建空仓库

打开 https://github.com/new ，按以下设置创建：

| 字段 | 值 |
| --- | --- |
| Owner | `jiangyo` |
| Repository name | `daily-tracker` |
| Description | 一个用 Rust 编写的桌面端日常工作与行程记录应用（学习项目） |
| Visibility | Private 或 Public 都可以 |
| Initialize this repository with | **全部不勾选**（README / .gitignore / license 都不要，因为我们要 push 已有历史） |

点击 **Create repository**。

或者，如果你本地的 `gh` 已经登录：

```bash
gh repo create jiangyo/daily-tracker --private \
  --description "一个用 Rust 编写的桌面端日常工作与行程记录应用（学习项目）"
```

---

## 第 2 步：把已经准备好的独立分支 push 到新仓库

我已经在 `jiangyo/anydoor` 里把 `daily-tracker/` 子目录用 `git subtree split` 抽出来，做成了一个独立分支 [`daily-tracker-standalone`](https://github.com/jiangyo/anydoor/tree/daily-tracker-standalone)：

- 内容已经提到了仓库根目录（不再嵌套在 `daily-tracker/` 里）
- 保留了两个原始 commit：
  - `feat(daily-tracker): 初始化 Rust 桌面应用骨架 (eframe + egui)`
  - `docs(daily-tracker): 添加 49 天 / 每天 2 小时的 Rust 学习计划`

在你本地任意空目录下执行：

```bash
# 把 anydoor 仓库里准备好的独立分支克隆下来
git clone --single-branch --branch daily-tracker-standalone \
    https://github.com/jiangyo/anydoor.git daily-tracker

cd daily-tracker

# 把当前分支重命名为 master（或 main，按你新仓库默认分支的名字）
git branch -M master

# 把 remote 指向你刚创建的新仓库
git remote set-url origin https://github.com/jiangyo/daily-tracker.git

# 推上去
git push -u origin master
```

完成后，访问 https://github.com/jiangyo/daily-tracker 应该能看到完整的项目，并且根目录直接是 `Cargo.toml` / `src/` / `LEARNING_PLAN.md` 等。

---

## 第 3 步：清理 `jiangyo/anydoor`（可选）

迁移确认无误后，可以把 `anydoor` 仓库里的 `daily-tracker/` 子目录和当前这个 PR 关掉：

```bash
# 在 anydoor 的 master 分支上
cd /path/to/anydoor
git checkout master
git pull

# 关掉 PR 后，把残留的分支删除
git push origin --delete cursor/init-rust-daily-tracker-app-2840
git push origin --delete daily-tracker-standalone
```

并且在 GitHub 上把这个 PR (#1) **手动 Close**（不要 Merge）即可。

如果你希望 `anydoor` 仓库继续保留之前的 TypeScript 文件，那这一步什么都不用做；如果你打算让 `anydoor` 也"清空开始另做一件事"，自己决定要不要 reset。

---

## 验证清单

迁移完成后，最好跑一遍：

```bash
cd daily-tracker
cargo check   # 应该能通过
cargo clippy --no-deps  # 应该无警告
cargo run     # 应该能弹出 GUI 窗口（本地有图形环境的话）
```

`git log --oneline` 应该看到两个 commit，不是一个 squash 后的大 commit——说明历史保住了。
