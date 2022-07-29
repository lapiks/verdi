use verdi_window::Window;
use verdi_renderer::Renderer;

use futures::executor::block_on;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn run(&self) {
        block_on(self.inner_run());
    }

    async fn inner_run(&self) {
        Renderer::new(&self.window).await;
    }
}

impl Default for App {
    fn default() -> Self {
        let window = Window::new(1024, 768);

        Self { window: window }
    }
}