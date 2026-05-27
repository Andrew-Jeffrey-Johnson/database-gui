
use chrono::TimeZone;
use std::cmp;
use chrono::Utc;
use std::collections::HashMap;
use crate::experience::Experience;
use crate::achievement::Achievement;
use crate::employing_entity::EmployingEntity;
use crate::postal_address::PostalAddress;
use crate::app::Screen;

fn date(
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

fn select_or_add_postal_address
(
    ui: &mut egui::Ui,
    query: &mut HashMap<i32, PostalAddress>,
    selected_address: &mut i32,
    new_postal_address: &mut PostalAddress,
) 
{
    // Put a bounding box around all ui elements
    egui::Frame::default().stroke(egui::Stroke::new(1.0_f32, egui::Color32::BLACK)).show(ui, |ui| {
        // If we already selected an address
        if *selected_address != 0 {
            if let Some(addr) = query.get(selected_address) {
                ui.label(&addr.name);
                ui.label(&addr.line_1);
                ui.label(&addr.line_2);
                ui.label(&addr.line_3);
                ui.label(format!("{}, {} {}", addr.city, addr.state, addr.zip_code));
            }
            else {
                ui.label("Oops. Something went wrong.");
            }
            return;
        }
        ui.columns_const(|[col_1, col_2]| {
            col_1.vertical(|col_1| {
                col_1.label("Select Address");
                if query.is_empty() {
                    query.extend(PostalAddress::fetch(0, 100));
                }
                for (id, addr) in query {
                    col_1.radio_value(
                        selected_address, 
                        *id,
                        format!("[id: {}]", id)
                    );
                    col_1.label(format!("{}, {}, {}", addr.city, addr.state, addr.zip_code));
                }
            });
            col_2.vertical(|col_2| {
                col_2.label("Enter New Address");
                col_2.label("Name:");
                col_2.text_edit_singleline(&mut new_postal_address.name);
                col_2.label("Line 1:");
                col_2.text_edit_singleline(&mut new_postal_address.line_1);
                col_2.label("Line 2:");
                col_2.text_edit_singleline(&mut new_postal_address.line_2);
                col_2.label("Line 3:");
                col_2.text_edit_singleline(&mut new_postal_address.line_3);
                col_2.label("City:");
                col_2.text_edit_singleline(&mut new_postal_address.city);
                col_2.label("State:");
                col_2.text_edit_singleline(&mut new_postal_address.state);
                col_2.label("Zip Code:");
                col_2.text_edit_singleline(&mut new_postal_address.zip_code);
                if col_2.button("Confirm Add").clicked() {
                    // Insert into databse
                    new_postal_address.insert_into_db();
                    *selected_address = new_postal_address.id;
                }
            });
        });
    });
}

fn select_or_add_employing_entity
(
    ui: &mut egui::Ui,
    query: &mut HashMap<i32, EmployingEntity>,
    selected_employing_entity: &mut i32,
    new_employing_entity: &mut EmployingEntity,
    addr_query: &mut HashMap<i32, PostalAddress>,
    address: &mut PostalAddress,
) 
{
    // If we already selected an employing entity
    if *selected_employing_entity != 0 {
        if let Some(ee) = query.get(selected_employing_entity) {
            ui.label(&ee.name);
            ui.label(&ee.url);
            if let Some(a) = addr_query.get(&ee.headquarters_postal_address_id) {
                ui.label(format!("{}, {}, {}", a.city, a.state, a.zip_code));
            }
        }
        else {
            ui.label("Oops. Something went wrong.");
        }
        if ui.button("Change EmployingEntity").clicked() {
            *selected_employing_entity = 0;
        }
        return;
    }
    ui.columns_const(|[col_1, col_2]| {
        col_1.vertical(|col_1| {
            col_1.label("Select Employing Entity");
            if query.is_empty() {
                query.extend(EmployingEntity::fetch(0, 100));
            }
            for (id, ee) in &mut *query {
                col_1.radio_value(
                    selected_employing_entity, 
                    *id,
                    format!("[id: {}]", id)
                );
                col_1.label(format!("{}, {}", ee.name, ee.url));
                if let Some(a) = addr_query.get(&ee.headquarters_postal_address_id) {
                    col_1.label(format!("{}, {}, {}", a.city, a.state, a.zip_code));
                }
            }
        });
        col_2.vertical(|col_2| {
            col_2.label("Enter New Employing Entity");
            col_2.label("Name:");
            col_2.text_edit_singleline(&mut new_employing_entity.name);
            col_2.label("URL:");
            col_2.text_edit_singleline(&mut new_employing_entity.url);
            let mut ai = address.id;
            select_or_add_postal_address(col_2, addr_query, &mut ai, address);
            address.id = ai;
            if col_2.button("Confirm Add").clicked() {
                // Insert into databse
                new_employing_entity.insert_into_db();
                *selected_employing_entity = new_employing_entity.id;
                query.extend(EmployingEntity::fetch(0, 100));
            }
        });
    });
}

pub fn new_experience
(
    ui: &mut egui::Ui, 
    e: &mut Experience, 
    e_postal_address: &mut PostalAddress,
    start_year: &mut i32,
    start_month: &mut u32,
    start_day: &mut u32,
    end_year: &mut i32,
    end_month: &mut u32,
    end_day: &mut u32,
    ee: &mut EmployingEntity,
    ee_postal_address: &mut PostalAddress,
    ee_query: &mut HashMap<i32, EmployingEntity>,
    addr_query: &mut HashMap<i32, PostalAddress>,
)
    -> bool
{
    let mut is_done = false;
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label("New Experience");
        let mut eei = ee.id;
        egui::Frame::default().stroke(egui::Stroke::new(1.0_f32, egui::Color32::BLACK)).show(ui, |ui| {
            select_or_add_employing_entity(ui, ee_query, &mut eei, ee, addr_query, ee_postal_address);
        });
        ee.id = eei;
        ui.label("Experience Title");
        ui.text_edit_singleline(&mut e.title);
        ui.label("Start Date");
        date(ui, start_year, start_month, start_day, 0);
        ui.label("End Date");
        date(ui, end_year, end_month, end_day, 1);
        ui.label("Address of where you actually worked at");
        let mut epi = e_postal_address.id;
        select_or_add_postal_address(ui, addr_query, &mut epi, e_postal_address);
        e_postal_address.id = epi;
        if epi != 0 && ui.button("Change Address").clicked() {
            e_postal_address.id = 0;
        }
        if ui.button("Confirm Add").clicked() {
            e.employing_entity_id = ee.id;
            e.start_timestamptz = Utc.with_ymd_and_hms(
                    *start_year,
                    *start_month,
                    *start_day,
                    0,
                    0,
                    0
                ).unwrap();
            e.end_timestamptz = Utc.with_ymd_and_hms(
                    *end_year,
                    *end_month,
                    *end_day,
                    0,
                    0,
                    0
                ).unwrap();
            e.postal_address_id = e_postal_address.id;
            e.insert_into_db();
            is_done = true;
        }
        else if ui.button("Cancel").clicked() {
            is_done = true;
        }
        else {
            is_done = false;
        }
    });
    return is_done;
}
