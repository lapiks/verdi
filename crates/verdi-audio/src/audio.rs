use std::{rc::Rc, cell::RefCell, path::Path};

#[derive(Clone)]
pub struct AudioHandle {
    inner: Rc<RefCell<Audio>>,
}

impl AudioHandle {
    pub fn new_clip<P: AsRef<Path>>(&self, path: P) {
        self.inner.borrow().new_clip(path);
    }
}

pub struct Audio {

}

pub struct AudioClip {

}

impl Audio {
    pub fn new_clip<P: AsRef<Path>>(&self, path: P) -> AudioClip {
        AudioClip {

        }
    }
}