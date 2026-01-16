mod deps;
mod gui;
mod install;
mod os;
mod platform;
mod uninstall;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 350.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "GS-Music Installer",
        options,
        Box::new(|cc| Ok(Box::new(gui::App::new(cc)))),
    )
}
