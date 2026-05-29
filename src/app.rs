// Go here for inspiration
// https://www.egui.rs/#demo
// For inspiration and more examples, go to https://emilk.github.io/egui

use chrono::TimeZone;
use std::cmp;
use chrono::Utc;
use std::collections::HashMap;
use crate::postal_address::PostalAddress;
//use crate::contact::Contact;
use crate::employing_entity::EmployingEntity;
use crate::experience::Experience;
use crate::achievement::Achievement;
//use crate::achievement_variant::AchievementVariant;
//use crate::project::Project;
//use crate::project_highlight::ProjectHighlight;
//use crate::project_highlight_variant::ProjectHighlightVariant;
use crate::listing_host::ListingHost;
use crate::listing::Listing;
use crate::application::Application;
//use crate::application_selection::ApplicationSelection;
use crate::application_question_answer::ApplicationQuestionAnswer;
use crate::my_text;
//use crate::my_database;
use crate::shared_functions::date;

#[derive(Default)]
pub struct App {
    // PostalAddress
    pub postal_address_query: HashMap<i32, PostalAddress>,
    pub new_postal_address: PostalAddress,
    pub selected_postal_address: i32,
    // Application
    pub application_query: HashMap<i32, Application>,
    pub selected_application: i32,
    pub new_application: Application,
    // Listing
    pub listing_query: HashMap<i32, Listing>,
    pub new_listing: Listing,
    pub selected_listing: i32,
    posted_year: i32,
    posted_month: u32,
    posted_day: u32,
    // ListingHost
    pub listing_host_query: HashMap<i32, ListingHost>,
    pub new_listing_host: ListingHost,
    pub selected_listing_host: i32,
    // EmployingEntity
    pub employing_entity_query: HashMap<i32, EmployingEntity>,
    pub new_employing_entity: EmployingEntity,
    // Experience
    pub experience_query: HashMap<i32, Experience>,
    pub new_experience: Experience,
    pub currently_selected_experience: i32,
    // Achievement
    // hashmap of hashmaps indexed by experience_id
    pub achievement_queries: HashMap<i32, HashMap<i32, Achievement>>,
    pub new_achievement: Achievement,
    // Project
    //pub project_query: HashMap<i32, Project>,
    //pub new_project: Project,
}

fn add_question_answer
(
    ui: &mut egui::Ui, 
    qas: &mut Vec<ApplicationQuestionAnswer>,
) 
{
    for (id, qa) in qas.iter_mut().enumerate() {
        ui.label(format!("Question {}:", id));
        ui.text_edit_multiline(&mut qa.question);
        ui.label(format!("Answer {}:", id));
        ui.text_edit_multiline(&mut qa.answer);
    }
    if ui.button("New Question").clicked() {
        qas.push(ApplicationQuestionAnswer::default());
    }
}

pub fn add_achievement
(
    ui: &mut egui::Ui,
    new_achievement: &mut Achievement,
    currently_selected_experience: i32,
) 
{
    ui.label(format!("Currently Selected Experience: {}", currently_selected_experience));
    ui.label("New Achievement");
    ui.label("Short Description");
    ui.text_edit_singleline(&mut new_achievement.short_description);
    ui.label("Defense");
    ui.text_edit_singleline(&mut new_achievement.defense);
    if ui.button("Confirm").clicked() {
        new_achievement.short_description = new_achievement.short_description.replace("'", "''");
        new_achievement.defense = new_achievement.defense.replace("'", "''");
        new_achievement.short_description = new_achievement.short_description.replace("&", r"\&");
        new_achievement.defense = new_achievement.defense.replace("&", r"\&");
        new_achievement.experience_id = currently_selected_experience;
        new_achievement.insert_into_db();
        *new_achievement = Default::default();
    }
}

pub fn view_application_query
(
    ui: &mut egui::Ui,
    application_query: &mut HashMap<i32, Application>,
    selected_application: &mut i32,
) 
{
    if application_query.is_empty() {
        *application_query = Application::fetch(0, 100);
    }
    egui::Grid::new("view_application_query").show(ui, |ui| {
        ui.label("ID");
        ui.label("Start");
        ui.label("End");
        ui.label("Listing ID");
        ui.end_row();
        for (id, app) in & *application_query {
            ui.radio_value(
                selected_application,
                *id,
                format!("{}", id)
            );
            ui.label(format!("{}", app.start_timestamptz));
            ui.label(format!("{}", app.submitted_timestamptz));
            ui.label(format!("{}", app.listing_id));
            ui.end_row();
        }
    });
}

