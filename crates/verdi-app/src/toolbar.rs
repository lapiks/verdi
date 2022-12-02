use crate::{
    gui::GUIPanel, 
    app::App, 
    commands::Command, app_commands::{Run, Paused, Stop}
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

    fn show(&mut self, ctx: &egui::Context, _open: &mut bool, app: &App) -> Option<Box<dyn Command>> {
        let mut command: Option<Box<dyn Command>> = None;
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            command = self.ui(ui, app);
        });

        command
    }
}

impl Toolbar {
    fn ui(&mut self, ui: &mut egui::Ui, app: &App) -> Option<Box<dyn Command>> {
        let mut command: Option<Box<dyn Command>> = None;
        ui.horizontal_centered(|ui| {
            let game_loaded = app.get_game().is_some();

            if ui.button("Run").clicked() && game_loaded {
                command = Some(
                    Box::new(
                        Run {}
                    )
                );
            }
            if ui.button("Pause").clicked() && game_loaded{
                command = Some(
                    Box::new(
                        Paused {}
                    )
                );
            }
            if ui.button("Stop").clicked() && game_loaded {
                command = Some(
                    Box::new(
                        Stop {}
                    )
                );
            }
            ui.add_space(30.0);
            if game_loaded {
                ui.label("FPS: ");
                if let Some(game) = app.get_game() {
                    ui.label(game.time_step.get_fps().to_string());
                }
            }
        });

        command
    }
}
