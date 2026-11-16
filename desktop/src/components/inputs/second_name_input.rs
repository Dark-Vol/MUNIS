use eframe::egui;

pub fn second_name_input(ui: &mut egui::Ui, value: &mut String) {
    ui.horizontal(|ui| {
        ui.label("Прізвище:");

        ui.text_edit_singleline(value);
    });
}