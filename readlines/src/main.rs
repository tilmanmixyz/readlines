use eframe::{
    egui::{self, FontDefinitions, FontFamily, ScrollArea, Vec2, Color32, Label, Layout, Hyperlink, Separator},
    epi::App,
    run_native,
};
use std::borrow::Cow;

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(220, 215, 186);
const CYAN: Color32 = Color32::from_rgb(106, 149, 137);

struct Readlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    url: String,
    desc: String,
}

impl App for Readlines {
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
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

impl Readlines {
    fn new() -> Readlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            desc: format!("Lorem Ipsum {}", a),
            url: format!("https://example.org/{}", a),
        });
        Readlines {
            articles: Vec::from_iter(iter),
        }
    }

    fn configure_fonts(&self, ctx: &egui::CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "Lato".to_string(),
            Cow::Borrowed(include_bytes!("../../assets/lato-v23-latin-regular.ttf")),
        );

        font_def
            .family_and_size
            .insert(egui::TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def
            .family_and_size
            .insert(egui::TextStyle::Body, (FontFamily::Proportional, 20.));

        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Lato".to_string());

        ctx.set_fonts(font_def);
    }

    fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {

            // render title
            ui.add_space(PADDING);
            let title = format!("# {}", a.title);
            ui.colored_label(WHITE, title);

            // render description
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);
            
            // render hyperlinks to article
            ui.add_space(PADDING);
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("Read more..."));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540.0, 960.0));
    let app = Readlines::new();
    run_native(Box::new(app), native_options);
}
