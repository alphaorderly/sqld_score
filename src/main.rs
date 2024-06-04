use std::borrow::Cow;

use eframe::{egui::{self, Color32, CtxRef, FontDefinitions, FontFamily, TextStyle}, epi};
use rust::request;

// Data struct for the app
struct MyApp {
    id: String,
    pw: String,
    label: String,
}

// Implement the App trait for the data struct
impl epi::App for MyApp {
    fn update(&mut self, ctx: &CtxRef, _: &mut eframe::epi::Frame<'_>) {    

        // Load the custom font
        let mut fonts = FontDefinitions::default();
        
        // Add the custom font
        fonts.font_data.insert(
            "my_font".to_owned(),
            Cow::Borrowed(include_bytes!("../fonts/NotoSansKR-Regular.ttf")),
        );

        fonts.fonts_for_family.insert(
            FontFamily::Proportional,
            vec![String::from("my_font")],
        );

        // Use the custom font for all text styles
        for text_style in [
            TextStyle::Heading,
            TextStyle::Body,
            TextStyle::Monospace,
            TextStyle::Button,
            TextStyle::Small,
        ] {
            fonts.family_and_size.insert(text_style, (FontFamily::Proportional, 20.0));
        }

        ctx.set_fonts(fonts);

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

            let visual = egui::Visuals::light();
            ctx.set_visuals(visual); // Set the theme (light or dark)

            ui.add(egui::Label::new("Dataq 아이디/비밀번호 입력").text_style(egui::TextStyle::Heading));

            ui.horizontal(|ui| {
                ui.label("아이디:");
                ui.text_edit_singleline(&mut self.id);
            });

            ui.horizontal(|ui| {
                ui.label("비밀번호:");
                ui.add(egui::TextEdit::singleline(&mut self.pw).password(true));
            });

            if ui.button("결과 확인하기").clicked() { 
                let session_id: String = request::login(&self.id, &self.pw);
                let tests = request::get_tests(&session_id);
                request::get_test_results(&session_id, &tests, &mut self.label)
            }

            ui.add(
                egui::Label::new(format!("{}", self.label)).background_color(Color32::WHITE),
            );
        });
    }

    fn name(&self) -> &str {
        "SQLD Score Checker"
    }
}


fn main() {
    let app = MyApp { id: "".to_string(), pw: "".to_string(), label: "".to_string()};
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
