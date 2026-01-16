use eframe::egui;

pub fn header(ui: &mut egui::Ui) {
    ui.add_space(40.0);
    ui.label(egui::RichText::new("GS-Music").size(32.0).strong());
    ui.label(egui::RichText::new("Installer").size(16.0).color(egui::Color32::GRAY));
    ui.add_space(40.0);
}

pub fn ready(ui: &mut egui::Ui) -> bool {
    ui.label("Click below to install dependencies");
    ui.add_space(20.0);
    ui.add(egui::Button::new(egui::RichText::new("Install").size(16.0)).min_size(egui::vec2(180.0, 45.0))).clicked()
}

pub fn installing(ui: &mut egui::Ui) {
    ui.add(egui::Spinner::new().size(40.0));
    ui.add_space(15.0);
    ui.label("Installing...");
}

pub fn uninstalling(ui: &mut egui::Ui) {
    ui.add(egui::Spinner::new().size(40.0));
    ui.add_space(15.0);
    ui.label("Removing...");
}

pub fn done(ui: &mut egui::Ui) -> (bool, bool) {
    ui.label(egui::RichText::new("Ready to go!").size(18.0).color(egui::Color32::from_rgb(100, 200, 100)));
    ui.add_space(8.0);
    ui.label("All dependencies installed.");
    ui.add_space(25.0);
    let close = ui.add(egui::Button::new(egui::RichText::new("Close").size(16.0)).min_size(egui::vec2(180.0, 45.0))).clicked();
    ui.add_space(15.0);
    let uninstall = ui.add(egui::Button::new(egui::RichText::new("Uninstall").size(12.0).color(egui::Color32::from_rgb(180, 80, 80))).min_size(egui::vec2(120.0, 30.0))).clicked();
    (close, uninstall)
}

pub fn removed(ui: &mut egui::Ui) -> (bool, bool) {
    ui.label(egui::RichText::new("Dependencies removed").size(18.0).color(egui::Color32::from_rgb(200, 150, 100)));
    ui.add_space(25.0);
    let close = ui.add(egui::Button::new(egui::RichText::new("Close").size(16.0)).min_size(egui::vec2(180.0, 45.0))).clicked();
    ui.add_space(15.0);
    let reinstall = ui.add(egui::Button::new(egui::RichText::new("Reinstall").size(12.0)).min_size(egui::vec2(120.0, 30.0))).clicked();
    (close, reinstall)
}

pub fn error(ui: &mut egui::Ui) -> bool {
    ui.label(egui::RichText::new("Operation failed").size(18.0).color(egui::Color32::from_rgb(200, 100, 100)));
    ui.add_space(20.0);
    ui.add(egui::Button::new(egui::RichText::new("Retry").size(16.0)).min_size(egui::vec2(180.0, 45.0))).clicked()
}
