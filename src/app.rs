// Go here for inspiration
// https://www.egui.rs/#demo
// For inspiration and more examples, go to https://emilk.github.io/egui

use chrono::TimeZone;
use std::cmp;
use chrono::Utc;
use std::collections::HashMap;
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

pub fn select_application_query
(
    ui: &mut egui::Ui,
    application_query: &Vec<Application>,
    mut selected_application: Application,
) 
    -> Application
{
    egui::Grid::new("view_application_query").show(ui, |ui| {
        ui.label("ID");
        ui.label("Start");
        ui.label("End");
        ui.label("Listing ID");
        ui.end_row();
        for app in application_query {
            ui.radio_value(
                &mut selected_application,
                app.clone(),
                format!("{}", app.id)
            );
            ui.label(format!("{}", app.start_timestamptz));
            ui.label(format!("{}", app.submitted_timestamptz));
            ui.label(format!("{}", app.listing_id));
            ui.end_row();
        }
    });
    return selected_application;
}

fn select_or_add_listing_host
(
    ui: &mut egui::Ui,
    new_listing_host: &mut ListingHost,
    listing_host_query: &mut HashMap<i32, ListingHost>,
) 
{
    egui::Frame::default().stroke(egui::Stroke::new(1.0_f32, egui::Color32::BLACK)).show(ui, |ui| {
        if new_listing_host.id != 0 {
            if let Some(host) = listing_host_query.get(&new_listing_host.id) {
                ui.label(&host.name);
                ui.label(&host.url);
            }
            else {
                ui.label("Oops. Something went wrong.");
            }
            if ui.button("Change Listing Host").clicked() {
                new_listing_host.id = 0;
            }
        }
        else {
            ui.columns_const(|[col_1, col_2]| {
                col_1.vertical(|col_1| {
                    col_1.label("Select Listing Host");
                    if listing_host_query.is_empty() {
                        *listing_host_query = ListingHost::fetch(0, 100);
                    }
                    for (id, listing_host) in listing_host_query {
                        col_1.radio_value(
                            &mut new_listing_host.id, 
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
                    col_2.text_edit_multiline(&mut new_listing_host.name);
                    col_2.label("URL:");
                    col_2.text_edit_multiline(&mut new_listing_host.url);
                    if col_2.button("Confirm Add").clicked() {
                        // Insert into databse
                        new_listing_host.insert_into_db();
                    }
                });
            });
        }
    });
}

fn select_or_add_listing
(
    ui: &mut egui::Ui,
    listing_query: &mut HashMap<i32, Listing>,
    new_listing: &mut Listing,
    new_listing_host: &mut ListingHost,
    listing_host_query: &mut HashMap<i32, ListingHost>,
    posted_year: &mut i32, 
    posted_month: &mut u32,
    posted_day: &mut u32,
) 
{
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Frame::default().stroke(egui::Stroke::new(1.0_f32, egui::Color32::BLACK)).show(ui, |ui| {
            if new_listing.id != 0 {
                if let Some(listing) = listing_query.get(&new_listing.id) {
                    ui.label(format!("{}...", &listing.description[0..std::cmp::min(50, listing.description.len())]));
                    ui.label(format!("{}...", &listing.url[0..std::cmp::min(20, listing.url.len())]));
                }
                else {
                    ui.label("Oops. Something went wrong.");
                }
                if ui.button("Change Listing").clicked() {
                    new_listing.id = 0;
                }
            }
            else {
                ui.columns_const(|[col_1, col_2]| {
                    col_1.vertical(|col_1| {
                        col_1.label("Select Listing");
                        if listing_query.is_empty() {
                            *listing_query = Listing::fetch(0, 100);
                        }
                        for (id, listing) in &*listing_query {
                            col_1.radio_value(
                                &mut new_listing.id, 
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
                        select_or_add_listing_host(col_2, new_listing_host,listing_host_query);
                        col_2.label("Description:");
                        col_2.text_edit_multiline(&mut new_listing.description);
                        col_2.label("URL:");
                        col_2.text_edit_multiline(&mut new_listing.url);
                        col_2.label("Posted Date:");
                        date(col_2, posted_year, posted_month, posted_day, 3);
                        col_2.label("Recruiter Contact (NOT IMPLEMENTED YET)");
                        if col_2.button("Confirm Add").clicked() {
                            // Insert into database
                            new_listing.description = new_listing.description.replace("'", "''");
                            new_listing.posted_timestamptz = Utc.with_ymd_and_hms(
                                *posted_year,
                                *posted_month,
                                *posted_day,
                                0,
                                0,
                                0
                            ).unwrap();
                            new_listing.insert_into_db();
                            listing_query.insert(new_listing.id, new_listing.clone());
                        }
                    });
                });
            }
        });
    });
}

pub fn add_application
(
    ui: &mut egui::Ui, 
    new_application: &mut Application,
    experience_vec: &Vec<Experience>,
    employing_entity_query: &mut HashMap<i32, EmployingEntity>,
    achievement_queries: &mut HashMap<i32, HashMap<i32, Achievement>>,
    qas: &mut Vec<ApplicationQuestionAnswer>,
    new_listing_host: &mut ListingHost,
    listing_host_query: &mut HashMap<i32, ListingHost>,
    listing_host_query_order: &Vec<&ListingHost>,
    posted_year: &mut i32, 
    posted_month: &mut u32,
    posted_day: &mut u32,
    new_listing: &mut Listing,
    listing_query: &mut HashMap<i32, Listing>,
) 
    -> bool
{
    let mut is_submitted = false;
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.columns_const(|[col_1, col_2, col_3]| {
            // application itself
            col_1.vertical(|col_1| {
                col_1.label("New Application");
                if !new_application.is_started {
                    new_application.start_timestamptz = chrono::offset::Utc::now();
                    new_application.is_started = true;
                }
                col_1.label(format!("Started: {}", new_application.start_timestamptz));
                col_1.label("Select or Add Listing");
                select_or_add_listing(
                    col_1, 
                    listing_query,
                    new_listing,
                    new_listing_host,
                    listing_host_query, 
                    posted_year, 
                    posted_month, 
                    posted_day
                );
            });
            // achievements and project highlights
            col_2.vertical(|col_2| {
                col_2.label("Experiences");
                if col_2.button("Add Experience").clicked() {
                    println!("This button does nothing");
                }
                // Display all achievements from all experiences
                for experience in experience_vec {
                    // Employing Entity
                    if !employing_entity_query.contains_key(&experience.employing_entity_id) {
                        // Get a bunch at a time. Not just one at a time
                        let new_query = EmployingEntity::fetch(
                            cmp::max(experience.employing_entity_id-10, 0), // Lower cap = 0
                            experience.employing_entity_id+10 // no upper cap
                        );
                        employing_entity_query.extend(new_query.into_iter());
                    }
                    let employing_entity = employing_entity_query.get(&experience.employing_entity_id);
                    // Achievements
                    if !achievement_queries.contains_key(&experience.id) {
                        let new_hashmap = Achievement::fetch_using_experience(0, 100, experience.id);
                        achievement_queries.insert(experience.id, new_hashmap);
                    }
                    col_2.label(format!("[ID: {}] Title: {}", experience.id, experience.title));
                    match employing_entity {
                        None => col_2.label("No Employing Entity Found"),
                        Some(e) => col_2.label(format!("Employing Entity: {}", e.name))
                    };
                    for (_a_id, a) in achievement_queries.get_mut(&experience.id).unwrap() {
                        col_2.checkbox(&mut a.is_selected, &a.short_description);
                    }
                }
                col_2.label("Projects");
            });
            // buttons and questions
            col_3.vertical(|col_3| {
                if col_3.button("Generate PDF").clicked() {
                    let summary = String::from("Security-focused software engineer with a Master of Engineering in Computer Science and over a year of work
experience in software engineering, security, web development, and databases.");
                    new_application.resume = my_text::latex_gen(&summary, &*experience_vec, &employing_entity_query, &achievement_queries);
                }
                if col_3.button("Submit").clicked() {
                    let summary = String::from("Security-focused software engineer with a Master of Engineering in Computer Science and over a year of work
experience in software engineering, security, web development, and databases.");
                    new_application.resume = my_text::latex_gen(&summary, &*experience_vec, &employing_entity_query, &achievement_queries);
                    new_application.submitted_timestamptz = chrono::offset::Utc::now();
                    new_application.listing_id = new_listing.id;
                    // Insert into databse
                    new_application.insert_into_db();
                    // Insert all questions and answers
                    for qa in &mut *qas {
                        qa.question = qa.question.replace("'", "''");
                        qa.answer = qa.answer.replace("'", "''");
                        qa.application_id = new_application.id;
                        qa.insert_into_db();
                    }
                    is_submitted = true;
                }
                add_question_answer(col_3, qas);
            });
        });
    });
    return is_submitted;
}

