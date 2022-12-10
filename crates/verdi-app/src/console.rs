use std::collections::HashMap;

use crate::{
    gui::GUIPanel, 
    app::App, 
    commands::Command, 
    app_commands::{Help, Load, Shutdown},
};

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

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, _: &App) -> Option<Box<dyn Command>> {
        let mut cmd: Option<Box<dyn Command>> = None;
        egui::CentralPanel::default().show(ctx, |ui| { 
            if ui.input().key_pressed(egui::Key::Escape) {
                *open = false;
            }
            cmd = self.ui(ui);
        });

        cmd
    }
}

impl Console {
    pub fn init(&mut self) {
        self.add_command(Box::new(Help {}));
        self.add_command(Box::new(Load { folder: String::new() }));
        self.add_command(Box::new(Shutdown {}));
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Command>> {
        let mut cmd_res: Option<Box<dyn Command>> = None;

        ui.label("Verdi-0.1.0");
        ui.label("(C) 2022 JD Games");
        ui.label("Type HELP for help");
        ui.add_space(10.0);

        ui.label(&self.previous_text);

        egui::ScrollArea::both()
            .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("> ");
                        let text_edit = ui.add(
                            egui::TextEdit::multiline(&mut self.current_text)
                                .font(egui::TextStyle::Monospace) // for cursor height
                                .code_editor()
                                .lock_focus(true)
                                .cursor_at_end(true)
                                .desired_width(f32::INFINITY)
                                .frame(false) // to mask borders          
                        );

                        // set focus at the end of the text
                        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit.id) {
                            let ccursor = egui::text::CCursor::new(self.current_text.chars().count());
                            state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
                            state.store(ui.ctx(), text_edit.id);
                            ui.ctx().memory().request_focus(text_edit.id); // give focus back to the `TextEdit`.
                        }

                        if ui.input().key_pressed(egui::Key::Enter) {
                            let new_text = "> ".to_owned() + &self.current_text;
                            self.draw_text(&new_text);

                            if let Some(cmd) = self.make_command(self.current_text.clone()) {
                                cmd_res = Some(cmd);
                            }
                            else {
                                self.draw_text(&"Unknown command".to_owned());
                                self.new_line();
                            }
                            self.current_text.clear();
                        }
                    });
                    
                }
            );

            cmd_res
    }

    fn make_command(&mut self, str_cmd: String) -> Option<Box<dyn Command>> {
        let mut split = str_cmd.split_whitespace();

        let first_word = split
            .next()
            .unwrap_or("");

        let second_word = split
            .next()
            .unwrap_or("");

        if first_word.to_string() == Help::name() {
            self.draw_help();
        }
        
        if let Some(cmd) = self.commands.get(&first_word.to_string()) {
            if cmd.name() == Load::name() {
                return Some(Box::new(Load { folder: second_word.to_string() }));
            }
            else if cmd.name() == Shutdown::name() {
                return Some(Box::new(Shutdown {}));
            }
        }

        None
    }
    
    fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.commands.insert(cmd.name().to_string(), cmd);
    }

    fn draw_text(&mut self, text: &String) {
        self.previous_text += text;
    }

    fn new_line(&mut self) {
        self.previous_text += &"\n".to_owned();
    }

    fn add_space(&mut self) {
        self.previous_text += &" ".to_owned();
    }

    fn draw_help(&mut self) {
        let mut text = String::new();
        for command in self.commands.iter() {
            let line = command.1.name().to_string() + &" -> ".to_owned() + &command.1.desc().to_string();
            text += &line;
            text += &"\n".to_owned();
        }

        self.draw_text(&text);
    }
}