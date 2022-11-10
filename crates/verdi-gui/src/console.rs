use std::collections::HashMap;

use crate::{
    gui::GUIPanel, 
    commands::{Command, Help, Load}
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("Unknown command")]
    UnknownCommand(),
}

pub struct Console {
    current_text: String,
    previous_text: String,
    commands: HashMap<String, Box<dyn Command>>,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            current_text: String::default(),
            previous_text: String::default(),
            commands: HashMap::default(),
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
    pub fn init(&mut self) {
        self.add_command(Box::new(Help {}));
        self.add_command(Box::new(Load {}));
    }

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
                            let new_text = "> ".to_owned() + &self.current_text;
                            self.draw_text(&new_text);
                            // execute command
                            if let Err(_) = self.execute(self.current_text.clone()) {
                                let err_msg = "Unknown command".to_owned() + &"\n".to_owned();
                                self.draw_text(&err_msg);
                            }
                            self.current_text.clear();
                        }
                    });
                    
                }
            );
    }

    fn execute(&mut self, str_cmd: String) -> Result<(), ConsoleError> {
        let first_word = str_cmd
            .split_whitespace()
            .next()
            .unwrap_or("");

        if let Some(cmd) = self.commands.get(&first_word.to_string()) {

        }
        else {
            return Err(ConsoleError::UnknownCommand());
        }

        Ok(())
    }
    
    fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.name().to_string(), cmd);
    }

    fn draw_text(&mut self, text: &String) {
        self.previous_text += text;
    }

    fn draw_help(&mut self) {
        //self.previous_text.
    }
}