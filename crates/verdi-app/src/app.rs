use verdi_window::Window;

use futures::executor::block_on;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> App {
        let window = Window::new(1024, 768);

        Self { window: window }
    }

    pub fn run(&self) {
       verdi_window::run();
    }
}