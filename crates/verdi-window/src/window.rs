use glium::{
    glutin::{
        event_loop::EventLoop, window::WindowBuilder, ContextBuilder
    },
    Display
};

pub struct Window {
    event_loop: Option<EventLoop<()>>,
    display: Display,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();
        
        Self {
            event_loop: Some(event_loop),
            display,
            width,
            height
        }
    }

    pub fn take_event_loop(&mut self) -> Option<EventLoop<()>> {
        self.event_loop.take()
    }

    pub fn get_display(&self) -> &Display {
        &self.display
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}