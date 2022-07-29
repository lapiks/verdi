use verdi_window::Window;
use verdi_renderer::Renderer;

use futures::executor::block_on;

pub struct App {
    window: Window,
    renderer: Renderer,
}

impl App {
    pub fn new() -> App {
        let window = Window::new(1024, 768);

        let renderer = block_on(create_renderer(&window));

        Self { window: window, renderer: renderer }
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }
}

async fn create_renderer(window: &Window) -> Renderer {
    return Renderer::new(window).await;
}