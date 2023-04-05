use crate::{
    gui::GUIPanel, 
    commands::Command, 
    app_commands::{
        Run, 
        Paused, 
        Stop, 
        ShowModeler, 
        ShowEditor
    }
};

pub struct Toolbar {
}

impl Toolbar {
    pub fn new() -> Self {
        Self { 

        }
    }
}

impl GUIPanel for Toolbar {
    fn name(&self) -> &'static str {
        "Toolbar"
    }

    fn show(&mut self, ctx: &egui::Context, _open: &mut bool) -> Option<Box<dyn Command>> {
        let mut command = None;
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            command = self.ui(ui);
        });

        command
    }
}

impl Toolbar {
    fn ui(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Command>> {
        let mut command: Option<Box<dyn Command>> = None;
        ui.horizontal_centered(|ui| {
            if ui.button("Run").clicked() {
                command = Some(
                    Box::new(
                        Run {}
                    )
                );
            }
            if ui.button("Pause").clicked() {
                command = Some(
                    Box::new(
                        Paused {}
                    )
                );
            }
            if ui.button("Stop").clicked() {
                command = Some(
                    Box::new(
                        Stop {}
                    )
                );
            }
            if ui.button("Editor").clicked() {
                command = Some(
                    Box::new(
                        ShowEditor {}
                    )
                );
            }
            ui.add_space(30.0);
            //if game_loaded {
                ui.label("FPS: ");
                //if let Some(game) = app.get_game() {
                    //ui.label(game.time_step.get_fps().to_string());
                //}
            //}
            if ui.button("3D Modeler").clicked() {
                command = Some(
                    Box::new(
                        ShowModeler {}
                    )
                );
            }
        });

        command
    }
}
