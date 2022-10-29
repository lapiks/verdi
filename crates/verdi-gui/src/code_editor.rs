use crate::gui::GUIPanel;

pub struct CodeEditor {
    pub code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self { 
            code: "Salut".into(),
         }
    }
}

impl GUIPanel for CodeEditor {
    fn name(&self) -> &'static str {
        "Code Editor"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .default_height(800.0)
            .show(ctx, |ui| self.draw(ui));
    }
}

impl CodeEditor {
    fn draw(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.code_editor(&mut self.code);
        });
    }
}