use eframe::egui;

pub fn clothe_size_upper_input(ui: &mut egui::Ui, input: &mut String) {
    let sizes = [
        "XS", "S", "M", "L", "XL", "XXL", "XXXL",
        "34", "36", "38", "40", "42", "44",
    ];

    ui.label("Виберіть розмір одягу:");

    egui::ComboBox::from_id_salt("clothe_size_upper")
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