use std::{rc::Rc, cell::RefCell, path::Path};

use slotmap::SlotMap;

use crate::audio_clip::{AudioClipHandle, AudioClipId, AudioClip};

#[derive(Clone)]
pub struct AudioHandle {
    inner: Rc<RefCell<Audio>>,
}

impl AudioHandle {
    pub fn new_clip<P: AsRef<Path>>(&self, path: P) -> AudioClipHandle {
        self.inner.borrow_mut().new_clip(path)
    }
}

pub struct Audio {
    clips: SlotMap<AudioClipId, AudioClip>,
}

impl Audio {
    pub fn new_clip<P: AsRef<Path>>(&mut self, path: P) -> AudioClipHandle {
        let id = self.clips.insert(AudioClip {
            
        });

        AudioClipHandle::new(id)
    }
}