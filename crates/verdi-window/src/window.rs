use crate::winit_window;

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub internal_window: winit::window::Window,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        let window = winit_window::create_and_run();

        Self { internal_window: window, width: width, height: height }
    }

    pub fn get_id(&self) -> winit::window::WindowId {
        self.internal_window.id()
    }
}
