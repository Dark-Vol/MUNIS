
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Моё приложение",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    counter: i32,
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

            if ui.button("Натисни мене").clicked() {
                self.counter += 1;
            }

            ui.label(format!("Нажатий: {}", self.counter));
        });
    }
}
