use egui_glium::{EguiGlium, egui_winit::egui};
use glium::{Frame, Display, glutin::event::WindowEvent};

use verdi_game::prelude::Game;

use crate::{
    code_editor::CodeEditor, 
    console::Console
};

pub struct Gui {
    egui_glium: EguiGlium,
    code_editor: CodeEditor,
    console: Console,
    show_console: bool,
}

impl Gui {
    pub fn new(egui_glium: EguiGlium) -> Self {
        Self {
            egui_glium,
            code_editor: CodeEditor::new(),
            console: Console::default(),
            show_console: true,
        }
    }

    pub fn init(&mut self) {
        self.console.init();
    }

    pub fn render(&mut self, display: &Display, target: &mut Frame, game: &mut Game) {
        self.egui_glium.run(display, |ctx| {
            egui::SidePanel::left("my_side_panel").show(ctx, |ui| {
                if ui.button("run").clicked() {
                    game.running = true;
                }
                if ui.button("stop").clicked() {
                    game.running = false;
                }
            });

            if self.show_console {
                self.console.show(ctx, &mut self.show_console, game);
            }
            else {
                //let mut open_editor = true;
                self.code_editor.show(ctx, &mut self.show_console,game);
            }
        });

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
    fn show(&mut self, ctx: &egui::Context, open: &mut bool, game: &mut Game);
}