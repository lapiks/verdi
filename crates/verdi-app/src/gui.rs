use crate::{
    code_editor::CodeEditor, 
    console::Console, 
    toolbar::Toolbar, 
    commands::Command, 
    modeler::Modeler, viewport::Viewport
};

pub struct Gui {
    //egui_quad: egui_miniquad::EguiMq,
    viewport: Viewport,
    code_editor: CodeEditor,
    console: Console,
    show_console: bool,
    toolbar: Toolbar,
    modeler: Modeler,
    show_modeler: bool,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            //egui_quad,
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

//     pub fn ui(&mut self, display: &Display) -> Option<Box<dyn Command>> {
//         let mut cmd: Option<Box<dyn Command>> = None;
//         self.egui_glium.run(display, |ctx| {
//             if self.show_console {
//                 cmd = self.console.show(ctx, &mut self.show_console);
//             }
//             else if self.show_modeler {
//                 cmd = self.modeler.show(ctx, &mut self.show_modeler);
//             }
//             else {
//                 //let mut open_viewport = true;
//                 //self.viewport.show(ctx, &mut open_viewport);

//                 //let mut open_editor = true;
//                 if let Some(editor_cmd) = self.code_editor.show(ctx, &mut self.show_console) {
//                     cmd = Some(editor_cmd);
//                 }

//                 let mut show_toolbar = true;
//                 if let Some(toolbar_cmd) = self.toolbar.show(ctx, &mut show_toolbar) {
//                     cmd = Some(toolbar_cmd);
//                 }
//             }
//         });

//         cmd
//     }

//     pub fn paint(&mut self, display: &Display, target: &mut Frame) {
//         self.egui_glium.paint(display, target);
//     }

    // TODO: put this in a System trait
    pub fn on_mouse_move(&mut self, x: f32, y: f32) -> bool {
        false
    }

    pub fn on_mouse_wheel(&mut self, x: f32, y: f32)  -> bool {
        false
    }

    pub fn on_mouse_button_down(&mut self, button: miniquad::MouseButton, x: f32, y: f32)  -> bool {
        false
    }

    pub fn on_mouse_button_up(&mut self, button: miniquad::MouseButton, x: f32, y: f32)  -> bool {
        false
    }

    pub fn on_key_down(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods, repeat: bool) -> bool {
        false
    }

    pub fn on_key_up(&mut self, keycode: miniquad::KeyCode, keymods: miniquad::KeyMods) -> bool {
        false
    }

    // pub fn get_egui_miniquad(&self) -> &egui_miniquad::EguiMq {
    //     &self.egui_quad
    // } 

    // pub fn get_egui_miniquad_mut(&mut self) -> &mut egui_miniquad::EguiMq {
    //     &mut self.egui_quad
    // } 

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

// /// A panel of the GUI
// pub trait GUIPanel {
//     /// Panel's name
//     fn name(&self) -> &'static str;

//     /// Show the panel
//     /// Eventually returns a command of what happened 
//     fn show(&mut self, ctx: &egui::Context, open: &mut bool) -> Option<Box<dyn Command>>;

//     /// Executes a command
//     fn execute(&mut self, cmd: Box<dyn Command>) {}
// }