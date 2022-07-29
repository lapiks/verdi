use verdi_window::Window;
use verdi_renderer::Renderer;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub async fn run(&self) {
        Renderer::new(&self.window).await;
    }
}

impl Default for App {
    fn default() -> Self {
        let window = Window::new(1024, 768);

        Self { window: window }
    }
}