use egui::TextureId;

use crate::{
    gui::GUIPanel,
    commands::Command, 
};

pub struct Viewport {
    texture_id: Option<TextureId>,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            texture_id: None,
        }
    }

    pub fn set_texture(&mut self, texture_id: TextureId) {
        self.texture_id = Some(texture_id);
    }
}

impl GUIPanel for Viewport {
    fn name(&self) -> &'static str {
        "3D View"
    }

    fn show(&mut self, ctx: &egui::Context, _open: &mut bool) -> Option<Box<dyn Command>> {
        let mut command = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            command = self.ui(ui);
        });

        command
    }

    fn execute(&mut self, cmd: Box<dyn Command>) {

    }
}

impl Viewport {
    fn ui(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Command>> {
        if let Some(texture_id) = self.texture_id {
            ui.image(
                texture_id, 
                ui.available_size()
            );
        }

        None
    }
}
