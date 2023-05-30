mod readlines;

const PADDING: f32 = 5.0;
const CYAN: Color32 = Color32::from_rgb(106, 149, 137);

pub(crate) use std::{sync::mpsc::channel, thread};

use eframe::egui::{
    self, Color32, CtxRef, Hyperlink, ScrollArea, Separator, TopBottomPanel, Ui, Vec2, Visuals,
};
use eframe::{epi::App, run_native};

use newsapi::NewsApi;
use readlines::{NewsCardData, Readlines};

impl App for Readlines {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        let api_key = self.config.api_key.to_string();

        let (news_tx, news_rx) = channel();

        self.news_rx = Some(news_rx);

        thread::spawn(move || {
            if let Ok(response) = NewsApi::new(&api_key).fetch() {
                let response_articles = response.articles();
                for a in response_articles.iter() {
                    let news = NewsCardData {
                        title: a.title().to_string(),
                        url: a.url().to_string(),
                        desc: a.desc().map(|s| s.to_string()).unwrap_or("...".to_string()),
                    };

                    if let Err(e) = news_tx.send(news) {
                        tracing::error!("Error sending news data {}", e);
                    }
                }
            }
        });
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_initialized {
            self.render_config(ctx);
        }

        self.preload_articles();

        self.render_top_panel(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Readlines"
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("ReadLines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(25.0);
    ui.add(sep);
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.style_mut().visuals.hyperlink_color = CYAN;
        ui.vertical_centered(|ui| {
            ui.add(
                Hyperlink::new("https:://newsapi.org/")
                    .text("News Source")
                    .text_style(egui::TextStyle::Monospace),
            );
            ui.add(
                Hyperlink::new("https://egui.rs/")
                    .text("Graphics Framework")
                    .text_style(egui::TextStyle::Monospace),
            );
            ui.add(
                Hyperlink::new("https://codeberg.org/tilmanmixyz/readlines")
                    .text("tilmanmixyz/readlines")
                    .text_style(egui::TextStyle::Monospace),
            );
        });
    });
}

fn main() {
    tracing_subscriber::fmt::init();

    let native_options: eframe::NativeOptions = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(540.0, 960.0)),
        ..Default::default()
    };
    let app = Readlines::new();
    run_native(Box::new(app), native_options);
}
