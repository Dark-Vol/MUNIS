mod ui;

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
    // counter: i32,
    first_name: String,
    second_name: String,
    last_name: String,
    clothe_name: String,
    clothe_color: String,
    clothe_quantity: String,
    clothe_size: String,
    clothe_type: String,
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

            ui::inputs::secondName_input::second_name_input(
                ui,
                &mut self.second_name,
            );

            // if ui.button("Натисни мене").clicked() {
            //     self.counter += 1;
            // }

            // ui.label(format!("Натиснуто: {}", self.counter));
            ui.label(format!("Введене ім'я: {}", self.first_name));
            ui.label(format!("Введене прізвище: {}", self.second_name));
            ui.label(format!("Введене по-батькові: {}", self.last_name));

            //
            ui::inputs::clotheName_input::clothe_name_input(
                ui,
                &mut self.clothe_name,
            );
            ui::inputs::clotheColor_input::clothe_color_input(
                ui,
                &mut self.clothe_color,
            );
            ui::inputs::clotheQuantity_input::clothe_quantity_input(
                ui,
                &mut self.clothe_quantity,
            );
            ui::inputs::clotheSize_input::clothe_size_input(
                ui,
                &mut self.clothe_size,
            );
            ui::inputs::clotheType_input::clothe_type_input(
                ui,
                &mut self.clothe_type,
            );
        });
    }
}