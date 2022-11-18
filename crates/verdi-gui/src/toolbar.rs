use verdi_game::prelude::{
    Game
};

use crate::gui::GUIPanel;

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

    fn show(&mut self, ctx: &egui::Context, _open: &mut bool, game: &mut Game) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.draw(ui, game);
        });
    }
}

impl Toolbar {
    fn draw(&mut self, ui: &mut egui::Ui, game: &mut Game) {
        ui.horizontal_centered(|ui| {
            if ui.button("Run").clicked() {
                game.running = true;
            }
            if ui.button("Stop").clicked() {
                game.running = false;
            }
        });
    }
}
