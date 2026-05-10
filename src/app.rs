// Go here for inspiration
// https://www.egui.rs/#demo
// For inspiration and more examples, go to https://emilk.github.io/egui

use std::collections::HashMap;
use crate::postal_address::PostalAddress;
//use crate::contact::Contact;
//use crate::employing_entity::EmployingEntity;
//use crate::experience::Experience;
//use crate::achievement::Achievement;
//use crate::achievement_variant::AchievementVariant;
//use crate::project::Project;
//use crate::project_highlight::ProjectHighlight;
//use crate::project_highlight_variant::ProjectHighlightVariant;
//use crate::listing_host::ListingHost;
//use crate::listing::Listing;
//use crate::application::Application;
//use crate::application_selection::ApplicationSelection;
//use crate::application_question_answer::ApplicationQuestionAnswer;
//use crate::my_text;
//use crate::my_database;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum Screen {
    #[default]
    Home,
    Back,
    SelectOrAddPostalAddress,
}
#[derive(Default)]
pub struct App {
    pub requested_screen: Screen,
    pub postal_address_query: HashMap<i32, PostalAddress>,
    pub new_postal_address: PostalAddress,
    pub selected_postal_address: i32,
}

impl App {
    pub fn home(&mut self, ui: &mut egui::Ui) {
        ui.columns_const(|[col_1, col_2, col_3]| {
            col_1.vertical(|col_1| {
                if col_1.button("Select or Add Address").clicked() {
                    self.requested_screen = Screen::SelectOrAddPostalAddress;
                }
            });
            col_2.vertical(|col_2| {
                col_2.label("Builder");
            });
            col_3.vertical(|col_3| {
                col_3.label("Resume");
            });
        });
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
                    // Reset all but selected address id
                    *self = App{
                        selected_postal_address: self.selected_postal_address, 
                        ..Default::default()
                    };
                    self.requested_screen = Screen::Back; // Go back to previous screen
                }
            });
        });
    }
}

/*
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
   //#[serde(skip)] // This how you opt-out of serialization of a field
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        // CentralPanel should always be last
        egui::CentralPanel::default().show(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.columns_const(|[col_1, col_2, col_3]| {
                col_1.vertical(|col_1| {
                    col_1.label("Description");
                });
                col_2.vertical(|col_2| {
                    col_2.label("Builder");
                });
                col_3.vertical(|col_3| {
                    col_3.label("Resume");
                });
            });
        });
    }
}
*/
