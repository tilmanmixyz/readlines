const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(220, 215, 186);
const BLACK: Color32 = Color32::from_rgb(29, 32, 33);
const CYAN: Color32 = Color32::from_rgb(106, 149, 137);
const RED: Color32 = Color32::from_rgb(195, 64, 67);

use std::borrow::Cow;

use eframe::egui::{
    self, Button, Color32, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator,
    TopBottomPanel, Ui, CtxRef, Window, Response,
};
use serde::{Deserialize, Serialize};
struct NewsCardData {
    title: String,
    url: String,
    desc: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String,
}

pub struct Readlines {
    articles: Vec<NewsCardData>,
    pub config: ReadlinesConfig,
}

impl Readlines {
    pub fn new() -> Readlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            desc: format!("Lorem Ipsum {}", a),
            url: format!("https://example.org/{}", a),
        });

        let config: ReadlinesConfig = confy::load("readlines").unwrap_or_default();
        
        Readlines {
            articles: Vec::from_iter(iter),
            config,
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

    pub fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            // render title
            ui.add_space(PADDING);
            let title = format!("# {}", a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }

            // render description
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            // render hyperlinks to article
            ui.add_space(PADDING);
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::new(&a.url).text("Read more..."));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub fn render_top_panel(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.add_space(5.0);
            egui::menu::bar(ui, |ui| {
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                });
                // Controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));
                    // Closeing applicarion
                    if close_btn.clicked() {
                        frame.quit();
                    }
                    let refresh_btn = ui.add(Button::new("🔄").text_style(egui::TextStyle::Body));
                    let theme_btn = ui.add(
                        Button::new({
                            if self.config.dark_mode {
                                "🌞"
                            } else {
                                "🌙"
                            }
                        })
                        .text_style(egui::TextStyle::Body),
                    );

                    // Theme changer
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                        if let Err(e) = confy::store("readlines", ReadlinesConfig {
                            dark_mode: self.config.dark_mode,
                            api_key: self.config.api_key.to_string(),
                        }) {
                            tracing::error!("Saving app state failed {}", e);
                        }

                        tracing::error!("Theme set");
                    }
                });
            });
            ui.add_space(5.0);
        });
    }

    pub fn render_config(&mut self, ctx: &CtxRef) {
        Window::new("Configuration").show(ctx, |ui| {
            ui.label("Enter your https://newsapi.org API-Key here");
            let text_input: Response = ui.text_edit_singleline(&mut self.config.api_key);
            if text_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if let Err(e) = confy::store("readlines", ReadlinesConfig {
                    dark_mode: self.config.dark_mode,
                    api_key: self.config.api_key.to_string(),
                }) {
                    tracing::error!("Saving app state failed {}", e);
                }

                tracing::error!("api key set");
            }
            tracing::error!("{}", &self.config.api_key);
        });
    }
}

// impl ReadlinesConfig {
//     fn new() -> ReadlinesConfig {
//         ReadlinesConfig {
//             dark_mode: false,
//             api_key: String::new(),
//         }
//     }
// }

impl Default for ReadlinesConfig {
    fn default() -> Self {
        ReadlinesConfig {
            dark_mode: false,
            api_key: String::new(),
        }
    }
}
