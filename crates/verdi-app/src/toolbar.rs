use verdi_game::prelude::GameState;

use crate::{
    gui::GUIPanel, 
    app::App
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

    fn show(&mut self, ctx: &egui::Context, _open: &mut bool, app: &mut App) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.draw(ui, app);
        });
    }
}

impl Toolbar {
    fn draw(&mut self, ui: &mut egui::Ui, app: &mut App) {
        ui.horizontal_centered(|ui| {
            if ui.button("Run").clicked() {
                if app.game_state == GameState::Paused {
                    app.game_state = GameState::Running;
                }
                else if app.game_state == GameState::Stopped {
                    app.game_state = GameState::Start;
                }
            }
            if ui.button("Pause").clicked() {
                app.game_state = GameState::Paused;
            }
            if ui.button("Stop").clicked() {
                app.game_state = GameState::Stopped;
            }
            ui.add_space(30.0);
            ui.label("FPS: ");
            if let Some(game) = app.get_game() {
                ui.label(game.time_step.get_fps().to_string());
            }
        });
    }
}
