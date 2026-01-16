use eframe::egui;

pub fn error(ctx: &egui::Context, show: &mut bool) {
    egui::Window::new("Oh No!")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size([300.0, 150.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label("Install failed.");
                ui.add_space(8.0);
                ui.label("The log has been copied to your clipboard.");
                ui.label("Please report this in the GS Music Discord.");
                ui.add_space(15.0);
                if ui.button("OK").clicked() {
                    *show = false;
                }
            });
        });
}

pub fn unsupported(ctx: &egui::Context) -> bool {
    let mut closed = false;
    egui::Window::new("Unsupported System")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size([320.0, 200.0])
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label("Your package manager is not supported.");
                ui.add_space(10.0);
                ui.label("Please install these manually:");
                ui.add_space(8.0);
                ui.label(egui::RichText::new("mpv, yt-dlp, ffmpeg, socat").strong());
                ui.add_space(15.0);
                ui.label(egui::RichText::new("(socat is Linux only)").size(11.0).color(egui::Color32::GRAY));
                ui.add_space(15.0);
                if ui.button("Close").clicked() {
                    closed = true;
                }
            });
        });
    closed
}