impl App {
    fn select_or_add_listing_host(&mut self, ui: &mut egui::Ui) {
        egui::Frame::default().stroke(egui::Stroke::new(1.0_f32, egui::Color32::BLACK)).show(ui, |ui| {
            if self.new_listing_host.id != 0 {
                if let Some(host) = self.listing_host_query.get(&self.new_listing_host.id) {
                    ui.label(&host.name);
                    ui.label(&host.url);
                }
                else {
                    ui.label("Oops. Something went wrong.");
                }
                if ui.button("Change Listing Host").clicked() {
                    self.new_listing_host.id = 0;
                }
            }
            else {
                ui.columns_const(|[col_1, col_2]| {
                    col_1.vertical(|col_1| {
                        col_1.label("Select Listing Host");
                        if self.listing_host_query.is_empty() {
                            self.listing_host_query = ListingHost::fetch(0, 100);
                        }
                        for (id, listing_host) in &mut self.listing_host_query {
                            col_1.radio_value(
                                &mut self.new_listing_host.id, 
                                *id,
                                format!("[id: {}]", id)
                            );
                            col_1.label(format!("{}, {}", 
                                listing_host.name, 
                                listing_host.url));
                        }
                    });
                    col_2.vertical(|col_2| {
                        col_2.label("Add Listing Host");
                        col_2.label("Name:");
                        col_2.text_edit_multiline(&mut self.new_listing_host.name);
                        col_2.label("URL:");
                        col_2.text_edit_multiline(&mut self.new_listing_host.url);
                        if col_2.button("Confirm Add").clicked() {
                            // Insert into databse
                            self.new_listing_host.insert_into_db();
                            self.selected_listing_host = self.new_listing_host.id;
                        }
                    });
                });
            }
        });
    }

