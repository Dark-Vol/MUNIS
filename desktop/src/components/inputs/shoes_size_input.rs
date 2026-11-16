use eframe::egui;

pub fn shoes_size_input(ui: &mut egui::Ui, input: &mut String) {
    let sizes = [
        "35", "36", "37", "38", "39", 
        "40", "41", "42", "43", "44", "45",
    ];

    ui.label("Виберіть розмір взуття:");

    egui::ComboBox::from_id_salt("shoe_size")
        .selected_text(if input.is_empty() {
            "Оберіть розмір"
        } else {
            input.as_str()
        })
        .show_ui(ui, |ui| {
            for size in sizes {
                ui.selectable_value(input, size.to_string(), size);
            }
        });
}