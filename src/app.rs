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
//use crate::application_question_answer::ApplicationQuestionAnswer;
use crate::my_text;
//use crate::my_database;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum Screen {
    #[default]
    Home,
    Back,
    SelectOrAddPostalAddress,
    SelectApplication,
    AddApplication,
    AddAchievement,
    SelectOrAddListing,
    SelectOrAddListingHost,
}
#[derive(Default)]
pub struct App {
    // App itself
    pub requested_screen: Screen,
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

impl App {
    // year, month, day
    fn date(
        ui: &mut egui::Ui, 
        year: &mut i32, 
        month: &mut u32, 
        day: &mut u32) 
    {
        ui.label("Year:");
        let mut tmp_year = format!("{}", year);
        ui.text_edit_singleline(&mut tmp_year);
        if let Ok(result) = tmp_year.parse() {
            *year = result;
        }
        egui::ComboBox::from_label("Month")
            .selected_text(format!("{:?}", month))
            .show_ui(ui, |ui| {
                ui.selectable_value(month, 0, "January");
                ui.selectable_value(month, 1, "February");
                ui.selectable_value(month, 2, "March");
                ui.selectable_value(month, 3, "April");
                ui.selectable_value(month, 4, "May");
                ui.selectable_value(month, 5, "June");
                ui.selectable_value(month, 6, "July");
                ui.selectable_value(month, 7, "August");
                ui.selectable_value(month, 8, "September");
                ui.selectable_value(month, 9, "October");
                ui.selectable_value(month, 10, "November");
                ui.selectable_value(month, 11, "December");
            }
        );
        ui.label("Day:");
        let mut tmp_day = format!("{}", day);
        ui.text_edit_singleline(&mut tmp_day);
        if let Ok(result) = tmp_day.parse() {
            *day = result;
        }
    }

    pub fn home(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[col_1, col_2, col_3]| {
            col_1.vertical(|col_1| {
                if col_1.button("Select or Add Address").clicked() {
                    self.requested_screen = Screen::SelectOrAddPostalAddress;
                }
            });
            col_2.vertical(|col_2| {
                col_2.label("Applications");
                if col_2.button("View Applications").clicked() {
                    self.requested_screen = Screen::SelectApplication;
                }
                if col_2.button("New Application").clicked() {
                    self.requested_screen = Screen::AddApplication;
                }
            });
            col_3.vertical(|col_3| {
                col_3.label("Resume");
            });
        });
    }

    pub fn select_or_add_listing_host(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[col_1, col_2]| {
            col_1.vertical(|col_1| {
                col_1.label("Select Listing Host");
                if self.listing_host_query.is_empty() {
                    self.listing_host_query = ListingHost::fetch(0, 100);
                }
                for (id, listing_host) in &mut self.listing_host_query {
                    col_1.radio_value(
                        &mut self.selected_listing, 
                        *id,
                        format!("[id: {}]", id)
                    );
                    col_1.label(format!("{}, {}", 
                        listing_host.name, 
                        listing_host.url));
                }
                if col_1.button("Confirm Selection").clicked() {
                    self.requested_screen = Screen::Back; // Go back to previous screen
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
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
            });
        });
    }

