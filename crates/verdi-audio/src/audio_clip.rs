use std::path::Path;

use mlua::UserData;
use slotmap::new_key_type;

new_key_type! {
    pub struct AudioClipId;
}

pub struct AudioClip {

}

impl AudioClip {
    pub fn new<P: AsRef<Path>>(path: P) -> AudioClip {
        AudioClip {  }
    }
}

pub struct AudioClipHandle {
    id: AudioClipId,
}

impl AudioClipHandle {
    pub fn new(id: AudioClipId) -> Self {
        Self {
            id,
        }
    }
}

impl UserData for AudioClipHandle {}