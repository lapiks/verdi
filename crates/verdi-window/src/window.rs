use winit::{
    event_loop::{EventLoop},
    window::WindowBuilder,
};

pub struct Window {
    pub width: u32,
    pub height: u32,
    pub internal_window: winit::window::Window
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>, width: u32, height: u32) -> Window {
        let internal_window = WindowBuilder::new().build(event_loop).unwrap();

        Self { width, height, internal_window }
    }
}
