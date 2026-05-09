use job_application_helper::app;

fn main() -> eframe::Result {
    // Application state
    let mut job_app = app::App::default();
    let options = eframe::NativeOptions::default();
    eframe::run_ui_native("My egui App", options, move |ui, _frame| {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        // Wrap everything in a CentralPanel so we get some margins and a background color:
        egui::CentralPanel::default().show_inside(ui, |ui| {
            job_app.primary(ui);
        });
    })
}
