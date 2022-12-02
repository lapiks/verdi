use egui_glium::EguiGlium;
use glium::{Frame, Display, glutin::event::WindowEvent};

use crate::{
    code_editor::CodeEditor, 
    console::Console, 
    toolbar::Toolbar, 
    app::App, 
    commands::Command, 
    modeler::Modeler
};

pub struct Gui {
    egui_glium: EguiGlium,
    code_editor: CodeEditor,
    console: Console,
    show_console: bool,
    toolbar: Toolbar,
    modeler: Modeler,
    show_modeler: bool,
}

impl Gui {
    pub fn new(egui_glium: EguiGlium) -> Self {
        Self {
            egui_glium,
            code_editor: CodeEditor::new(),
            console: Console::default(),
            show_console: true,
            toolbar: Toolbar::new(),
            modeler: Modeler::new(),
            show_modeler: false,
        }
    }

    pub fn init(&mut self) {
        self.console.init();
    }

    pub fn ui(&mut self, app: &App) -> Option<Box<dyn Command>> {
        let mut cmd: Option<Box<dyn Command>> = None;
        self.egui_glium.run(app.get_window().get_display(), |ctx| {
            if self.show_console {
                cmd = self.console.show(ctx, &mut self.show_console, app);
            }
            else if self.show_modeler {
                cmd = self.modeler.show(ctx, &mut self.show_modeler, app);
            }
            else {
                //let mut open_editor = true;
                if let Some(editor_cmd) = self.code_editor.show(ctx, &mut self.show_console, app) {
                    cmd = Some(editor_cmd);
                }

                let mut show_toolbar = true;
                if let Some(toolbar_cmd) = self.toolbar.show(ctx, &mut show_toolbar, app) {
                    cmd = Some(toolbar_cmd);
                }
            }
        });

        cmd
    }

    pub fn paint(&mut self, display: &Display, target: &mut Frame) {
        self.egui_glium.paint(display, target);
    }

    pub fn on_event(&mut self, event: &WindowEvent) -> bool {
        self.egui_glium.on_event(event)
    }
}

/// A panel of the GUI
pub trait GUIPanel {
    /// Panel's name
    fn name(&self) -> &'static str;

    /// Show the panel
    fn show(&mut self, ctx: &egui::Context, open: &mut bool, app: &App) -> Option<Box<dyn Command>>;
}