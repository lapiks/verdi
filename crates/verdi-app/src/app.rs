use verdi_window::Window;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> App{
        App::default()
    }

    pub fn run(&self) {
        verdi_window::run();
    }
}

impl Default for App {
    fn default() -> Self {
        let window = Window::new();

        verdi_renderer::initialise(&window);

        Self { window: window }
    }
}