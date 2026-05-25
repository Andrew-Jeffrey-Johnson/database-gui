use job_application_helper::app;
use job_application_helper::add_experience;
use job_application_helper::experience::Experience;
use job_application_helper::postal_address::PostalAddress;
use job_application_helper::employing_entity::EmployingEntity;
use std::collections::HashMap;

fn main() -> eframe::Result {
    // Application state
    let mut job_app = app::App::default();
    let mut ex = Experience::default();
    let mut exp = PostalAddress::default();
    let mut start_year: i32 = 0;
    let mut start_month: u32 = 0;
    let mut start_day: u32 = 0;
    let mut end_year: i32 = 0;
    let mut end_month: u32 = 0;
    let mut end_day: u32 = 0;
    let mut ee = EmployingEntity::default();
    let mut eep = PostalAddress::default();
    let mut eeq = HashMap::<i32, EmployingEntity>::default();
    let mut pq = HashMap::<i32, PostalAddress>::default();
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
            //println!("Requested Screen: {:?}", job_app.requested_screen);
            match job_app.requested_screen {
                app::Screen::AddExperience => { 
                    let is_done = add_experience::new_experience(
                        ui,
                        &mut ex,
                        &mut exp,
                        &mut start_year, &mut start_month, &mut start_day,
                        &mut end_year, &mut end_month, &mut end_day,
                        &mut ee,
                        &mut eep,
                        &mut eeq,
                        &mut pq
                    );
                    if is_done {
                        job_app.requested_screen = app::Screen::Home;
                        job_app.home(ui);
                    }

                },
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
