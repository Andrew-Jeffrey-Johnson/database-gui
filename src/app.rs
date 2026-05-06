// Go here for inspiration
// https://www.egui.rs/#demo

use crate::postal_address::PostalAddress;
use crate::my_text;
use crate::my_database;

#[derive(PartialEq)]
enum ScreenView {
    Primary,
    Select,
    AddingExperience,
    AddingAchievement,
    AddingAchievementVariant,
    AddingPostalAddress,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_screen_view: ScreenView,
    #[serde(skip)] // This how you opt-out of serialization of a field
    is_diff_screen_view: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    postal_addresses: Vec<PostalAddress>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    screen_view_history: Vec<(ScreenView, i32)>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_experience: usize,
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_achievement: usize,
    #[serde(skip)] // This how you opt-out of serialization of a field
    current_achievement_variant: usize,
    #[serde(skip)] // This how you opt-out of serialization of a field
    experiences: Vec<my_text::Experience>,
    #[serde(skip)] // This how you opt-out of serialization of a field
    new_experience: my_text::Experience,
    #[serde(skip)] // This how you opt-out of serialization of a field
    new_achievement: my_text::Achievement,
    #[serde(skip)] // This how you opt-out of serialization of a field
    new_achievement_variant: my_text::AchievementVariant,
    #[serde(skip)] // This how you opt-out of serialization of a field
    new_postal_address: PostalAddress,
    description: String,
    summary: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    description_segments: Vec<Vec<my_text::LabelPkg>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            current_screen_view: ScreenView::Primary,
            is_diff_screen_view: false,
            postal_addresses: Vec::<PostalAddress>::new(),
            screen_view_history: vec![(ScreenView::Primary, 0)],
            current_experience: 0,
            current_achievement: 0,
            current_achievement_variant: 0,
            experiences: Vec::<my_text::Experience>::new(),
            new_experience: my_text::Experience {
                id: 0,
                start: chrono::offset::Utc::now(),
                end: chrono::offset::Utc::now(),
                company: String::from(""),
                title: String::from(""),
                address: PostalAddress {
                    id: 0,
                    name: String::from(""),
                    line_1: String::from(""),
                    line_2: String::from(""),
                    line_3: String::from(""),
                    city: String::from(""),
                    state: String::from(""),
                    zip_code: String::from(""),
                },
                achievements: Vec::<my_text::Achievement>::new(),
            },
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
            new_achievement_variant: my_text::AchievementVariant {
                id: 0,
                description: String::from(""),
                defense: String::from(""),
            },
            new_postal_address: PostalAddress {
                id: 0,
                name: String::from(""),
                line_1: String::from(""),
                line_2: String::from(""),
                line_3: String::from(""),
                city: String::from(""),
                state: String::from(""),
                zip_code: String::from(""),
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
        // Get information from database
        let ex = my_database::get_experience(0);
        match ex {
            Ok(e) => println!("ID:{}, Employer ID:{}, Title:{}", e.id, e.employing_entity_id, e.title),
            Err(v) => println!("Failed to Get Experience: {v:?}"),
        }

        println!("I am going to print all experiences in the database:");
        let exs = my_database::get_all_experiences();
        match exs {
            Ok(a) => for row in a {
                println!("ID:{}, Employer ID:{}, Title:{}", row.id, row.employing_entity_id, row.title);
            },
            Err(v) => println!("Failed to Get Experiences: {v:?}"),
        }
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
        let id = ui.next_auto_id().with(format!("{}{}{}", &a.short_description, e_index, a_index));
        let state = egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
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
                self.is_diff_screen_view = true;
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
                self.is_diff_screen_view = true;
                self.current_experience = e_index;
            }
            
        });
    }

    fn view_primary(&mut self, ctx: &egui::Context) {
        match self.screen_view_history.last() {
            Some((a, b)) if *a == ScreenView::Primary => (),
            _ => {
                self.screen_view_history.push((ScreenView::Primary, 0));
                println!("{}", self.screen_view_history.len());
            },
        };
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
                    if col_2.button("Add Experience").clicked() {
                        self.current_screen_view = ScreenView::AddingExperience;
                        self.is_diff_screen_view = true;
                    }
                });
                col_3.vertical(|col_3| {
                    for e in &mut self.experiences {
                        show_resume_content(col_3, e);
                    }
                    if col_3.button("Generate LaTex").clicked() {
                        let sum = String::from("This is a summary");
                        my_text::latex_gen(&sum, &self.experiences);
                    }
                });
            });
        });
    }

    fn view_select(&mut self, ctx: &egui::Context) {
        match self.screen_view_history.last() {
            Some((a, b)) if *a == ScreenView::Select => (),
            _ => {
                self.screen_view_history.push((ScreenView::Select, 0));
                println!("{}", self.screen_view_history.len());
            },
        };
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
                    if col_2.button("Add Experience").clicked() {
                        self.current_screen_view = ScreenView::AddingExperience;
                        self.is_diff_screen_view = true;
                    }
                });
                col_3.vertical(|col_3| {
                    for e in &mut self.experiences {
                        show_resume_content(col_3, e);
                    }
                    if col_3.button("Generate LaTex").clicked() {
                        let sum = String::from("This is a summary");
                        my_text::latex_gen(&sum, &self.experiences);
                    }
                });
            });
        });
    }

    fn view_adding_experience(&mut self, ctx: &egui::Context) {
        match self.screen_view_history.last() {
            Some((ScreenView::AddingExperience, _b)) => (),
            Some((ScreenView::AddingPostalAddress, _b)) => {
                ()
            },
            _ => {
                self.screen_view_history.push((ScreenView::AddingExperience, self.new_experience.id));
                println!("{}", self.screen_view_history.len());
            },
        };
        // CentralPanel should always be last
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Adding Experience");
            // Form to add experience
            ui.label("Title");
            ui.text_edit_singleline(&mut self.new_experience.title);
            ui.label("Postal Address");
            if ui.button("Select Postal Address").clicked() {
            }
            if ui.button("Confirm Add").clicked() {
                // Add it to the database
                let _id = my_database::create_experience(0, &self.new_experience.title, &self.new_experience.start, &self.new_experience.end, 0);
                // Add it to the cache
                self.experiences.push(self.new_experience.clone());
                // Reset
                self.is_diff_screen_view = true;
                self.current_screen_view = ScreenView::Primary;
                self.new_experience.id = 0;
                self.new_experience.start = chrono::offset::Utc::now();
                self.new_experience.end = chrono::offset::Utc::now();
                self.new_experience.company = String::from("");
                self.new_experience.title = String::from("");
                self.new_experience.address = PostalAddress {
                    id: 0,
                    name: String::from(""),
                    line_1: String::from(""),
                    line_2: String::from(""),
                    line_3: String::from(""),
                    city: String::from(""),
                    state: String::from(""),
                    zip_code: String::from(""),
                };
                self.new_experience.achievements = Vec::<my_text::Achievement>::new();
            }
            if ui.button("Add New Postal Address").clicked() {
                self.current_screen_view = ScreenView::AddingPostalAddress;
                self.is_diff_screen_view = true;
            }
            if ui.button("Back").clicked() {
                self.current_screen_view = ScreenView::Primary;
                self.is_diff_screen_view = true;
            }
        });
    }

    fn view_adding_postal_address(&mut self, ctx: &egui::Context) {
        // First Time Entering Screen View
        if self.is_diff_screen_view {
            self.is_diff_screen_view = false;
            self.postal_addresses = PostalAddress::fetch_many(0, 100);
            for address in &self.postal_addresses {
                println!("{}{}{}{}{}{}{}", address.name, address.line_1, address.line_2, address.line_3, address.city, address.state, address.zip_code);
            }
        }
        // CentralPanel should always be last
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Adding Postal Address");
            // Form to add experience
            ui.label("ID");
            ui.label(format!("{}", self.new_postal_address.id));
            ui.label("Name");
            ui.text_edit_singleline(&mut self.new_postal_address.name);
            ui.label("Line 1");
            ui.text_edit_singleline(&mut self.new_postal_address.line_1);
            ui.label("Line 2");
            ui.text_edit_singleline(&mut self.new_postal_address.line_2);
            ui.label("Line 3");
            ui.text_edit_singleline(&mut self.new_postal_address.line_3);
            ui.label("City");
            ui.text_edit_singleline(&mut self.new_postal_address.city);
            ui.label("State");
            ui.text_edit_singleline(&mut self.new_postal_address.state);
            ui.label("Zip Code");
            ui.text_edit_singleline(&mut self.new_postal_address.zip_code);
            if ui.button("Confirm Add").clicked() {
                self.is_diff_screen_view = true;
                // Add it to the database
                self.new_postal_address.insert_into_db();
                // Reset
                self.current_screen_view = ScreenView::Primary;
                self.new_postal_address.reset();
            }
            if ui.button("Back").clicked() {
                self.is_diff_screen_view = true;
                self.current_screen_view = match self.screen_view_history.pop() {
                    Some((a, _b)) => a,
                    _ => ScreenView::Primary,
                };
            }
            ui.label("All Current Postal Addresses");
            for address in &self.postal_addresses {
                if !address.name.is_empty() {ui.label(&address.name);};
                if !address.line_1.is_empty() {ui.label(&address.line_1);};
                if !address.line_2.is_empty() {ui.label(&address.line_2);};
                if !address.line_3.is_empty() {ui.label(&address.line_3);};
                ui.label(format!("{}, {} {}", address.city, address.state, address.zip_code));
            }
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
                // Add it to the database
                my_database::create_achivement(e.id, &self.new_achievement.short_description, &self.new_achievement.defense);
                // Add it to the cache
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
            // Show experience as heading
            let e = &mut self.experiences[self.current_experience];
            ui.style_mut().override_text_style = Some(egui::style::TextStyle::Heading);
            ui.label(&e.company);
            ui.style_mut().override_text_style = Some(egui::style::TextStyle::Body);
            ui.label(e.get_address());
            ui.label(format!("{} - {}", e.get_start(), e.get_end()));
            // Show achivement as heading
            let a = &mut e.achievements[self.current_achievement];
            ui.style_mut().override_text_style = Some(egui::style::TextStyle::Heading);
            ui.label(&a.short_description);
            ui.style_mut().override_text_style = Some(egui::style::TextStyle::Body);
            for v in &mut a.variants {
                ui.label(&v.description).on_hover_text(&v.defense);
            }
            // Form to add achievement
            ui.label("Variant");
            ui.label("Description");
            ui.text_edit_singleline(&mut self.new_achievement_variant.description);
            ui.label("Defense");
            ui.text_edit_singleline(&mut self.new_achievement_variant.defense);
            if ui.button("Confirm Add").clicked() {
                self.new_achievement_variant.id = a.variants.len();
                a.selected_variant = self.new_achievement_variant.id;
                a.variants.push(self.new_achievement_variant.clone());
                self.new_achievement_variant = my_text::AchievementVariant {
                    id: 0,
                    description: String::from(""),
                    defense: String::from(""),
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
            ScreenView::AddingExperience => self.view_adding_experience(ctx),
            ScreenView::AddingAchievement => self.view_adding_achivement(ctx),
            ScreenView::AddingAchievementVariant => self.view_adding_achivement_variant(ctx),
            ScreenView::AddingPostalAddress => self.view_adding_postal_address(ctx),
            ScreenView::Select => self.view_select(ctx),
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
