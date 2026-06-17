use job_application_helper::app;
use job_application_helper::add_experience;
use job_application_helper::experience::Experience;
use job_application_helper::postal_address::PostalAddress;
use job_application_helper::employing_entity::EmployingEntity;
use job_application_helper::application_question_answer::ApplicationQuestionAnswer;
use job_application_helper::achievement::Achievement;
use job_application_helper::application::Application;
use job_application_helper::listing_host::ListingHost;
use job_application_helper::listing::Listing;

use std::collections::HashMap;

#[derive(PartialEq, Default)]
enum Tab {
    #[default]
    Home,
    NewApplication,
    ViewApplications,
    NewExperience,
    NewAchievement,
}

fn main() -> eframe::Result {
    // gui state
    let mut ex = Experience::default();
    let mut exp = PostalAddress::default();
    let mut start_year: i32 = 0;
    let mut start_month: u32 = 0;
    let mut start_day: u32 = 0;
    let mut end_year: i32 = 0;
    let mut end_month: u32 = 0;
    let mut end_day: u32 = 0;
    let mut posted_year: i32 = 0;
    let mut posted_month: u32 = 0;
    let mut posted_day: u32 = 0;
    let mut ee = EmployingEntity::default();
    let mut eep = PostalAddress::default();
    let mut eeq = HashMap::<i32, EmployingEntity>::default();
    let mut pq = HashMap::<i32, PostalAddress>::default();
    let mut new_application = Application::default();
    // Get experiences (first 100) and sort them in reverse chronological order
    let mut experience_query = Experience::fetch(0, 100);
    let mut experience_vec: Vec<_> = experience_query.clone().into_iter().map(|(_k, v)| v).collect();
    experience_vec.sort_by(|a, b| b.start_timestamptz.cmp(&a.start_timestamptz)); 
    // For each experience, fetch all the achievements
    let mut achievement_queries = HashMap::<i32,Vec<Achievement>>::default();
    for experience in &experience_vec {
        let new_hashmap = Achievement::fetch_using_experience(0, 1000, experience.id);
        let mut new_vec: Vec<_> = new_hashmap.clone().into_iter().map(|(_k, v)| v).collect();
        new_vec.sort_by(|a, b| a.id.cmp(&b.id));
        achievement_queries.insert(experience.id, new_vec);
    }
    let mut employing_entity_query = HashMap::<i32, EmployingEntity>::default();
    let mut question_answer_vec = Vec::<ApplicationQuestionAnswer>::default();
    let mut new_achievement = Achievement::default();
    let mut currently_selected_experience = 0;
    // Get previous applications (first 100) and sort them by submitted date
    let application_query = Application::fetch(0,100);
    let mut application_vec: Vec<_> = application_query.clone().into_iter().map(|(_k, v)| v).collect();
    application_vec.sort_by(|a, b| a.submitted_timestamptz.cmp(&b.submitted_timestamptz));
    let mut selected_application = new_application.clone();
    let mut new_listing_host = ListingHost::default();
    let mut listing_host_query = ListingHost::fetch(0,100);
    let mut listing_host_sorted = Vec::<&ListingHost>::default();
    let mut new_listing = Listing::default();
    let mut listing_query = HashMap::<i32, Listing>::default();
    let mut current_tab = Tab::Home;
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
            // Tab menu
            ui.horizontal(|ui| {
                ui.radio_value(&mut current_tab, Tab::Home, "Home");
                ui.radio_value(&mut current_tab, Tab::ViewApplications, "View Applications");
                ui.radio_value(&mut current_tab, Tab::NewApplication, "New Application");
                ui.radio_value(&mut current_tab, Tab::NewExperience, "New Experience");
                ui.radio_value(&mut current_tab, Tab::NewAchievement, "New Achievement");
            });
            match current_tab {
                Tab::ViewApplications => {
                    selected_application = app::select_application_query(ui, &application_vec, selected_application.clone());
                },
                Tab::NewApplication => {
                    let submitted = app::add_application(
                        ui,
                        &mut new_application,
                        &experience_vec,
                        &mut employing_entity_query,
                        &mut achievement_queries,
                        &mut question_answer_vec,
                        &mut new_listing_host,
                        &mut listing_host_query,
                        &mut listing_host_sorted,
                        &mut posted_year,
                        &mut posted_month,
                        &mut posted_day,
                        &mut new_listing,
                        &mut listing_query,
                    );
                    if submitted {
                        new_application = Default::default();
                        experience_query = Experience::fetch(0, 100);
                        experience_vec = experience_query.clone().into_iter().map(|(_k, v)| v).collect();
                        experience_vec.sort_by(|a, b| b.start_timestamptz.cmp(&a.start_timestamptz)); 
                        employing_entity_query = Default::default();
                        achievement_queries = Default::default();
                        question_answer_vec = Default::default();
                        new_listing_host = Default::default();
                        listing_host_query = Default::default();
                        listing_host_sorted = Default::default();
                        posted_year = Default::default();
                        posted_month = Default::default();
                        posted_day = Default::default();
                        new_listing = Default::default();
                        listing_query = Default::default();
                    }
                },
                Tab::NewExperience => {
                    let _is_done = add_experience::new_experience(
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
                },
                Tab::NewAchievement => {
                    app::add_achievement(ui, &mut new_achievement, currently_selected_experience);
                },
                _ => {
                    ui.label("Welcome home!");
                },
            };
        });
    })
}

