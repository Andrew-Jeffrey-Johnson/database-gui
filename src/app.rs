// Go here for inspiration
// https://www.egui.rs/#demo

mod my_database;
mod my_text;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    description: String,
    summary: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    description_segments: Vec<Vec<my_text::LabelPkg>>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    achievements: Vec<my_text::Achievement>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            description: String::from("Begin typing"),
            summary: String::from("Begin typing"),
            description_segments: Vec::<Vec<my_text::LabelPkg>>::new(),
            achievements: Vec::<my_text::Achievement>::new(),
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
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.columns_const(|[col_1, col_2]| {
                col_1.vertical(|col_1| {
                    col_1.label("Description");
                    if col_1.button("Send Description").clicked() {
                        my_database::create_description(&self.description);
                    }
                    add_original_description(col_1, &mut self.description);
                    col_1.separator();
                    if col_1.button("Segment Description").clicked() {
                        self.description_segments = my_text::segment_description(&self.description);
                    }
                    add_annotated_description(col_1, &self.description_segments);
                });
                col_2.vertical(|col_2| {
                    col_2.label("Resume");
                    if col_2.button("Get Achievements").clicked() {
                        self.achievements = my_text::get_achievements();
                    }
                    for a in &mut self.achievements {
                        col_2.checkbox(&mut a.in_resume, &a.description);
                    }
                   // if col_2.button("Get Application").clicked() {
                   //     self.application = my_text::get_application();
                   // }
                    //add_application(col_2, &mut self.application)
                });
            });
        });
    }
}

fn add_application(ui: &mut egui::Ui, application: &mut String) {
    ui.label("Summary");
    ui.add_enabled(true, egui::TextEdit::multiline(application)
        .desired_rows(10)
        .desired_width(f32::INFINITY)
    );
}

fn add_acronym_label(ui: &mut egui::Ui, acronym: &String, tooltip: &String) {
    ui.label(egui::RichText::new(acronym).color(egui::Color32::RED).underline()).on_hover_text(tooltip);
}
fn add_jargon_label(ui: &mut egui::Ui, jargon: &String, tooltip: &String) {
    ui.label(egui::RichText::new(jargon).color(egui::Color32::BLUE).underline()).on_hover_text(tooltip);
}
fn add_normal_label(ui: &mut egui::Ui, text: &String) {
    ui.label(egui::RichText::new(text));
}

fn add_original_description(ui: &mut egui::Ui, description: &mut String) {
    let horizontal_scroll: bool = false;
    let vertical_scroll: bool = true;
    egui::ScrollArea::new([horizontal_scroll,vertical_scroll])
        .id_salt("Original")
        .auto_shrink(true)
        .max_height(200.0)
        .show(ui, |ui| {
           ui.add_enabled(true, egui::TextEdit::multiline(description)
                .desired_rows(10)
                .desired_width(f32::INFINITY)
            );
        });
}

fn add_annotated_description(ui: &mut egui::Ui, description_segments: &Vec<Vec<my_text::LabelPkg>>) {
    let horizontal_scroll: bool = false;
    let vertical_scroll: bool = true;
    egui::ScrollArea::new([horizontal_scroll,vertical_scroll])
        .id_salt("Annotated")
        .auto_shrink(true)
        .max_height(500.0)
        .show(ui, |ui| {
            let error: String = String::from("ERROR: No Tooltip Found");
            for seg in description_segments {
                ui.horizontal_wrapped(|ui| {
                    // Trick to add spaces
                    let width = ui.fonts_mut(|f|f.glyph_width(&egui::TextStyle::Body.resolve(ui.style()), ' '));
                    ui.spacing_mut().item_spacing.x = width;
                    for cap in seg {
                        match &cap.category {
                            my_text::TextCategory::Jargon => 
                                add_jargon_label(ui, &cap.text, &cap.tooltip.as_ref().unwrap_or_else(|| &error)),
                            my_text::TextCategory::Acronym => 
                                add_acronym_label(ui, &cap.text, &cap.tooltip.as_ref().unwrap_or_else(|| &error)),
                            my_text::TextCategory::Normal => 
                                add_normal_label(ui, &cap.text)
                        }
                    }
                });
            }
        });
}
