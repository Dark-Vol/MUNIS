use eframe::egui;

pub fn second_name_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть ваше прізвище:");
    ui.text_edit_singleline(input);
}