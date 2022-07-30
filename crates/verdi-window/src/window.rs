use winit::{
    event_loop::{EventLoop},
};

use verdi_renderer::Renderer;
use verdi_game;

use crate::winit_window;

pub struct Window {
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Window {
        Self { width, height }
    }
}

pub fn run() {
    let event_loop = EventLoop::new();
    let internal_window = winit_window::create(&event_loop);

    Renderer::new(&internal_window);

    verdi_game::run(event_loop, internal_window.id());
}
