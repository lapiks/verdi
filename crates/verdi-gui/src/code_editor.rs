use std::{rc::Rc, path::PathBuf, cell::RefCell, sync::{Mutex, Arc}};

use verdi_game::prelude::Scripts;
use verdi_input::prelude::*;

use crate::gui::GUIPanel;

pub struct CodeEditor {
    scripts: Rc<RefCell<Scripts>>,
    current_script: PathBuf,
}

impl CodeEditor {
    pub fn new(scripts: Rc<RefCell<Scripts>>) -> Self {
        Self { 
            scripts,
            current_script: PathBuf::new(),
         }
    }
}

impl GUIPanel for CodeEditor {
    fn name(&self) -> &'static str {
        "Code Editor"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_height(800.0)
            .show(ctx, |ui| self.draw(ui));
        // egui::CentralPanel::default().show(ctx, |ui| { 
        //     self.draw(ui);
        // });
    }
}

impl CodeEditor {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for script in self.scripts.borrow().get_scripts() {
                ui.selectable_value(
                    &mut self.current_script, 
                    script.0.to_path_buf(), 
                    script.0.to_str().unwrap()
                );
            }
        });
        
        ui.separator();

        egui::ScrollArea::both()
            .show(ui, |ui| {
                if let Some(script) = self.scripts.borrow_mut().get_script_mut(&self.current_script) {
                    ui.add(
                        egui::TextEdit::multiline(&mut script.code)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .frame(false) // to mask borders
                    );

                    if ui.input().modifiers.ctrl == true && ui.input().keys_down.get(&egui::Key::S).is_some() {
                        script
                            .save_at(&self.current_script)
                            .expect("Unable to save script");
                    }
                }
            });
    }
}