use egui_glium::EguiGlium;
use glium::{Frame, Display, glutin::event::WindowEvent};

use crate::{
    code_editor::CodeEditor, 
    console::Console, 
    toolbar::Toolbar, 
    commands::Command, 
    modeler::Modeler, viewport::Viewport
};

pub struct Gui {
    egui_glium: EguiGlium,
    viewport: Viewport,
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
            viewport: Viewport::new(),
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

    pub fn ui(&mut self, display: &Display) -> Option<Box<dyn Command>> {
        let mut cmd: Option<Box<dyn Command>> = None;
        self.egui_glium.run(display, |ctx| {
            if self.show_console {
                cmd = self.console.show(ctx, &mut self.show_console);
            }
            else if self.show_modeler {
                cmd = self.modeler.show(ctx, &mut self.show_modeler);
            }
            else {
                //let mut open_viewport = true;
                //self.viewport.show(ctx, &mut open_viewport);

                //let mut open_editor = true;
                if let Some(editor_cmd) = self.code_editor.show(ctx, &mut self.show_console) {
                    cmd = Some(editor_cmd);
                }

                let mut show_toolbar = true;
                if let Some(toolbar_cmd) = self.toolbar.show(ctx, &mut show_toolbar) {
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
        self.egui_glium.on_event(event).consumed
    }

    pub fn get_egui_glium(&self) -> &EguiGlium {
        &self.egui_glium
    } 

    pub fn get_egui_glium_mut(&mut self) -> &mut EguiGlium {
        &mut self.egui_glium
    } 

    pub fn get_viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn get_viewport_mut(&mut self) -> &mut Viewport {
        &mut self.viewport
    }

    pub fn get_code_editor(&self) -> &CodeEditor {
        &self.code_editor
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
    /// Eventually returns a command of what happened 
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> Option<Box<dyn Command>>;

    /// Executes a command
    fn execute(&mut self, cmd: Box<dyn Command>) {}
}