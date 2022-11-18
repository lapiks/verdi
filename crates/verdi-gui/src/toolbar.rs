use std::{rc::Rc, path::PathBuf, cell::RefCell};

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
        egui::SidePanel::left("my_side_panel").show(ctx, |ui| {
            if ui.button("run").clicked() {
                game.running = true;
            }
            if ui.button("stop").clicked() {
                game.running = false;
            }
        });
    }
}

