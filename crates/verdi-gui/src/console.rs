use crate::gui::GUIPanel;

pub struct Console {
    current_text: String,
    previous_text: String,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            current_text: String::default(),
            previous_text: String::default(),
        }
    }
}

impl GUIPanel for Console {
    fn name(&self) -> &'static str {
        "Console"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::CentralPanel::default().show(ctx, |ui| { 
            if ui.input().key_pressed(egui::Key::Escape) {
                *open = false;
            }
            self.draw(ui);
        });
    }
}

impl Console {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.label("Verdi-0.1.0");
        ui.label("(C) 2022 JD Games");
        ui.label("Type HELP for help");
        ui.add_space(10.0);

        ui.label(&self.previous_text);

        egui::ScrollArea::both()
            .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("> ");
                        ui.add(
                            egui::TextEdit::multiline(&mut self.current_text)
                                .font(egui::TextStyle::Monospace) // for cursor height
                                .code_editor()
                                .lock_focus(true)
                                .cursor_at_end(true)
                                .desired_width(f32::INFINITY)
                                .frame(false) // to mask borders
                        );

                        if ui.input().key_pressed(egui::Key::Enter) {
                            // execute command
                            
                            let line = format!("{}{}", "> ", self.current_text);
                            self.previous_text += &line;
                            self.current_text.clear();
                        }
                    });
                    
                }
            );
    }
}