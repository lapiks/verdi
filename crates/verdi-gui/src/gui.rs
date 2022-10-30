use std::{rc::Rc, cell::RefCell};

use egui_glium::{EguiGlium, egui_winit::egui};
use glium::{Frame, Display, glutin::event::WindowEvent};
use verdi_game::prelude::Scripts;

use crate::code_editor::CodeEditor;

pub struct Gui {
    egui_glium: EguiGlium,
    code_editor: CodeEditor
}

impl Gui {
    pub fn new(egui_glium: EguiGlium, scripts: Rc<RefCell<Scripts>>) -> Self {
        Self {
            egui_glium,
            code_editor: CodeEditor::new(scripts),
        }
    }

    pub fn run(&mut self, display: &Display, fps: u32) {
        self.egui_glium.run(display, |ctx| {
            egui::SidePanel::left("my_side_panel").show(ctx, |ui| {
                ui.label("fps ");
                ui.label(fps.to_string());

                if ui.button("click me").clicked() {

                }
            });

            let mut open_editor = true;
            self.code_editor.show(ctx, &mut open_editor);
        });
    }

    pub fn render(&mut self, display: &Display, target: &mut Frame) {
        self.egui_glium.paint(&display, target);
    }

    pub fn on_event(&mut self, event: &WindowEvent) -> bool {
        self.egui_glium.on_event(event)
    }

    pub fn get_code_editor_mut(&mut self) -> &mut CodeEditor {
        &mut self.code_editor
    }
}

/// A panel of the GUI
pub trait GUIPanel {
    /// Panel's name
    fn name(&self) -> &'static str;

    /// Show the panel
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}