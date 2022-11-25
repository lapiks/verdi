use egui_glium::EguiGlium;
use glium::{Frame, Display, glutin::event::WindowEvent};

use crate::{
    code_editor::CodeEditor, 
    console::Console, 
    toolbar::Toolbar, 
    app::App
};

pub struct Gui {
    egui_glium: EguiGlium,
    code_editor: CodeEditor,
    console: Console,
    show_console: bool,
    toolbar: Toolbar,
}

impl Gui {
    pub fn new(egui_glium: EguiGlium) -> Self {
        Self {
            egui_glium,
            code_editor: CodeEditor::new(),
            console: Console::default(),
            show_console: true,
            toolbar: Toolbar::new()
        }
    }

    pub fn init(&mut self) {
        self.console.init();
    }

    pub fn render(&mut self, display: &Display, target: &mut Frame, app: &mut App) {
        self.egui_glium.run(display, |ctx| {
            if self.show_console {
                self.console.show(ctx, &mut self.show_console, app);
            }
            else {
                //let mut open_editor = true;
                self.code_editor.show(ctx, &mut self.show_console, app);
                
                let mut show_toolbar = true;
                self.toolbar.show(ctx, &mut show_toolbar, app);
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
    fn show(&mut self, ctx: &egui::Context, open: &mut bool, app: &mut App);
}