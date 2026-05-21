use job_application_helper::app;

fn main() -> eframe::Result {
    // Application state
    let mut job_app = app::App::default();
    let mut screen_history = vec![app::Screen::Home];
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
            //println!("Screen history: {:?}", screen_history);
            match job_app.requested_screen {
                app::Screen::SelectOrAddListingHost => { 
                    match screen_history.last().clone() {
                        Some(app::Screen::SelectOrAddListingHost) => (),
                        _ => screen_history.push(app::Screen::SelectOrAddListingHost),
                    };
                    job_app.select_or_add_listing_host(ui);
                },
                app::Screen::SelectOrAddListing => { 
                    match screen_history.last().clone() {
                        Some(app::Screen::SelectOrAddListing) => (),
                        _ => screen_history.push(app::Screen::SelectOrAddListing),
                    };
                    job_app.select_or_add_listing(ui);
                },
                app::Screen::AddApplication => { 
                    match screen_history.last().clone() {
                        Some(app::Screen::AddApplication) => (),
                        _ => screen_history.push(app::Screen::AddApplication),
                    };
                    job_app.add_application(ui);
                },
                app::Screen::AddAchievement => { 
                    match screen_history.last().clone() {
                        Some(app::Screen::AddAchievement) => (),
                        _ => screen_history.push(app::Screen::AddAchievement),
                    };
                    job_app.add_achievement(ui);
                },
                app::Screen::SelectApplication => { 
                    match screen_history.last().clone() {
                        Some(app::Screen::SelectApplication) => (),
                        _ => screen_history.push(app::Screen::SelectApplication),
                    };
                    job_app.view_application_query(ui);
                },
                app::Screen::SelectOrAddPostalAddress => { 
                    match screen_history.last().clone() {
                        Some(app::Screen::SelectOrAddPostalAddress) => (),
                        _ => screen_history.push(app::Screen::SelectOrAddPostalAddress),
                    };
                    job_app.select_or_add_postal_address(ui);
                },
                app::Screen::Back => {
                    // Use pattern matching to get last two elements
                    let mut hist = screen_history.iter().rev();
                    let (_from_screen, to_screen)= (hist.next(), hist.next());
                    // Go back to the previous screen if we can
                    match (_from_screen, to_screen) {
                        (_, Some(t)) => {
                            job_app.requested_screen = *t;
                        },
                        _ => {
                            println!("Nothing in history. Resetting");
                            job_app = app::App::default();
                        },
                    };
                    screen_history.pop();
                },
                app::Screen::Home => {// Go home as default
                    match screen_history.last().clone() {
                        Some(app::Screen::Home) => (),
                        _ => {
                            println!("Back at home. Resetting");
                            // Always reset when we get back home
                            job_app = app::App::default();
                            screen_history = vec![app::Screen::Home];
                        },
                    };
                    job_app.home(ui);
                },
                _ => { 
                    // Reset if lost
                    println!("Uh oh. We're lost. Resetting");
                    job_app = app::App::default();
                    screen_history = vec![app::Screen::Home];
                },
            };
        });
    })
}