    pub fn select_or_add_listing(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[col_1, col_2]| {
            col_1.vertical(|col_1| {
                col_1.label("Select Listing");
                if self.listing_query.is_empty() {
                    self.listing_query = Listing::fetch(0, 100);
                }
                for (id, listing) in &mut self.listing_query {
                    col_1.radio_value(
                        &mut self.selected_listing, 
                        *id,
                        format!("[id: {}]", id)
                    );
                    col_1.label(format!("{}, {}, {}, {}", 
                        listing.listing_host_id, 
                        listing.description, 
                        listing.posted_timestamptz,
                        listing.recruiter_contact_id));
                }
                if col_1.button("Confirm Selection").clicked() {
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
            });
            col_2.vertical(|col_2| {
                col_2.label("Add Listing");
                col_2.label("Listing Host");
                if col_2.button("Select or Add Listing Host").clicked() {
                    self.requested_screen = Screen::SelectOrAddListingHost;
                }
                col_2.label("Description:");
                col_2.text_edit_multiline(&mut self.new_listing.description);
                col_2.label("Posted Date:");
                Self::date(col_2, &mut self.posted_year, &mut self.posted_month, &mut self.posted_day);
                col_2.label("Recruiter Contact (NOT IMPLEMENTED YET)");
                if col_2.button("Confirm Add").clicked() {
                    // Insert into databse
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
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
            });
        });
    }

    pub fn add_achievement(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Currently Selected Experience: {}", self.currently_selected_experience));
        ui.label("New Achievement");
        ui.label("Short Description");
        ui.text_edit_singleline(&mut self.new_achievement.short_description);
        ui.label("Defense");
        ui.text_edit_singleline(&mut self.new_achievement.defense);
        if ui.button("Confirm").clicked() {
            self.new_achievement.experience_id = self.currently_selected_experience;
            self.new_achievement.insert_into_db();
            self.new_achievement = Default::default();
            self.requested_screen = Screen::Back;
        }
        if ui.button("Cancel").clicked() {
            self.requested_screen = Screen::Back;
        }
    }
    pub fn add_application(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[col_1, col_2, col_3, col_4]| {
            // application itself
            col_1.vertical(|col_1| {
                col_1.label("New Application");
                if !self.new_application.is_started {
                    self.new_application.start_timestamptz = chrono::offset::Utc::now();
                    self.new_application.is_started = true;
                }
                col_1.label(format!("Started: {}", self.new_application.start_timestamptz));
                col_1.label(format!("Listing ID: {}", self.selected_listing));
                if col_1.button("Select or Add Listing").clicked() {
                    self.requested_screen = Screen::SelectOrAddListing;
                }
            });
            // achievements and project highlights
            col_2.vertical(|col_2| {
                col_2.label("Experiences");
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
                    if col_2.button("Add Achievement").clicked() {
                        self.currently_selected_experience = *id;
                        self.requested_screen = Screen::AddAchievement;
                        self.employing_entity_query = Default::default();
                        self.achievement_queries = Default::default();
                        return;
                    }
                }
                col_2.label("Projects");
            });
            // Resume preview
            col_3.vertical(|col_3| {
                col_3.label("Resume Preview TODO");
                for (id, experience) in &mut self.experience_query {
                    let mut has_achievements = false;
                    let maybe_achievements = self.achievement_queries.get_mut(id);
                    if let Some(achievements) = maybe_achievements {
                        for (_a_id, a) in achievements {
                            if a.is_selected {
                                has_achievements = true;
                                break;
                            }
                        }
                    }
                    if has_achievements {
                        col_3.label(format!("{}, {}", experience.employing_entity_id, experience.title));
                        for (_a_id, a) in self.achievement_queries.get_mut(id).unwrap() {
                            if a.is_selected {
                                col_3.label(&a.short_description);
                            }
                        }
                    }
                }
            });
            // Done
            col_4.vertical(|col_4| {
                if col_4.button("Generate PDF").clicked() {
                    my_text::latex_gen(&"This is a summary".to_string(), self);
                }
                if col_4.button("Submit").clicked() {
                    self.new_application.submitted_timestamptz = chrono::offset::Utc::now();
                    self.new_application.listing_id = self.selected_listing;
                    // Insert into databse
                    self.new_application.insert_into_db();
                    self.selected_application = self.new_application.id;
                    // Reset all but selected application id
                    *self = App{
                        selected_application: self.selected_application, 
                        ..Default::default()
                    };
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
                if col_4.button("Cancel").clicked() {
                    // Reset everything
                    *self = Default::default();
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
            });
        });
    }

    pub fn view_application_query(&mut self, ui: &mut egui::Ui) {
        if self.application_query.is_empty() {
            self.application_query = Application::fetch(0, 100);
        }
        ui.columns_const(|[col_1, col_2, col_3, col_4]| {
            col_1.vertical(|col_1| {
                col_1.label("ID");
                for (id, _app) in &mut self.application_query {
                    col_1.radio_value(
                        &mut self.selected_application,
                        *id,
                        format!("{}", id)
                    );
                }
            });
            col_2.vertical(|col_2| {
                col_2.label("Start");
                for (_id, app) in &self.application_query {
                    col_2.label(format!("{}", app.start_timestamptz));
                }
            });
            col_3.vertical(|col_3| {
                col_3.label("End");
                for (_id, app) in &self.application_query {
                    col_3.label(format!("{}", app.submitted_timestamptz));
                }
            });
            col_4.vertical(|col_4| {
                col_4.label("Listing ID");
                for (_id, app) in &self.application_query {
                    col_4.label(format!("{}", app.listing_id));
                }
            });
        });
        if ui.button("Back").clicked() {
            self.requested_screen = Screen::Back;
        }
    }
    
    pub fn select_or_add_postal_address(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[col_1, col_2]| {
            col_1.vertical(|col_1| {
                col_1.label("Select Address");
                if self.postal_address_query.is_empty() {
                    self.postal_address_query = PostalAddress::fetch(0, 100);
                }
                for (id, addr) in &mut self.postal_address_query {
                    col_1.radio_value(
                        &mut self.selected_postal_address, 
                        *id,
                        format!("[id: {}]", id)
                    );
                    col_1.label(format!("{}, {}, {}", addr.city, addr.state, addr.zip_code));
                }
                if col_1.button("Confirm Selection").clicked() {
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
            });
            col_2.vertical(|col_2| {
                col_2.label("Enter New Address");
                col_2.label("Name:");
                col_2.text_edit_singleline(&mut self.new_postal_address.name);
                col_2.label("Line 1:");
                col_2.text_edit_singleline(&mut self.new_postal_address.line_1);
                col_2.label("Line 2:");
                col_2.text_edit_singleline(&mut self.new_postal_address.line_2);
                col_2.label("Line 3:");
                col_2.text_edit_singleline(&mut self.new_postal_address.line_3);
                col_2.label("City:");
                col_2.text_edit_singleline(&mut self.new_postal_address.city);
                col_2.label("State:");
                col_2.text_edit_singleline(&mut self.new_postal_address.state);
                col_2.label("Zip Code:");
                col_2.text_edit_singleline(&mut self.new_postal_address.zip_code);
                if col_2.button("Confirm Add").clicked() {
                    // Insert into databse
                    self.new_postal_address.insert_into_db();
                    self.selected_postal_address = self.new_postal_address.id;
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
            });
        });
    }
}

