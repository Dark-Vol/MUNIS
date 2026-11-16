use eframe::egui;

pub fn clothe_type_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть тип одягу:");
    ui.text_edit_singleline(input);
}