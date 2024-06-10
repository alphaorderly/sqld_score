use eframe::{egui::{self, FontDefinitions, FontFamily}, App};
use egui::FontData;
use ::egui::RichText;
use rust::request;

// Data struct for the app
struct MyApp {
    id: String,
    pw: String,
    label: String,
}

// Implement the App trait for the data struct
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {    

        // Load the custom font
        let mut fonts = FontDefinitions::default();
        
        // Add the custom font
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("../fonts/NotoSansKR-Regular.ttf")),
        );

        fonts.families.insert(
            FontFamily::Proportional,
            vec![String::from("my_font")],
        );

        // Use the custom font for all text styles
        fonts.families.insert(FontFamily::Proportional, vec![String::from("my_font"); 4]);
        fonts.families.insert(FontFamily::Monospace, vec![String::from("my_font")]);

        ctx.set_fonts(fonts);

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

            let visual = egui::Visuals::light();
            ctx.set_visuals(visual); // Set the theme (light or dark)

            let main_label = RichText::new("SQLD 점수 확인하기").size(40.0).strong();

            ui.label(main_label);

            ui.horizontal(|ui| {

                ui.spacing_mut().item_spacing.y = 20.0;

                ui.vertical(|ui| {
                    ui.label("아이디");
                    ui.add_space(5.0);
                    ui.label("비밀번호");
                });

                ui.vertical(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.id));
                    ui.add(egui::TextEdit::singleline(&mut self.pw).password(true));
                });
            });

            ui.add_space(30.0);

            if ui.button("결과 확인하기").clicked() { 
                let session_id = request::login(&self.id, &self.pw);
                let session_id = match session_id {
                    Ok(session_id) => session_id,
                    Err(message) => {
                        self.label = message;
                        return;
                    },
                };

                let tests = request::get_tests(&session_id);
                let tests = match tests {
                    Ok(tests) => tests,
                    Err(message) => {
                        self.label = message;
                        return;
                    },
                };

                request::get_test_results(&session_id, &tests, &mut self.label)
            }

            ui.add(
                egui::Label::new(format!("{}", self.label)),
            );
        });
    }
}


fn app_creator() -> Box<dyn for<'a, 'b> FnOnce(&'a eframe::CreationContext<'b>) -> Box<dyn eframe::App> + 'static> {
    Box::new(|_| Box::new(MyApp { id: "".to_string(), pw: "".to_string(), label: "".to_string() }))
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("SQLD Score", native_options, app_creator());
}
