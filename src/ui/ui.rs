use eframe;
use egui::{self, Ui, CentralPanel};
use base::data_class::data_class::*;
use std::rc::Rc;

struct UiTemplate {
    pub messages: MessagesList,
    pub input: String,
    pub user: Rc<User>,
}

impl UiTemplate {
    pub fn new(cc: &eframe::CreationContext<'_>, user: User) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert("my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../font/font.ttf")));

        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());

        fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        let user = Rc::new(user);

        let sample_message = Message::new(
            Rc::clone(&user), 
            Rc::new(User::new("2".to_string())),
            "你好".to_string()
        );

        Self { 
            messages: MessagesList::new(vec![sample_message]),
            // messages: MessagesList::new(Vec::new()),
            input: String::new(),
            user,
        }
    }
}

impl eframe::App for UiTemplate {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let mut to_send = Vec::new();
            ui.heading("Side-to-side Communication");
            ui.separator();
            ui.horizontal(|ui: &mut Ui| {
                ui.text_edit_singleline(&mut self.input);
                if ui.button("enter").clicked() && self.input != String::new() {
                    to_send.push(self.input.clone());
                    self.input = String::new();
                };
            });
            ui.columns(3, |columns| {
                columns[0].label("Time");
                columns[1].label("From");
                columns[2].label("Text");
                for i in self.messages.messages.iter() {
                    columns[0].label(format!("{}", i.time.format("%T %D")));
                    columns[1].label(&i.from.name);
                    columns[2].label(&i.text);
                }
            });

            for i in to_send.iter() {
                self.messages.push(Message::new(Rc::clone(&self.user), Rc::clone(&self.user), i.to_string()))
            }
        });
    }
}

pub mod stater {
    use eframe::{run_native, NativeOptions};
    use super::UiTemplate;
    use base::data_class::data_class::User;

    pub fn stater(user: User) {
        let options:NativeOptions = NativeOptions {..Default::default()};
        run_native(
            &user.name.clone(), options, 
            Box::new(|cc| Box::new(UiTemplate::new(cc, user)))
        ).unwrap();
    }
}