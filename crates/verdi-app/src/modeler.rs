use crate::{gui::GUIPanel, commands::Command, prelude::App};

pub struct Modeler;

impl Modeler {
    pub fn new() -> Self {
        Self {

        }
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> Option<Box<dyn Command>> {
        None
    }
}
impl GUIPanel for Modeler {
    fn name(&self) -> &'static str {
        "3D Modeler"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> Option<Box<dyn Command>> {
        let mut cmd = None;
        egui::CentralPanel::default().show(ctx, |ui| { 
            cmd = self.ui(ui);
        });

        cmd
    }
}