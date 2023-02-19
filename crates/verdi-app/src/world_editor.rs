use std::sync::{Arc, Mutex};

use verdi_graphics::prelude::{
    GraphicsChip, 
    Renderer, 
    DataBase,
};

pub struct WorldEditor {
    gpu: GraphicsChip,
    renderer: Renderer,
}

impl WorldEditor {
    pub fn new(db: Arc<Mutex<DataBase>>) -> Self {
        let gpu = GraphicsChip::new(db)
            .expect("World Editor GraphicsChip initialisation failed");

        Self {
            gpu,
            renderer: Renderer::new(),
        }
    }
}

