use job_application_helper::app;
use job_application_helper::add_experience;
use job_application_helper::experience::Experience;
use job_application_helper::postal_address::PostalAddress;
use job_application_helper::employing_entity::EmployingEntity;
use job_application_helper::application_question_answer::ApplicationQuestionAnswer;
use job_application_helper::achievement::Achievement;
use job_application_helper::application::Application;
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
    let mut question_answer_vec = Vec::<ApplicationQuestionAnswer>::default();
    let mut new_achievement = Achievement::default();
    let mut currently_selected_experience = 0;
    let mut application_query = HashMap::<i32, Application>::default();
    let mut selected_application = 0;
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
                    app::view_application_query(ui, &mut application_query, &mut selected_application);
                },
                Tab::NewApplication => {
                    job_app.add_application(
                        ui,
                        &mut question_answer_vec,
                    );
                    return;
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
