use eframe::egui;

pub fn clothe_color_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть колір одягу:");
    ui.text_edit_singleline(input);
}