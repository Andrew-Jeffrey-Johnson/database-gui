// Go here for inspiration
// https://www.egui.rs/#demo
// For inspiration and more examples, go to https://emilk.github.io/egui

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
use egui::cache::{ComputerMut, FrameCache};

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
        let min: i32 = 0;
        let max: i32 = 100;
        let cache = &mut ctx.memory_mut().caches;
        let my_cache: &mut FrameCache<(i32, i32), Vec::<PostalAddress>> =
            cache.cache("frame_cache");
        let addresses = my_cache.compute((min, max));
        ui.label("It runs");
        for addr in addresses {
            ui.label(addr.city)
        }
        //ui.text_edit_singleline
    }
}

