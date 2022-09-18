use egui_glium::{EguiGlium, egui_winit::egui};
use glium::{Frame, Display, glutin::event::WindowEvent};

pub struct Gui {
    egui_glium: EguiGlium
}

impl Gui {
    pub fn new(egui_glium: EguiGlium) -> Self {
        Self {
            egui_glium
        }
    }

    pub fn run(&mut self, display: &Display) {
        self.egui_glium.run(display, |egui_ctx| {
            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                ui.label("Hello world!");

                if ui.button("click me").clicked() {

                }
            });
        });
    }

    pub fn render(&mut self, display: &Display, target: &mut Frame) {
        self.egui_glium.paint(&display, target);
    }

    pub fn on_event(&mut self, event: &WindowEvent) {
        self.egui_glium.on_event(event);
    }
}