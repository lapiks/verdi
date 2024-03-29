use std::{path::PathBuf, rc::Rc, cell::RefCell};

use verdi_system::prelude::Scripts;

use crate::{
    //gui::GUIPanel, 
    commands::Command
};

pub struct CodeEditor {
    current_script: PathBuf,
    scripts: Option<Rc<RefCell<Scripts>>>,
}

impl CodeEditor {
    pub fn new() -> Self {
        Self { 
            current_script: PathBuf::new(),
            scripts: None
         }
    }
}

// impl GUIPanel for CodeEditor {
//     fn name(&self) -> &'static str {
//         "Code Editor"
//     }

//     fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> Option<Box<dyn Command>> {
//         egui::Window::new(self.name())
//             //.open(open)
//             .default_height(800.0)
//             .show(ctx, |ui| { 
//                 if ui.input().key_pressed(egui::Key::Escape) {
//                     *open = true;
//                 }
//                 return self.ui(ui); 
//             }
//         );
//         // egui::CentralPanel::default().show(ctx, |ui| { 
//         //     self.draw(ui);
//         // });

//         None
//     }
// }

impl CodeEditor {
    pub fn set_scripts(&mut self, scripts: Rc<RefCell<Scripts>>) {
        self.scripts = Some(scripts);
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Command>> {
        // script tabs
        ui.horizontal(|ui| {
            if let Some(scripts) = self.scripts.as_ref() {
                for script in scripts.borrow().get_scripts() {
                    ui.selectable_value(
                        &mut self.current_script, 
                        script.0.to_path_buf(), 
                        script.0.to_str().unwrap()
                    );
                }
            }
        });
        
        ui.separator();

        // code
        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                crate::syntax_highlighting::highlight(ui.ctx(), string);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };

        egui::ScrollArea::both()
            .show(ui, |ui| {
                if let Some(scripts) = &self.scripts {
                    if let Some(script) = scripts.borrow_mut().get_script_mut(&self.current_script) {
                        ui.add(
                            egui::TextEdit::multiline(&mut script.code)
                                .font(egui::TextStyle::Monospace) // for cursor height
                                .code_editor()
                                .lock_focus(true)
                                .desired_width(f32::INFINITY)
                                .frame(false) // to mask borders
                                .layouter(&mut layouter)
                        );
    
                        if ui.input().modifiers.ctrl == true && ui.input().key_pressed(egui::Key::S) {
                            script
                                .save_at(&self.current_script)
                                .expect("Unable to save script");
                        }
                    }
                }
            });

            None
    }
}

