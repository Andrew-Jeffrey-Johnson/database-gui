
use chrono::Utc;
use chrono::TimeZone;

pub fn date(
    ui: &mut egui::Ui, 
    year: &mut i32, 
    month: &mut u32, 
    day: &mut u32,
    id: i32,
) 
{
    ui.label("Year:");
    let mut tmp_year = format!("{}", year);
    ui.text_edit_singleline(&mut tmp_year);
    if let Ok(result) = tmp_year.parse() {
        *year = result;
    }
    ui.push_id(id, |ui| {
        egui::ComboBox::from_label("Month")
            .selected_text(format!("{:?}", month))
            .show_ui(ui, |ui| {
                ui.selectable_value(month, 1, "January");
                ui.selectable_value(month, 2, "February");
                ui.selectable_value(month, 3, "March");
                ui.selectable_value(month, 4, "April");
                ui.selectable_value(month, 5, "May");
                ui.selectable_value(month, 6, "June");
                ui.selectable_value(month, 7, "July");
                ui.selectable_value(month, 8, "August");
                ui.selectable_value(month, 9, "September");
                ui.selectable_value(month, 10, "October");
                ui.selectable_value(month, 11, "November");
                ui.selectable_value(month, 12, "December");
            }
        );
    });
    ui.label("Day:");
    let mut tmp_day = format!("{}", day);
    ui.text_edit_singleline(&mut tmp_day);
    if let Ok(result) = tmp_day.parse() {
        *day = result;
    }
}

