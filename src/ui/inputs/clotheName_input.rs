use eframe::egui;

pub fn clothe_name_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть назву одягу:");
    ui.text_edit_singleline(input);
}