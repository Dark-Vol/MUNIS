use eframe::egui;

pub fn clothe_size_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть розмір одягу:");
    ui.text_edit_singleline(input);
}