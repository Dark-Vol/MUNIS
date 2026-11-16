use eframe::egui;

pub fn last_name_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть ваше по-батькові:");
    ui.text_edit_singleline(input);
}