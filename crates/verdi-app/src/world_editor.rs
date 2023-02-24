use std::{rc::Rc, cell::RefCell};

use verdi_graphics::prelude::{
    GraphicsChip, 
    Renderer, 
    DataBase, 
    Globals,
};

pub struct WorldEditor {
    gpu: GraphicsChip,
    renderer: Renderer,
}

impl WorldEditor {
    pub fn new(db: Rc<RefCell<DataBase>>, globals: Rc<Globals>) -> Self {
        let gpu = GraphicsChip::new(db, globals)
            .expect("World Editor GraphicsChip initialisation failed");

        Self {
            gpu,
            renderer: Renderer::new(),
        }
    }
}

