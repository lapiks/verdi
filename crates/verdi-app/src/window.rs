use glium::{
    glutin::{
        event_loop::EventLoop, window::{WindowBuilder, Fullscreen}, ContextBuilder, self
    },
    Display
};

pub struct Window {
    width: u32,
    height: u32,
    event_loop: Option<EventLoop<()>>,
    display: Option<Display>,
}

impl Window {
    pub fn new(width: u32, height: u32, fullscreen: bool) -> Self {
        let event_loop = EventLoop::new();

        let wb = WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize {
                width,
                height,
            })
            .with_title("Verdi Engine")
            .with_fullscreen(
                match fullscreen {
                    true => Some(Fullscreen::Borderless(None)),
                    false => None,
                }
            );

        let cb = ContextBuilder::new()
            .with_depth_buffer(24);
            
        let display = Display::new(
            wb, 
            cb, 
            &event_loop).expect("Unable to create glium Display");
        
        Self {
            width,
            height,
            event_loop: Some(event_loop),
            display: Some(display),
        }
    }

    pub fn get_event_loop(&self) -> Option<&EventLoop<()>> {
        self.event_loop.as_ref()
    }

    pub fn take_event_loop(&mut self) -> Option<EventLoop<()>> {
        self.event_loop.take()
    }

    pub fn get_display(&self) -> &Option<Display> {
        &self.display
    }

    pub fn take_display(&mut self) -> Option<Display> {
        self.display.take()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}