    fn select_or_add_listing(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::default().stroke(egui::Stroke::new(1.0_f32, egui::Color32::BLACK)).show(ui, |ui| {
                if self.new_listing.id != 0 {
                    if let Some(listing) = self.listing_query.get(&self.new_listing.id) {
                        ui.label(format!("{}...", &listing.description[0..std::cmp::min(50, listing.description.len())]));
                        ui.label(format!("{}...", &listing.url[0..std::cmp::min(20, listing.url.len())]));
                    }
                    else {
                        ui.label("Oops. Something went wrong.");
                    }
                    if ui.button("Change Listing").clicked() {
                        self.new_listing.id = 0;
                    }
                }
                else {
                    ui.columns_const(|[col_1, col_2]| {
                        col_1.vertical(|col_1| {
                            col_1.label("Select Listing");
                            if self.listing_query.is_empty() {
                                self.listing_query = Listing::fetch(0, 100);
                            }
                            for (id, listing) in &mut self.listing_query {
                                col_1.radio_value(
                                    &mut self.new_listing.id, 
                                    *id,
                                    format!("[id: {}]", id)
                                );
                                col_1.label(format!("{}, {}, {}, {}", 
                                    listing.listing_host_id, 
                                    &listing.description[0..std::cmp::min(50, listing.description.len())], 
                                    listing.posted_timestamptz,
                                    listing.recruiter_contact_id));
                            }
                        });
                        col_2.vertical(|col_2| {
                            col_2.label("Add Listing");
                            self.select_or_add_listing_host(col_2);
                            col_2.label("Description:");
                            col_2.text_edit_multiline(&mut self.new_listing.description);
                            col_2.label("URL:");
                            col_2.text_edit_multiline(&mut self.new_listing.url);
                            col_2.label("Posted Date:");
                            date(col_2, &mut self.posted_year, &mut self.posted_month, &mut self.posted_day, 3);
                            col_2.label("Recruiter Contact (NOT IMPLEMENTED YET)");
                            if col_2.button("Confirm Add").clicked() {
                                // Insert into database
                                self.new_listing.description = self.new_listing.description.replace("'", "''");
                                self.new_listing.posted_timestamptz = Utc.with_ymd_and_hms(
                                    self.posted_year,
                                    self.posted_month,
                                    self.posted_day,
                                    0,
                                    0,
                                    0
                                ).unwrap();
                                self.new_listing.insert_into_db();
                                self.selected_listing = self.new_listing.id;
                                self.listing_query.insert(self.new_listing.id, self.new_listing.clone());
                            }
                        });
                    });
                }
            });
        });
    }

    pub fn add_application
    (
        &mut self, 
        ui: &mut egui::Ui, 
        qas: &mut Vec<ApplicationQuestionAnswer>,
    ) 
    {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.columns_const(|[col_1, col_2, col_3]| {
                // application itself
                col_1.vertical(|col_1| {
                    col_1.label("New Application");
                    if !self.new_application.is_started {
                        self.new_application.start_timestamptz = chrono::offset::Utc::now();
                        self.new_application.is_started = true;
                    }
                    col_1.label(format!("Started: {}", self.new_application.start_timestamptz));
                    col_1.label("Select or Add Listing");
                    self.select_or_add_listing(col_1);
                });
                // achievements and project highlights
                col_2.vertical(|col_2| {
                    col_2.label("Experiences");
                    if col_2.button("Add Experience").clicked() {
                        println!("This button does nothing");
                    }
                    if self.experience_query.is_empty() {
                        self.experience_query = Experience::fetch(0, 100);
                    }
                    // Display all achievements from all experiences
                    for (id, experience) in &mut self.experience_query {
                        // Employing Entity
                        if !self.employing_entity_query.contains_key(&experience.employing_entity_id) {
                            // Get a bunch at a time. Not just one at a time
                            let new_query = EmployingEntity::fetch(
                                cmp::max(experience.employing_entity_id-10, 0), // Lower cap = 0
                                experience.employing_entity_id+10 // no upper cap
                            );
                            self.employing_entity_query.extend(new_query.into_iter());
                        }
                        let employing_entity = self.employing_entity_query.get(&experience.employing_entity_id);
                        // Achievements
                        if !self.achievement_queries.contains_key(id) {
                            let new_hashmap = Achievement::fetch_using_experience(0, 100, *id);
                            self.achievement_queries.insert(*id, new_hashmap);
                        }
                        col_2.label(format!("[ID: {}] Title: {}", *id, experience.title));
                        match employing_entity {
                            None => col_2.label("No Employing Entity Found"),
                            Some(e) => col_2.label(format!("Employing Entity: {}", e.name))
                        };
                        for (_a_id, a) in self.achievement_queries.get_mut(id).unwrap() {
                            col_2.checkbox(&mut a.is_selected, &a.short_description);
                        }
                    }
                    col_2.label("Projects");
                });
                // Done and questions
                col_3.vertical(|col_3| {
                    if col_3.button("Generate PDF").clicked() {
                        let summary = String::from("Security-focused software engineer with a Master of Engineering in Computer Science and over a year of work
    experience in software engineering, security, web development, and databases.");
                        self.new_application.resume = my_text::latex_gen(&summary, self);
                    }
                    if col_3.button("Submit").clicked() {
                        let summary = String::from("Security-focused software engineer with a Master of Engineering in Computer Science and over a year of work
    experience in software engineering, security, web development, and databases.");
                        self.new_application.resume = my_text::latex_gen(&summary, self);
                        self.new_application.submitted_timestamptz = chrono::offset::Utc::now();
                        self.new_application.listing_id = self.new_listing.id;
                        // Insert into databse
                        self.new_application.insert_into_db();
                        self.selected_application = self.new_application.id;
                        // Insert all questions and answers
                        for qa in &mut *qas {
                            qa.application_id = self.new_application.id;
                            qa.insert_into_db();
                        }
                        // Reset all but selected application id
                        *self = App{
                            selected_application: self.selected_application, 
                            ..Default::default()
                        };
                        println!("This is the part where you turn around");
                    }
                    if col_3.button("Cancel").clicked() {
                        // Reset everything
                        *self = Default::default();
                        println!("This is the part where you turn around");
                    }
                    add_question_answer(col_3, qas);
                });
            });
        });
    }
}

