use eframe::{epi::App, run_native, egui};

struct Readlines;

impl App for Readlines {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Lorem Ipsum")
        });
    }

    fn name(&self) -> &str {
        "Readlines"
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let app = Readlines;
    run_native(Box::new(app), native_options);
}
