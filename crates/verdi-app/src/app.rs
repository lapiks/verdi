use verdi_window::Window;

pub struct Application {
    window: Window,
}

impl Application {
    pub fn new() -> Application{
        Application::default()
    }

    pub fn run(&self) {
        //self.window.run();
    }
}

impl Default for Application {
    fn default() -> Self {
        let window = Window::new();
        Self { window: window }
    }
}