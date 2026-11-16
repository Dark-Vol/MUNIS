use eframe::egui;

pub struct VestaApp;

impl VestaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(
            crate::ui::theme::visuals::create_theme()
        );

        Self
    }
}

impl eframe::App for VestaApp {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VESTA");

            ui.label("Система учёта вещевого имущества");

            if ui.button("Добавить военнослужащего").clicked() {
                println!("Открыть форму");
            }
        });
    }
}