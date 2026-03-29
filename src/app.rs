// Go here for inspiration
// https://www.egui.rs/#demo

mod my_database;
mod my_text;

enum ScreenView {
    Primary,
    AddingAchievement,
    AddingAchievementVariant,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_screen_view: ScreenView,
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_experience: usize,
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_achievement: usize,
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_achievement_variant: usize,
    #[serde(skip)] // This how you opt-out of serialization of a field
    experiences: Vec<my_text::Experience>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    new_achievement: my_text::Achievement,
    description: String,
    summary: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    description_segments: Vec<Vec<my_text::LabelPkg>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            current_screen_view: ScreenView::Primary,
            current_experience: 0,
            current_achievement: 0,
            current_achievement_variant: 0,
            experiences: Vec::<my_text::Experience>::new(),
            new_achievement: my_text::Achievement {
                short_description: String::from(""),
                defense: String::from(""),
                variants: vec![my_text::AchievementVariant {
                    id: 0,
                    description: String::from(""),
                    defense: String::from(""),
                }],
                in_resume: false,
                selected_variant: 0,
            },
            description: String::from("Begin typing"),
            summary: String::from("Begin typing"),
            description_segments: Vec::<Vec<my_text::LabelPkg>>::new(),
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

    fn add_achievement(&mut self, ui: &mut egui::Ui, e_index: usize, a_index: usize) {
        let a = &mut self.experiences[e_index].achievements[a_index];
        let id = ui.next_auto_id().with(format!("{}", &a.short_description));
        let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
        state.set_open(a.in_resume);
        state.show_header(ui, |ui| {
            ui.checkbox(&mut a.in_resume, &a.short_description).on_hover_ui(|ui| {
                ui.label(&a.defense);
            });
        }).body(|ui| {
            for v in &mut a.variants {
                ui.radio_value(&mut a.selected_variant, v.id, &v.description).on_hover_text(&v.defense);
            }
            if ui.button("Add Achivement Variant").clicked() {
                self.current_screen_view = ScreenView::AddingAchievementVariant;
                self.current_experience = e_index;
                self.current_achievement = a_index;
            }
        });
    }

    fn add_experience(&mut self, ui: &mut egui::Ui, e_index: usize) {
        ui.scope(|ui| {
            let e = &self.experiences[e_index];
            ui.style_mut().interaction.tooltip_delay = 0.0;
            ui.style_mut().interaction.show_tooltips_only_when_still = false;
            ui.label(e.get_company());
            ui.label(e.get_address());
            ui.label(format!("{} - {}", e.get_start(), e.get_end()));
            for a_index in 0..e.achievements.len() {
                self.add_achievement(ui, e_index, a_index as usize);
            }
            if ui.button("Add Achivement").clicked() {
                self.current_screen_view = ScreenView::AddingAchievement;
                self.current_experience = e_index;
            }
        });
    }

    fn view_primary(&mut self, ctx: &egui::Context) {
        // CentralPanel should always be last
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.columns_const(|[col_1, col_2, col_3]| {
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
                    if col_2.button("Get Experiences").clicked() {
                        self.experiences = my_text::get_experiences();
                    }
                    for e_index in 0..self.experiences.len() {
                        self.add_experience(col_2, e_index);
                    }
                });
                col_3.vertical(|col_3| {
                    for e in &mut self.experiences {
                        show_resume_content(col_3, e);
                    }
                });
            });
        });
    }

    fn view_adding_achivement(&mut self, ctx: &egui::Context) {
        // CentralPanel should always be last
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Adding Achivement");
            // Show experience as heading
            let e = &mut self.experiences[self.current_experience];
            ui.style_mut().override_text_style = Some(egui::style::TextStyle::Heading);
            ui.label(&e.company);
            ui.style_mut().override_text_style = Some(egui::style::TextStyle::Body);
            ui.label(e.get_address());
            ui.label(format!("{} - {}", e.get_start(), e.get_end()));
            // Form to add achievement
            ui.label("Short Description");
            ui.text_edit_singleline(&mut self.new_achievement.short_description);
            ui.label("Defense");
            ui.text_edit_singleline(&mut self.new_achievement.defense);
            ui.label("Variant");
            ui.label("Description");
            ui.text_edit_singleline(&mut self.new_achievement.variants[0].description);
            ui.label("Defense");
            ui.text_edit_singleline(&mut self.new_achievement.variants[0].defense);
            if ui.button("Confirm Add").clicked() {
                self.new_achievement.in_resume = true;
                e.achievements.push(self.new_achievement.clone());
                self.new_achievement = my_text::Achievement {
                    short_description: String::from(""),
                    defense: String::from(""),
                    variants: vec![my_text::AchievementVariant {
                        id: 0,
                        description: String::from(""),
                        defense: String::from(""),
                    }],
                    in_resume: false,
                    selected_variant: 0,
                };
                self.current_screen_view = ScreenView::Primary;
                self.current_experience = 0;
                self.current_achievement = 0;
                self.current_achievement_variant = 0;
            }
            if ui.button("Back").clicked() {
                self.current_screen_view = ScreenView::Primary;
                self.current_experience = 0;
                self.current_achievement = 0;
                self.current_achievement_variant = 0;
            }
        });
    }

    fn view_adding_achivement_variant(&mut self, ctx: &egui::Context) {
        // CentralPanel should always be last
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Adding Achivement Variant");
            self.add_achievement(ui, self.current_experience, self.current_achievement);
            if ui.button("Back").clicked() {
                self.current_screen_view = ScreenView::Primary;
                self.current_experience = 0;
                self.current_achievement = 0;
                self.current_achievement_variant = 0;
            }
        });
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
        
        match self.current_screen_view {
            ScreenView::Primary => self.view_primary(ctx),
            ScreenView::AddingAchievement => self.view_adding_achivement(ctx),
            ScreenView::AddingAchievementVariant => self.view_adding_achivement_variant(ctx),
        }
    }
}

fn show_resume_content(ui: &mut egui::Ui, e: &mut my_text::Experience) {
    // if we didn't select any achievements from this experience, do nothing
    let mut any_selected: bool = false;
    for a in &mut e.achievements {
        if a.in_resume {
            any_selected = true;
            break;
        }
    }
    if !any_selected {
        return;
    }
    ui.scope(|ui| {
        ui.style_mut().override_text_style = Some(egui::style::TextStyle::Heading);
        ui.label(&e.company);
        ui.style_mut().override_text_style = Some(egui::style::TextStyle::Body);
        ui.label(e.get_address());
        ui.label(format!("{} - {}", e.get_start(), e.get_end()));
        for a in &mut e.achievements {
            for v in &mut a.variants {
                if a.in_resume && v.id == a.selected_variant {
                    ui.label(&v.description);
                }
            }
        }
    });
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
