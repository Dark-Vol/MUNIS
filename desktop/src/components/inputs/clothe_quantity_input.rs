use eframe::egui;

pub fn clothe_quantity_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть кількість одягу:");
    ui.text_edit_singleline(input);
}