use eframe::egui;

pub fn first_name_input(ui: &mut egui::Ui, input: &mut String) {
    ui.label("Введіть ваше ім'я:");
    ui.text_edit_singleline(input);
}