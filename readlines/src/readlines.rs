const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(220, 215, 186);
const CYAN: Color32 = Color32::from_rgb(106, 149, 137);

use std::borrow::Cow;

use eframe::{
    egui::{self, FontDefinitions, FontFamily,Color32, Label, Layout, Hyperlink, Separator, Ui, TopBottomPanel, Button},
};
struct NewsCardData {
    title: String,
    url: String,
    desc: String,
}

pub struct Readlines {
    articles: Vec<NewsCardData>,
}

impl Readlines {
    pub fn new() -> Readlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            desc: format!("Lorem Ipsum {}", a),
            url: format!("https://example.org/{}", a),
        });
        Readlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&self, ctx: &egui::CtxRef) {
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

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
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

    pub fn render_top_panel(&self, ctx: &egui::CtxRef) {
        TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.add_space(5.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                });
                // Controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let theme_btn = ui.add(Button::new("🌙").text_style(egui::TextStyle::Body));
                });
            });
            ui.add_space(5.0);
        });
    }
}
