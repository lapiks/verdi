use verdi_graphics::prelude::{GraphicsChip, Renderer};

pub struct WorldEditor {
    gpu: GraphicsChip,
    renderer: Renderer,
}

impl WorldEditor {
    pub fn new() -> Self {
        let gpu = GraphicsChip::new()
            .expect("World Editor GraphicsChip initialisation failed");

        Self {
            gpu,
            renderer: Renderer::new(),
        }
    }
}

