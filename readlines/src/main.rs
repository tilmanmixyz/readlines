mod readlines;

const PADDING: f32 = 5.0;
const CYAN: Color32 = Color32::from_rgb(106, 149, 137);

use eframe::{
    egui::{self, Color32, CtxRef, Hyperlink, ScrollArea, Separator, TopBottomPanel, Ui, Vec2, Visuals, Response},
    epi::App,
    run_native,
};
use newsapi::{NewsApi, NewsApiError, NewsApiResponse};
use readlines::{Readlines, NewsCardData};

fn fetch_articles(api_key: &str, articles: &mut Vec<NewsCardData>) {
    if let Ok(response) = NewsApi::new(api_key).fetch() {
        let response_articles =  response.articles();
        for a in response_articles.iter() {
            let news = NewsCardData {
                title: a.title().to_string(),
                url: a.url().to_string(),
                desc: a.desc().map(|s| s.to_string()).unwrap_or("...".to_string()),
            };
            articles.push(news);
        }
    }
}

impl App for Readlines {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        fetch_articles(&self.config.api_key, &mut self.articles);
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
        
        self.render_top_panel(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_news_cards(ui);
            });
            render_footer(ui, ctx);
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

fn render_footer(ui: &mut Ui, ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.style_mut().visuals.hyperlink_color = CYAN;
        ui.vertical_centered(|ui| {
            ui.add(Hyperlink::new("https:://newsapi.org/").text("News Source").text_style(egui::TextStyle::Monospace));
            ui.add(Hyperlink::new("https://egui.rs/").text("Graphics Framework").text_style(egui::TextStyle::Monospace));
            ui.add(
                Hyperlink::new("https://codeberg.org/tilmanmixyz/readlines")
                    .text("tilmanmixyz/readlines").text_style(egui::TextStyle::Monospace)
            );
        });
    });
}

fn main() {
    tracing_subscriber::fmt::init();

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540.0, 960.0));
    let app = Readlines::new();
    run_native(Box::new(app), native_options);
}
