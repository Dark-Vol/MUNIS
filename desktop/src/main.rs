pub mod components;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Моє застосування",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    first_name: String,
    second_name: String,
    last_name: String,

    clothe_name: String,
    clothe_color: String,
    clothe_quantity: String,
    clothe_size_upper: String,
    clothe_size_lower: String,
    clothe_type: String,

    shoes_size: String,
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.heading("Облік майна військового у військовій частині");

            ui.label("Ласкаво просимо!");

            // Введення особистих даних
            components::inputs::first_name_input::first_name_input(
                ui,
                &mut self.first_name,
            );
            components::inputs::second_name_input::second_name_input(
                ui,
                &mut self.second_name,
            );
            components::inputs::last_name_input::last_name_input(
                ui,
                &mut self.last_name,
            );

            ui.label(format!("Введене ім'я: {}", self.first_name));
            ui.label(format!("Введене прізвище: {}", self.second_name));
            ui.label(format!("Введене по-батькові: {}", self.last_name));

            ui.separator();

            // Введення інформації про одяг

            components::inputs::clothe_name_input::clothe_name_input(
                ui,
                &mut self.clothe_name,
            );

            components::inputs::clothe_color_input::clothe_color_input(
                ui,
                &mut self.clothe_color,
            );

            components::inputs::clothe_quantity_input::clothe_quantity_input(
                ui,
                &mut self.clothe_quantity,
            );

            components::inputs::clothe_size_upper_input::clothe_size_upper_input(
                ui,
                &mut self.clothe_size_upper,
            );

            components::inputs::clothe_size_lower_input::clothe_size_lower_input(
                ui,
                &mut self.clothe_size_lower,
            );

            components::inputs::clothe_type_input::clothe_type_input(
                ui,
                &mut self.clothe_type,
            );

            components::inputs::shoes_size_input::shoes_size_input(
                ui,
                &mut self.shoes_size,
            );
        });
    }
}
