use std::{rc::Rc, cell::RefCell};

use verdi_database::Assets;

pub struct Math {
    pub(crate) assets: Rc<RefCell<Assets>>,
} 

impl Math {
    pub fn new() -> Self {
        Self {
            assets: Rc::new(RefCell::new(Assets::new())),
        }
    }
}