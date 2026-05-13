use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, ScrollArea, SidePanel, TopBottomPanel};

use crate::models::{AppData, Entry, EntryKind};
use crate::storage::Storage;

/// 应用主结构。
///
/// 这里把"持久化的数据 (data)"和"只在内存里存在的 UI 状态 (draft / filter)"
/// 分开放，这样以后想保存别的字段也不会污染 UI 状态。
pub struct DailyTrackerApp {
    data: AppData,
    storage: Storage,

    // 新增条目的输入区
    draft_title: String,
    draft_detail: String,
    draft_kind: EntryKind,

    // 列表筛选
    filter: Filter,

    // 用来在 UI 上短暂提示一条消息
    toast: Option<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Filter {
    #[default]
    All,
    Kind(EntryKind),
    Pending,
    Done,
}

impl DailyTrackerApp {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        let storage = Storage::new();
        let data = storage.load();
        Self {
            data,
            storage,
            draft_title: String::new(),
            draft_detail: String::new(),
            draft_kind: EntryKind::default(),
            filter: Filter::default(),
            toast: None,
        }
    }

    fn add_entry(&mut self) {
        let title = self.draft_title.trim();
        if title.is_empty() {
            self.toast = Some("标题不能为空".to_string());
            return;
        }
        let entry = Entry::new(title, self.draft_detail.trim(), self.draft_kind);
        self.data.entries.push(entry);
        self.draft_title.clear();
        self.draft_detail.clear();
        self.storage.save(&self.data);
        self.toast = Some("已添加".to_string());
    }

    fn entry_passes_filter(&self, entry: &Entry) -> bool {
        match self.filter {
            Filter::All => true,
            Filter::Kind(kind) => entry.kind == kind,
            Filter::Pending => !entry.done,
            Filter::Done => entry.done,
        }
    }
}

impl App for DailyTrackerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Daily Tracker");
                ui.separator();
                ui.label("记录我的日常工作与行程");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("共 {} 条", self.data.entries.len()));
                });
            });
        });

        SidePanel::left("left_panel")
            .resizable(true)
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.heading("新增一条");
                ui.add_space(6.0);

                ui.label("类型");
                ui.horizontal(|ui| {
                    for kind in EntryKind::all() {
                        ui.selectable_value(&mut self.draft_kind, kind, kind.label());
                    }
                });

                ui.add_space(6.0);
                ui.label("标题");
                ui.text_edit_singleline(&mut self.draft_title);

                ui.add_space(6.0);
                ui.label("详细描述（可选）");
                ui.add(
                    egui::TextEdit::multiline(&mut self.draft_detail)
                        .desired_rows(4)
                        .desired_width(f32::INFINITY),
                );

                ui.add_space(8.0);
                if ui.button("➕ 添加").clicked() {
                    self.add_entry();
                }

                ui.separator();
                ui.heading("筛选");
                ui.selectable_value(&mut self.filter, Filter::All, "全部");
                ui.selectable_value(&mut self.filter, Filter::Kind(EntryKind::Work), "只看工作");
                ui.selectable_value(&mut self.filter, Filter::Kind(EntryKind::Trip), "只看行程");
                ui.selectable_value(&mut self.filter, Filter::Pending, "未完成");
                ui.selectable_value(&mut self.filter, Filter::Done, "已完成");

                ui.separator();
                ui.collapsing("数据位置", |ui| {
                    ui.monospace(self.storage.path().display().to_string());
                });
            });

        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(msg) = &self.toast {
                    ui.label(format!("ℹ {msg}"));
                } else {
                    ui.label("准备就绪");
                }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("记录列表");
            ui.add_space(4.0);

            let mut data_changed = false;
            let mut to_delete: Option<usize> = None;

            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                // 倒序展示：最近添加的排在最上面
                for idx in (0..self.data.entries.len()).rev() {
                    if !self.entry_passes_filter(&self.data.entries[idx]) {
                        continue;
                    }

                    let entry = &mut self.data.entries[idx];
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut entry.done, "").changed() {
                                data_changed = true;
                            }

                            let title_text = if entry.done {
                                egui::RichText::new(&entry.title)
                                    .strikethrough()
                                    .weak()
                            } else {
                                egui::RichText::new(&entry.title).strong()
                            };
                            ui.label(title_text);

                            ui.separator();
                            ui.label(entry.kind.label());
                            ui.separator();
                            ui.label(entry.created_at.format("%Y-%m-%d %H:%M").to_string());

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("🗑 删除").clicked() {
                                        to_delete = Some(idx);
                                    }
                                },
                            );
                        });

                        if !entry.detail.is_empty() {
                            ui.label(&entry.detail);
                        }
                    });
                    ui.add_space(4.0);
                }
            });

            if let Some(idx) = to_delete {
                self.data.entries.remove(idx);
                data_changed = true;
                self.toast = Some("已删除".to_string());
            }

            if data_changed {
                self.storage.save(&self.data);
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // 当窗口被关闭、系统让出焦点时 eframe 会调用 save。
        // 这里再保险地落盘一次，防止有未保存的改动。
        self.storage.save(&self.data);
    }
}
