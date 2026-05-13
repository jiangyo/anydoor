// 在 Windows 的 release 构建中不弹出额外的控制台窗口。
// debug 构建时仍会显示控制台，方便调试时看 println! 输出。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod models;
mod storage;

use app::DailyTrackerApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("Daily Tracker - 日常工作与行程记录"),
        ..Default::default()
    };

    eframe::run_native(
        "Daily Tracker",
        native_options,
        Box::new(|cc| Ok(Box::new(DailyTrackerApp::new(cc)))),
    )
}
