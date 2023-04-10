mod readlines;

use eframe::{
    egui::{self, ScrollArea, Vec2},
    epi::App,
    run_native,
};
use readlines::Readlines;


impl App for Readlines {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "Readlines"
    }
}


fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540.0, 960.0));
    let app = Readlines::new();
    run_native(Box::new(app), native_options);
}
