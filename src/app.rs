mod my_database;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Clone)]
enum TokenType {
    Jargon,
    Other,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    description: String,
    formatted_description: Vec<(String, Option<String>, TokenType)>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            description: String::from("Begin typing"),
            formatted_description: Vec::<(String, Option<String>, TokenType)>::new(),
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
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            if ui.button("Send Description").clicked() {
                my_database::create_description(&self.description);
            }
            let horizontal_scroll: bool = false;
            let vertical_scroll: bool = true;

            egui::ScrollArea::new([horizontal_scroll,vertical_scroll])
                .id_salt("First")
                .auto_shrink(true)
                .max_height(200.0)
                .show(ui, |ui| {
                   ui.add_enabled(true, egui::TextEdit::multiline(&mut self.description)
                        .code_editor()
                        //.layouter(&mut layouter)
                        .desired_rows(10)
                        .desired_width(f32::INFINITY)
                        .frame(true)
                    );
                });
            
            ui.separator();

           // let lines = self.description.split('\n');
           // for line in lines {
           //     ui.horizontal(|ui| {
           //         ui.spacing_mut().item_spacing.x = 0.0;
           //         ui.label(line);
           //     });
           // }

            if ui.button("Format Description").clicked() {
                self.formatted_description = annotate_description(&self.description);
            }
            egui::ScrollArea::new([horizontal_scroll,vertical_scroll])
                .id_salt("Second")
                .auto_shrink(true)
                .max_height(500.0)
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        for (text, raw_tooltip, tt) in self.formatted_description.clone() {
                            if raw_tooltip.is_some() {
                                let formatted_token = egui::RichText::new(&text);
                                ui.label(formatted_token.color(egui::Color32::RED).underline()).on_hover_text(raw_tooltip.unwrap());
                            }
                            else {
                                let formatted_token = egui::RichText::new(&text);
                                ui.label(formatted_token.color(egui::Color32::BLACK));
                            }
                        }
                    });
                });

            //let _response = ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.description));
            //if response.changed() {
            //    // …
            //}

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
//https://regex101.com/r/lf8r4y/1
// The regex expression to group every word separate from punctuation and newlines 
// except spaces. Ignores case for the first group
//(?i:Cascade Life Alliance|organ procurement organization)|(\n)|(\b\w+\b)|([•])|([^ ])/gm
// without groups:
// /Cascade Life Alliance|organ procurement organization|\b\w+\b|[\n\r\v\f]|\S|[\t ]*/gmi
fn annotate_description(original_description: &str) -> Vec<(String, Option<String>, TokenType)> {
    // Create dictionary
    let mut dictionary = std::collections::HashMap::new();
    dictionary.insert(
        "ITG".to_string(),
        "Information Technology Group".to_string(),
    );
    dictionary.insert(
        "Information Technology Group".to_string(),
        "ITG".to_string(),
    );
    dictionary.insert(
        "OHSU".to_string(),
        "Oregon Health and Science University".to_string(),
    );
    dictionary.insert(
        "Oregon Health and Science University".to_string(),
        "OHSU".to_string(),
    );
    dictionary.insert(
        "CLA".to_string(),
        "Cascade Life Alliance".to_string(),
    );
    dictionary.insert(
        "Cascade Life Alliance".to_string(),
        "CLA".to_string(),
    );
    // Create regex expression
    let mut expr = String::from(r"/");
    // Add all keys 
    let itr = dictionary.clone().into_keys();
    for key in itr {
        expr.push_str(format!("{key}|").as_str());
    }
    expr.push_str(r"\b\w+\b|[\n\r\v\f]|\S|[\t ]*/gmi");
    // Compile regex
    let re = regex::Regex::new(expr.as_str()).unwrap();
    // Get all matches
    let it = re.captures_iter(original_description);
    // Make a vector containing labels for all matches 
    let mut labels: Vec<(String, Option<String>, TokenType)> = Vec::<(String, Option<String>, TokenType)>::new();
    for cap in it {
        let token: String = cap[0].to_string();
        let raw_tooltip: Option<String> = dictionary.get(&token).cloned();
        let label = ();
        if raw_tooltip.is_some() {
            labels.push((token, raw_tooltip, TokenType::Jargon))
        }
        else {
            labels.push((token, raw_tooltip, TokenType::Other))
        }
    }
    return labels;
}

fn word_formatting(ui: &mut egui::Ui, original_word: &str) {
    let formatted_word = egui::RichText::new(original_word);
    match original_word {
        "ITG" => ui.label(formatted_word.color(egui::Color32::RED).underline()).on_hover_text("Oregon Health and Science University"),
        "OHSU" => ui.label(formatted_word.color(egui::Color32::RED).underline()).on_hover_text("Oregon Health and Science University"),
        _ => ui.label(original_word) // Regular word
    };
}
